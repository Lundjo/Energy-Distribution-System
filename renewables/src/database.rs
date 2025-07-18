use rusqlite::{Connection, Result};
use std::path::Path;

use crate::models::RenewableEnergy;

pub fn create_db() -> Result<()> {
    let db_file_path = Path::new("renewables/src/renewables.db");

    if !db_file_path.exists() {
        let conn = Connection::open(&db_file_path)?;

        conn.execute(
            "CREATE TABLE renewables (
                id   INTEGER PRIMARY KEY,
                wind_generators INTEGER NOT NULL,
                wind_production REAL NOT NULL,
                solar_panels INTEGER NOT NULL,
                solar_production REAL NOT NULL,
                time DATETIME DEFAULT current_timestamp
            )",
            (),
        )?;

        let _ = conn.close();
    }
    Ok(())
}

pub fn insert_into_db(re: &mut RenewableEnergy) -> Result<()> {
    let db_file_path = Path::new("renewables/src/renewables.db");

    if db_file_path.exists() {
        let conn = Connection::open(&db_file_path)?;

        conn.execute(
            "INSERT INTO renewables (wind_generators, wind_production, solar_panels, solar_production) VALUES (?1, ?2, ?3, ?4)",
            (&re.wind_generators, &re.wind_production, &re.solar_panels, &re.solar_production),
        )?;
    }

    Ok(())
}