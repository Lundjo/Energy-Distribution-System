mod connection;
use connection::send_message;
use std::error::Error;
use tokio::io::AsyncBufReadExt;
mod models;
use models::Devices;
mod inputs;
use inputs::select_device;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut devices = Devices::new();
    let _ = select_device(&mut devices).await?;

    Ok(())
}
