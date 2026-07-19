## Hint 2: Tools & Types

- **`.find(|m| m.code == code)`:** Iterator method that returns `Option<&Mission>` (first match or None).
- **`.and_then()`:** Chain Option operations; if Some, apply a function; if None, pass through None.
- **`.map()`:** Transform the inner value (e.g., extract reward) without changing None.
- **Match on Option:** Pattern match `Some(mission)` and `None` to handle both cases.

Combining `.find().and_then()` or nested `.find().map()` is idiomatic.

**Spoiler threshold:** Medium—names the methods, not the exact combination.
