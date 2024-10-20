use anyhow::{Ok, Result};
use std::time::Duration;

use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client,
};

pub fn parse_headers(headers: Option<String>) -> Option<HeaderMap> {
    if headers.is_none() {
        return None;
    }

    // Split CLI header option by comma
    let headers: Vec<String> = headers
        .unwrap()
        .split(',')
        .map(|header| header.to_string())
        .collect();

    // Create new header map
    let mut header_map = HeaderMap::new();

    // Parse header string for key and value pairs and push to the HeaderMap
    for header in headers {
        let parts: Vec<&str> = header.splitn(2, ": ").collect();
        if parts.len() == 2 {
            header_map.insert(
                HeaderName::from_bytes(parts[0].as_bytes()).unwrap(),
                HeaderValue::from_str(parts[1]).unwrap(),
            );
        }
    }

    Some(header_map)
}

pub fn create_http_client(timeout: u64, headers: Option<HeaderMap>) -> Result<Client> {
    // HTTP timeout duration
    let http_timeout = Duration::from_secs(timeout);

    // Create new HTTP client and assign timeout duration
    let http_client = Client::builder().timeout(http_timeout);

    // If headers are provided, set HTTP client's default headers to them and return the client
    if headers.is_some() {
        let http_client = http_client.default_headers(headers.unwrap()).build()?;
        return Ok(http_client);
    }

    // Otherwise, build client without headers and return
    let http_client = http_client.build()?;

    Ok(http_client)
}
