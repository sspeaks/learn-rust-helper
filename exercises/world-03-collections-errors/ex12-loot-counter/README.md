# Quest 12: Loot Counter

**🎮 Quest:** Count salvaged items by type using a hash map. You'll practice `HashMap`, iteration, and the entry API for efficient counting.

## Objective

Implement `count_loot` to build a frequency map of items. This teaches `HashMap<String, usize>`, the entry API, and why maps are useful for grouping.

## Public API

```rust
use std::collections::HashMap;

pub fn count_loot(items: &[&str]) -> HashMap<String, usize> {
    // Your implementation
}

pub fn total_items(counts: &HashMap<String, usize>) -> usize {
    // Provided
}
```

## Behavioral Rules

From `src/lib.rs`, the function must:

1. Accept a slice of item names (`&[&str]`).
2. Count how many times each item name appears.
3. Return a `HashMap<String, usize>` where keys are item names and values are counts.
4. Item names should be converted to owned `String` (keys in the map).

Example: `["gold", "silver", "gold"]` → `{"gold": 2, "silver": 1}`

## Concepts Practiced

- **HashMap:** Key-value storage with O(1) average lookups.
- **The entry API:** `.entry(key).or_insert()` for idiomatic counting.
- **Type inference:** HashMap can infer key and value types.
- **Ownership:** Keys are owned `String`; values are `usize`.

## Edge Cases

- Empty items list.
- Single item repeated many times.
- All items unique.
- Case sensitivity: `"Gold"` and `"gold"` are different keys.

## Commands to Run

```bash
cargo xtask verify ex12-loot-counter
cargo test -p ex12-loot-counter
cargo xtask hint ex12-loot-counter
```

## XP Reward

**210 XP** for first completion.

## Prerequisites

Complete **Log Analyzer** (ex11).

## Success Criteria

- All item names are counted correctly.
- HashMap keys are owned `String`.
- Counts are accurate (`usize` values).
- Empty list returns an empty HashMap.

## Next Steps

Complete this quest to unlock **Mission Lookup**, where you'll practice the `Option` type.
