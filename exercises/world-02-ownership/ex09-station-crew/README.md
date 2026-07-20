# Quest 9: Station Crew

**🎮 Quest:** Build a crew member record with rank badges. Implement methods on a struct to construct members, promote them, and generate display badges. Practice struct impl blocks.

## Objective

Implement three methods on the `CrewMember` struct: `new()`, `promote()`, and `badge()`. This teaches struct methods, self borrowing, and field mutation.

## Public API

```rust
pub struct CrewMember {
    pub name: String,
    pub role: String,
    pub level: u8,
}

impl CrewMember {
    pub fn new(name: impl Into<String>, role: impl Into<String>, level: u8) -> Self {
        // Your implementation
    }

    pub fn promote(&mut self, new_role: impl Into<String>) {
        // Your implementation
    }

    pub fn badge(&self) -> String {
        // Your implementation
    }
}
```

## Behavioral Rules

From `src/lib.rs`, the methods must:

1. **`new(name, role, level) -> Self`:**
   - Construct a CrewMember with the given fields.
   - Accept `impl Into<String>` for name and role (allows `&str` or `String`).

2. **`promote(&mut self, new_role) -> ()`:**
   - Update the member's role to `new_role`.
   - Increment level by 1.
   - Cap level at 99 (max level).

3. **`badge(&self) -> String`:**
   - Return a formatted badge like `"[L05] Nova — Navigator"`.
   - Format: `"[L{:02}] {name} — {role}"` (level zero-padded to 2 digits).

## Concepts Practiced

- **Impl blocks:** Defining methods on structs.
- **Self, &self, &mut self:** Different forms of borrowing.
- **Trait bounds:** `impl Into<String>` for flexibility.
- **String formatting:** Building display strings.

## Edge Cases

- Promotion at level 99 (stay at 99, don't overflow).
- New members at level 0.
- Roles and names with spaces or special characters.

## How to Work on This Quest

**From inside `nix develop path:.`:**

```bash
# Check your work
learn check ex09-station-crew

# Get a hint if stuck
learn hint ex09-station-crew

# Or see the next hint level
learn hint ex09-station-crew --level 2
```

**Omit the exercise ID to use your current recommendation:**

```bash
learn check    # Checks current exercise
learn hint     # Hints current exercise
```

## XP Reward

**180 XP** for first completion.

## Prerequisites

Complete **Borrow Checkpoint** (ex08).

## Success Criteria

- `new()` creates a member with the right fields.
- `promote()` increments level and updates role, capping at 99.
- `badge()` returns the exact format `"[L##] Name — Role"`.
- `impl Into<String>` trait bound works for `&str` and `String`.

## Next Steps

Complete this quest to unlock **Command Router**, where you'll practice enums and pattern matching.
