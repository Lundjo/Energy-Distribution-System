mod connection;
use connection::start_server;
mod models;
use models::HydroEnergy;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    //let (tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async {
        start_server().await;
    });

    let mut hydro = HydroEnergy::new();

    /*Some((message, mut stream)) = rx.recv() => {
        let response = select_method(&mut renewables, message);
        if let Err(e) = stream.write_all(response.as_bytes()).await {
            eprintln!("Failed to send response from main: {}", e);
        }
    }*/

    let var = hydro.change_production("40.0".to_string());
    println!("{}", var);
}
