mod connection;
use std::error::Error;
mod models;
use models::Devices;
mod inputs;
use inputs::select_device;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut devices = Devices::new();

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
