# Quest 18: Lifetime Observer

**🎮 Quest:** Your APIs now return borrowed data instead of owned copies. To keep those references valid, you'll make lifetime relationships explicit in both a function and a borrowing struct.

## Objective

Implement explicit lifetime-based APIs for choosing slices, building a borrowing struct, and returning borrowed prefixes.

## Public API

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MissionView<'a> {
    pub code: &'a str,
    pub captain: &'a str,
}

pub fn longer_label<'a>(left: &'a str, right: &'a str) -> &'a str;
pub fn mission_view<'a>(code: &'a str, captain: &'a str) -> MissionView<'a>;
pub fn clipped_prefix<'a>(text: &'a str, max_len: usize) -> &'a str;
```

## Behavioral Rules

1. `longer_label`:
   - Return whichever input has greater `len()`.
   - When lengths are equal, return `left`.
2. `mission_view`:
   - Return `MissionView { code, captain }` borrowing both inputs directly.
3. `clipped_prefix`:
   - Return `""` when `max_len == 0`.
   - Return all of `text` when `max_len >= text.len()`.
   - Otherwise return `&text[..max_len]`.

## Concepts Practiced

- Explicit lifetime parameters in function signatures.
- Borrowing struct fields with `<'a>`.
- Returning borrowed slices tied to input lifetimes.
- Avoiding unnecessary `String` allocation.

## Edge Cases

- Equal-length strings in `longer_label` (must return left input).
- Zero-length clip in `clipped_prefix`.
- `max_len` larger than input length.
- This quest uses ASCII slice boundaries in tests; `max_len` is interpreted as a byte count.

## How to Work on This Quest

**From inside `nix develop path:.`:**

```bash
learn check ex18-lifetime-observer
learn hint ex18-lifetime-observer
learn hint ex18-lifetime-observer --level 2
```

## XP Reward

**300 XP** for first completion.

## Prerequisites

Complete **Generic Samples** (ex17).

## Success Criteria

- All three functions compile with explicit lifetimes.
- Returned references point into input data (no owned string allocations needed).
- Behavior matches every listed boundary case.

## Next Steps

Complete this quest to unlock **Beacon Ping** (ex19), where networking begins.
