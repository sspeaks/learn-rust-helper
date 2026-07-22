use rusqlite::Connection;

#[derive(Debug)]
pub enum SchemaBootstrapError {
    Sql(rusqlite::Error),
}

pub fn bootstrap_archive_schema(conn: &Connection) -> Result<(), SchemaBootstrapError> {
    // ══════════════════════════════════════════════════════════════
    // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
    // ══════════════════════════════════════════════════════════════
    todo!("Create the base archive tables and indexes")
}

pub fn ensure_schema_version(conn: &Connection, version: i64) -> Result<(), SchemaBootstrapError> {
    // ══════════════════════════════════════════════════════════════
    // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
    // ══════════════════════════════════════════════════════════════
    todo!("Persist the schema version in SQLite metadata")
}
