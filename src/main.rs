mod pegassas_aggregator;
use pegassas_aggregator::discovery;

use tokio;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    println!("{:?}", discovery::discover_probes().await);
    
}
