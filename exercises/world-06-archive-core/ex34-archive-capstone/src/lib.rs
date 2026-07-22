use rusqlite::Connection;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct RemoteArchiveRecord {
    pub mission_code: String,
    pub artifact: String,
    pub priority: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchiveBatch {
    pub fetched_at: String,
    pub records: Vec<RemoteArchiveRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistReport {
    pub inserted: usize,
    pub skipped: usize,
}

#[derive(Debug)]
pub enum ArchiveCapstoneError {
    Request(reqwest::Error),
    InvalidStatus(reqwest::StatusCode),
    Decode(reqwest::Error),
    Sql(rusqlite::Error),
}

/// Stage 1 (async): fetch and decode remote records.
pub async fn fetch_archive_batch(
    base_url: &str,
    mission_code: &str,
) -> Result<ArchiveBatch, ArchiveCapstoneError> {
    // ══════════════════════════════════════════════════════════════
    // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
    // ══════════════════════════════════════════════════════════════
    todo!("Fetch remote JSON records and decode them into ArchiveBatch")
}

/// Stage 2 (sync): persist an already-fetched batch.
///
/// Keep database writes synchronous so `&Connection` never crosses an `.await` boundary.
pub fn persist_archive_batch(
    conn: &Connection,
    batch: &ArchiveBatch,
) -> Result<PersistReport, ArchiveCapstoneError> {
    // ══════════════════════════════════════════════════════════════
    // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
    // ══════════════════════════════════════════════════════════════
    todo!("Insert decoded records with transactions and report inserted/skipped counts")
}

pub fn load_archive_preview(
    conn: &Connection,
    limit: usize,
) -> Result<Vec<RemoteArchiveRecord>, ArchiveCapstoneError> {
    // ══════════════════════════════════════════════════════════════
    // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
    // ══════════════════════════════════════════════════════════════
    todo!("Read back a deterministic preview of persisted records")
}
