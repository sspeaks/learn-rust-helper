## Hint 2: Tools & Types

- **SQL for prefix match:** `mission_code LIKE ?1 || '%'` or pass the prefix with `%` appended: `format!("{}%", filter.mission_prefix)`.
- **SQL for `include_archived = false`:** Add `AND (archived = 0 OR ?N = 1)` where `?N` is bound to `filter.include_archived as i64`. This lets one query handle both cases without string building.
  - Alternatively, build two separate SQL strings with `if filter.include_archived`.
- **Binding `u8`:** Cast to `i64` for SQLite: `filter.min_priority as i64`.
- **Binding `bool`:** Cast to `i64`: `filter.include_archived as i64`.
- **Row mapping:** `row.get::<_, i64>(3).map(|v| v != 0)` — converts INTEGER to `bool`.
- **`conn.prepare(&sql)?` + `stmt.query_map(params![...], |row| {...})`:** Standard pattern.

**Spoiler threshold:** Medium—names every binding strategy and row mapping.
