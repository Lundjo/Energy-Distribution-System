mod connection;
use connection::start_server;
mod inputs;
use inputs::change_number_of_generators;

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        start_server().await;
    });

    match change_number_of_generators().await{
        Ok(_) => println!("Session ended safely"),
        Err(e) => eprintln!("Error running program: {}", e),
    }
}
