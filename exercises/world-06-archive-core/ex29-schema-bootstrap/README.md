# Quest 29: Schema Bootstrap

**🎮 Quest:** The fleet's long-term memory is a SQLite archive. Before any data can be stored, the database needs its tables and a version record. You'll write the initialization routine that bootstraps the schema from scratch.

## Objective

Implement `bootstrap_archive_schema` (create tables and indexes) and `ensure_schema_version` (persist a version number in SQLite metadata). This exercise introduces rusqlite: executing DDL statements and reading/writing SQLite metadata.

## Public API

```rust
pub enum SchemaBootstrapError {
    Sql(rusqlite::Error),
}

pub fn bootstrap_archive_schema(conn: &Connection) -> Result<(), SchemaBootstrapError>

pub fn ensure_schema_version(
    conn: &Connection,
    version: i64,
) -> Result<(), SchemaBootstrapError>
```

## Behavioral Rules

### `bootstrap_archive_schema`
1. Create the base archive tables. At minimum, create:
   - An `archive_records` table with columns: `id INTEGER PRIMARY KEY AUTOINCREMENT`, `mission_code TEXT NOT NULL`, `priority INTEGER NOT NULL`, `archived INTEGER NOT NULL DEFAULT 0`.
   - Any indexes the tests require (check `tests/solve.rs` for table names and column expectations).
2. Use `CREATE TABLE IF NOT EXISTS` so the function is idempotent.
3. Any SQL error wraps in `SchemaBootstrapError::Sql`.

### `ensure_schema_version`
1. Persist the `version` in SQLite's built-in `PRAGMA user_version = N` mechanism or in a dedicated `schema_version` table.
2. Subsequent calls with the same version should be a no-op (idempotent).
3. Any SQL error wraps in `SchemaBootstrapError::Sql`.

## Concepts Practiced

- **`rusqlite::Connection::execute`:** Run a SQL statement that returns no rows.
- **`CREATE TABLE IF NOT EXISTS`:** Idempotent DDL.
- **`PRAGMA user_version`:** SQLite's built-in integer slot for schema versioning.
- **Error wrapping:** Convert `rusqlite::Error` to your own error enum.
- **`&Connection` parameter:** Functions take a shared reference—the connection is managed by the caller.

## Setup Notes

SQLite uses the **bundled** feature in this project—no system SQLite installation is needed. **The first build of World 6 exercises may take 30–45 seconds** while the bundled C library compiles. This is expected and only happens once per clean build; subsequent builds are fast.

Tests use `Connection::open_in_memory()` for full isolation. Each test gets a fresh, empty database. No test files are created on disk.

## Edge Cases

- Calling `bootstrap_archive_schema` twice on the same connection (idempotent with `IF NOT EXISTS`).
- Calling `ensure_schema_version` with the same version twice (second call is a no-op).
- The connection is valid; error paths are triggered by intentionally malformed SQL in tests.

## How to Work on This Quest

**From inside `nix develop path:.`:**

```bash
# Check your work
learn check ex29-schema-bootstrap

# Get a hint if stuck
learn hint ex29-schema-bootstrap

# Jump to a specific hint level
learn hint ex29-schema-bootstrap --level 2
```

**Omit the exercise ID to use your current recommendation:**

```bash
learn check
learn hint
```

## XP Reward

**380 XP** for first completion.

## Prerequisites

Complete **Parallel Ops Capstone** (ex28).

## Success Criteria

- `bootstrap_archive_schema` creates all required tables without error.
- The function is idempotent: calling it twice does not fail.
- `ensure_schema_version` persists the version number.
- Calling `ensure_schema_version` twice with the same version succeeds.
- All SQL errors are wrapped in `SchemaBootstrapError::Sql`.

## Next Steps

Complete this quest to unlock **Crew Manifest** (ex30), where you'll insert and retrieve rows from your newly created tables.
