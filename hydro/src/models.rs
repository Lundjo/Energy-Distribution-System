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
        let additional_production = match message.parse::<f64>() {
            Ok(num) => num,
            Err(_) => return String::from("Invalid power sent"),
        };

        if(self.production + additional_production > 200.0) {
            return String::from("Not enough power can be supplied");
        }

        self.production += additional_production;
        self.usage = self.production * 100.0 / 200.0;
        let _ = insert_into_db(self);

        return String::from(format!("Power production changed. Current load: {}", self.usage));
    }
}