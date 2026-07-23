## Hint 2: Tools & Types

- **`fetch_archive_batch`:**
  - `reqwest::get(&url).await?` mapped to `ArchiveCapstoneError::Request`.
  - `response.status().is_success()` check → `ArchiveCapstoneError::InvalidStatus(status)`.
  - The server returns a JSON **object** `{ "fetched_at": "<timestamp>", "records": [...] }`, not a bare array. Decode the whole object: `response.json::<ArchiveBatchPayload>().await` where `ArchiveBatchPayload` is a helper struct you define with `fetched_at: String` and `records: Vec<RemoteArchiveRecord>`.
  - **`fetched_at` comes from the server response**, not the client clock. Do not call `chrono::Utc::now()` here—use the value decoded from the JSON object.

- **`persist_archive_batch`:**
  - `conn.unchecked_transaction()` to begin a transaction.
  - `INSERT OR IGNORE INTO archive_records (mission_code, artifact, priority) VALUES (?1, ?2, ?3)`.
  - `tx.execute(...)` returns the number of rows changed (`usize`). A count of 0 means the row was skipped.
  - `tx.commit()` mapped to `ArchiveCapstoneError::Sql`.

- **`load_archive_preview`:**
  - `SELECT mission_code, artifact, priority FROM archive_records ORDER BY mission_code, artifact LIMIT ?1`.

**Spoiler threshold:** Medium—names every key API call for all three functions.
