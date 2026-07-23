## Hint 3: Algorithm Outline

```
function bootstrap_archive_schema(conn):
    Step 1: Execute CREATE TABLE IF NOT EXISTS archive_records (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                mission_code TEXT NOT NULL,
                priority INTEGER NOT NULL,
                archived INTEGER NOT NULL DEFAULT 0
            )
            → map SQL error to SchemaBootstrapError::Sql

    Step 2: Create any additional indexes with
            CREATE INDEX IF NOT EXISTS ... ON archive_records(...)
            (check test expectations for indexed columns)

    Step 3: Return Ok(())

function ensure_schema_version(conn, version):
    Step 1: Execute PRAGMA user_version = {version}
            (embed the version number directly in the SQL string—
             PRAGMAs do not accept bound parameters)
            → map SQL error to SchemaBootstrapError::Sql

    Step 2: Return Ok(())
```

**Note:** `PRAGMA user_version = N` cannot use `?` parameter binding—the value must be embedded in the SQL string directly using `format!`. This is safe because `version` is an `i64`, not user-supplied text.

**Spoiler threshold:** High—complete DDL structure with the PRAGMA limitation explained.
