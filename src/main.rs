mod discovery;
mod worker;
mod parser;
use worker::ProbeWorker;

use std::collections::hash_map::HashMap;
use tokio::runtime::Runtime;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use log::{debug, error, log_enabled, info, Level};



struct ProbeConfig{
    frequency: i32,
    should_collect: bool
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    env_logger::init();


    let rt = Arc::new(match Runtime::new(){
        Ok(runtime) => runtime,
        Err(e) => {
            error!("Failed to create tokio runtime: {:?}", e);
            return Err(Box::new(e));
        }
    });

    let mut workers : HashMap<String, (Arc<ProbeWorker>, std::thread::JoinHandle<()>)> = HashMap::new();
    let http_client = reqwest::Client::new();
    let host = "localhost";
    let user = "aggregator";
    let dbname = "pegassas";
    let mut postgres_client = match postgres::Client::connect(format!("host={} user={} dbname={}", host, user, dbname), postgres::NoTls){
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
    let mut probe_configs : HashMap<String, ProbeConfig> = HashMap::new();
    loop {
        if Instant::now() >= check_instant {
            let probe_configs_vec = match postgres_client.query("SELECT probe_id, request_interval FROM probe_config;", &[]) {
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
                // TODO: parse rows into probe_configs hashmap
            }
        }
        let start_discovery_instant = Instant::now();
        let mut responses = std::vec::Vec::new();
        match rt.block_on(discovery::discover_probes()) {
            Ok(mut v) => responses.append(&mut v),
            Err(e) => println!("{:?}", e)
        }
        for response in responses {
            let usn = response.usn().to_string();
            
            match workers.get(&usn) {
                Some((worker, _)) => {
                    // A worker already exists so update it
                    let w = worker.clone();
                    std::thread::spawn(move || w.update(response));
                },
                None => {
                    // No worker for this probe already exists
                    let worker = Arc::new(ProbeWorker::new(usn.clone(), rt.clone()));
                    let w = worker.clone();
                    let worker_join = std::thread::spawn(move || {
                            w.update(response);
                            w.run()
                    });
                    workers.insert(usn, (worker.clone(), worker_join));
                }
            };
        }
        match Duration::from_secs(20).checked_sub(start_discovery_instant.elapsed()) {
            Some(wait_duration) => std::thread::sleep(wait_duration),
            None => ()
        };
    }
    
}
