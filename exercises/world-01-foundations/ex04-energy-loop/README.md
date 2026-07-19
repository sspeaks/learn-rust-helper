# Quest 4: Energy Loop

**🎮 Quest:** Energy is harvested over a range of cycles. Given a range (e.g., cycles 5 to 10), compute total energy harvested. You'll practice ranges and loops.

## Objective

Implement the `total_harvest` function to sum energy across a cycle range. This teaches iteration, range types, and accumulation.

## Public API

```rust
use std::ops::RangeInclusive;

pub fn total_harvest(cycles: RangeInclusive<u32>) -> u32 {
    // Your implementation
}

pub fn mission_harvest_report(missions: &[RangeInclusive<u32>]) -> Vec<u32> {
    // Calls your function above
}
```

## Behavioral Rules

From `src/lib.rs`, the function must:

1. Accept a `RangeInclusive<u32>` (e.g., `5..=10` includes both 5 and 10).
2. For each cycle in the range, harvest energy equal to the cycle number.
3. Return the total as u32.
4. **Energy formula:** Sum of all cycle numbers in the range. (e.g., 5..=7 → 5 + 6 + 7 = 18)

## Concepts Practiced

- **Ranges:** `RangeInclusive` and iteration
- **Loops:** `for` loops over ranges
- **Accumulation:** Summing values in a loop
- **Type signatures:** Working with generic range types

## Edge Cases

- Range can be a single cycle (e.g., `5..=5` → 5).
- Range boundaries are inclusive (both start and end included).
- Large ranges should still compute correctly.

## Commands to Run

```bash
cargo xtask verify ex04-energy-loop
cargo test -p ex04-energy-loop
cargo xtask hint ex04-energy-loop
```

## XP Reward

**130 XP** for first completion.

## Prerequisites

Complete **Access Control** (ex03).

## Success Criteria

- Correct sum for any given range.
- Handles single-cycle ranges.
- Works with inclusive range semantics.

## Next Steps

Complete this quest to unlock **Message Normalizer**, where you'll practice String and &str.
