use std::collections::HashMap;

use crate::http::{status::StatusCode, version::Version};

const DELIMITERS: &str = "\r\n";

#[derive(Debug)]
pub struct Parts {
    pub status_code: StatusCode,
    pub version: Version,
    pub headers: HashMap<String, String>,
}

impl Parts {
    pub fn new(status_code: StatusCode, version: Version) -> Self {
        Self {
            status_code,
            version,
            headers: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct Response {
    pub head: Parts,
    pub body: Option<Vec<u8>>,
}

impl Response {
    pub fn new(head: Parts, body: Option<Vec<u8>>) -> Self {
        Self { head, body }
    }

    pub fn to_http(&self) -> String {
        let body = self.body.clone().unwrap_or_default();

        // Generate Status Line
        let mut response = format!(
            "HTTP/1.1 {} {}{}",
            self.head.status_code.as_str(),
            self.head.status_code.canonical_reason(),
            DELIMITERS
        );

        // Generate Headers
        for (key, value) in &self.head.headers {
            response.push_str(&format!("{}: {}{}", key, value, DELIMITERS));
        }
        response.push_str(DELIMITERS);

        // Generate Body
        response.push_str(&String::from_utf8_lossy(&body));
        response
    }
}
