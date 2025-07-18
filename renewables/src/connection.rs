use tokio::net::TcpListener;
use tokio::io::AsyncReadExt;
use tokio::sync::mpsc;

pub async fn start_server(tx: mpsc::Sender<(String, tokio::net::TcpStream)>) {
    let listener = TcpListener::bind("127.0.0.1:8081").await.unwrap();
    println!("Listening on port na 8081...");

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
                        println!("Received message: {}", message);

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