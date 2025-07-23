use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::error::Error;

pub async fn send_message(message: &str) -> Result<String, Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8083").await.map_err(|e| {
        format!("Failed to connect to server at 127.0.0.1:8083: {}", e)
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
    
    let response = String::from_utf8_lossy(&buffer[..n]).into_owned();
    Ok(response)
}