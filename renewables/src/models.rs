pub struct RenewableEnergy {
    pub wind_generators: u32,
    pub solar_panels: u32,
    pub wind_production: f64,
    pub solar_production: f64,
}

impl RenewableEnergy {
    pub fn new(wind_generators: u32, solar_panels: u32) -> Self {
        RenewableEnergy {
            wind_generators,
            solar_panels,
            wind_production: 0.0,
            solar_production: 0.0,
        }
    }

    pub fn simulate_production(&mut self) {
        self.wind_production = rand::random::<f64>() * 100.0;
        self.solar_production = rand::random::<f64>() * 100.0;
    }
}