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
                thread::spawn(|| {
                    if let Err(e) = handle_connection(stream) {
                        eprintln!("Error handling connection: {}", e);
                    }
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let request = Request::parse_request(&mut stream)?;
    println!("{:?}", request);

    let mut router = Router::new();
    router.add_route("GET", "/", handlers::root_handler);
    router.add_route("GET", "/echo/:msg", handlers::echo_handler);
    router.add_route("GET", "/user-agent", handlers::user_agent_handler);
    router.add_route("GET", "/files/:filename", handlers::files_handler);
    router.add_route("POST", "/files/:filename", handlers::post_file_handler);

    let response = router.route(&request);
    println!("{:?}", response);

    stream
        .write_all(response.to_http_headers_only().as_bytes())
        .unwrap();
    stream
        .write_all(&response.body.clone().unwrap_or_default())
        .unwrap();
    stream.flush()
}
