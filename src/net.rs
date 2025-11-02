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
    // let mut buf_reader = BufReader::new(stream);
    // let mut stream_string = Vec::new();
    // loop {
    //     let mut line = String::new();

    //     if let Err(e) = buf_reader.read_line(&mut line) {
    //         eprintln!("Failed to read from connection: {}", e);
    //         let response = Response::new(
    //             StatusCode::from_u16(500).unwrap(),
    //             None,
    //             Some("Internal Server Error".to_string()),
    //         );
    //         stream.write_all(response.to_http().as_bytes()).unwrap();
    //         stream.flush().unwrap();

    //         return Ok(());
    //     }

    //     let line = line.trim();
    //     if line.is_empty() {
    //         break;
    //     }
    //     stream_string.push(line.to_string());
    // }

    let request = parse_request(&mut stream)?;
    println!("{:?}", request);

    let mut router = Router::new();
    router.add_route("GET", "/", root_handler);
    router.add_route("GET", "/echo/:msg", echo_handler);
    router.add_route("GET", "/user-agent", user_agent_handler);

    let response = router.route(&request);
    stream.write_all(response.to_http().as_bytes()).unwrap();
    Ok(())
}
