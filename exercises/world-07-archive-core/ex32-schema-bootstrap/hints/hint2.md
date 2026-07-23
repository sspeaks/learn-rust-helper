## Hint 2: Tools & Types

- **`conn.execute("SQL", [])`:** Runs a DDL or DML statement. Returns `Result<usize, rusqlite::Error>`. Map errors to `SchemaBootstrapError::Sql`.
- **`CREATE TABLE IF NOT EXISTS table_name (...)`:** Standard idempotent DDL.
- **`PRAGMA user_version = N;`:** Sets SQLite's built-in version slot. Read with `PRAGMA user_version;`.
- **`conn.query_row("PRAGMA user_version", [], |row| row.get::<_, i64>(0))`:** Reads the current version.
- **`rusqlite::params![]`:** The macro for passing typed parameters to SQL statements (use `[]` when there are none).
- **Error mapping:** `.map_err(SchemaBootstrapError::Sql)?` after each `execute` or `query_row` call.

**Spoiler threshold:** Medium—names the exact rusqlite methods and PRAGMA syntax.
