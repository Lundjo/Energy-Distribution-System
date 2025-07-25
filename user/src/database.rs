use rusqlite::{Connection, Result};
use std::path::Path;
use crate::models::Devices;

pub fn create_db() -> Result<()> {
    let db_file_path = Path::new("user/src/user.db");

    if !db_file_path.exists() {
        let conn = Connection::open(&db_file_path)?;

        conn.execute(
            "CREATE TABLE user (
                id   INTEGER PRIMARY KEY,
                d1 INTEGER NOT NULL,
                d2 INTEGER NOT NULL,
                d3 INTEGER NOT NULL,
                d4 INTEGER NOT NULL,
                d5 INTEGER NOT NULL
            )",
            (),
        )?;

        let _ = conn.close();
    }
    Ok(())
}

pub fn insert_into_db(dev: &mut Devices) -> Result<()> {
    let db_file_path = Path::new("user/src/user.db");

    if db_file_path.exists() {
        let conn = Connection::open(&db_file_path)?;

        conn.execute(
            "INSERT INTO user (d1, d2, d3, d4, d5) VALUES (?1, ?2, ?3, ?4, ?5)",
            (&dev.d1, &dev.d2, &dev.d3, &dev.d4, &dev.d5),
        )?;
    }

    Ok(())
}

pub fn get_initial_values(dev: &mut Devices) -> Result<()> {
    let db_file_path = Path::new("user/src/user.db");

    if db_file_path.exists() {
        let conn = Connection::open(&db_file_path)?;

        let count: i64 = conn.query_row("SELECT COUNT(*) FROM user", [], |row| row.get(0))?;
        if count > 0 {
            let mut stmt = conn.prepare(
                "SELECT d1, d2, d3, d4, d5 
                FROM user 
                ORDER BY id DESC 
                LIMIT 1"
            )?;
            
            let row = stmt.query_row([], |row| {
                Ok((
                    row.get::<_, i32>(0)?,
                    row.get::<_, i32>(1)?,
                    row.get::<_, i32>(2)?,
                    row.get::<_, i32>(3)?,
                    row.get::<_, i32>(4)?,
                ))
            })?;
            
            dev.d1 = row.0;
            dev.d2 = row.1;
            dev.d3 = row.2;
            dev.d4 = row.3;
            dev.d5 = row.4;
        }
    }

    Ok(())
}