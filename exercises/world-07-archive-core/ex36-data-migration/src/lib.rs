use rusqlite::Connection;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationStep {
    pub from_version: i64,
    pub to_version: i64,
    pub description: String,
}

#[derive(Debug)]
pub enum DataMigrationError {
    Sql(rusqlite::Error),
    UnsupportedVersion(i64),
}

pub fn current_schema_version(conn: &Connection) -> Result<i64, DataMigrationError> {
    // ══════════════════════════════════════════════════════════════
    // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
    // ══════════════════════════════════════════════════════════════
    todo!("Read the current schema version from SQLite")
}

pub fn migrate_schema(
    conn: &Connection,
    target_version: i64,
) -> Result<Vec<MigrationStep>, DataMigrationError> {
    // ══════════════════════════════════════════════════════════════
    // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
    // ══════════════════════════════════════════════════════════════
    todo!("Apply ordered ALTER TABLE migrations until target_version")
}
