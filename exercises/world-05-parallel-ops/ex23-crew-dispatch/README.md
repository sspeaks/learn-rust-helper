# Quest 23: Crew Dispatch

**🎮 Quest:** The fleet's automated dispatch system needs to process crew tasks simultaneously. Sequential processing won't meet the deadline—you need to run each task in its own thread and collect the results in a deterministic order.

## Objective

Implement `run_crew_dispatch` to spawn one OS thread per `CrewTask`, perform the work inside each thread, join all threads, and return results in the same order as the input. This exercise introduces `std::thread::spawn` and the pattern of collecting thread handles before joining.

## Public API

```rust
pub struct CrewTask {
    pub crew_id: String,
    pub units: u32,
}

pub struct CrewTaskResult {
    pub crew_id: String,
    pub delivered_units: u32,
}

pub enum CrewDispatchError {
    WorkerPanicked,
}

pub fn run_crew_dispatch(tasks: Vec<CrewTask>) -> Result<Vec<CrewTaskResult>, CrewDispatchError>
```

## Behavioral Rules

1. **Spawn one thread per task.** Each thread processes exactly one `CrewTask`.
2. **Thread work:** Produce a `CrewTaskResult` with `crew_id` from the task and `delivered_units` equal to `task.units`.
3. **Join all threads** after spawning them. A panic in any worker thread returns `CrewDispatchError::WorkerPanicked`.
4. **Preserve input order:** Results must appear in the same index position as their input tasks.
5. **Empty input** returns `Ok(vec![])`.

## Concepts Practiced

- **`std::thread::spawn`:** Creating OS threads in Rust.
- **`JoinHandle<T>`:** The handle returned by `spawn`, used to join and retrieve the result.
- **`.join()`:** Blocking until a thread finishes; returns `Result<T, Box<dyn Any + Send>>` where `Err` means the thread panicked.
- **`move` closures:** Transferring ownership of data into a thread closure.
- **Deterministic output:** Collecting handles in order and joining in order preserves result ordering.

## Setup Notes

Concurrency tests are deterministic. Tests verify outcome values, not timing. No `thread::sleep` is used in tests or required in your implementation. The test harness does not depend on thread scheduling order.

## Edge Cases

- Empty task list (return `Ok(vec![])` without spawning threads).
- Single task (spawns one thread, joins it, returns one result).
- Many tasks—each must produce a result in the same index as its input task.

## How to Work on This Quest

**From inside `nix develop path:.`:**

```bash
# Check your work
learn check ex23-crew-dispatch

# Get a hint if stuck
learn hint ex23-crew-dispatch

# Jump to a specific hint level
learn hint ex23-crew-dispatch --level 2
```

**Omit the exercise ID to use your current recommendation:**

```bash
learn check
learn hint
```

## XP Reward

**350 XP** for first completion.

## Prerequisites

Complete **Deep Signal Capstone** (ex22).

## Success Criteria

- One thread spawned per task.
- Results returned in the same order as input tasks.
- A panicking worker returns `CrewDispatchError::WorkerPanicked`.
- Empty input returns `Ok(vec![])`.

## Next Steps

Complete this quest to unlock **Cargo Channel** (ex24), where threads communicate through channels rather than being joined directly.
