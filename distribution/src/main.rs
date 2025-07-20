mod connection;
use connection::start_server;
mod inputs;
use inputs::select_renewables;

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        start_server().await;
    });

    loop {
        match select_renewables().await{
            Ok(_) => println!("Session ended safely"),
            Err(e) => eprintln!("Error running program: {}", e),
        }
    }
}
