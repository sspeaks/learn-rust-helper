# Quest 11: Log Analyzer

**🎮 Quest:** Parse sensor logs and extract failure summaries using iterators. Filter and map over events without explicit loops. Learn functional iteration patterns.

## Objective

Implement `summarize_failures` to collect failure descriptions from a log using iterator adapters. This teaches `Vec`, iterators, `.filter()`, `.map()`, and `.collect()`.

## Public API

```rust
pub struct LogEvent {
    pub system: String,
    pub success: bool,
    pub code: u16,
}

pub fn summarize_failures(events: &[LogEvent], max_items: usize) -> Vec<String> {
    // Your implementation
}
```

## Behavioral Rules

From `src/lib.rs`, the function must:

1. Accept a slice of `LogEvent` and a maximum number of items to return.
2. Filter to events where `success == false`.
3. Map each failure to a summary string: `"{system}: error code {code}"`
4. Return up to `max_items` summaries as a `Vec<String>`.
5. If fewer failures than `max_items`, return all.

## Concepts Practiced

- **Vectors:** `Vec<T>` for collecting results.
- **Slices:** Borrowing immutable arrays of events.
- **Iterator adapters:** `.iter()`, `.filter()`, `.map()`, `.take()`, `.collect()`.
- **Closure syntax:** `|event| ...` in filter and map.
- **Trait methods:** Using standard library iterator methods.

## Edge Cases

- No failures in the log.
- Fewer failures than `max_items`.
- `max_items` is 0 (return empty vector).
- System names with special characters.

## Commands to Run

```bash
cargo xtask verify ex11-log-analyzer
cargo test -p ex11-log-analyzer
cargo xtask hint ex11-log-analyzer
```

## XP Reward

**200 XP** for first completion.

## Prerequisites

Complete **Command Router** (ex10).

## Success Criteria

- Failures are filtered correctly.
- Summaries are formatted exactly as `"{system}: error code {code}"`.
- At most `max_items` summaries are returned.
- Uses iterator adapters (no explicit loops required).

## Next Steps

Complete this quest to unlock **Loot Counter**, where you'll practice hash maps.
