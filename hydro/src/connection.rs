use tokio::net::TcpListener;
use tokio::io::AsyncReadExt;

pub async fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:8082").await.unwrap();
    println!("Listening on port na 8082...");

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