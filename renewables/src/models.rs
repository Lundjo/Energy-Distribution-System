use crate::database::insert_into_db;

pub struct RenewableEnergy {
    pub wind_generators: i32,
    pub solar_panels: i32,
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
        let _ = insert_into_db(self);
    }

    pub fn add_generators(&mut self, message: String) -> String {
        let parts: Vec<&str> = message.split_whitespace().collect();

        if parts.len() != 2 {
            return String::from("Invalid number of values sent");
        }
        
        let wind_generators = match parts[0].parse::<i32>() {
            Ok(num) => num,
            Err(_) => return String::from("Invalid number of wind generators sent"),
        };

        let solar_panels = match parts[1].parse::<i32>() {
            Ok(num) => num,
            Err(_) => return String::from("Invalid number of solar panels sent"),
        };

        self.wind_generators += wind_generators;
        self.solar_panels += solar_panels;

        if self.wind_generators < 0 {
            self.wind_generators = 0;
        }
        
        if self.solar_panels < 0 {
            self.solar_panels = 0;
        }

        let _ = insert_into_db(self);

        return String::from(format!("Wind generators: {}, Solar panels: {}", self.wind_generators, self.solar_panels));
    }

    pub fn calculate_production(&self) -> String {
        return String::from(format!("{} {}", self.wind_generators as f64 * self.wind_production, self.solar_panels as f64 * self.solar_production));
    }
}