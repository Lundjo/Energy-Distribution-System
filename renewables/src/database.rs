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

pub fn get_initial_values(re: &mut RenewableEnergy) -> Result<()> {
    let db_file_path = Path::new("renewables/src/renewables.db");

    if db_file_path.exists() {
        let conn = Connection::open(&db_file_path)?;

        let count: i64 = conn.query_row("SELECT COUNT(*) FROM renewables", [], |row| row.get(0))?;
        if count > 0 {
            let mut stmt = conn.prepare(
                "SELECT wind_generators, wind_production, solar_panels, solar_production 
                FROM renewables 
                ORDER BY id DESC 
                LIMIT 1"
            )?;
            
            let row = stmt.query_row([], |row| {
                Ok((
                    row.get::<_, i32>(0)?,
                    row.get::<_, f64>(1)?,
                    row.get::<_, i32>(2)?,
                    row.get::<_, f64>(3)?,
                ))
            })?;
            
            re.wind_generators = row.0;
            re.wind_production = row.1;
            re.solar_panels = row.2;
            re.solar_production = row.3;
        }
    }

    Ok(())
}