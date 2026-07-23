# Quest 10: Command Router

**🎮 Quest:** A command dispatcher needs to route enum variants to different systems. Pattern-match each command type and generate a routing message. Practice enums and exhaustive matching.

## Objective

Implement `route_command` to match on a `Command` enum and produce routing instructions. This teaches enum pattern matching and how Rust enforces exhaustiveness.

## Public API

```rust
pub enum Command {
    Dock { bay: u8 },
    Launch { window: u8 },
    Broadcast(String),
    Abort,
}

pub fn route_command(command: Command) -> String {
    // Your implementation
}

pub fn route_batch(commands: Vec<Command>) -> Vec<String> {
    // Calls your function above
}
```

## Behavioral Rules

From `src/lib.rs`, the function must:

1. Accept a `Command` enum value (consumed by ownership).
2. Match each variant and return a routing message containing the required content:
   - **Dock { bay }:** Must include the bay number (e.g., `"Routing to Bay {bay}"`).
   - **Launch { window }:** Must include the window number (e.g., `"Launch window {window} locked"`).
   - **Broadcast(msg):** Must include the message payload (e.g., `"Broadcasting: {msg}"`).
   - **Abort:** Must signal an abort condition (e.g., `"Abort signal received"`).
3. Return an owned `String` with the message.

## Concepts Practiced

- **Enums:** Named variants with associated data.
- **Pattern matching:** Exhaustive `match` on enum variants.
- **Named and tuple variants:** `Dock { bay }` vs `Broadcast(String)`.
- **Consuming patterns:** Taking ownership in match arms.

## Edge Cases

- Empty Broadcast message.
- Bay number or window number boundary values (0, 255).
- Abort command (no associated data).

## How to Work on This Quest

**From inside `nix develop path:.`:**

```bash
# Check your work
learn check ex10-command-router

# Get a hint if stuck
learn hint ex10-command-router

# Or see the next hint level
learn hint ex10-command-router --level 2
```

**Omit the exercise ID to use your current recommendation:**

```bash
learn check    # Checks current exercise
learn hint     # Hints current exercise
```

## XP Reward

**190 XP** for first completion.

## Prerequisites

Complete **Station Crew** (ex09).

## Success Criteria

- All four command variants are handled.
- Each routing message contains the required semantic content (bay number, window number, payload, or abort signal).
- Pattern matching is exhaustive (compiler enforces it).

## Next Steps

Complete this quest to unlock **Log Analyzer**, where you'll practice vectors and iterators.
