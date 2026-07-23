# Quest 31: Parallel Ops Capstone

**🎮 Quest:** The fleet's processing pipeline has two stages: transform each job's payload, then aggregate checksums across all results. Workers run in threads, data flows through channels, and a shared accumulator tracks the running total. Combine everything you've learned—threads, channels, and locks—into one deterministic pipeline.

## Objective

Implement `run_parallel_pipeline` to build a two-stage multithreaded pipeline: Stage 1 workers transform jobs and send outputs via a channel; Stage 2 accumulates a running checksum in shared state. Return a `ParallelPipelineReport` with all outputs and the final checksum.

## Public API

```rust
pub struct PipelineJob {
    pub job_id: String,
    pub payload: i32,
}

pub struct PipelineStageOutput {
    pub job_id: String,
    pub stage_one: i32,   // payload * 2
    pub stage_two: i32,   // payload * 3
}

pub struct ParallelPipelineReport {
    pub outputs: Vec<PipelineStageOutput>,
    pub checksum: i32,    // sum of all stage_two values
}

pub enum ParallelOpsError {
    WorkerPanicked,
    ChannelClosed,
    LockPoisoned,
}

pub fn run_parallel_pipeline(
    jobs: Vec<PipelineJob>,
) -> Result<ParallelPipelineReport, ParallelOpsError>
```

## Behavioral Rules

1. **Stage 1 — Transform:** Spawn one thread per job. Each worker computes:
   - `stage_one = payload * 2`
   - `stage_two = payload * 3`
   - Sends a `PipelineStageOutput` through an mpsc channel.
2. **Stage 2 — Accumulate:** On the main thread, receive outputs from the channel. For each output, add `stage_two` to a shared `Arc<Mutex<i32>>` checksum.
3. **After collection:** Sort outputs by `job_id` (ascending) to guarantee deterministic order.
4. **Final report:** Return `ParallelPipelineReport { outputs, checksum }` where `checksum` is the sum of all `stage_two` values.
5. **Error mapping:**
   - Worker panic → `ParallelOpsError::WorkerPanicked`
   - Channel send/receive failure → `ParallelOpsError::ChannelClosed`
   - Mutex poison → `ParallelOpsError::LockPoisoned`

## Concepts Practiced

- **Multi-tool combination:** Threads + channels + `Arc<Mutex<T>>` in one function.
- **Deterministic output:** Sorting by `job_id` after parallel collection.
- **Staged pipeline:** Workers produce; the main thread consumes and aggregates.
- **Error taxonomy:** Distinguishing thread, channel, and lock failures.

## Setup Notes

Concurrency tests are deterministic. Outputs are sorted by `job_id` after collection, so test assertions do not depend on thread scheduling order. The checksum is independent of collection order (addition is commutative). No `thread::sleep` is used.

## Edge Cases

- Empty `jobs` slice: return `Ok(ParallelPipelineReport { outputs: vec![], checksum: 0 })`.
- Single job.
- Jobs with the same `job_id` (tests use unique IDs).

## How to Work on This Quest

**From inside `nix develop path:.`:**

```bash
# Check your work
learn check ex31-parallel-ops-capstone

# Get a hint if stuck
learn hint ex31-parallel-ops-capstone

# Jump to a specific hint level
learn hint ex31-parallel-ops-capstone --level 2
```

**Omit the exercise ID to use your current recommendation:**

```bash
learn check
learn hint
```

## XP Reward

**520 XP** for first completion.

## Prerequisites

Complete **RwLock Protocol** (ex30).

## Success Criteria

- Stage 1 computes `stage_one = payload * 2` and `stage_two = payload * 3`.
- Results arrive via mpsc channel.
- Checksum equals sum of all `stage_two` values.
- Outputs sorted by `job_id` ascending.
- All three error variants are handled.
- Empty input returns a zero checksum and empty outputs.

## What's Next?

**Congratulations!** You've mastered World 6: Parallel Ops. You can now:

- **Spawn threads** and collect results in deterministic order.
- **Use mpsc channels** to move data between threads without shared state.
- **Protect shared state** with `Arc<Mutex<T>>` and `Arc<RwLock<T>>`.
- **Apply data parallelism** with rayon's parallel iterators.

Continue to **World 7: Archive Core** for persistent storage with SQLite.

---

**World 6 XP:** 2,560 | **Rank at this point:** Architect ⬟ (7,500+ XP cumulative)
