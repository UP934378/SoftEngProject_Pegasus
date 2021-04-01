//!Worker structure that handles connecting to a data 
//! probe and parsing the returned data and inserting 
//! it into the database.

use std::time::{Duration, Instant};
use std::string::String;
use serde_json::Deserializer;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use crate::parser::{Data, parse_data};
use crate::request::{get_data_url, make_request};
use log::{debug, error, warn, log_enabled, info, Level};

pub struct ProbeWorker {
    pub url : std::sync::Mutex<String>,
    usn: String,
    pub ttl: Mutex<Instant>,
    request_interval: Duration,
    pub should_work: Mutex<bool>,
    rt: Arc<Runtime>
}

impl ProbeWorker {

    /// Build a new Worker 
    pub fn new(usn: String, rt: Arc<Runtime>, request_interval: Duration) -> ProbeWorker {
        ProbeWorker {
            url: std::sync::Mutex::new("".to_string()),
            usn,
            ttl: Mutex::new(Instant::now() + (request_interval * 3)),
            request_interval,
            should_work: Mutex::new(true),
            rt
        }
    }

    fn check_should_work(&self) -> bool {
        match self.should_work.lock(){
            Ok(b) => *b,
            Err(e) => {
                error!("worker USN: {} could not lock should_work mutex, error: {}", self.usn, e);
                false
            }
        }
    }

    fn check_ttl(&self) -> bool {
        match self.ttl.lock(){
            Ok(ttl) => Instant::now() < *ttl,
            Err(e) => {
                error!("worker USN: {} could not lock ttl mutex, error: {}", self.usn, e);
                false
            }
        }
    }

    
    
    /// Update Worker informations
    pub fn update(&self, response: ssdp_client::SearchResponse) {
        match self.url.lock() {
            Ok(mut url) => *url = match get_data_url(&self.rt, &response){
                Some(s) => s,
                None => return
            },
            Err(e) => {
                error!("worker USN: {} could not lock url mutex, error: {}",self.usn, e);
            }
        }
        match self.ttl.lock() {
            Ok(mut ttl) => *ttl = Instant::now() + Duration::from_secs(60),
            Err(e) => ()
        }
    }
    
    

    pub fn run(&self) {
        debug!("Worker USN: {} started", self.usn);
        while self.check_should_work() && self.check_ttl() {
            debug!("Worker USN: {} top of loop", self.usn);
            let loop_start_instant = Instant::now();
            let url;
            match self.url.lock() {
                Ok(a) => {
                    url = a.deref().clone();
                },
                Err(e) => {
                    println!("Couldn't acquire url mutex: {:?}", e);
                    continue
                }
            };
            
            let json_string = make_request(&url, &self.rt);
            match json_string {
                Ok(json) => {
                    let stream = Deserializer::from_str(&json);
                    let stream_iter = stream.into_iter::<Data>();
                    for data in stream_iter {
                        match data {
                            Ok(d) => {
                                let influx_client = influx_db_client::Client::new(reqwest::Url::parse("http://localhost:8086").unwrap(), "pegassas");
                                let points = parse_data(d);
                                match self.rt.block_on(influx_client.write_points(points, Some(influx_db_client::Precision::Seconds), None)){
                                    Ok(_) => (),
                                    Err(e) => {
                                        error!("Writing to influx db failed with error: {}", e);
                                    }
                                }
                            },
                            Err(e) => {
                                error!("Parsing json response into Data struct failed with error: {}", e);
                            }
                        }
                    }
                },
                Err(e) => {
                    error!("Data request to URL: {} failed with error: {}", url, e);
                }
            }
            // Wait for discovery interval to expire
            match self.request_interval.checked_sub(loop_start_instant.elapsed()){
                Some(wait_duration) => {
                    std::thread::sleep(wait_duration);
                },
                None => {
                    warn!("worker for probe usn: {} falling behind request interval: {:?}", self.usn, self.request_interval);
                }
            };
        }
    }

    
}
