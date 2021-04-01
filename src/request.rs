use log::{debug, error, log_enabled, info, Level};
use tokio::runtime::Runtime;
/// Request data from probes

/// Read data url from ssdp client response
pub fn get_data_url(rt: &Runtime, response: &ssdp_client::SearchResponse) -> Option<String> {
    let http_client = reqwest::Client::new();
    // Request schema from probe
    let schema_tree = match reqwest::Url::parse(response.location()) {
        Ok(schema_url) => {
            match rt.block_on(http_client.get(schema_url).send()) {
                Ok(a) => {
                    match rt.block_on(a.text()) {
                        Ok(schema_text) => {
                            match xmltree::Element::parse(schema_text.as_bytes()){
                                Ok(xml) => Some(xml),
                                // TODO: Add error handling
                                Err(e) => {
                                    error!("{}", e);
                                    None
                                }
                            }
                        },
                        // TODO: Add error handling
                        Err(e) => {
                            error!("{}", e);
                            None
                        }
                    }
                },
                // TODO: Add error handling
                Err(e) => {
                    error!("{}", e);
                    None
                }
            }
        },
        // TODO: Add error handling
        Err(e) => {
            error!("{}", e);
            None
        }
    };

    match schema_tree {
        Some(ref st) => parse_presentation_url(st),
        None => None
    }
}

/// Extract data url from XML tree
fn parse_presentation_url(schema: &xmltree::Element) -> Option<String> {
    let base_url = match schema.get_child("URLBase") {
        Some(base) => match base.get_text() {
            Some(url) => (*url).to_string(),
            None => return None
        },
        None => return None
    };
    match schema.get_child("device") {
        Some(device) => {
            match device.get_child("presentationURL") {
                Some(presentation_url) => match presentation_url.get_text() {
                    Some(url) => Some(base_url + &*url),
                    None => None
                },
                None => None
            }
        },
        None => None
    }
}

/// Make data request from URL
pub fn make_request(url: &String, rt: &Runtime) -> Result<String, Box<dyn std::error::Error>> {
    let http_client = reqwest::Client::new();
    let request_url = match reqwest::Url::parse(url) {
        Ok(r) => r,
        Err(e) => {
            error!("parsing url: {} produced error: {}", url, e);
            return Err(Box::new(e));
        }
    };
    let response = match rt.block_on(http_client.get(request_url).send()){
        Ok(r) => r,
        Err(e) => {
            error!("{}", e);
            return Err(Box::new(e));
        }
    };

    let json_string = match rt.block_on(response.text()) {
        Ok(r) => r,
        Err(e) => {
            error!("{}", e);
            return Err(Box::new(e));
        }
    };

    return Ok(json_string.clone());
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_presentation_url() -> Result<(), xmltree::ParseError> {
        let case1 = xmltree::Element::parse("<root></root>".as_bytes())?;
        let case2 = xmltree::Element::parse("<root><notdevice><child1></child1><presentationURL>sometext</presentationURL></notdevice></root>".as_bytes())?;
        let case3 = xmltree::Element::parse("<root><device><child1></child1><child2>sometext</child2></device></root>".as_bytes())?;
        let case4 = xmltree::Element::parse("<root><device><child1></child1><presentationURL>sometext</presentationURL></device></root>".as_bytes())?;
        let case5 = xmltree::Element::parse("<root><URLBase>URLBasetext</URLBase><device><child1></child1><presentationURL>sometext</presentationURL></device></root>".as_bytes())?;
        let text: Option<String> = Some("URLBasetextsometext".to_string());
        assert_eq!(None, parse_presentation_url(&case1));
        assert_eq!(None, parse_presentation_url(&case2));
        assert_eq!(None, parse_presentation_url(&case3));
        assert_eq!(None, parse_presentation_url(&case4));
        assert_eq!(text, parse_presentation_url(&case5));
        Ok(())
    }
}
