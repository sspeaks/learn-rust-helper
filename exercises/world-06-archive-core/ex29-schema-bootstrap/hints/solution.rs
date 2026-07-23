use rusqlite::Connection;

#[derive(Debug)]
pub enum SchemaBootstrapError {
    Sql(rusqlite::Error),
}

pub fn bootstrap_archive_schema(conn: &Connection) -> Result<(), SchemaBootstrapError> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS archive_records (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            mission_code TEXT NOT NULL,
            priority INTEGER NOT NULL,
            archived INTEGER NOT NULL DEFAULT 0
        );
        CREATE INDEX IF NOT EXISTS idx_archive_records_mission_code
            ON archive_records(mission_code);
        ",
    )
    .map_err(SchemaBootstrapError::Sql)?;

    Ok(())
}

pub fn ensure_schema_version(conn: &Connection, version: i64) -> Result<(), SchemaBootstrapError> {
    let pragma = format!("PRAGMA user_version = {version};");
    conn.execute_batch(&pragma)
        .map_err(SchemaBootstrapError::Sql)?;
    Ok(())
}
