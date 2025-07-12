use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn send_message(message: &str) {
    let mut stream = TcpStream::connect("127.0.0.1:8081").unwrap();
    stream.write(message.as_bytes()).unwrap();
    println!("Process B poslao: {}", message);
}

fn main() {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        send_message(input.trim());
    }
}
