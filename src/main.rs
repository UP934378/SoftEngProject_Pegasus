mod discovery;
mod worker;
mod parser;
mod request;
use worker::ProbeWorker;

use std::collections::hash_map::HashMap;
use tokio::runtime::Runtime;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use log::{debug, error, log_enabled, info, Level};
use eui48::MacAddress;
use regex::Regex;
use serde_json::Deserializer;
use std::convert::TryInto;

/// Frequency of the request
struct ProbeConfig{
    frequency: i32,
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    // Setup logging
    env_logger::init();

    // Setup tokio runtime
    let rt = Arc::new(match Runtime::new(){
        Ok(runtime) => runtime,
        Err(e) => {
            error!("Failed to create tokio runtime: {:?}", e);
            return Err(Box::new(e));
        }
    });
    let mac_regex_string = r"-[\dA-F]{12}";
    let mac_regex = match Regex::new(mac_regex_string) {
        Ok(r) => r,
        Err(e) => {
            error!("Compiling mac_regex: {} failed with error: {}", mac_regex_string, e);
            return Err(Box::new(e));
        }
    };

    let ip_regex_string = r"(\d{1,3}\.){3}\d{1,3}";
    let ip_regex = match Regex::new(ip_regex_string) {
        Ok(r) => r,
        Err(e) => {
            error!("Compiling ip_regex: {} failed with error: {}", ip_regex_string, e);
            return Err(Box::new(e));
        }
    };
    
    let mut workers : HashMap<String, (Arc<ProbeWorker>, std::thread::JoinHandle<()>)> = HashMap::new();
    let http_client = reqwest::Client::new();

    // Connect to postgreSQL database
    let host = "localhost";
    let user = "aggregator";
    let dbname = "pegassas";
    let password = "123";
    let mut postgres_client = match postgres::Client::connect(format!("host={} user={} dbname={} password={}", host, user, dbname, password).as_str(), postgres::NoTls){
        Ok(c) => {
            info!("Connection to postgres database successful. host: {}, user: {}, dbname: {}", host, user, dbname);
            c
        },
        Err(e) => {
            error!("Failed to connect to postgres database: {} at host: {} with user: {}", dbname, host, user);
            return Err(Box::new(e));
        }
    };
    info!("Aggregator started successfully");

    let mut check_instant = Instant::now();
    let mut probe_configs : HashMap<MacAddress, ProbeConfig> = HashMap::new();

    // Main loop
    loop {
        // Refresh probe_configs from database
        if Instant::now() >= check_instant {
            let probe_configs_vec = match postgres_client.query("SELECT mac_address, request_interval FROM probe_config;", &[]) {
                Ok(rows) => {
                    check_instant = Instant::now();
                    rows
                },
                Err(e) => {
                    error!("Failed to get probe configurations from database: {:?}", e);
                    Vec::new()
                }
            };
            for config in probe_configs_vec {
                let probe_id : MacAddress = match config.try_get("mac_address") {
                    Ok(id) => id,
                    Err(e) => {
                        error!("Failed to get mac_address from configuration response: {:?}", e);
                        continue;
                    }
                };

                let frequency : i32 = match config.try_get("request_interval") {
                    Ok(f) => f,
                    Err(e) => {
                        error!("Failed to get request_interval from configuration response: {:?}", e);
                        continue;
                    }
                };
                probe_configs.insert(probe_id, ProbeConfig{frequency});
            }
        }

        // Discover data probes
        let start_discovery_instant = Instant::now();
        let mut responses = std::vec::Vec::new();
        match rt.block_on(discovery::discover_probes()) {
            Ok(mut v) => responses.append(&mut v),
            Err(e) => error!("probe discovery failed with error: {:?}", e)
        };
        for response in responses {
            let macaddr_string = match mac_regex.find(response.usn()) {
                Some(mac_match) => {
                    // mac_match includes leading "-" so it is sliced off
                    String::from("0x") + &mac_match.as_str()[1..].to_string()
                },
                None => {
                    error!("Probe SSDP response USN: {} not formatted correctly", response.usn());
                    continue;
                }
            };
            let macaddr = match MacAddress::parse_str(&macaddr_string) {
                Ok(mac) => mac,
                Err(e) => {
                    error!("parsing MAC address from probe USN: {} yielded error: {}", response.usn(), e);
                    continue;
                }
            };

            match probe_configs.get(&macaddr){
                // If the frequnecy is 0, don't make a worker for it
                Some(ProbeConfig{frequency :0}) => (),
                Some(config) => {
                    let usn = response.usn().to_string();
                    let request_interval : u64 = match config.frequency.try_into(){
                        Ok(r) => r,
                        Err(e) => {
                            error!("Request interval for probe: {} produced error: {}", macaddr, e);
                            continue;
                        }
                    };
                    match workers.get(&usn) {
                        Some((worker, _)) => {
                            // A worker already exists so update it
                            let w = worker.clone();
                            std::thread::spawn(move || w.update(response, Some(Duration::from_secs(request_interval))));
                        },
                        None => {
                            // No worker for this probe already exists
                            
                            let worker = Arc::new(ProbeWorker::new(usn.clone(), rt.clone(), Duration::from_secs(request_interval)));
                            let w = worker.clone();
                            let worker_join = std::thread::spawn(move || {
                                    w.update(response, None);
                                    w.run()
                            });
                            workers.insert(usn, (worker.clone(), worker_join));
                        }
                    };
                },
                None => {
                    probe_configs.insert(macaddr, ProbeConfig{frequency: 0});
                    let ip_address = match ip_regex.find(response.location()){
                        Some(m) => m.as_str(),
                        None => {
                            error!("parsing IP address from porbe USN: {} location: {} failed", response.usn(), response.location());
                            continue;
                        }
                    };
                    postgres_client.execute(
                        "INSERT INTO probe_config (mac_address, ip_address, request_interval) VALUES ($1, $2, $3);",
                        &[&macaddr, &ip_address, &0]
                    );
                }
            }
        }
        
        // Wait until next loop
        match Duration::from_secs(20).checked_sub(start_discovery_instant.elapsed()) {
            Some(wait_duration) => std::thread::sleep(wait_duration),
            None => ()
        };
    }
    
}
