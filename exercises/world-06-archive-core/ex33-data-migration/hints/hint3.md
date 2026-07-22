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
                1 → ALTER TABLE archive_records ADD COLUMN <column1> ...
                2 → ALTER TABLE archive_records ADD COLUMN <column2> ...
                N → (each step adds what the tests expect)
            On unknown version:
                → return DataMigrationError::UnsupportedVersion(version)

            Update PRAGMA user_version = to_v

            Push MigrationStep { from_version: from_v, to_version: to_v, description }

    Step 6: Return Ok(steps)
```

**Note:** Check `tests/solve.rs` to see what column names and types each migration step must add. Your `match` arms must align with what the tests verify.

**Spoiler threshold:** High—complete algorithm. Column names are intentionally left to the tests to specify.
