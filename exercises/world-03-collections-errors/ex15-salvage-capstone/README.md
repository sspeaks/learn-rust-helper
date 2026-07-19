# Quest 15: Salvage Capstone

**🎮 Quest:** The final mission combines all prior concepts. Parse a salvage manifest, validate contents, aggregate metrics, and enforce capacity limits. You'll use structs, enums, Result, Vec, pattern matching, and custom errors.

## Objective

Implement `build_salvage_plan` to parse a manifest format, aggregate item data, validate constraints, and return a plan or detailed error. This is a capstone synthesizing Foundations, Ownership, and Collections concepts.

## Public API

```rust
pub struct SalvageItem {
    pub name: String,
    pub mass: u32,
    pub priority: u8,
    pub fragile: bool,
}

pub struct SalvagePlan {
    pub total_mass: u32,
    pub fragile_count: usize,
    pub top_targets: Vec<String>,
}

pub enum SalvageError {
    EmptyManifest,
    InvalidLine { line: usize, reason: String },
    OverCapacity { capacity: u32, total_mass: u32 },
}

pub fn build_salvage_plan(manifest_lines: &[&str], capacity: u32) -> Result<SalvagePlan, SalvageError> {
    // Your implementation
}
```

## Behavioral Rules

From `src/lib.rs`, the function must:

1. **Parse manifest lines** in format: `name|mass|priority|fragile` (tab or pipe-delimited).
   - `name`: String (item name)
   - `mass`: u32 (weight in units)
   - `priority`: u8 (0-255)
   - `fragile`: bool (`true` or `false`)

2. **Validate:**
   - Reject if manifest is empty: `EmptyManifest`
   - Reject if a line is malformed: `InvalidLine { line: N, reason: "..." }`
   - Reject if total mass exceeds capacity: `OverCapacity { capacity, total_mass }`

3. **Aggregate** (on success):
   - `total_mass`: Sum of all item masses
   - `fragile_count`: Count of items where `fragile == true`
   - `top_targets`: All item names sorted by priority descending

4. **Return:**
   - `Ok(SalvagePlan { ... })` on success
   - `Err(SalvageError::...)` on any failure

## Concepts Practiced

- **Structs:** Organizing related data.
- **Enums & pattern matching:** Error variants and handling.
- **Result:** Error propagation and recovery.
- **Vectors & iterators:** Collecting and sorting.
- **Parsing:** Splitting strings, converting types.
- **Validation:** Capacity checks, field validation.
- **Ownership:** Transferring names into the plan.

## Edge Cases

- Manifest with one item.
- All items fragile, none fragile.
- Items with equal priority (stable sort).
- Total mass exactly at capacity, or one unit over.
- Item names with spaces, punctuation.
- Priority 0 vs 255 boundaries.

## Commands to Run

```bash
cargo xtask verify ex15-salvage-capstone
cargo test -p ex15-salvage-capstone
cargo xtask hint ex15-salvage-capstone
```

## XP Reward

**300 XP** for first completion.

## Prerequisites

Complete **Config Loader** (ex14).

## Success Criteria

- Manifest is parsed line by line; errors report the line number.
- Capacity constraint is enforced.
- Aggregates total mass and fragile count correctly.
- Top targets contain all items sorted by priority (highest first).
- Error messages are descriptive.

## What's Next?

**Congratulations!** You've completed the learn-rust campaign. You now understand:

- **Foundations:** Functions, variables, control flow, strings.
- **Ownership:** Moves, borrows, mutable references, lifetimes.
- **Collections & Errors:** Vectors, maps, Option, Result, custom errors.

Next steps:
- Write your own small Rust project.
- Read ["The Rust Book"](https://doc.rust-lang.org/book/) to deepen your knowledge.
- Explore [Exercism](https://exercism.org/tracks/rust) for more practice.
- Join the [Rust community](https://www.rust-lang.org/what/community) and build something amazing.

---

**Total campaign XP:** 2,300 | **Your rank:** Legend ◈
