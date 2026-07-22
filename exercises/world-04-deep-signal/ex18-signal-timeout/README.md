# Quest 18: Signal Timeout

**🎮 Quest:** Sector nodes sometimes go silent—or worse, respond slowly enough to stall the whole relay chain. You need a timeout-aware GET request that distinguishes between a slow network, a bad HTTP status, and a JSON decode failure.

## Objective

Implement `fetch_signal_with_timeout` to send a GET request with a configurable timeout, decode the JSON response body into `SignalEnvelope`, and map every failure mode into the appropriate `SignalTimeoutError` variant. This exercise deepens your ureq error-handling skills: distinguishing transport errors from HTTP errors from decode errors.

## Public API

```rust
pub struct SignalEnvelope {
    pub node_id: String,
    pub status: String,
    pub latency_ms: u64,
}

pub enum SignalTimeoutError {
    TimedOut,
    HttpStatus(u16),
    Transport(ureq::Transport),
    Decode(serde_json::Error),
    ReadBody(std::io::Error),
}

pub fn fetch_signal_with_timeout(
    base_url: &str,
    timeout: Duration,
) -> Result<SignalEnvelope, SignalTimeoutError>
```

## Behavioral Rules

1. **Configure the timeout:** Build a ureq `Agent` with the given `timeout` applied to the request.
2. **Request URL:** Send a GET to `{base_url}/signal`.
3. **Map ureq errors:**
   - A timeout is reported as `ureq::Error::Transport` whose inner `ureq::Transport::kind()` is `ureq::ErrorKind::Io` with an underlying `io::ErrorKind::TimedOut` — return `SignalTimeoutError::TimedOut`.
   - Any non-2xx HTTP status — return `SignalTimeoutError::HttpStatus(status_code)`.
   - Other transport errors — return `SignalTimeoutError::Transport(transport)`.
4. **Read the body:** Call `into_string()` on the response. An I/O failure returns `SignalTimeoutError::ReadBody`.
5. **Decode JSON:** Parse the body string into `SignalEnvelope` using `serde_json`. Failure returns `SignalTimeoutError::Decode`.

## Concepts Practiced

- **ureq `AgentBuilder`:** Configuring timeout on an HTTP agent rather than a one-off call.
- **Error variant inspection:** Matching on `ureq::Error` sub-types to distinguish timeout vs. HTTP status vs. transport.
- **Layered error mapping:** Different error types at each stage (transport, I/O, JSON).
- **`serde_json::from_str`:** Deserializing a pre-read body string.

## Setup Notes

Tests use a local **wiremock** mock server. The mock simulates timeouts by delaying responses beyond the configured duration. No internet connection is required. The timeout duration in tests is chosen to be reliably short without flaking.

## Edge Cases

- Timeout duration of zero (immediate timeout).
- Server responds with 503 (maps to `HttpStatus(503)`, not `TimedOut`).
- Server returns valid JSON but unexpected fields (ignored by serde).
- Body contains valid envelope data at non-200 status (still returns `HttpStatus` error—do not decode bodies on error responses).

## How to Work on This Quest

**From inside `nix develop path:.`:**

```bash
# Check your work
learn check ex18-signal-timeout

# Get a hint if stuck
learn hint ex18-signal-timeout

# Jump to a specific hint level
learn hint ex18-signal-timeout --level 2
```

**Omit the exercise ID to use your current recommendation:**

```bash
learn check
learn hint
```

## XP Reward

**370 XP** for first completion.

## Prerequisites

Complete **Packet Decode** (ex17).

## Success Criteria

- Timeout is configured on the ureq agent (not hard-coded).
- A timed-out request returns `SignalTimeoutError::TimedOut`.
- A non-2xx response returns `SignalTimeoutError::HttpStatus(code)`.
- Body read failures return `SignalTimeoutError::ReadBody`.
- JSON decode failures return `SignalTimeoutError::Decode`.
- A valid 200 response with correct JSON returns `Ok(SignalEnvelope { ... })`.

## Next Steps

Complete this quest to unlock **Relay Dispatch** (ex19), where you'll send data outbound via HTTP POST.
