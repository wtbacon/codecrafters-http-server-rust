use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read, Result},
    net::TcpStream,
};

use crate::http::version::Version;

#[derive(Debug)]
pub struct Parts {
    pub method: String,
    pub path: String,
    pub version: Version,
    pub headers: HashMap<String, String>,
}

impl Parts {
    pub fn new(request_line: String) -> Self {
        let mut parts = request_line.splitn(3, ' ');
        let method = parts.next().unwrap_or("").to_string();
        let path = parts.next().unwrap_or("").to_string();
        let version = parts.next().unwrap_or("").to_string();

        Self {
            method,
            path,
            version: Version::from_str(&version),
            headers: HashMap::new(),
        }
    }
}
#[derive(Debug)]
pub struct Request {
    pub head: Parts,
    pub body: Option<String>,
}

impl Request {
    pub fn new(head: Parts, body: Option<String>) -> Self {
        Self { head, body }
    }
}

impl Request {
    pub fn parse_request(stream: &mut TcpStream) -> Result<Request> {
        let mut buf_reader = BufReader::new(stream);

        let mut first_line = String::new();
        buf_reader.read_line(&mut first_line)?;
        let request_line = first_line.trim().to_string();

        let mut parts = Parts::new(request_line);
        loop {
            let mut line = String::new();
            buf_reader.read_line(&mut line)?;
            let line = line.trim_end();
            if line.is_empty() {
                break;
            }
            if let Some((key, value)) = line.split_once(": ") {
                parts.headers.insert(key.to_string(), value.to_string());
            }
        }

        let content_length = parts
            .headers
            .get("Content-Length")
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(0);

        if content_length == 0 {
            Ok(Request::new(parts, None))
        } else {
            let mut buf = vec![0u8; content_length];
            buf_reader.read_exact(&mut buf)?;
            match String::from_utf8(buf) {
                Ok(body) => Ok(Request::new(parts, Some(body))),
                Err(e) => {
                    eprintln!("Failed to parse body as UTF-8: {}", e);
                    Ok(Request::new(parts, None))
                }
            }
        }
    }
}
