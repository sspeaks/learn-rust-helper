# Quest 29: Sensor Array

**🎮 Quest:** The station has hundreds of sensors, and calibrating them sequentially takes too long. The rayon data-parallelism library can transform a sequential iterator into a parallel one with a single method change. Calibrate the array at full speed.

## Objective

Implement `calibrate_readings_parallel` and `total_calibrated_power_parallel` using rayon parallel iterators. This exercise teaches data parallelism: applying a transformation to all elements in a collection using the system's thread pool—without manually spawning threads or managing synchronization.

## Public API

```rust
pub struct SensorReading {
    pub sensor_id: String,
    pub raw_value: i64,
}

pub struct CalibratedReading {
    pub sensor_id: String,
    pub calibrated_value: i64,
}

pub fn calibrate_readings_parallel(
    readings: &[SensorReading],
    offset: i64,
) -> Vec<CalibratedReading>

pub fn total_calibrated_power_parallel(readings: &[CalibratedReading]) -> i64
```

## Behavioral Rules

### `calibrate_readings_parallel`
1. Process all `readings` in parallel using rayon.
2. For each reading, produce a `CalibratedReading` with:
   - `sensor_id` cloned from the input.
   - `calibrated_value` = `raw_value + offset`.
3. Return `Vec<CalibratedReading>`.
4. Output order must match input order (rayon's `par_iter` preserves order with `.collect()`).

### `total_calibrated_power_parallel`
1. Sum all `calibrated_value` fields in parallel using rayon.
2. Return the total as `i64`.

## Concepts Practiced

- **`rayon::prelude::*`:** The prelude brings `par_iter`, `par_iter_mut`, and parallel `sum`/`collect` into scope.
- **`.par_iter()`:** Converts a slice reference into a parallel iterator. Works like `.iter()` but distributes work across threads.
- **`.map(...).collect::<Vec<_>>()`:** Parallel map with collect—rayon handles thread distribution and result ordering.
- **`.par_iter().map(|r| r.calibrated_value).sum()`:** Parallel sum.
- **Data parallelism vs. task parallelism:** Rayon processes a uniform operation over all elements; ex26-ex28 dispatched heterogeneous tasks.

## Setup Notes

Concurrency tests are deterministic. Rayon uses a global thread pool and automatically handles work distribution. Tests verify result values, not thread counts or timing. No manual thread management or synchronization code is required in your implementation.

## Edge Cases

- Empty readings slice (both functions return `Vec::new()` / `0i64`).
- Offset of zero (calibrated values equal raw values).
- Negative offset.
- All readings with the same sensor_id.

## How to Work on This Quest

**From inside `nix develop path:.`:**

```bash
# Check your work
learn check ex29-sensor-array

# Get a hint if stuck
learn hint ex29-sensor-array

# Jump to a specific hint level
learn hint ex29-sensor-array --level 2
```

**Omit the exercise ID to use your current recommendation:**

```bash
learn check
learn hint
```

## XP Reward

**440 XP** for first completion.

## Prerequisites

Complete **Mutex Lockdown** (ex28).

## Success Criteria

- `calibrate_readings_parallel` uses rayon parallel iteration.
- `calibrated_value` equals `raw_value + offset` for each reading.
- Output order matches input order.
- `total_calibrated_power_parallel` returns the correct sum.
- Empty slices are handled gracefully.

## Next Steps

Complete this quest to unlock **RwLock Protocol** (ex30), where you'll optimize read-heavy shared state with a read-write lock.
