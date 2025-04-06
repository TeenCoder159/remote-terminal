#[allow(unused)]
use std::net::{SocketAddr, TcpListener, TcpStream};

fn main() {
    let listener =
        TcpListener::bind("127.0.0.1:8080").expect("Error while binding to 127.0.0.1:8080");
}
