use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

pub fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:8081").unwrap();
    println!("Listening on port na 8081...");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        println!("Received message: {}", String::from_utf8_lossy(&buffer[..]));
    }
}