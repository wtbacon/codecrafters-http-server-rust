mod http;
mod net;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    net::run_server("127.0.0.1:4221");
}
