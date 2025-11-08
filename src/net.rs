use crate::handlers;
use crate::http::request::Request;
use crate::route::Router;
use std::io::{self, Write};
use std::{
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
                thread::spawn(move || loop {
                    match handle_connection(&stream) {
                        Ok(should_close) => {
                            if should_close {
                                println!("Closing connection");
                                break;
                            }
                        }
                        Err(e) => {
                            eprintln!("Error handling connection: {}", e);
                            break;
                        }
                    }
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: &TcpStream) -> io::Result<bool> {
    let request = Request::parse_request(stream)?;
    println!("{:?}", request);

    let mut router = Router::new();
    router.add_route("GET", "/", handlers::root_handler);
    router.add_route("GET", "/echo/:msg", handlers::echo_handler);
    router.add_route("GET", "/user-agent", handlers::user_agent_handler);
    router.add_route("GET", "/files/:filename", handlers::files_handler);
    router.add_route("POST", "/files/:filename", handlers::post_file_handler);

    let response = router.route(&request);
    println!("{:?}", response);

    stream.write_all(response.to_http_headers_only().as_bytes())?;
    stream.write_all(&response.body.unwrap_or_default())?;

    let should_close = response
        .head
        .headers
        .get("Connection")
        .is_some_and(|conn| conn.to_lowercase() == "close");

    Ok(should_close)
}
