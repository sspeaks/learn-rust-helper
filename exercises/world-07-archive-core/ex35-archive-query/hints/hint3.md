## Hint 3: Algorithm Outline

```
function query_archive(conn, filter):
    Step 1: Prepare the SQL:
            SELECT id, mission_code, priority, archived
            FROM archive_records
            WHERE priority >= ?1
              AND mission_code LIKE ?2
              AND (?3 = 1 OR archived = 0)
            ORDER BY id

    Step 2: Bind parameters:
            ?1 = filter.min_priority as i64
            ?2 = format!("{}%", filter.mission_prefix)
            ?3 = filter.include_archived as i64

    Step 3: query_map over the result rows:
            For each row:
                id           = row.get::<_, i64>(0)
                mission_code = row.get::<_, String>(1)
                priority     = row.get::<_, i64>(2) as u8
                archived     = row.get::<_, i64>(3) != 0  (convert 0/1 to bool)
                Return Ok(ArchiveRecord { id, mission_code, priority, archived })

    Step 4: Collect into Result<Vec<ArchiveRecord>, rusqlite::Error>
            Map error to ArchiveQueryError::Sql

    Step 5: Return Ok(records)
```

**Note:** The `AND (?3 = 1 OR archived = 0)` clause is a single-SQL approach to the optional archived filter. When `?3 = 1` (include_archived = true), the OR short-circuits and the `archived = 0` condition is ignored.

**Spoiler threshold:** High—complete query and row mapping without raw Rust.
