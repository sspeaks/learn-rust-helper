## Hint 3: Algorithm Outline

```
function current_schema_version(conn):
    Step 1: Execute PRAGMA user_version query
            → map SQL error to DataMigrationError::Sql
    Step 2: Return Ok(version as i64)

function migrate_schema(conn, target_version):
    Step 1: current = current_schema_version(conn)?

    Step 2: If target_version < current:
            → return DataMigrationError::UnsupportedVersion(target_version)

    Step 3: If target_version == current:
            → return Ok(vec![])

    Step 4: Let steps = vec![]

    Step 5: For each version step from (current + 1) to target_version (inclusive):
            from_v = version - 1
            to_v   = version

            Apply the migration SQL for this step:
            Match on version:
                1 → CREATE TABLE IF NOT EXISTS archive_records (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        mission_code TEXT NOT NULL,
                        priority INTEGER NOT NULL
                    )
                    description: "create archive_records base table"
                2 → ALTER TABLE archive_records
                        ADD COLUMN archived INTEGER NOT NULL DEFAULT 0
                    description: "add archived flag to archive_records"
                N (> 2 or < 0) → return DataMigrationError::UnsupportedVersion(version)

            Update PRAGMA user_version = to_v

            Push MigrationStep { from_version: from_v, to_version: to_v, description }

    Step 6: Return Ok(steps)
```

**Note:** `PRAGMA user_version = N` cannot use `?` parameter binding—the value must be embedded in the SQL string using `format!`. Step 2 also applies when the target exceeds the maximum supported migration version (2); reject any target greater than 2 with `UnsupportedVersion`.
