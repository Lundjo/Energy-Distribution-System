mod connection;
use connection::start_server;
mod inputs;
use inputs::select_renewables;

use crate::connection::send_message_to_hydro;

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        start_server().await;
    });

    match send_message_to_hydro("40.0").await{
        Ok(response) => println!("Server responded: {}", response),
        Err(e) => eprintln!("Error running program: {}", e),
    }
}
