use rusqlite::Connection;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchiveRecord {
    pub id: i64,
    pub mission_code: String,
    pub priority: u8,
    pub archived: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchiveFilter {
    pub min_priority: u8,
    pub mission_prefix: String,
    pub include_archived: bool,
}

#[derive(Debug)]
pub enum ArchiveQueryError {
    Sql(rusqlite::Error),
}

pub fn query_archive(
    conn: &Connection,
    filter: &ArchiveFilter,
) -> Result<Vec<ArchiveRecord>, ArchiveQueryError> {
    // ══════════════════════════════════════════════════════════════
    // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
    // ══════════════════════════════════════════════════════════════
    todo!("Run parameterized SQLite queries with optional archived filtering")
}
