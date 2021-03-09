use std::time::{Duration, Instant};
use std::string::String;
use serde_json::{Deserializer, Value};
use crate::pegassas_aggregator::parser::{Data, parse_data};

pub struct ProbeUrl{
    ip_address: std::net::Ipv4Addr,
    port: u16,
    location: String,
}

pub struct ProbeWorker {
    url : ProbeUrl,
    usn: String,
    ttl: Instant,
    request_interval: Duration
}

impl ProbeWorker {
    pub fn new(url: ProbeUrl, usn: String) -> ProbeWorker {
        let ttl = Instant::now() + Duration::from_secs(10);
        let request_interval = Duration::from_secs(30);
        ProbeWorker {
            url,
            usn,
            ttl,
            request_interval
        }
    }
    

    pub async fn run(&self){
        loop{
            let http_client = reqwest::Client::new();
            let mut request_url_string = String::from("http://");
            request_url_string += format!("{}", self.url.ip_address).as_str();
            request_url_string += ":";
            request_url_string += format!("{}", self.url.port).as_str();
            request_url_string += self.url.location.as_str();
            let request_url = reqwest::Url::parse(request_url_string.as_str());
            let request_url = match request_url {
                Ok(r) => Some(r),
                Err(_) => {
                    println!("error parsing url: {}", request_url_string);
                    None
                }
            };
            let tunnel = 

            let response = match request_url {
                Some(r) => Some(http_client.get(r).send().await),
                None => None
            };
            // TODO: add logging for posible error that includes details of the error
            let response = match response {
                Some(Ok(r)) => Some(r),
                _ => {
                    println!("error making request");
                    None
                }
            };

            let json_string = match response {
                Some(r) => match r.text().await {
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
                                influx_client.write_points(points, Some(influx_db_client::Precision::Seconds), None).await;
                            },
                            Err(_) => ()
                        }
                    }
                },
                None => ()
            }

        }
    }

    
}
