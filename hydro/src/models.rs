use crate::database::insert_into_db;

pub struct HydroEnergy {
    pub production: f64,
    pub usage: f64,
}

impl HydroEnergy {
    pub fn new() -> Self {
        HydroEnergy {
            production: 0.0,
            usage: 0.0,
        }
    }

    pub fn change_production(&mut self, message: String) -> String {
        let parts: Vec<&str> = message.split_whitespace().collect();

        let additional_production = match parts[1].trim_end().parse::<f64>() {
            Ok(num) => num,
            Err(_) => return String::from("Invalid power sent {}"),
        };

        if self.production + additional_production > 50.0 {
            return String::from("Not enough power can be supplied");
        }

        self.production += additional_production;
        self.usage = self.production * 100.0 / 50.0;
        let _ = insert_into_db(self);

        return String::from(format!("Power production changed. Current load: {}", self.usage));
    }

    pub fn return_production(&self) -> String {
        return String::from(format!("{}", self.production));
    }
}