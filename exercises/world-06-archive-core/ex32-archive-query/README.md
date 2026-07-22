# Quest 32: Archive Query

**🎮 Quest:** The archive holds thousands of mission records. Retrieving all of them every time is wasteful—you need targeted queries. Build a filter that lets callers specify minimum priority, a mission code prefix, and whether to include archived records.

## Objective

Implement `query_archive` to execute a dynamic filtered SELECT query using an `ArchiveFilter`. This exercise teaches parameterized WHERE clauses, conditional query logic, and mapping multi-column rows to Rust structs.

## Public API

```rust
pub struct ArchiveRecord {
    pub id: i64,
    pub mission_code: String,
    pub priority: u8,
    pub archived: bool,
}

pub struct ArchiveFilter {
    pub min_priority: u8,
    pub mission_prefix: String,
    pub include_archived: bool,
}

pub enum ArchiveQueryError {
    Sql(rusqlite::Error),
}

pub fn query_archive(
    conn: &Connection,
    filter: &ArchiveFilter,
) -> Result<Vec<ArchiveRecord>, ArchiveQueryError>
```

## Behavioral Rules

1. **Filter by `min_priority`:** Return only records where `priority >= filter.min_priority`.
2. **Filter by `mission_prefix`:** Return only records where `mission_code` starts with `filter.mission_prefix`. Use SQL `LIKE 'prefix%'` or `GLOB 'prefix*'`—both work; use parameterized binding to avoid injection.
3. **Filter by `include_archived`:** If `filter.include_archived` is `false`, exclude rows where `archived = 1`. If `true`, return all records regardless of `archived`.
4. **Map each row** to `ArchiveRecord { id, mission_code, priority: priority as u8, archived: archived != 0 }`.
5. **Wrap any SQL error** in `ArchiveQueryError::Sql`.

## Concepts Practiced

- **Parameterized WHERE clauses:** Binding filter values to `?` placeholders.
- **Conditional filtering:** The `include_archived` flag changes the query's WHERE clause.
- **`bool` vs. `INTEGER`:** SQLite stores booleans as 0/1; map back to Rust `bool` when reading.
- **`conn.prepare` + `query_map`:** Compiling a query and iterating over rows.

## Setup Notes

SQLite uses the **bundled** feature. The first build may take 30–45 seconds. Tests use `Connection::open_in_memory()`. The test harness seeds the database with known records before calling `query_archive`.

## Edge Cases

- `min_priority = 0` (includes all priorities).
- Empty `mission_prefix` string (matches all mission codes).
- `include_archived = true` with no archived records (returns all active records).
- Query matching no rows (return `Ok(vec![])`, not an error).

## How to Work on This Quest

**From inside `nix develop path:.`:**

```bash
# Check your work
learn check ex32-archive-query

# Get a hint if stuck
learn hint ex32-archive-query

# Jump to a specific hint level
learn hint ex32-archive-query --level 2
```

**Omit the exercise ID to use your current recommendation:**

```bash
learn check
learn hint
```

## XP Reward

**470 XP** for first completion.

## Prerequisites

Complete **Mission Ledger** (ex31).

## Success Criteria

- Records with `priority < min_priority` are excluded.
- Records whose `mission_code` does not start with `mission_prefix` are excluded.
- When `include_archived = false`, records with `archived = true` are excluded.
- Row mapping correctly converts SQLite integer `archived` to Rust `bool`.
- No records matching the filter returns `Ok(vec![])`.
- SQL errors wrap in `ArchiveQueryError::Sql`.

## Next Steps

Complete this quest to unlock **Data Migration** (ex33), where you'll evolve the schema with incremental ALTER TABLE migrations.
