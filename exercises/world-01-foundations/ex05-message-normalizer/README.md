# Quest 5: Message Normalizer

**🎮 Quest:** Call signs need normalization: consistent spacing and casing. Implement a function to clean up a call sign string and return an owned copy. You'll practice String and &str distinctions.

## Objective

Implement the `normalize_call_sign` function to clean a call sign: trim whitespace, collapse internal spaces to single spaces, and convert to uppercase. This teaches String ownership and common text operations.

## Public API

```rust
pub fn normalize_call_sign(input: &str) -> String {
    // Your implementation
}

pub fn normalize_manifest(manifest: &[&str]) -> Vec<String> {
    // Calls your function above
}
```

## Behavioral Rules

From `src/lib.rs`, the function must:

1. Accept a borrowed string slice (`&str`).
2. Trim leading/trailing whitespace.
3. Collapse multiple internal spaces into single spaces.
4. Convert to uppercase.
5. Return an owned `String`.

Example: `"  alpha   bravo  "` → `"ALPHA BRAVO"`

## Concepts Practiced

- **String vs &str:** Borrowed slices vs owned Strings
- **Ownership:** Taking ownership of a String via methods like `to_string()`, `to_uppercase()`
- **String methods:** `trim()`, `split_whitespace()`, `to_uppercase()`, `join()`
- **Iteration:** Collecting and joining strings

## Edge Cases

- Input may have leading/trailing spaces.
- Input may have multiple spaces between words.
- Input may be all spaces (normalize to empty string).
- Unicode characters should be preserved (if any).

## How to Work on This Quest

**From inside `nix develop path:.`:**

```bash
# Check your work
learn check ex05-message-normalizer

# Get a hint if stuck
learn hint ex05-message-normalizer

# Or see the next hint level
learn hint ex05-message-normalizer --level 2
```

**Omit the exercise ID to use your current recommendation:**

```bash
learn check    # Checks current exercise
learn hint     # Hints current exercise
```

## XP Reward

**140 XP** for first completion.

## Prerequisites

Complete **Energy Loop** (ex04).

## Success Criteria

- Whitespace is trimmed and collapsed correctly.
- Output is uppercase.
- Return type is owned `String`.
- All test cases pass.

## Next Steps

Complete this quest to unlock **Slice Telemetry**, where you'll practice references and slices.
