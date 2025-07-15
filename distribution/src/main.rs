mod connection;
use connection::start_server;
mod inputs;
use inputs::input;

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        start_server().await;
    });

    match input().await{
        Ok(_) => println!("Message sent successfully"),
        Err(e) => eprintln!("Message could not be sent: {}", e),
    }
}
