use std::time::Duration;
mod connection;
use connection::start_server;

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        start_server().await;
    });

    loop {
        println!("Alive!");
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
