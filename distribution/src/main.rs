mod connection;
use connection::start_server;
mod inputs;
use inputs::select_operation;

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        start_server().await;
    });

    loop {
        match select_operation().await{
            Ok(_) => print!(""),
            Err(e) => eprintln!("Error running program: {}", e),
        }
    }
}
