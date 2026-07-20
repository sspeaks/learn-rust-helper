# Quest 2: Reactor Calibration

**🎮 Quest:** A reactor core needs calibration. Given a base output, drift factor, and efficiency percentage, compute the final calibrated output. You'll work with integers and arithmetic operators.

## Objective

Implement the `calibrate_reactor` function to apply drift and efficiency calculations to a reactor's base output. This teaches variable binding, arithmetic operations, and working with numeric types.

## Public API

```rust
pub struct CalibrationInput {
    pub base_output: i32,
    pub drift: i32,
    pub efficiency_percent: u8,
}

pub fn calibrate_reactor(base_output: i32, drift: i32, efficiency_percent: u8) -> i32 {
    // Your implementation
}

pub fn calibrate_batch(inputs: &[CalibrationInput]) -> Vec<i32> {
    // Calls your function above
}
```

## Behavioral Rules

From `src/lib.rs`, the function must:

1. Accept base output (i32), drift (i32), and efficiency as a percentage (u8: 0–100).
2. Compute: `(base_output + drift) * efficiency_percent / 100`
3. Return the final calibrated output as an i32.
4. Use integer arithmetic (no floating-point).

## Concepts Practiced

- **Variables:** Binding and naming values
- **Arithmetic:** Addition, multiplication, division with integers
- **Type coercion:** Working with i32 and u8 in calculations
- **Expressions:** Return values from functions

## Edge Cases

- Drift can be negative, reducing base output.
- Efficiency is 0–100; 50% efficiency means half power.
- Integer division truncates (e.g., 3 / 2 = 1, not 1.5).
- Large values may overflow i32; tests use reasonable ranges.

## How to Work on This Quest

**From inside `nix develop path:.`:**

```bash
# Check your work
learn check ex02-reactor-calibration

# Get a hint if stuck
learn hint ex02-reactor-calibration

# Or see the next hint level
learn hint ex02-reactor-calibration --level 2
```

**Omit the exercise ID to use your current recommendation:**

```bash
learn check    # Checks current exercise
learn hint     # Hints current exercise
```

## XP Reward

**110 XP** for first completion.

## Prerequisites

Complete **Format Scoreboard** (ex01).

## Success Criteria

- All test cases pass.
- Negative drift is applied correctly.
- Efficiency percentage reduces output proportionally.
- Integer division is used (not floating-point).

## Next Steps

Complete this quest to unlock **Access Control**, where you'll practice booleans and pattern matching.
