# Quest 25: Deep Signal Capstone

**🎮 Quest:** The sector network is live, async, and broadcasting. Now the Admiral wants a full health report: query every node in the fleet, classify which are healthy vs. degraded, and compute the average signal strength—all in one pipeline. This is World 5's capstone.

## Objective

Implement three functions that form a complete async data pipeline:
- `gather_deep_signals`: collect per-node samples via async HTTP
- `build_deep_signal_report`: analyze samples into a health report
- `run_deep_signal_pipeline`: orchestrate both steps end-to-end

## Public API

```rust
pub struct DeepSignalSample {
    pub node: String,
    pub strength: i32,
    pub healthy: bool,
}

pub struct DeepSignalReport {
    pub healthy_nodes: Vec<String>,
    pub degraded_nodes: Vec<String>,
    pub average_strength: f64,
}

pub enum DeepSignalError {
    EmptyNodeList,
    Request { node: String, source: reqwest::Error },
    InvalidStatus { node: String, status: reqwest::StatusCode },
    Decode { node: String, source: reqwest::Error },
}

pub async fn gather_deep_signals(
    base_url: &str,
    nodes: &[String],
) -> Result<Vec<DeepSignalSample>, DeepSignalError>

pub fn build_deep_signal_report(
    samples: &[DeepSignalSample],
) -> Result<DeepSignalReport, DeepSignalError>

pub async fn run_deep_signal_pipeline(
    base_url: &str,
    nodes: &[String],
) -> Result<DeepSignalReport, DeepSignalError>
```

## Behavioral Rules

### `gather_deep_signals`
1. If `nodes` is empty, return `DeepSignalError::EmptyNodeList`.
2. For each node, send a GET request to `{base_url}/deep-signals/{node}`.
3. Network failures return `DeepSignalError::Request { node: node.to_string(), source }`.
4. Non-2xx responses return `DeepSignalError::InvalidStatus { node: node.to_string(), status }`.
5. Decode failures return `DeepSignalError::Decode { node: node.to_string(), source }`.
6. Requests may be sent concurrently. Return samples in the same order as `nodes`.

### `build_deep_signal_report`
1. If `samples` is empty, return `DeepSignalError::EmptyNodeList`.
2. Partition nodes into `healthy_nodes` (where `sample.healthy == true`) and `degraded_nodes` (where `sample.healthy == false`). Preserve input order within each partition.
3. Compute `average_strength` as the mean of all `sample.strength` values as `f64`.
4. Return `Ok(DeepSignalReport { ... })`.

### `run_deep_signal_pipeline`
1. Call `gather_deep_signals(base_url, nodes).await` and propagate any error.
2. Call `build_deep_signal_report(&samples)` and propagate any error.
3. Return the report.

## Concepts Practiced

- **Async pipeline:** Combining async data gathering with synchronous analysis.
- **Struct-attached errors:** Error variants carrying node identity for diagnostics.
- **Concurrent collection:** Gathering results from multiple async sources.
- **Data partitioning:** Splitting a slice into two groups by predicate.
- **Floating-point averages:** Computing mean from integer data.

## Setup Notes

Tests use a local **wiremock** mock server for HTTP. Tests are annotated `#[tokio::test(flavor = "multi_thread")]`. No internet connection is required. `build_deep_signal_report` is purely synchronous—no mock server needed for that function alone.

## Edge Cases

- All nodes healthy (degraded list is empty, vice versa).
- Single node.
- `average_strength` computed correctly when strengths are negative.
- `gather_deep_signals` with an empty nodes slice returns `EmptyNodeList`.
- `build_deep_signal_report` with an empty samples slice returns `EmptyNodeList`.

## How to Work on This Quest

**From inside `nix develop path:.`:**

```bash
# Check your work
learn check ex25-deep-signal-capstone

# Get a hint if stuck
learn hint ex25-deep-signal-capstone

# Jump to a specific hint level
learn hint ex25-deep-signal-capstone --level 2
```

**Omit the exercise ID to use your current recommendation:**

```bash
learn check
learn hint
```

## XP Reward

**500 XP** for first completion.

## Prerequisites

Complete **Channel Broadcast** (ex24).

## Success Criteria

- `gather_deep_signals` returns `EmptyNodeList` for empty input.
- Node errors carry the node name in the error variant.
- Samples are returned in the same order as input nodes.
- `build_deep_signal_report` correctly partitions healthy vs. degraded nodes.
- `average_strength` is a correctly computed floating-point mean.
- `run_deep_signal_pipeline` composes both steps end-to-end.

## What's Next?

**Congratulations!** You've mastered World 5: Deep Signal. You now know:

- **Sync HTTP:** ureq GET and POST, error mapping, timeouts.
- **JSON:** serde_json deserialization and the Serialize/Deserialize derive macros.
- **Async Rust:** async fn, .await, reqwest, and the Tokio runtime.
- **Concurrent futures:** Running multiple async requests in parallel with join_all.

Continue to **World 6: Parallel Ops** for threads, channels, and shared mutable state.

---

**World 5 XP:** 2,800 | **Rank at this point:** Admiral ⬡ (5,500+ XP cumulative)
