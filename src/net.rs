use crate::handlers::{echo_handler, root_handler, user_agent_handler};
use crate::http::request::parse_request;
use crate::route::Router;
use std::io::{Error, Write};
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

fn handle_connection(mut stream: TcpStream) -> Result<(), Error> {
    let request = parse_request(&mut stream)?;
    println!("{:?}", request);

    let mut router = Router::new();
    router.add_route("GET", "/", root_handler);
    router.add_route("GET", "/echo/:msg", echo_handler);
    router.add_route("GET", "/user-agent", user_agent_handler);
    router.add_route("GET", "/files/:filename", crate::handlers::files_handler);
    router.add_route(
        "POST",
        "/files/:filename",
        crate::handlers::post_file_handler,
    );

    let response = router.route(&request);
    println!("{:?}", response);

    stream.write_all(response.to_http().as_bytes()).unwrap();
    Ok(())
}
