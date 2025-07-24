mod connection;
use connection::send_message;
use std::error::Error;
use tokio::io::AsyncBufReadExt;
mod models;
use models::Devices;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut devices = Devices::new();
    devices.list_devices();

    let mut stdin = tokio::io::BufReader::new(tokio::io::stdin());
    let mut input = String::new();

    loop {
        input.clear();
        
        stdin.read_line(&mut input).await?;
        let message = input.trim();
        
        match send_message(message).await {
            Ok(reposne) => println!("Server response: '{}'", reposne),
            Err(e) => eprintln!("Message could not be sent: {}", e),
        }
    }
}
