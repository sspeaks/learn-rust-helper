# Quest 33: Data Migration

**🎮 Quest:** The archive has been running for months, and the schema needs to evolve. Adding new columns, renaming fields, or restructuring tables requires ordered migrations that transition the database from one version to the next. Build the migration engine.

## Objective

Implement `current_schema_version` (read the current schema version) and `migrate_schema` (apply incremental ALTER TABLE steps until reaching a target version). This exercise teaches schema evolution: reading version state and applying ordered, idempotent migrations.

## Public API

```rust
pub struct MigrationStep {
    pub from_version: i64,
    pub to_version: i64,
    pub description: String,
}

pub enum DataMigrationError {
    Sql(rusqlite::Error),
    UnsupportedVersion(i64),
}

pub fn current_schema_version(conn: &Connection) -> Result<i64, DataMigrationError>

pub fn migrate_schema(
    conn: &Connection,
    target_version: i64,
) -> Result<Vec<MigrationStep>, DataMigrationError>
```

## Behavioral Rules

### `current_schema_version`
1. Read and return the current schema version from `PRAGMA user_version`.
2. Returns `0` if no version has been set yet.
3. Wrap SQL errors in `DataMigrationError::Sql`.

### `migrate_schema`
1. Read `current_schema_version`.
2. If `target_version` is negative, less than the current version, **or greater than the maximum supported version (2)**, return `DataMigrationError::UnsupportedVersion(target_version)`.
3. If `target_version` equals the current version, return `Ok(vec![])` (already at target).
4. For each step from `current_version + 1` to `target_version` (inclusive):
   - Apply the migration: execute the appropriate SQL `ALTER TABLE` statement.
   - Update the schema version to `to_version` using `PRAGMA user_version`.
   - Record the step as a `MigrationStep { from_version, to_version, description }`.
5. All steps must succeed or none are committed — do not leave the database at an intermediate version on failure.
6. Return `Ok(Vec<MigrationStep>)` listing all applied steps in order.

### Supported migrations

| Step | from → to | SQL applied | Description |
|------|-----------|-------------|-------------|
| 1 | 0 → 1 | `CREATE TABLE IF NOT EXISTS archive_records (id INTEGER PRIMARY KEY AUTOINCREMENT, mission_code TEXT NOT NULL, priority INTEGER NOT NULL);` | create archive_records base table |
| 2 | 1 → 2 | `ALTER TABLE archive_records ADD COLUMN archived INTEGER NOT NULL DEFAULT 0;` | add archived flag to archive_records |

## Concepts Practiced

- **`PRAGMA user_version`:** Reading and writing the schema version.
- **Incremental migrations:** Applying steps one at a time from current to target.
- **`ALTER TABLE ... ADD COLUMN`:** The main SQLite DDL for schema evolution.
- **Migration records:** Returning a log of what was applied.
- **Guard against downward migration:** Returning a domain error for unsupported transitions.

## Setup Notes

SQLite uses the **bundled** feature. The first build may take 30–45 seconds. Tests use `Connection::open_in_memory()`. The supported migration versions are listed in the Behavioral Rules section above.

## Edge Cases

- `target_version = 0` when current is already `0` (return `Ok(vec[])`, no steps).
- `target_version < current_version` (return `UnsupportedVersion`).
- `target_version = current_version` (already there; return `Ok(vec![])`, no steps applied).
- A multi-step migration from version 1 to version 3 applies steps 1→2 and 2→3 in order.

## How to Work on This Quest

**From inside `nix develop path:.`:**

```bash
# Check your work
learn check ex33-data-migration

# Get a hint if stuck
learn hint ex33-data-migration

# Jump to a specific hint level
learn hint ex33-data-migration --level 2
```

**Omit the exercise ID to use your current recommendation:**

```bash
learn check
learn hint
```

## XP Reward

**490 XP** for first completion.

## Prerequisites

Complete **Archive Query** (ex32).

## Success Criteria

- `current_schema_version` reads `PRAGMA user_version` and returns the integer.
- `migrate_schema` returns `UnsupportedVersion` for target < current or target > 2 (maximum supported version).
- `migrate_schema` returns `Ok(vec![])` for target == current.
- All steps between current and target are applied in order.
- Schema version is updated to `target_version` after completion.
- Returned `Vec<MigrationStep>` lists each applied step with accurate `from_version`, `to_version`, and `description`.

## Next Steps

Complete this quest to unlock **Archive Capstone** (ex34), the World 6 finale combining async HTTP with synchronous SQLite persistence.
