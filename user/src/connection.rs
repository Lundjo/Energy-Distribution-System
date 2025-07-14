use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::error::Error;

pub async fn send_message(message: &str) -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8083").await.map_err(|e| {
        format!("Failed to connect to server at 127.0.0.1:8083: {}", e)
    })?;
    
    stream.write_all(message.as_bytes()).await.map_err(|e| {
        format!("Failed to send message: {}", e)
    })?;
    
    Ok(())
}