use std::time::{Duration, Instant};
use std::string::String;
use serde_json::Deserializer;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use crate::pegassas_aggregator::parser::{Data, parse_data};

pub struct ProbeWorker {
    pub url : std::sync::Mutex<String>,
    usn: String,
    pub ttl: Mutex<Instant>,
    request_interval: Duration,
    pub should_work: Mutex<bool>,
    rt: Arc<Runtime>
}

impl ProbeWorker {
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

    fn get_data_url(&self, response: ssdp_client::SearchResponse) -> Option<String> {
        let http_client = reqwest::Client::new();
        // Request schema from probe
        let schema_tree = match reqwest::Url::parse(response.location()) {
            Ok(schema_url) => {
                match self.rt.block_on(http_client.get(schema_url).send()) {
                    Ok(a) => {
                        match self.rt.block_on(a.text()) {
                            Ok(schema_text) => {
                                match xmltree::Element::parse(schema_text.as_bytes()){
                                    Ok(xml) => Some(xml),
                                    // TODO: Add error handling
                                    Err(e) => None
                                }
                            },
                            // TODO: Add error handling
                            Err(e) => None
                        }
                    },
                    // TODO: Add error handling
                    Err(e) => None
                }
            },
            // TODO: Add error handling
            Err(e) => None
        };

        match schema_tree {
            Some(ref st) => ProbeWorker::parse_presentation_url(st),
            None => None
        }
    }

    fn parse_presentation_url(schema: &xmltree::Element) -> Option<String> {
        match schema.get_child("device") {
            Some(device) => {
                match device.get_child("presentationURL") {
                    Some(presentation_url) => match presentation_url.get_text() {
                        Some(url) => Some(url.into_owned()),
                        None => None
                    },
                    None => None
                }
            },
            None => None
        }
    }
    

    pub fn update(&self, response: ssdp_client::SearchResponse) {
        match self.url.lock() {
            Ok(mut url) => *url = match self.get_data_url(response){
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
            let http_client = reqwest::Client::new();
            let request_url = reqwest::Url::parse(url.as_str());
            let request_url = match request_url {
                Ok(r) => Some(r),
                Err(_) => {
                    println!("error parsing url: {}", url);
                    None
                }
            };
            let response = match request_url {
                Some(r) => Some(self.rt.block_on(http_client.get(r).send())),
                None => None
            };
            // TODO: add logging for possible error that includes details of the error
            let response = match response {
                Some(Ok(r)) => Some(r),
                _ => {
                    println!("error making request");
                    None
                }
            };

            let json_string = match response {
                Some(r) => match self.rt.block_on(r.text()) {
                    Ok(r) => Some(r),
                    Err(e) => None
                }
                None => None
            };
            
            let json_stream = match json_string {
                Some(ref s) => Some(Deserializer::from_str(s.as_str())),
                None => None
            };

            match json_stream {
                Some(stream) => {
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
                None => ()
            }
            // Wait for discovery interval to expire
            match self.request_interval.checked_sub(loop_start_instant.elapsed()){
                Some(wait_duration) => {
                    std::thread::sleep(wait_duration);
                },
                None => ()
            };
        }
    }

    
}
