# Quest 17: Generic Samples

**🎮 Quest:** One telemetry algorithm is not enough. You need reusable logic that works for many sample types without copy-pasting implementations.

## Objective

Implement generic functions over `SampleWindow<T>` using standard-library trait bounds.

## Public API

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SampleWindow<T> {
    pub source: String,
    pub samples: Vec<T>,
}

pub fn newest_sample<T: Clone>(window: &SampleWindow<T>) -> Option<T>;
pub fn strongest_sample<T: PartialOrd + Copy>(window: &SampleWindow<T>) -> Option<T>;
pub fn format_window<T: Display>(window: &SampleWindow<T>) -> String;
```

## Behavioral Rules

1. `newest_sample`:
   - Return `None` when `samples` is empty.
   - Otherwise return a cloned copy of the last sample.
2. `strongest_sample`:
   - Return `None` when `samples` is empty.
   - Otherwise return the maximum sample value.
   - On ties, keep the first maximum encountered.
3. `format_window`:
   - Return `"<source> []"` for empty sample lists.
   - Return `"<source> [v1, v2, ...]"` for non-empty sample lists.
   - Values use each sample's `Display` output.

## Concepts Practiced

- Generic struct usage with `SampleWindow<T>`.
- Generic function type parameters.
- Practical trait bounds: `Clone`, `PartialOrd + Copy`, and `Display`.
- Reusing one implementation across multiple concrete types.

## Edge Cases

- Empty windows for all three functions.
- Signed numeric values (including negatives).
- Non-numeric ordered types (for example, `char`) in `strongest_sample`.
- String-like values in `newest_sample` requiring cloning.

## How to Work on This Quest

**From inside `nix develop path:.`:**

```bash
learn check ex17-generic-samples
learn hint ex17-generic-samples
learn hint ex17-generic-samples --level 2
```

## XP Reward

**280 XP** for first completion.

## Prerequisites

Complete **Telemetry Methods** (ex16).

## Success Criteria

- Functions compile with generic signatures and stated bounds.
- Behavior matches the required output for empty and non-empty inputs.
- `strongest_sample` works across multiple concrete ordered types.

## Next Steps

Complete this quest to unlock **Lifetime Observer** (ex18).
