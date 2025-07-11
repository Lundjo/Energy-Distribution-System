mod models;
use models::RenewableEnergy;
use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() {
    let mut renewables = RenewableEnergy::new(3, 5);

    loop {
        renewables.simulate_production();
        println!("Wind: {:.1} kWh| Sun: {:.1} kWh", renewables.wind_production, renewables.solar_production);
        time::sleep(Duration::from_secs(5)).await;
    }
}
