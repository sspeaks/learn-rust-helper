# Quest 7: Move Inventory

**🎮 Quest:** A supply manifest holds crates. Find a crate by ID, remove it, and transfer ownership to the caller. Practice ownership transfer and mutation.

## Objective

Implement `move_crate_to_shuttle` to locate a crate in a mutable vector, remove it, and hand over ownership. This teaches how `Option`, `Vec::remove()`, and ownership transfer work together.

## Public API

```rust
pub struct SupplyCrate {
    pub id: String,
    pub contents: Vec<String>,
}

pub fn move_crate_to_shuttle(manifest: &mut Vec<SupplyCrate>, crate_id: &str) -> Option<SupplyCrate> {
    // Your implementation
}
```

## Behavioral Rules

From `src/lib.rs`, the function must:

1. Accept a mutable reference to a vector of crates and a crate ID string.
2. Search the vector for a crate with matching ID.
3. If found: remove it from the vector and return `Some(crate)`.
4. If not found: leave the vector unchanged and return `None`.
5. The caller takes ownership of the returned crate.

## Concepts Practiced

- **Ownership transfer:** Moving a value from the vector to the caller.
- **Mutable borrowing:** `&mut Vec<T>` allows removing elements.
- **`Option` type:** `Some(T)` vs `None` for optional results.
- **`Vec::remove()`:** Removing an element by index.

## Edge Cases

- Manifest is empty.
- No crate with the given ID.
- Multiple calls to the same manifest (only removes the first matching crate).
- Crate ID comparison is case-sensitive and exact.

## How to Work on This Quest

**From inside `nix develop path:.`:**

```bash
# Check your work
learn check ex07-move-inventory

# Get a hint if stuck
learn hint ex07-move-inventory

# Or see the next hint level
learn hint ex07-move-inventory --level 2
```

**Omit the exercise ID to use your current recommendation:**

```bash
learn check    # Checks current exercise
learn hint     # Hints current exercise
```

## XP Reward

**160 XP** for first completion.

## Prerequisites

Complete **Slice Telemetry** (ex06).

## Success Criteria

- Found crates are removed from the manifest and returned.
- Missing crates return `None`.
- Manifest is left unchanged if no match.
- Ownership of the crate is transferred to the caller.

## Next Steps

Complete this quest to unlock **Borrow Checkpoint**, where you'll practice mutable borrowing.
