# Quest 13: Mission Lookup

**🎮 Quest:** Search an active mission catalog and return its reward. The `Option` type lets you represent the possibility of success or failure. Practice using Option for queries.

## Objective

Implement `active_reward_for_code` to find a mission by code and return its reward only if active. This teaches the `Option` type and how to use it for safe queries.

## Public API

```rust
pub struct Mission {
    pub code: String,
    pub reward: u32,
    pub active: bool,
}

pub fn active_reward_for_code(missions: &[Mission], code: &str) -> Option<u32> {
    // Your implementation
}

pub fn reward_or_default(missions: &[Mission], code: &str, default_reward: u32) -> u32 {
    // Provided; calls your function
}
```

## Behavioral Rules

From `src/lib.rs`, the function must:

1. Accept a slice of missions and a mission code to search for.
2. Find the mission with matching code.
3. If found and `active == true`: return `Some(reward)`.
4. Otherwise (not found or not active): return `None`.

## Concepts Practiced

- **Option<T>:** `Some(T)` for success, `None` for failure.
- **Finding in slices:** `.find()` iterator method.
- **Filtering:** Checking conditions before returning.
- **Composing Options:** Using `.and_then()` or nested logic.

## Edge Cases

- Code doesn't exist.
- Code exists but `active == false`.
- Multiple missions with the same code (match first).
- Empty mission list.

## How to Work on This Quest

**From inside `nix develop path:.`:**

```bash
# Check your work
learn check ex13-mission-lookup

# Get a hint if stuck
learn hint ex13-mission-lookup

# Or see the next hint level
learn hint ex13-mission-lookup --level 2
```

**Omit the exercise ID to use your current recommendation:**

```bash
learn check    # Checks current exercise
learn hint     # Hints current exercise
```

## XP Reward

**220 XP** for first completion.

## Prerequisites

Complete **Loot Counter** (ex12).

## Success Criteria

- Returns `Some(reward)` only for active missions with matching code.
- Returns `None` if code not found or mission inactive.
- Handles empty lists correctly.

## Next Steps

Complete this quest to unlock **Config Loader**, where you'll practice `Result` and error handling.
