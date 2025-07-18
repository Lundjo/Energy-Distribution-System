use rusqlite::{params, Connection, Result};

pub fn create_db() -> Result<()> {
    let db_file_path = "renewables/src/renewables.db";
    let conn = Connection::open(&db_file_path)?;

    conn.execute(
        "CREATE TABLE renewables (
            id   INTEGER PRIMARY KEY,
            wind_generators INTEGER NOT NULL,
            wind_production REAL NOT NULL,
            solar_panels INTEGER NOT NULL,
            solar_production REAL NOT NULL,
            time DATETIME
        )",
        (),
    )?;

    Ok(())
}