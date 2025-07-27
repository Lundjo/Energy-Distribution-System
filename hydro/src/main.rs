mod connection;
use connection::{start_server, select_method};
mod models;
use models::HydroEnergy;
use tokio::sync::mpsc;
use tokio::io::AsyncWriteExt;
mod database;
use database::{create_db, get_initial_values};

#[tokio::main]
async fn main() {
    let _ = create_db();
    let (tx, mut rx) = mpsc::channel(2);

    tokio::spawn(async {
        start_server(tx).await;
    });

    let mut hydro = HydroEnergy::new();
    let _ = get_initial_values(&mut hydro);

    loop {
        while let Some((message, mut stream)) = rx.recv().await {
            let response = select_method(&mut hydro, message);
            if let Err(e) = stream.write_all(response.as_bytes()).await {
                eprintln!("Failed to send response: {}", e);
            }
        }
    }
}
