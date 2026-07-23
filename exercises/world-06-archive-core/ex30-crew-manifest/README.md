# Quest 30: Crew Manifest

**🎮 Quest:** The schema exists. Now the archive needs crew records. Insert a crew member, then retrieve the full manifest. These two operations—one write, one read—are the foundation of every database-backed application.

## Objective

Implement `insert_crew_member` (parameterized INSERT) and `load_crew_manifest` (SELECT all rows mapped to Rust structs). This exercise teaches the core rusqlite CRUD loop: binding parameters, executing statements, and mapping row data back to typed Rust values.

## Public API

```rust
pub struct CrewManifestEntry {
    pub crew_id: String,
    pub role: String,
    pub rank: u8,
}

pub enum CrewManifestError {
    Sql(rusqlite::Error),
}

pub fn insert_crew_member(
    conn: &Connection,
    entry: &CrewManifestEntry,
) -> Result<(), CrewManifestError>

pub fn load_crew_manifest(conn: &Connection) -> Result<Vec<CrewManifestEntry>, CrewManifestError>
```

## Behavioral Rules

### `insert_crew_member`
1. Insert one row into the `crew_manifest` table.
2. Bind all three fields (`crew_id`, `role`, `rank`) as parameters. **Never interpolate user values directly into SQL strings** (parameterized queries prevent SQL injection).
3. Return `Ok(())` on success; wrap any SQL error in `CrewManifestError::Sql`.

### `load_crew_manifest`
1. Select all rows from the `crew_manifest` table ordered by `crew_id` ascending.
2. Map each row to a `CrewManifestEntry` struct.
3. Return `Ok(Vec<CrewManifestEntry>)`.
4. Wrap any SQL error in `CrewManifestError::Sql`.

## Concepts Practiced

- **`conn.execute("INSERT ...", params![a, b, c])`:** Parameterized INSERT.
- **`rusqlite::params![]`:** Binds typed Rust values to SQL `?` placeholders safely.
- **`conn.prepare("SELECT ...")`:** Compiles a query into a reusable `Statement`.
- **`stmt.query_map([], |row| { ... })`:** Iterates over result rows and maps each to a value.
- **`row.get::<_, T>(index)`:** Extracts a typed value from a row column by index.
- **Collecting rows:** Use `.collect::<Result<Vec<_>, _>>()?` to short-circuit on the first row error.

## Setup Notes

SQLite uses the **bundled** feature—no system installation needed. The first build may take 30–45 seconds. Each test uses `Connection::open_in_memory()` with a freshly bootstrapped schema. Tests call your `bootstrap_archive_schema` (or create the table inline) before calling your functions, so you don't need to create the table in these functions.

## Edge Cases

- Inserting the same `crew_id` twice when the table has a `UNIQUE` constraint (SQL error propagated).
- `load_crew_manifest` on an empty table (returns `Ok(vec![])`, not an error).
- `rank` is stored as an integer in SQLite and retrieved as `u8`.

## How to Work on This Quest

**From inside `nix develop path:.`:**

```bash
# Check your work
learn check ex30-crew-manifest

# Get a hint if stuck
learn hint ex30-crew-manifest

# Jump to a specific hint level
learn hint ex30-crew-manifest --level 2
```

**Omit the exercise ID to use your current recommendation:**

```bash
learn check
learn hint
```

## XP Reward

**410 XP** for first completion.

## Prerequisites

Complete **Schema Bootstrap** (ex29).

## Success Criteria

- `insert_crew_member` inserts exactly one row with correctly bound parameters.
- `load_crew_manifest` returns all rows as `CrewManifestEntry` structs.
- Fields are correctly mapped from SQLite column types to Rust types.
- Empty table returns `Ok(vec![])`.
- SQL errors are wrapped in `CrewManifestError::Sql`.

## Next Steps

Complete this quest to unlock **Mission Ledger** (ex31), where you'll apply database transactions and enforce business rules.
