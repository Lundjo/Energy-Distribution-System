use std::time::Duration;
use std::thread;
mod connection;
use connection::start_server;

fn main() {
    thread::spawn(start_server);

    loop {
        println!("Alive!");
        std::thread::sleep(Duration::from_secs(5));
    }
}
