pub struct RenewableEnergy {
    pub wind_generators: u32,
    pub solar_panels: u32,
    pub wind_production: f64,
    pub solar_production: f64,
}

impl RenewableEnergy {
    pub fn new() -> Self {
        RenewableEnergy {
            wind_generators: 5,
            solar_panels: 5,
            wind_production: 0.0,
            solar_production: 0.0,
        }
    }

    pub fn simulate_production(&mut self) {
        self.wind_production = rand::random::<f64>() * 100.0;
        self.solar_production = rand::random::<f64>() * 100.0;
    }

    pub fn add_generators(&mut self, wind_generators: u32, solar_panels: u32) {
        self.wind_generators = wind_generators;
        self.solar_panels = solar_panels;
    }
}