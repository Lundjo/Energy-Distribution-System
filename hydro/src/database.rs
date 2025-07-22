use rusqlite::{Connection, Result};
use std::path::Path;
use crate::models::HydroEnergy;

pub fn create_db() -> Result<()> {
    let db_file_path = Path::new("hydro/src/hydro.db");

    if !db_file_path.exists() {
        let conn = Connection::open(&db_file_path)?;

        conn.execute(
            "CREATE TABLE hydro (
                id   INTEGER PRIMARY KEY,
                production REAL NOT NULL,
                usage REAL NOT NULL
            )",
            (),
        )?;

        let _ = conn.close();
    }
    Ok(())
}

pub fn insert_into_db(h: &mut HydroEnergy) -> Result<()> {
    let db_file_path = Path::new("hydro/src/hydro.db");

    if db_file_path.exists() {
        let conn = Connection::open(&db_file_path)?;

        conn.execute(
            "INSERT INTO hydro (production, usage) VALUES (?1, ?2)",
            (&h.production, &h.usage),
        )?;
    }

    Ok(())
}

pub fn get_initial_values(h: &mut HydroEnergy) -> Result<()> {
    let db_file_path = Path::new("hydro/src/hydro.db");

    if db_file_path.exists() {
        let conn = Connection::open(&db_file_path)?;

        let count: i64 = conn.query_row("SELECT COUNT(*) FROM hydro", [], |row| row.get(0))?;
        if count > 0 {
            let mut stmt = conn.prepare(
                "SELECT production, usage
                FROM hydro
                ORDER BY id DESC 
                LIMIT 1"
            )?;
            
            let row = stmt.query_row([], |row| {
                Ok((
                    row.get::<_, f64>(0)?,
                    row.get::<_, f64>(1)?,
                ))
            })?;
            
            h.production = row.0;
            h.usage = row.1;
        }
    }

    Ok(())
}