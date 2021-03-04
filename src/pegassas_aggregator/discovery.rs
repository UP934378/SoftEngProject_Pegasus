use ssdp_client::{URN, SearchTarget, search, Error, SearchResponse};
use futures_util::{stream::StreamExt};

const DOMAIN : &'static str = "pegassas";
const TYPE : &'static str = "data-probe";
const PROBE_URN : URN = URN::Service(std::borrow::Cow::Borrowed(DOMAIN),
                                        std::borrow::Cow::Borrowed(TYPE), 1);
const ST : SearchTarget = SearchTarget::URN(PROBE_URN);

pub async fn discover_probes() -> Result<std::vec::Vec<SearchResponse>, Error>{
    let mut response_stream = search(&SearchTarget::All, std::time::Duration::from_secs(5), 4).await?;
    let mut responses = std::vec::Vec::new();
    while let Some(response) = response_stream.next().await{
        responses.push(response?);
    }
    Ok(responses)
}