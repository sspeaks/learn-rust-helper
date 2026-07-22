# Quest 24: Cargo Channel

**🎮 Quest:** The dispatch system has grown: workers and the controller no longer live in the same thread. Cargo receipts need to travel over a message channel—from worker threads back to the main coordinator. Implement the channel-based job dispatch.

## Objective

Implement `dispatch_cargo_jobs` using `std::sync::mpsc` (multi-producer, single-consumer) channels. Worker threads send their results through the channel; the main thread receives and collects them. This exercise teaches channels as an alternative to directly returning values from threads.

## Public API

```rust
pub struct CargoJob {
    pub cargo_id: String,
    pub destination: String,
}

pub struct CargoReceipt {
    pub cargo_id: String,
    pub delivered_to: String,
}

pub enum CargoChannelError {
    SendFailed,
    ReceiveFailed,
    WorkerPanicked,
}

pub fn dispatch_cargo_jobs(jobs: Vec<CargoJob>) -> Result<Vec<CargoReceipt>, CargoChannelError>
```

## Behavioral Rules

1. **Create an mpsc channel** for `CargoReceipt` values.
2. **Spawn one thread per job.** Each worker thread:
   - Processes its `CargoJob`.
   - Constructs a `CargoReceipt` with `cargo_id` from the job and `delivered_to` equal to `job.destination`.
   - Sends the receipt through the channel's sender. A send failure returns `CargoChannelError::SendFailed`.
3. **Drop extra senders** so the receiver can detect when all workers are done.
4. **Collect results** on the main thread by receiving from the channel until it closes. A receive failure returns `CargoChannelError::ReceiveFailed`.
5. **Join all worker threads.** A panic in any worker returns `CargoChannelError::WorkerPanicked`.
6. **Empty input** returns `Ok(vec![])`.

## Concepts Practiced

- **`std::sync::mpsc::channel()`:** Creating a sender/receiver pair.
- **Cloning senders:** `Sender<T>` is `Clone`—each worker gets its own clone.
- **Dropping the original sender:** When all sender clones are dropped, the receiver gets `RecvError` signaling the channel is closed.
- **`receiver.iter()`:** Iterates over all received values until all senders are dropped.
- **Channels vs. join:** Channels allow workers to send partial results before all are done.

## Setup Notes

Concurrency tests are deterministic. Tests verify that all receipts are collected and no errors occur on the happy path. The order of receipts may differ from input order (arrival depends on scheduling); tests that check order sort results first.

## Edge Cases

- Empty job list (no threads spawned, no sends, no receives).
- A single job.
- Many jobs where results arrive in any order.

## How to Work on This Quest

**From inside `nix develop path:.`:**

```bash
# Check your work
learn check ex24-cargo-channel

# Get a hint if stuck
learn hint ex24-cargo-channel

# Jump to a specific hint level
learn hint ex24-cargo-channel --level 2
```

**Omit the exercise ID to use your current recommendation:**

```bash
learn check
learn hint
```

## XP Reward

**380 XP** for first completion.

## Prerequisites

Complete **Crew Dispatch** (ex23).

## Success Criteria

- A channel is used to transport results from worker threads to the main thread.
- Each worker receives exactly one job and sends exactly one receipt.
- All receipts are collected before returning.
- Panicking workers return `CargoChannelError::WorkerPanicked`.
- Empty input returns `Ok(vec![])`.

## Next Steps

Complete this quest to unlock **Mutex Lockdown** (ex25), where multiple threads share and mutate a single piece of state safely.
