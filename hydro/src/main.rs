mod connection;
use connection::start_server;
mod models;
use models::HydroEnergy;
use tokio::sync::mpsc;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async {
        start_server(tx).await;
    });

    let mut hydro = HydroEnergy::new();

    loop {
        tokio::select! {
            Some((message, mut stream)) = rx.recv() => {
                let response = hydro.change_production(message);
                if let Err(e) = stream.write_all(response.as_bytes()).await {
                    eprintln!("Failed to send response from main: {}", e);
                }
            }
        }
    }
}
