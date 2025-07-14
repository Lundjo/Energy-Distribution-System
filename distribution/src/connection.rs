use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

pub fn send_message1(message: &str)  -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8081").map_err(|e| {
        format!("Failed to connect to server at 127.0.0.1:8081: {}", e)
    })?;
    
    stream.write_all(message.as_bytes()).map_err(|e| {
        format!("Failed to send message: {}", e)
    })?;
    
    Ok(())
}

pub fn send_message2(message: &str)  -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8082").map_err(|e| {
        format!("Failed to connect to server at 127.0.0.1:8082: {}", e)
    })?;
    
    stream.write_all(message.as_bytes()).map_err(|e| {
        format!("Failed to send message: {}", e)
    })?;
    
    Ok(())
}

pub fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:8083").unwrap();
    println!("Listening on port na 8083...");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        println!("Received message: {}", String::from_utf8_lossy(&buffer[..]));
    }
}