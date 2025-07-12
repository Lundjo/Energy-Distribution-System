use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

pub fn send_message(message: &str)  -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8081").map_err(|e| {
        format!("Failed to connect to server at 127.0.0.1:8081: {}", e)
    })?;
    
    stream.write_all(message.as_bytes()).map_err(|e| {
        format!("Failed to send message: {}", e)
    })?;
    
    Ok(())
}