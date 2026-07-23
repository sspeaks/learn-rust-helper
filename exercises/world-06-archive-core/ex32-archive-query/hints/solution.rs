use rusqlite::{params, Connection};

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
    let mut stmt = conn
        .prepare(
            "
            SELECT id, mission_code, priority, archived
            FROM archive_records
            WHERE priority >= ?1
              AND mission_code LIKE ?2
              AND (?3 = 1 OR archived = 0)
            ORDER BY id ASC
            ",
        )
        .map_err(ArchiveQueryError::Sql)?;

    let prefix = format!("{}%", filter.mission_prefix);
    let rows = stmt
        .query_map(
            params![
                i64::from(filter.min_priority),
                prefix,
                i64::from(filter.include_archived as u8)
            ],
            |row| {
                Ok(ArchiveRecord {
                    id: row.get(0)?,
                    mission_code: row.get(1)?,
                    priority: row.get(2)?,
                    archived: row.get::<_, i64>(3)? != 0,
                })
            },
        )
        .map_err(ArchiveQueryError::Sql)?;

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(ArchiveQueryError::Sql)
}
