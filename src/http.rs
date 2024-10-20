use anyhow::{Ok, Result};
use std::time::Duration;

use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client,
};

// TODO: Handle invalid inputs properly
pub fn parse_headers(headers: Option<String>, cookies: Option<String>) -> Option<HeaderMap> {
    // If there are no headers or cookies provided, return none
    if headers.is_none() && cookies.is_none() {
        return None;
    }

    // Create new header map
    let mut header_map = HeaderMap::new();

    // If headers are provided, parse and push to the HeaderMap
    if headers.is_some() {
        // Split CLI header option by comma
        let headers: Vec<String> = headers
            .unwrap()
            .split(',')
            .map(|header| header.to_string())
            .collect();

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
    }

    // If cookies are provided, push to the HeaderMap
    if cookies.is_some() {
        header_map.insert(
            HeaderName::from_bytes("Cookie".as_bytes()).unwrap(),
            HeaderValue::from_str(&cookies.unwrap()).unwrap(),
        );
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
