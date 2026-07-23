# Quest 16: Telemetry Methods

**🎮 Quest:** Your station already has basic struct methods from ex09. Now your telemetry module needs richer method receiver patterns: associated constructors, mutable update methods, read-only analysis methods, and a consuming final report.

## Objective

Implement the `TelemetryBuffer` methods to support initialization, bounded recording, averaging, and final reporting.

## Public API

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TelemetryBuffer {
    pub label: String,
    pub capacity: usize,
    pub readings: Vec<i32>,
}

impl TelemetryBuffer {
    pub fn with_capacity(label: impl Into<String>, capacity: usize) -> Self;
    pub fn record(&mut self, reading: i32) -> bool;
    pub fn average(&self) -> Option<f64>;
    pub fn into_report(self) -> String;
}
```

## Behavioral Rules

1. `with_capacity` creates a `TelemetryBuffer` with:
   - `label` set from the input,
   - `capacity` set from the input,
   - `readings` initialized empty.
2. `record` appends `reading` only when `readings.len() < capacity`.
   - Return `true` when the reading is appended.
   - Return `false` when the buffer is already full, leaving `readings` unchanged.
3. `average` returns:
   - `None` when no readings exist,
   - `Some(mean)` where `mean` is arithmetic average as `f64` otherwise.
4. `into_report` consumes `self` and returns:
   - `"<label>:0@n/a"` when empty,
   - `"<label>:<count>@<avg>"` when non-empty, with `<avg>` formatted to exactly one decimal place.

## Concepts Practiced

- Extending `impl` blocks beyond ex09 basics.
- Receiver choices: associated function, `&mut self`, `&self`, and consuming `self`.
- State transitions through methods.
- Numeric conversion for mean calculations.

## Edge Cases

- Capacity can be zero: all `record` calls must return `false`.
- Negative readings are valid and must affect the average.
- Full buffer must reject additional readings without modifying existing values.
- Empty buffer report must be exactly `"<label>:0@n/a"`.

## How to Work on This Quest

**From inside `nix develop path:.`:**

```bash
learn check ex16-telemetry-methods
learn hint ex16-telemetry-methods
learn hint ex16-telemetry-methods --level 2
```

## XP Reward

**260 XP** for first completion.

## Prerequisites

Complete **Salvage Capstone** (ex15).

## Success Criteria

- All methods compile and satisfy the behavior above.
- `record` enforces capacity exactly.
- `average` uses `Option` correctly for empty vs non-empty data.
- `into_report` produces exact expected strings.

## Next Steps

Complete this quest to unlock **Generic Samples** (ex17).
