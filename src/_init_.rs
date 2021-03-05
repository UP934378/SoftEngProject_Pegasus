struct Probe_URL{
    ip_address: std::net::IpAddr,
    port: u16,
    location: String,
}


struct Data_probe 
{
    
    usn: String,
    ttl: std::time::Instant,
    stop_event: stop_event,
    lock: lock,
    connnection_database: connnection_database, //Unsure if needed due to influx pointers?
}

impl data_probe {
    fn new(ip_address : String) -> data_probe {

    }
}

fn build_data_probe(ip_address: String, port: u16, location: String, usn: String, 
                    ttl: Int, stop_event: String, connnection_database: String) -> data_probe{
                        data_probe {
                            ip_address: ip_address,
                            port: port,
                            location: location,
                            usn: usn,
                            ttl: ttl,
                            stop_event: stop_event,
                            lock: lock,
                            connnection_database: connnection_database,
                        }
                    }
// Unsure if we need this?

fn build_array_data_probes(){
    let vecArryDataProbes: [data_probe;]
    for 
}

fn refresh (self, ip_address, port, location){
    with self.lock():
    self.ip_address == ip_address
    self.port == port
    self.location == location
self.ttl == time.time() + DATA_PROBE_TTL
}

fn is_expired(self){
return time.time() > self.ttl}

fn run(self){
        while not self.stop_event.isSet():
            try:
                with self.lock():
                    ip_address = self.ip_address
                    port = self.port
                    location = self.location
                if port == 80:
                    protocol = "http://"
                else:
                    protocol = "https://"
                schema_response = requests.get(protocol + ip_address + "/" + location) ##Location = URL
                if schema_response.ok:
                    schema = defusedxml.ElementTree.fromstring(schema_response.text)
                else:
                    self.stop_event.wait(timeout=WORKER_RETRY_PERIOD)
                    continue

                data_url = protocol + ip_address + "/" + schema[2][2].text ## Added through checking 
                data_response = requests.get(data_url)
                if data_response.ok and data_response.headers.get("Content-type", "").startswith("application/json"):
                    with print_lock:
                        print(usn, data_response.json()) ## USN - Service Identifier for SSDP
                    cur = self.conn.cursor()
                    for table, data in data_response.json().items():
                        columns = [column["name"] for column in data["columns"]]
                        query = psycopg2.sql.SQL("INSERT INTO {table} ({columns}) VALUES ({data})").format(
                            table=psycopg2.sql.Identifier(table),
                            columns=psycopg2.sql.SQL(", ").join(map(psycopg2.sql.Identifier, columns)),
                            data=psycopg2.sql.SQL(", ").join(psycopg2.sql.Placeholder() * len(columns))
                        )
                        for datapoint in data["values"]:
                            try:
                                cur.execute(query, datapoint)
                            except Exception as e:
                                self.conn.rollback()
                                with print_lock:
                                    print(e)
                                self.stop_event.wait(timeout=WORKER_RETRY_PERIOD)
                                continue
                    self.conn.commit()
                    self.stop_event.wait(timeout=WORKER_REQUEST_PERIOD)
                else:
                    self.stop_event.wait(timeout=WORKER_RETRY_PERIOD)
            except Exception as e:
                with print_lock:
                    print(e)
                self.stop_event.wait(timeout=WORKER_RETRY_PERIOD)
        self.conn.close()
}

