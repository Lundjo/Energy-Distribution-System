mod models;
use models::RenewableEnergy;

fn main() {
    let mut renewables = RenewableEnergy::new(3, 5);

    renewables.simulate_production();
}
