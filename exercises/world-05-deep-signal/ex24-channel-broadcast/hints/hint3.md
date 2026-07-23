## Hint 3: Algorithm Outline

```
async function fetch_broadcast_receipt(base_url, target):
    Step 1: Send GET to "{base_url}/broadcast/{target.channel}"
            → on error, return ChannelBroadcastError::Request

    Step 2: Check status — if not 2xx:
            → return ChannelBroadcastError::InvalidStatus { channel, status }

    Step 3: Decode body as BroadcastReceipt via .json().await
            → on error, return ChannelBroadcastError::Decode

    Step 4: Return Ok(receipt)

async function broadcast_channels(base_url, targets):
    Step 1: Map targets to un-awaited futures:
            for each target → fetch_broadcast_receipt(base_url, target)
            Collect into Vec<Future>   ← do NOT .await here

    Step 2: Wait for all futures concurrently:
            results = join_all(futures).await
            → this gives Vec<Result<BroadcastReceipt, ChannelBroadcastError>>

    Step 3: Convert to Result<Vec<_>, _> using .collect()
            → short-circuits on first Err

    Step 4: Return the collected result
```

**Critical:** Creating the future and awaiting it are separate steps. All futures must be created before any is awaited to achieve real concurrency.

**Spoiler threshold:** High—algorithm with the concurrency pattern explained clearly.
