mod connection;
use connection::start_server;
mod inputs;
use inputs::select_operation;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async {
        start_server(tx).await;
    });

    let mut client: Option<(String, tokio::net::TcpStream)> = None;

    loop {
        tokio::select! {
            Some((message, mut stream)) = rx.recv() => {
                println!("New client message: {:?}", message);
                client = Some((message, stream));
            }

            result = select_operation(&mut client) => {
                match result {
                    Ok(_) => (),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        }
    }
}
