## Hint 2: Tools & Types

- **`fetch_archive_batch`:**
  - `reqwest::get(&url).await?` mapped to `ArchiveCapstoneError::Request`.
  - `response.status().is_success()` check → `ArchiveCapstoneError::InvalidStatus(status)`.
  - `response.json::<Vec<RemoteArchiveRecord>>().await` mapped to `ArchiveCapstoneError::Decode`.
  - `fetched_at`: use `chrono::Utc::now().to_rfc3339()` or a static string for tests.

- **`persist_archive_batch`:**
  - `conn.unchecked_transaction()` to begin a transaction.
  - `INSERT OR IGNORE INTO archive_records (mission_code, artifact, priority) VALUES (?1, ?2, ?3)`.
  - `tx.execute(...)` returns the number of rows changed (`usize`). A count of 0 means the row was skipped.
  - `tx.commit()` mapped to `ArchiveCapstoneError::Sql`.

- **`load_archive_preview`:**
  - `SELECT mission_code, artifact, priority FROM archive_records ORDER BY mission_code, artifact LIMIT ?1`.

**Spoiler threshold:** Medium—names every key API call for all three functions.
