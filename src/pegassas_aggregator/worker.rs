use std::time::{Duration, Instant};
use std::string::String;
use serde_json::{Deserializer, Value};

// struct ProbeData{
//     cell_voltage : Option<CellVoltage>
// }

// struct CellVoltage {
//     unit : String,
//     data : std::vec::Vec<CellVoltageData>
// }

// struct CellVoltageData {
//     id : u16,
//     voltage : u16
// }


struct ProbeUrl{
    ip_address: std::net::Ipv4Addr,
    port: u16,
    location: String,
}

struct ProbeWorker {
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
                    let stream_iter = stream.into_iter::<Value>();
                    for value in stream_iter {
                        match value {
                            Ok(v) => {
                                let influx_client = influx_db_client::Client::new(reqwest::Url::parse("http://localhost:8086").unwrap(), "pegassas");
                                let points = ProbeWorker::parse_value(v);
                                influx_client.write_points(points, Some(influx_db_client::Precision::Seconds), None).await;
                            },
                            Err(_) => ()
                        }
                    }
                },
                None => ()
            }
            


            // if let Some(r) = request_url {
            //     http_client.get(r).send().await;
            // }


            // let response : Option<std::future::Future<Output = Result<reqwest::Response, reqwest::Error>>> = match request_url{
            //     Ok(r) => Some(http_client.get(r).send()),
            //     Err(_) => {
            //         println!(format!("error parsing url: {}", request_url_string));
            //         None
            //     }
            // };
        }
    }

    fn parse_value(v: Value) -> influx_db_client::Points{
        let mut points_vector = std::vec::Vec::new();
        
        match v {
            Value::Object(map) => {
                    match map.get("cell"){
                        Some(v) => points_vector.append(&mut ProbeWorker::parse_cell(v)),
                        None => ()
                    };
                    match map.get ("battery"){
                        Some(v) => ProbeWorker::parse_battery(v),
                        None => ()   
                    }


                },
            _ => ()
        }
        influx_db_client::Points::create_new(points_vector)
    }


    fn parse_cell(cell: &Value) -> std::vec::Vec<influx_db_client::Point> {
        let mut points_vector = std::vec::Vec::new();
        match cell {
            Value::Object(cell_map) => {
                match cell_map.get("data"){
                    Some(data) =>{
                        match data {
                            Value::Array(array) => {
                                for data_object in array{
                                    match ProbeWorker::parse_cell_data(data_object){
                                        Some(point) => points_vector.push(point),
                                        None => ()
                                    };
                                }
                            }
                            _ => ()
                        }
                    },
                    None => ()
                }
            },
            _ => ()
        };
        points_vector
    }

    fn parse_battery(cell: &Value){
        match cell {
            Value::Object(cell_map) => {
                match cell_map.get("data"){
                    Some(data) =>{
                        match data {
                            Value::Array(array) => {
                                for data_object in array{
                                    ProbeWorker::parse_battery_data(data_object);
                                }
                            }
                            _ => ()
                        }
                    },
                    None => ()
                }
            },
            _ => ()
        }
    }

    fn parse_cell_data(data: &Value) -> Option<influx_db_client::Point>{
        use influx_db_client::{Point, point};
        use influx_db_client::Value as InfluxValue;
        let point = point!("cell");
        match data {
            Value::Object(data_map) => {
                let id_num = match data_map.get("id"){
                    Some(id) =>{
                        match id {
                            Value::Number(id_num) => {
                                id_num.as_i64()
                            },
                            _ => None
                        }
                    },
                    None => None
                };

                let battery_id_num = match data_map.get("battery_id"){
                    Some(battery_id) =>{
                        match battery_id {
                            Value::Number(battery_id_num) => {
                                battery_id_num.as_i64()
                            },
                            _ => None
                        }
                    },
                    None => None
                };

                let voltage_num = match data_map.get("voltage") {
                    Some(Value::Object(voltage_map)) =>{
                        match voltage_map.get("voltage") {
                            Some(Value::Number(voltage)) => {
                                voltage.as_i64()
                            },
                            _ => None
                        }
                    },
                    _ => None
                };

                let balance_current_num = match data_map.get("ballance_current") {
                    Some(Value::Object(ballance_current_map)) =>{
                        match ballance_current_map.get("current") {
                            Some(Value::Number(current)) => {
                                current.as_i64()
                            },
                            _ => None
                        }
                    },
                    _ => None
                };

                let cell_temp_num = match data_map.get("cell_temp") {
                    Some(Value::Object(cell_temp_map)) =>{
                        match cell_temp_map.get("temp") {
                            Some(Value::Number(temp)) => {
                                temp.as_i64()
                            },
                            _ => None
                        }
                    },
                    _ => None
                };

                match (id_num, battery_id_num, voltage_num, balance_current_num, cell_temp_num) {
                    (Some(id),Some(battery_id), Some(voltage), Some(balance_current), Some(cell_temp)) => {
                        Some(point.add_tag("id", id)
                                  .add_tag("battery_id", battery_id)
                                  .add_field("voltage", voltage)
                                  .add_field("current", balance_current)
                                  .add_field("temp", cell_temp))
                    },
                    _ => {
                        println!("Something has gone wrong");
                        None
                    }
                }
            },
            _ => None
        }
    }

    fn parse_battery_data(data: &Value){
        use influx_db_client::{Point, point};
        use influx_db_client::Value as InfluxValue;
        let point = point!("cell");
        match data {
            Value::Object(data_map) => {
                // let id_num = match data_map.get("id"){
                //     Some(id) =>{
                //         match id {
                //             Value::Number(id_num) => {
                //                 id_num.as_i64()
                //             },
                //             _ => None
                //         }
                //     },
                //     None => None
                // };

                let battery_current_num = match data_map.get("battery_current") {
                    Some(Value::Object(battery_current_map)) =>{
                        match battery_current_map.get("current") {
                            Some(Value::Number(current)) => {
                                current.as_i64()
                            },
                            _ => None
                        }
                    },
                    _ => None
                };

                let state_charge_num = match data_map.get("state_charge") {
                    Some(Value::Object(state_charge_map)) =>{
                        match state_charge_map.get("charge") {
                            Some(Value::Number(charge)) => {
                                charge.as_i64()
                            },
                            _ => None
                        }
                    },
                    _ => None
                };

                let charge_perc_num = match data_map.get("charge_perc") {
                    Some(Value::Object(charge_perc_map)) =>{
                        match charge_perc_map.get("charge") {
                            Some(Value::Number(charge)) => {
                                charge.as_i64()
                            },
                            _ => None
                        }
                    },
                    _ => None
                };

                let battery_temp_num = match data_map.get("battery_temp") {
                    Some(Value::Object(battery_temp_map)) =>{
                        match battery_temp_map.get("temp") {
                            Some(Value::Number(temp)) => {
                                temp.as_i64()
                            },
                            _ => None
                        }
                    },
                    _ => None
                };
            },
            _ => ()
        }
    }

    
}
