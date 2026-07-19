# Quest 6: Slice Telemetry

**🎮 Quest:** Sensor telemetry arrives as strings and arrays. Extract portions without allocating new memory using slices. Learn how references and slices give you zero-copy views into data.

## Objective

Implement two functions using `&str` and slice types to extract portions of data without copying. This teaches references, lifetimes through practice, and memory efficiency.

## Public API

```rust
pub fn first_sentence(report: &str) -> &str {
    // Your implementation
}

pub fn trailing_readings(readings: &[i32], count: usize) -> &[i32] {
    // Your implementation
}
```

## Behavioral Rules

From `src/lib.rs`, the functions must:

1. **`first_sentence(report: &str) -> &str`:**
   - Extract the substring up to and including the first period (`.`).
   - If no period, return the entire string.
   - Return a borrowed slice, not an owned String.

2. **`trailing_readings(readings: &[i32], count: usize) -> &[i32]`:**
   - Return the last `count` elements as a slice.
   - If `count >= readings.len()`, return the entire slice.
   - Do not allocate; return a view into the original array.

## Concepts Practiced

- **Borrowed references:** `&str`, `&[T]` for viewing data without ownership.
- **Slicing syntax:** `&s[start..end]`, `&arr[..]`, etc.
- **Lifetimes (implicit):** The returned slice lives as long as the input.
- **Zero-copy efficiency:** No `Vec` or `String` allocation.

## Edge Cases

- Report with no period (return the whole string).
- Empty readings array.
- `count` larger than array length.
- Readings with exactly `count` elements.

## Commands to Run

```bash
cargo xtask verify ex06-slice-telemetry
cargo test -p ex06-slice-telemetry
cargo xtask hint ex06-slice-telemetry
```

## XP Reward

**150 XP** for first completion.

## Prerequisites

Complete **Message Normalizer** (ex05).

## Success Criteria

- First sentence extracted correctly (with period included).
- Trailing readings returned as a slice.
- No allocations (return borrowed slices, not owned collections).
- All edge cases handled.

## Next Steps

Complete this quest to unlock **Move Inventory**, where you'll practice ownership and moves.
