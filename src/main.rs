#[allow(unused_imports)]
use std::net::TcpListener;
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

fn parse_request_line(line: &str) -> (&str, &str, &str) {
    let mut parts = line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("");
    let version = parts.next().unwrap_or("");
    (method, path, version)
}

fn response_404() -> String {
    let status_line = "HTTP/1.1 404 Not Found";
    let content = "<html><body><h1>404 Not Found</h1></body></html>";
    format!(
        "{status_line}\r\nContent-Length: {}\r\n\r\n{}",
        content.len(),
        content
    )
}

fn response_200(contents: &str) -> String {
    let status_line = "HTTP/1.1 200 OK";
    format!(
        "{status_line}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    )
}

fn handle_client(mut stream: TcpStream) {
    let reader = BufReader::new(&stream);
    let request = reader.lines().next().unwrap().unwrap();
    let (method, path, version) = parse_request_line(&request);

    let response;
    match (method, version) {
        ("GET", "HTTP/1.1") => {
            if path == "/" {
                response = response_200("");
            } else if let Some(echo) = path.strip_prefix("/echo/") {
                response = response_200(echo);
            } else {
                response = response_404();
            }
        }
        _ => {
            response = response_404();
        }
    }

    match stream.write_all(response.as_bytes()) {
        Ok(_) => (),
        Err(e) => println!("could not send response: {:?}", e),
    };
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                handle_client(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
