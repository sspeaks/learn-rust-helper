use rusqlite::{params, Connection};
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

#[derive(Debug, Deserialize)]
struct ArchiveBatchPayload {
    fetched_at: String,
    records: Vec<RemoteArchiveRecord>,
}

/// Stage 1 (async): fetch and decode remote records.
pub async fn fetch_archive_batch(
    base_url: &str,
    mission_code: &str,
) -> Result<ArchiveBatch, ArchiveCapstoneError> {
    let endpoint = format!("{}/archive/{mission_code}", base_url.trim_end_matches('/'));

    let response = reqwest::get(&endpoint)
        .await
        .map_err(ArchiveCapstoneError::Request)?;

    let status = response.status();
    if !status.is_success() {
        return Err(ArchiveCapstoneError::InvalidStatus(status));
    }

    let payload = response
        .json::<ArchiveBatchPayload>()
        .await
        .map_err(ArchiveCapstoneError::Decode)?;

    Ok(ArchiveBatch {
        fetched_at: payload.fetched_at,
        records: payload.records,
    })
}

/// Stage 2 (sync): persist an already-fetched batch.
///
/// Keep database writes synchronous so `&Connection` never crosses an `.await` boundary.
pub fn persist_archive_batch(
    conn: &Connection,
    batch: &ArchiveBatch,
) -> Result<PersistReport, ArchiveCapstoneError> {
    let tx = conn
        .unchecked_transaction()
        .map_err(ArchiveCapstoneError::Sql)?;

    let mut inserted = 0usize;
    let mut skipped = 0usize;

    for record in &batch.records {
        let exists: i64 = tx
            .query_row(
                "
                SELECT EXISTS(
                    SELECT 1
                    FROM archive_records
                    WHERE mission_code = ?1 AND artifact = ?2
                )
                ",
                params![record.mission_code, record.artifact],
                |row| row.get(0),
            )
            .map_err(ArchiveCapstoneError::Sql)?;

        if exists != 0 {
            skipped += 1;
            continue;
        }

        tx.execute(
            "
            INSERT INTO archive_records (mission_code, artifact, priority)
            VALUES (?1, ?2, ?3)
            ",
            params![
                record.mission_code,
                record.artifact,
                i64::from(record.priority)
            ],
        )
        .map_err(ArchiveCapstoneError::Sql)?;

        inserted += 1;
    }

    tx.commit().map_err(ArchiveCapstoneError::Sql)?;

    Ok(PersistReport { inserted, skipped })
}

pub fn load_archive_preview(
    conn: &Connection,
    limit: usize,
) -> Result<Vec<RemoteArchiveRecord>, ArchiveCapstoneError> {
    if limit == 0 {
        return Ok(Vec::new());
    }

    let mut stmt = conn
        .prepare(
            "
            SELECT mission_code, artifact, priority
            FROM archive_records
            ORDER BY mission_code ASC, artifact ASC
            LIMIT ?1
            ",
        )
        .map_err(ArchiveCapstoneError::Sql)?;

    let rows = stmt
        .query_map(params![limit as i64], |row| {
            Ok(RemoteArchiveRecord {
                mission_code: row.get(0)?,
                artifact: row.get(1)?,
                priority: row.get(2)?,
            })
        })
        .map_err(ArchiveCapstoneError::Sql)?;

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(ArchiveCapstoneError::Sql)
}
