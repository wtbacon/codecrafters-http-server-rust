#[allow(unused_imports)]
use std::net::TcpListener;
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

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

fn handle_client(mut stream: TcpStream) {
    let reader = BufReader::new(&stream);
    let request = reader.lines().next().unwrap().unwrap();

    let status_line = match &request[..] {
        "GET / HTTP/1.1" => "HTTP/1.1 200 OK",
        _ => "HTTP/1.1 404 Not Found",
    };

    let response = format!("{status_line}\r\n\r\n");
    match stream.write_all(response.as_bytes()) {
        Ok(_) => (),
        Err(e) => println!("could not send response: {}", e),
    };
}
