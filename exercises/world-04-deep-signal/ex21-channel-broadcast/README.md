# Quest 21: Channel Broadcast

**🎮 Quest:** The relay net has dozens of channels. Polling them one-by-one would take too long—you need to broadcast to all of them simultaneously. Implement concurrent async requests so all channels are queried at once, not sequentially.

## Objective

Implement `fetch_broadcast_receipt` (single channel) and `broadcast_channels` (all channels concurrently). `broadcast_channels` must fire all requests at the same time and collect results without awaiting them sequentially. This exercise teaches structured async concurrency: launching multiple futures and joining them.

## Public API

```rust
pub struct BroadcastTarget {
    pub channel: String,
}

pub struct BroadcastReceipt {
    pub channel: String,
    pub acknowledged: bool,
}

pub enum ChannelBroadcastError {
    Request(reqwest::Error),
    InvalidStatus {
        channel: String,
        status: reqwest::StatusCode,
    },
    Decode(reqwest::Error),
}

pub async fn fetch_broadcast_receipt(
    base_url: &str,
    target: &BroadcastTarget,
) -> Result<BroadcastReceipt, ChannelBroadcastError>

pub async fn broadcast_channels(
    base_url: &str,
    targets: &[BroadcastTarget],
) -> Result<Vec<BroadcastReceipt>, ChannelBroadcastError>
```

## Behavioral Rules

### `fetch_broadcast_receipt`
1. Send a GET request to `{base_url}/broadcast/{channel}`.
2. Network failures return `ChannelBroadcastError::Request`.
3. Non-2xx status returns `ChannelBroadcastError::InvalidStatus { channel: target.channel.clone(), status }`.
4. Decode the JSON body as `BroadcastReceipt`. Decode failures return `ChannelBroadcastError::Decode`.

### `broadcast_channels`
1. Create one future per target by calling `fetch_broadcast_receipt`.
2. **Launch all futures concurrently**, not sequentially.
3. Await all futures and collect results.
4. If any channel fails, return the first error encountered.
5. On full success, return `Ok(Vec<BroadcastReceipt>)` preserving input order.

## Concepts Practiced

- **`futures::future::join_all`** or **`tokio::join!`:** Launching multiple futures simultaneously.
- **Structured concurrency:** All tasks start before any result is awaited.
- **Sequential vs. concurrent:** A `.for` loop with `.await` inside processes *one at a time*—use join utilities to run concurrently.
- **Important:** A sequential `for` loop will produce correct output in existing tests because result ordering masks the timing difference. This is intentional: the *learning objective* is structured concurrency. Implement `broadcast_channels` with `join_all` (or equivalent) so all requests are in-flight simultaneously.
- **Collecting results:** Turning a `Vec<Result<...>>` into a `Result<Vec<...>>`.

## Setup Notes

Tests use a local **wiremock** mock server. Concurrency is verified by the test harness through mock expectations (all channels receive exactly one request). No internet connection is required. Use `#[tokio::test(flavor = "multi_thread")]` for test functions.

## Edge Cases

- Empty `targets` slice (return `Ok(vec![])` immediately).
- Single target (behaves like a direct call to `fetch_broadcast_receipt`).
- One target fails while others succeed (return the error).
- All targets fail (return the first error).

## How to Work on This Quest

**From inside `nix develop path:.`:**

```bash
# Check your work
learn check ex21-channel-broadcast

# Get a hint if stuck
learn hint ex21-channel-broadcast

# Jump to a specific hint level
learn hint ex21-channel-broadcast --level 2
```

**Omit the exercise ID to use your current recommendation:**

```bash
learn check
learn hint
```

## XP Reward

**450 XP** for first completion.

## Prerequisites

Complete **Async Handshake** (ex20).

## Success Criteria

- `fetch_broadcast_receipt` correctly maps all three error variants.
- `broadcast_channels` launches all futures simultaneously (not sequentially).
- Output order matches input order.
- An empty targets slice returns `Ok(vec![])`.
- A single failure returns `Err` regardless of how many targets succeeded.

## Next Steps

Complete this quest to unlock **Deep Signal Capstone** (ex22), the World 4 finale combining async HTTP with data analysis.
