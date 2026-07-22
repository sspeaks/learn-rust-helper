## Hint 3: Algorithm Outline

```
async function fetch_archive_batch(base_url, mission_code):
    Step 1: Send GET "{base_url}/archive/{mission_code}" and await
            → on error, return ArchiveCapstoneError::Request

    Step 2: Check status — if not 2xx:
            → return ArchiveCapstoneError::InvalidStatus(status)

    Step 3: Decode body as Vec<RemoteArchiveRecord> via .json().await
            → on error, return ArchiveCapstoneError::Decode

    Step 4: Return Ok(ArchiveBatch { fetched_at: <timestamp>, records })

function persist_archive_batch(conn, batch):
    Step 1: Begin transaction on conn
            → map error to ArchiveCapstoneError::Sql

    Step 2: inserted = 0, skipped = 0

    Step 3: For each record in batch.records:
            Execute INSERT OR IGNORE INTO archive_records
                (mission_code, artifact, priority) VALUES (?1, ?2, ?3)
            → rows_changed = execute result (0 = skipped, 1 = inserted)
            → inserted += rows_changed; skipped += (1 - rows_changed)

    Step 4: Commit transaction
            → map error to ArchiveCapstoneError::Sql

    Step 5: Return Ok(PersistReport { inserted, skipped })

function load_archive_preview(conn, limit):
    Step 1: Prepare SELECT ... FROM archive_records
            ORDER BY mission_code, artifact LIMIT ?1

    Step 2: Query and map rows to RemoteArchiveRecord

    Step 3: Return Ok(records)
```

**Critical:** `persist_archive_batch` is `fn`, not `async fn`. Never add `.await` here and never hold a `&Connection` inside an `async fn`.

**Spoiler threshold:** High—complete staged algorithm with the !Send constraint highlighted.
