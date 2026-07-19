# Quest 8: Borrow Checkpoint

**🎮 Quest:** Turrets need rebalancing in-place. Implement a function that mutably borrows each turret and adjusts its state. Practice mutable references and in-place modification.

## Objective

Implement `rebalance_turrets` to update the charge and overheat state of multiple turrets through mutable borrowing. This teaches mutable references and modifying borrowed data.

## Public API

```rust
pub struct Turret {
    pub callsign: String,
    pub charge: i32,
    pub overheated: bool,
}

pub fn rebalance_turrets(turrets: &mut [Turret], emergency_boost: i32) {
    // Your implementation
}
```

## Behavioral Rules

From `src/lib.rs`, the function must:

1. Accept a mutable slice of turrets and an emergency boost value.
2. For each turret:
   - Add the emergency boost to its charge.
   - If charge exceeds 100, cap it at 100 and set overheated to true.
   - If charge is 100 or less, set overheated to false.
3. Modify turrets in-place; no return value.

## Concepts Practiced

- **Mutable references:** `&mut [T]` borrows for modification.
- **Iteration:** Looping over mutable slices.
- **In-place modification:** Changing struct fields through references.
- **Conditional logic:** Capping values and setting state.

## Edge Cases

- Emergency boost is 0 (turret may already be overheated).
- Emergency boost is negative (drains charge).
- Turret charge goes below 0 (handle the boundary).
- Multiple turrets with different states.

## Commands to Run

```bash
cargo xtask verify ex08-borrow-checkpoint
cargo test -p ex08-borrow-checkpoint
cargo xtask hint ex08-borrow-checkpoint
```

## XP Reward

**170 XP** for first completion.

## Prerequisites

Complete **Move Inventory** (ex07).

## Success Criteria

- Turrets are updated in-place.
- Charge is correctly boosted and capped at 100.
- Overheat flag is set correctly.
- No additional allocations needed.

## Next Steps

Complete this quest to unlock **Station Crew**, where you'll practice structs and methods.
