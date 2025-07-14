mod models;
use models::RenewableEnergy;
use std::time::Duration;
mod connection;
use connection::start_server;

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        start_server().await;
    });

    let mut renewables = RenewableEnergy::new(3, 5);

    loop {
        renewables.simulate_production();
        println!("Wind: {:.1} kWh| Sun: {:.1} kWh", renewables.wind_production, renewables.solar_production);
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
