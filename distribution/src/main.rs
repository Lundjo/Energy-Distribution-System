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

    let mut last_message: Option<String> = None;

    loop {
        tokio::select! {
            Some((message, mut stream)) = rx.recv() => {
                println!("New client message: {:?}", message);
                last_message = Some(message);
            }

            result = select_operation(&last_message) => {
                match result {
                    Ok(_) => (),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        }
    }
}
