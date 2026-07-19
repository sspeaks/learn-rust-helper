## Hint 2: Tools & Types

- **`str::lines()` and `.split('|')`:** Parse manifest lines into fields.
- **`.parse::<T>()`:** Convert strings to u32, u8, bool; returns Result.
- **`.collect::<Vec<_>>()`:** Gather parsed items into a Vec.
- **`.iter().filter(...).map(...).collect()`:** Aggregate metrics (count, sum).
- **`.sort_by(|a, b| b.priority.cmp(&a.priority))`:** Sort descending by priority.
- **Result propagation:** Use `?` operator or `match` to return errors early.

Break into steps: parse → validate → aggregate → sort → return plan or error.

**Spoiler threshold:** Medium—names the tools, not the logic.
