mod models;
use models::RenewableEnergy;
use std::time::Duration;
use tokio::time;
use std::thread;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:8081").unwrap();
    println!("Process B sluša na 8081...");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        println!("Process B primio: {}", String::from_utf8_lossy(&buffer[..]));
    }
}

#[tokio::main]
async fn main() {
    thread::spawn(|| {
        start_server();
    });

    let mut renewables = RenewableEnergy::new(3, 5);

    loop {
        renewables.simulate_production();
        println!("Wind: {:.1} kWh| Sun: {:.1} kWh", renewables.wind_production, renewables.solar_production);
        time::sleep(Duration::from_secs(5)).await;
    }
}
