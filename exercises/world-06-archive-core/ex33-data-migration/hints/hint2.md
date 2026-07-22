## Hint 2: Tools & Types

- **`conn.query_row("PRAGMA user_version", [], |r| r.get::<_, i64>(0))`:**
  Reads the current schema version. Wrap error as `DataMigrationError::Sql`.
- **`conn.execute(&format!("PRAGMA user_version = {}", version), [])`:**
  Updates the version. PRAGMA does not accept `?` parameter binding—use `format!`.
- **`conn.execute("ALTER TABLE archive_records ADD COLUMN ...", [])`:**
  The primary DDL for adding columns. SQLite supports a limited set of ALTER TABLE operations.
- **Versioned match:** Use `match from_version` (or a `for` loop from `current+1` to `target`) to select the correct SQL for each migration step.
- **`MigrationStep`:** Build one per applied step with `from_version: n - 1`, `to_version: n`, and a human-readable `description`.

**Spoiler threshold:** Medium—names the read/write PRAGMA pattern and the loop structure.
