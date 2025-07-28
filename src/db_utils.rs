use crate::Result;
use rusqlite::Connection;
pub fn create_schema(conn: &Connection) -> Result<()> {
    // org table: Create and delete
    conn.execute(
        "CREATE TABLE IF NOT EXISTS org (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        ) STRICT",
        (),
    )?;
    conn.execute("DELETE FROM org", ())?;

    // person table: create and delete
    conn.execute(
        "CREATE TABLE IF NOT EXISTS person (
            id INTEGER PRIMARY KEY,
            org_id INTEGER,
            name TEXT NOT NULL,
            yob INTEGER, -- year of birth
            data_t TEXT,
            data_b BLOB
        ) STRICT",
        (),
    )?;
    conn.execute("DELETE FROM person", ())?;

    Ok(())
}
