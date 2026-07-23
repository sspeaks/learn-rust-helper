# Quest 23: Async Handshake

**🎮 Quest:** The deep-sector relay network speaks a newer protocol—one that requires concurrent connections. The synchronous approach won't cut it here. Time to cross the bridge into async Rust: send the handshake request, verify the protocol, and bring back your session credentials.

## Objective

Implement `perform_async_handshake` as an `async fn` using the `reqwest` HTTP client and the `tokio` runtime. This exercise marks the transition from synchronous ureq to async/await: writing async functions, `.await`-ing futures, and mapping `reqwest` errors into your own error type.

## Public API

```rust
pub struct HandshakeReceipt {
    pub session_id: String,
    pub protocol: String,
    pub motd: String,
}

pub enum AsyncHandshakeError {
    Request(reqwest::Error),
    InvalidStatus(reqwest::StatusCode),
    Decode(reqwest::Error),
}

pub async fn perform_async_handshake(
    base_url: &str,
    call_sign: &str,
) -> Result<HandshakeReceipt, AsyncHandshakeError>
```

## Behavioral Rules

1. **Build the URL:** Combine `base_url` and `call_sign` to form `{base_url}/handshake/{call_sign}`.
2. **Send an async GET request** using `reqwest::get(&url)`. Network failures return `AsyncHandshakeError::Request`.
3. **Check the status:** If the HTTP status code is not in the 2xx range, return `AsyncHandshakeError::InvalidStatus(response.status())`.
4. **Decode the JSON body:** Call `.json::<HandshakeReceipt>().await` on the response. A decode failure returns `AsyncHandshakeError::Decode`.
5. **Return** `Ok(HandshakeReceipt { ... })` on success.

## Concepts Practiced

- **`async fn`:** Declaring and writing asynchronous functions.
- **`.await`:** Suspending execution at an async operation.
- **`reqwest`:** The async counterpart to ureq—similar concepts, different runtime.
- **`reqwest::Response::status()`:** Accessing the HTTP status before consuming the response.
- **`reqwest::Response::json::<T>()`:** Deserializing the response body as JSON (async).
- **Sync vs. async:** ureq blocks the thread; reqwest returns a future. Same mental model, different execution.

## Setup Notes

This exercise uses the **Tokio** async runtime. Tests are annotated `#[tokio::test(flavor = "multi_thread")]`. The runtime is provided by the test harness—you only write `async fn` and `.await` calls. A local **wiremock** mock server handles the HTTP endpoint. No internet connection is required.

## Edge Cases

- A `call_sign` that contains URL-special characters (the tests use safe ASCII).
- Server returns 401 (maps to `InvalidStatus(401 Unauthorized)`).
- Server returns 200 but with a malformed JSON body (maps to `Decode`).

## How to Work on This Quest

**From inside `nix develop path:.`:**

```bash
# Check your work
learn check ex23-async-handshake

# Get a hint if stuck
learn hint ex23-async-handshake

# Jump to a specific hint level
learn hint ex23-async-handshake --level 2
```

**Omit the exercise ID to use your current recommendation:**

```bash
learn check
learn hint
```

## XP Reward

**420 XP** for first completion.

## Prerequisites

Complete **Relay Dispatch** (ex22).

## Success Criteria

- Function signature is `pub async fn`.
- GET request is sent to the correct URL.
- Non-2xx responses return `AsyncHandshakeError::InvalidStatus`.
- JSON decode failures return `AsyncHandshakeError::Decode`.
- Transport failures return `AsyncHandshakeError::Request`.
- A valid response returns `Ok(HandshakeReceipt { ... })`.

## Next Steps

Complete this quest to unlock **Channel Broadcast** (ex24), where you'll fire async requests to multiple targets concurrently.
