use std::io::{BufRead, BufReader, Read, Result, Write};
use std::net::TcpStream;

use crate::http::{request::Request, status::StatusCode};

const DELIMITERS: &str = "\r\n\r\n";

pub fn handle_connection(mut stream: TcpStream) {
    let request = decode_request(&mut stream).unwrap();
    println!("{:?}", request);

    let response = match request.request_line.path.as_str() {
        "/" => encode(StatusCode::Ok, None, None),
        "/user-agent" => encode(
            StatusCode::Ok,
            None,
            Some(request.headers["User-Agent"].clone()),
        ),
        path if path.starts_with("/echo") => {
            let echoed_part = &path["/echo/".len()..];
            encode(StatusCode::Ok, None, Some(echoed_part.to_string()))
        }
        _ => encode(StatusCode::NotFound, None, None),
    };

    stream.write_all(response.as_bytes()).unwrap();
}

fn encode(
    status: StatusCode,
    headers: Option<Vec<(String, String)>>,
    body: Option<String>,
) -> String {
    let body = body.unwrap_or_default();
    let content_length = body.len();

    let mut response = format!("HTTP/1.1 {}{}", status.as_str(), DELIMITERS);

    if let Some(hdrs) = headers {
        for (key, value) in &hdrs {
            response.push_str(&format!("{}: {}{}", key, value, DELIMITERS));
        }
    }

    response.push_str(&format!("Content-Length: {}{}", content_length, DELIMITERS));
    response.push_str(&format!("Content-Type: text/plain{}", DELIMITERS));

    response.push_str(DELIMITERS);

    response.push_str(&body);

    response
}

fn decode_request(stream: &mut TcpStream) -> Result<Request> {
    let mut buf_reader = BufReader::new(stream);

    let mut first_line = String::new();
    buf_reader.read_line(&mut first_line)?;
    let request_line = first_line.trim().to_string();

    let mut headers = Vec::new();
    loop {
        let mut line = String::new();
        buf_reader.read_line(&mut line)?;
        let line = line.trim_end();
        if line.is_empty() {
            break;
        }
        headers.push(line.to_string());
    }

    let length: usize = headers
        .iter()
        .find(|x| x.starts_with("Content-Length:"))
        .and_then(|x| x.split_once(":"))
        .and_then(|(_, v)| v.trim().parse().ok())
        .unwrap_or(0);

    let mut buf = vec![0u8; length];
    buf_reader.read_exact(&mut buf).ok();

    let body = Some(String::from_utf8(buf).unwrap_or_default());

    Ok(Request::new(request_line, headers, body))
}
