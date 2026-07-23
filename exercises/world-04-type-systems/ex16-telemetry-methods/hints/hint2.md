## Hint 2: Tools & Types

- `Vec::new()` for empty readings.
- `self.readings.len()` for capacity checks.
- `self.readings.push(reading)` only when space exists.
- `iter().map(...).sum::<i32>()` to total readings.
- Cast counts/sums to `f64` before division.
- `format!("{}:{}@{:.1}", ...)` for one-decimal reports.

**Spoiler threshold:** Medium—names the standard-library tools you'll likely use.
