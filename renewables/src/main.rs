mod models;
use models::RenewableEnergy;
use std::time::Duration;
mod connection;
use connection::start_server;
use tokio::sync::mpsc;
use tokio::time::sleep;
use tokio::io::AsyncWriteExt;
mod database;
use database::create_db;

use crate::database::get_initial_values;

#[tokio::main]
async fn main() {
    let _ = create_db();

    let (tx, mut rx) = mpsc::channel(32);
    
    tokio::spawn(async move {
        start_server(tx).await;
    });

    let mut renewables = RenewableEnergy::new();

    let _ = get_initial_values(&mut renewables);

    loop {
        tokio::select! {
            Some((message, mut stream)) = rx.recv() => {
                println!("Received message from server side: {}", message);

                //let response = RenewableEnergy::add_generators(&mut renewables, message);
                let response = RenewableEnergy::calculate_production(&renewables);
                if let Err(e) = stream.write_all(response.as_bytes()).await {
                    eprintln!("Failed to send response from main: {}", e);
                }
            },
            _ = sleep(Duration::from_secs(5)) => {
                renewables.simulate_production();
                println!("Wind: {:.1} kWh| Sun: {:.1} kWh", renewables.wind_production, renewables.solar_production);
            }
        }
    }
}
