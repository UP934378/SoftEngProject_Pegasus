mod pegassas_aggregator;
use pegassas_aggregator::discovery;
use pegassas_aggregator::worker::ProbeWorker;

use std::collections::hash_map::HashMap;
use tokio::runtime::Runtime;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

fn main() {
    let rt = Arc::new(match Runtime::new() {
        Ok(r) => r,
        Err(e) => {
            println!("Could not create tokio runtime: {:?}", e);
            return;
        }
    });

    let mut workers : HashMap<String, (Arc<ProbeWorker>, std::thread::JoinHandle<()>)> = HashMap::new();
    // let mut workers  = std::vec::Vec::new();
    let http_client = reqwest::Client::new();
    println!("Aggregator started");
    loop {
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
