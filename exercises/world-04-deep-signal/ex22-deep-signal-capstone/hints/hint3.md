## Hint 3: Algorithm Outline

```
async function gather_deep_signals(base_url, nodes):
    Step 1: If nodes is empty → return EmptyNodeList

    Step 2: Map each node to an un-awaited async fetch:
            future = GET "{base_url}/deep-signals/{node}", decode as DeepSignalSample
            Map each error to the node-carrying error variant

    Step 3: join_all(futures).await → Vec<Result<DeepSignalSample, DeepSignalError>>

    Step 4: Collect into Result<Vec<DeepSignalSample>, DeepSignalError>
            (short-circuits on first Err)

    Step 5: Return Ok(samples)

function build_deep_signal_report(samples):
    Step 1: If samples is empty → return EmptyNodeList

    Step 2: Partition samples into healthy_nodes and degraded_nodes
            based on sample.healthy flag; collect node names only

    Step 3: Compute average_strength = sum(all strengths as f64) / count

    Step 4: Return Ok(DeepSignalReport { healthy_nodes, degraded_nodes, average_strength })

async function run_deep_signal_pipeline(base_url, nodes):
    Step 1: samples = gather_deep_signals(base_url, nodes).await?
    Step 2: report = build_deep_signal_report(&samples)?
    Step 3: Return Ok(report)
```

**Spoiler threshold:** High—full algorithm without Rust syntax.
