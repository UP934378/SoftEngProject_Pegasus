use ssdp_client::{URN, SearchTarget, search, Error, SearchResponse};
use futures::{stream::StreamExt};

/// SSDP service domain
const DOMAIN : &str = "pegassas";
/// SSDP service type
const TYPE : &str = "data-probe";
/// SSDP service Uniform Resource Name (URN)
const PROBE_URN : URN = URN::Service(std::borrow::Cow::Borrowed(DOMAIN),
                                        std::borrow::Cow::Borrowed(TYPE), 1);
/// SSDP search target
const ST : SearchTarget = SearchTarget::URN(PROBE_URN);

/// Finds all the data probes connected to the network in the same subnet
pub async fn discover_probes() -> Result<std::vec::Vec<SearchResponse>, Error>{
    let mut response_stream = search(&ST, std::time::Duration::from_secs(5), 4).await?;
    let mut responses = std::vec::Vec::new();
    while let Some(response) = response_stream.next().await{
        responses.push(response?);
    }
    Ok(responses)
}