use crate::http::request::parse_request;
use crate::http::{response::build_response, status::StatusCode};
use std::{
    io::Write,
    net::{TcpListener, TcpStream},
    thread,
};

pub fn run_server(addr: &str) {
    let listener = TcpListener::bind(addr).unwrap();
    println!("Listening on {}", addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                thread::spawn(|| {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let request = parse_request(&mut stream).unwrap();
    println!("{:?}", request);

    let response = match request.request_line.path.as_str() {
        "/" => build_response(StatusCode::Ok, None, None),
        "/user-agent" => build_response(
            StatusCode::Ok,
            None,
            Some(request.headers["User-Agent"].clone()),
        ),
        path if path.starts_with("/echo") => {
            let echoed_part = &path["/echo/".len()..];
            build_response(StatusCode::Ok, None, Some(echoed_part.to_string()))
        }
        _ => build_response(StatusCode::NotFound, None, None),
    };

    stream.write_all(response.as_bytes()).unwrap();
}
