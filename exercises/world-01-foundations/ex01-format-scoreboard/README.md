# Quest 1: Format Scoreboard

**🎮 Quest:** A tournament scoreboard is offline. Your job: build the `format_scoreboard_line` function to display each player's ranking, name, and score in a clean, fixed-width format.

## Objective

Implement a function that formats a single scoreboard entry as a string in the pattern:
- Rank (1-indexed, zero-padded to 2 digits)
- Player name
- Score (zero-padded to 4 digits, right-aligned)

You'll also see how this integrates into a full scoreboard render.

## Public API

```rust
pub struct ScoreEntry {
    pub player: String,
    pub score: i32,
    pub streak: u32,
}

impl ScoreEntry {
    pub fn new(player: impl Into<String>, score: i32, streak: u32) -> Self { ... }
}

pub fn format_scoreboard_line(player: &str, score: i32, rank: usize) -> String {
    // Your implementation
}

pub fn render_scoreboard(entries: &[ScoreEntry]) -> String {
    // Calls your function above
}
```

## Behavioral Rules

From `src/lib.rs`, the function must:

1. Accept a player name (`&str`), a score (`i32`), and a rank position (`usize`).
2. Return an owned `String` in the format: `"#RR | PlayerName | +SSSS"`
   - `RR` is the rank zero-padded to 2 digits (e.g., `01`, `09`, `10`)
   - `PlayerName` is the player name as-is
   - `SSSS` is the score zero-padded to 4 digits with a leading `+` for positive scores (e.g., `+0042`)
3. Multiple lines are joined with newlines by `render_scoreboard`.

## Concepts Practiced

- **String formatting:** `format!()` macro with format specifiers
- **String ownership:** Creating owned `String` from formatting
- **Positional parameters:** Using rank index and formatting rules

## Edge Cases

- Scores can be negative; display as `-SSSS` (e.g., `-0005`).
- Player names can contain spaces, punctuation, and Unicode.
- Ranks are 1-indexed (first player is rank 1, not 0).

## Commands to Run

```bash
# Verify your implementation
cargo xtask verify ex01-format-scoreboard

# Or use Cargo directly
cargo test -p ex01-format-scoreboard

# Get a hint if stuck
cargo xtask hint ex01-format-scoreboard
```

## XP Reward

**100 XP** for first completion.

## Prerequisites

None—this is the first quest.

## Success Criteria

- `cargo check --workspace` passes (should already).
- `cargo test -p ex01-format-scoreboard` runs all tests and all pass.
- Player names, ranks, and scores display in the exact format.
- Negative scores display correctly.

## Next Steps

Complete this quest to unlock **Reactor Calibration**, where you'll practice variables and arithmetic.
