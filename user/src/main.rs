mod connection;
use std::error::Error;
mod models;
use models::Devices;
mod inputs;
use inputs::select_device;
mod database;
use database::{create_db, get_initial_values};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = create_db();

    let mut devices = Devices::new();
    let _ = get_initial_values(&mut devices);

    loop {
        tokio::select! {
            result = select_device(&mut devices) => {
                match result {
                    Ok(_) => (),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        }
    }
}
