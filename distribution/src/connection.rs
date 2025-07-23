use tokio::net::{TcpStream, TcpListener};
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use std::error::Error;
use tokio::sync::mpsc;

pub async fn send_message_to_renewables(message: &str) -> Result<String, Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8081").await.map_err(|e| {
        format!("Failed to connect to renewables server: {}", e)
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

pub async fn send_message_to_hydro(message: &str) -> Result<String, Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8082").await.map_err(|e| {
        format!("Failed to connect to hydro server: {}", e)
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

pub async fn start_server(tx: mpsc::Sender<(String, tokio::net::TcpStream)>) {
    let listener = TcpListener::bind("127.0.0.1:8083").await.unwrap();

    loop {
        match listener.accept().await {
            Ok((mut stream, _)) => {
                let mut buffer = [0; 1024];
                match stream.read(&mut buffer).await {
                    Ok(n) => {
                        if n == 0 {
                            continue;
                        }
                        let message = String::from_utf8_lossy(&buffer[..n]).to_string();

                        if let Err(e) = tx.send((message, stream)).await {
                            eprintln!("Failed to send to main: {}", e);
                            break;
                        }
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