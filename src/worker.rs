use std::time::{Duration, Instant};
use std::string::String;
use serde_json::Deserializer;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use crate::parser::{Data, parse_data};
use crate::request::{get_data_url, make_request};
use log::{debug, error, warn, log_enabled, info, Level};

/**
 * Worker structure that handles connecting to a data probe
 * and parsing the returned data and inserting it into the
 * database.
 */
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
    pub fn new(usn: String, rt: Arc<Runtime>) -> ProbeWorker {
        ProbeWorker {
            url: std::sync::Mutex::new("".to_string()),
            usn,
            ttl: Mutex::new(Instant::now() + Duration::from_secs(60)),
            request_interval: Duration::from_secs(30),
            should_work: Mutex::new(true),
            rt
        }
    }

    fn check_should_work(&self) -> bool {
        match self.should_work.lock(){
            Ok(b) => *b,
            Err(_) => false
        }
    }

    fn check_ttl(&self) -> bool {
        match self.ttl.lock(){
            Ok(ttl) => Instant::now() < *ttl,
            Err(_) => false
        }
    }

    
    
    /// Update Worker informations
    pub fn update(&self, response: ssdp_client::SearchResponse) {
        match self.url.lock() {
            Ok(mut url) => *url = match get_data_url(&self.rt, &response){
                Some(s) => s,
                None => return
            },
            Err(e) => ()
        }
        match self.ttl.lock() {
            Ok(mut ttl) => *ttl = Instant::now() + Duration::from_secs(60),
            Err(e) => ()
        }
    }
    
    

    pub fn run(&self) {
        while self.check_should_work() && self.check_ttl() {
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
            println!("{:?}", url);
            
            let json_stream = make_request(&url, &self.rt);
            match json_stream {
                Ok(stream) => {
                    let stream_iter = stream.into_iter::<Data>();
                    for data in stream_iter {
                        match data {
                            Ok(d) => {
                                let influx_client = influx_db_client::Client::new(reqwest::Url::parse("http://localhost:8086").unwrap(), "pegassas");
                                let points = parse_data(d);
                                self.rt.block_on(influx_client.write_points(points, Some(influx_db_client::Precision::Seconds), None));
                            },
                            Err(_) => ()
                        }
                    }
                },
                Err(_) => ()
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
