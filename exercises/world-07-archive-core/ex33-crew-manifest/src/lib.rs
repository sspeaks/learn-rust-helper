use rusqlite::Connection;

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
    // ══════════════════════════════════════════════════════════════
    // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
    // ══════════════════════════════════════════════════════════════
    todo!("Insert one crew member row using a parameterized statement")
}

pub fn load_crew_manifest(conn: &Connection) -> Result<Vec<CrewManifestEntry>, CrewManifestError> {
    // ══════════════════════════════════════════════════════════════
    // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
    // ══════════════════════════════════════════════════════════════
    todo!("Select all crew rows and map them into CrewManifestEntry")
}
