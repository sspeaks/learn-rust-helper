# Quest 3: Access Control

**🎮 Quest:** A station has three security clearance levels. Your job: write a function that returns an access message based on clearance level and duty status. Practice pattern matching and control flow.

## Objective

Implement the `access_message` function to map each (clearance, on_duty) pair to an appropriate gate message. This teaches conditional logic and the `match` expression.

## Public API

```rust
pub enum Clearance {
    Visitor,
    Engineer,
    Captain,
}

pub fn access_message(clearance: Clearance, is_on_duty: bool) -> &'static str {
    // Your implementation
}

pub fn gate_announcement(name: &str, clearance: Clearance, is_on_duty: bool) -> String {
    // Calls your function above
}
```

## Behavioral Rules

From `src/lib.rs`, the function must:

1. Accept a `Clearance` enum and a boolean `is_on_duty`.
2. Return a static string (`&'static str`) with the gate message.
3. Message logic:
   - **Visitor, on duty:** "Welcome, Visitor—enjoy your tour."
   - **Visitor, off duty:** "Visitor pass expired."
   - **Engineer, on duty:** "Welcome, Engineer. Proceed to Bay A."
   - **Engineer, off duty:** "Engineer must check in first."
   - **Captain, on duty:** "Welcome back, Captain. All systems yours."
   - **Captain, off duty:** "Captain, command duty awaits."

## Concepts Practiced

- **Enums:** Pattern matching on enum variants
- **`match` expressions:** Exhaustive pattern coverage
- **Booleans:** Combining boolean conditions
- **String literals:** Using static string references

## Edge Cases

- All six (Clearance, bool) combinations must return distinct messages.
- Messages must match exactly (whitespace, punctuation matter).
- Return type is `&'static str`, not `String` (use literals, not `format!()`).

## How to Work on This Quest

**From inside `nix develop path:.`:**

```bash
# Check your work
learn check ex03-access-control

# Get a hint if stuck
learn hint ex03-access-control

# Or see the next hint level
learn hint ex03-access-control --level 2
```

**Omit the exercise ID to use your current recommendation:**

```bash
learn check    # Checks current exercise
learn hint     # Hints current exercise
```

## XP Reward

**120 XP** for first completion.

## Prerequisites

Complete **Reactor Calibration** (ex02).

## Success Criteria

- All six gate messages match exactly.
- Pattern matching covers all combinations.
- Return type is `&'static str` (string literals).

## Next Steps

Complete this quest to unlock **Energy Loop**, where you'll practice ranges and iteration.
