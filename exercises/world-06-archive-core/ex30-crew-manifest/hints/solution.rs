use rusqlite::{params, Connection};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrewManifestEntry {
    pub crew_id: String,
    pub role: String,
    pub rank: u8,
}

#[derive(Debug)]
pub enum CrewManifestError {
    Sql(rusqlite::Error),
}

pub fn insert_crew_member(
    conn: &Connection,
    entry: &CrewManifestEntry,
) -> Result<(), CrewManifestError> {
    conn.execute(
        "INSERT INTO crew_manifest (crew_id, role, rank) VALUES (?1, ?2, ?3)",
        params![entry.crew_id, entry.role, i64::from(entry.rank)],
    )
    .map_err(CrewManifestError::Sql)?;

    Ok(())
}

pub fn load_crew_manifest(conn: &Connection) -> Result<Vec<CrewManifestEntry>, CrewManifestError> {
    let mut stmt = conn
        .prepare("SELECT crew_id, role, rank FROM crew_manifest ORDER BY crew_id")
        .map_err(CrewManifestError::Sql)?;

    let rows = stmt
        .query_map([], |row| {
            Ok(CrewManifestEntry {
                crew_id: row.get(0)?,
                role: row.get(1)?,
                rank: row.get::<_, u8>(2)?,
            })
        })
        .map_err(CrewManifestError::Sql)?;

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(CrewManifestError::Sql)
}
