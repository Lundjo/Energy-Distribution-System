use tokio::net::TcpListener;
use tokio::io::AsyncReadExt;
use tokio::sync::mpsc;

use crate::models::HydroEnergy;

pub async fn start_server(tx: mpsc::Sender<(String, tokio::net::TcpStream)>) {
    let listener = TcpListener::bind("127.0.0.1:8082").await.unwrap();

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

pub fn select_method(h: &mut HydroEnergy, message: String) -> String {
    let parts: Vec<&str> = message.split_whitespace().collect();

    if parts[0] == "0" {
        return HydroEnergy::change_production(h, message);
    } else if parts[0] == "1" {
        return HydroEnergy::return_production(h);
    } else {
        return String::from("Unsupported method");
    }
}