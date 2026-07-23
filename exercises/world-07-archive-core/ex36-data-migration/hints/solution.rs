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
    conn.query_row("PRAGMA user_version", [], |row| row.get(0))
        .map_err(DataMigrationError::Sql)
}

pub fn migrate_schema(
    conn: &Connection,
    target_version: i64,
) -> Result<Vec<MigrationStep>, DataMigrationError> {
    let current = current_schema_version(conn)?;

    if target_version < 0 || target_version < current || target_version > 2 {
        return Err(DataMigrationError::UnsupportedVersion(target_version));
    }

    if target_version == current {
        return Ok(Vec::new());
    }

    let mut steps = Vec::new();

    for next_version in (current + 1)..=target_version {
        let description = match next_version {
            1 => {
                conn.execute_batch(
                    "
                    CREATE TABLE IF NOT EXISTS archive_records (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        mission_code TEXT NOT NULL,
                        priority INTEGER NOT NULL
                    );
                    ",
                )
                .map_err(DataMigrationError::Sql)?;
                "create archive_records base table"
            }
            2 => {
                conn.execute_batch(
                    "ALTER TABLE archive_records ADD COLUMN archived INTEGER NOT NULL DEFAULT 0;",
                )
                .map_err(DataMigrationError::Sql)?;
                "add archived flag to archive_records"
            }
            _ => return Err(DataMigrationError::UnsupportedVersion(next_version)),
        };

        let pragma = format!("PRAGMA user_version = {next_version};");
        conn.execute_batch(&pragma)
            .map_err(DataMigrationError::Sql)?;

        steps.push(MigrationStep {
            from_version: next_version - 1,
            to_version: next_version,
            description: description.to_string(),
        });
    }

    Ok(steps)
}
