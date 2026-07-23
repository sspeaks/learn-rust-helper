## Hint 2: Tools & Types

- **`conn.execute("INSERT INTO crew_manifest (crew_id, role, rank) VALUES (?1, ?2, ?3)", params![entry.crew_id, entry.role, entry.rank as i64])`:**
  Inserts one row. `params![]` binds Rust values to positional placeholders.
- **Note:** SQLite has no native `u8` type. Store `rank` as `i64` (or `i32`) and cast when reading back.
- **`conn.prepare("SELECT crew_id, role, rank FROM crew_manifest ORDER BY crew_id")`:**
  Compiles the SELECT into a `Statement`.
- **`stmt.query_map([], |row| { ... })`:**
  Iterates over result rows. The closure maps each `Row` to a value.
- **`row.get::<_, String>(0)`**, **`row.get::<_, String>(1)`**, **`row.get::<_, i64>(2)`:**
  Extract typed values by column index.
- **`.collect::<rusqlite::Result<Vec<_>>>()?`:** Collects the iterator, propagating the first error.

**Spoiler threshold:** Medium—names every method and the u8↔i64 cast pattern.
