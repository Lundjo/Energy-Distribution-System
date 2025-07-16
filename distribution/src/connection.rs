use tokio::net::{TcpStream, TcpListener};
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use std::error::Error;

pub async fn send_message_to_renewables(message: &str) -> Result<String, Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8081").await.map_err(|e| {
        format!("Failed to connect to server at 127.0.0.1:8081: {}", e)
    })?;
    
    stream.write_all(message.as_bytes()).await.map_err(|e| {
        format!("Failed to send message: {}", e)
    })?;
    
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await.map_err(|e| {
        format!("Failed to read response: {}", e)
    })?;
    
    if n == 0 {
        return Err("Connection closed by server".into());
    }
    
    let response = String::from_utf8(buffer[..n].to_vec()).map_err(|e| {
        format!("Invalid UTF-8 sequence in response: {}", e)
    })?;
    
    Ok(response)
}

pub async fn send_message2(message: &str) -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8082").await.map_err(|e| {
        format!("Failed to connect to server at 127.0.0.1:8082: {}", e)
    })?;
    
    stream.write_all(message.as_bytes()).await.map_err(|e| {
        format!("Failed to send message: {}", e)
    })?;
    
    Ok(())
}

pub async fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:8083").await.unwrap();
    println!("Listening on port na 8083...");

    loop {
        match listener.accept().await {
            Ok((mut stream, _)) => {
                let mut buffer = [0; 1024];
                match stream.read(&mut buffer).await {
                    Ok(n) => {
                        if n == 0 {
                            continue;
                        }
                        println!("Received message: {}", String::from_utf8_lossy(&buffer[..n]));
                    }
                    Err(e) => {
                        eprintln!("Failed to read from socket: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }
}