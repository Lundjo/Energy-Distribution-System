mod models;
use models::RenewableEnergy;
use std::time::Duration;
use std::thread;
mod connection;
use connection::start_server;

fn main() {
    thread::spawn(start_server);

    let mut renewables = RenewableEnergy::new(3, 5);

    loop {
        renewables.simulate_production();
        println!("Wind: {:.1} kWh| Sun: {:.1} kWh", renewables.wind_production, renewables.solar_production);
        std::thread::sleep(Duration::from_secs(5));
    }
}
