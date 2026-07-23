# Quest 19: Beacon Ping

**🎮 Quest:** The station's automated health monitor has gone dark. You have a beacon network scattered across the sector—query one node and prove the connection is alive. Your first real network call awaits.

## Objective

Implement `ping_beacon` to send a synchronous HTTP GET request and return the response status code, endpoint URL, and body as a `BeaconPing` struct. This exercise teaches the ureq synchronous HTTP client: building a request, reading a response, and mapping network errors into your own error type.

## Public API

```rust
pub struct BeaconPing {
    pub endpoint: String,   // full URL that was requested
    pub status: u16,        // HTTP status code
    pub body: String,       // response body as text
}

pub enum BeaconPingError {
    Request(ureq::Error),
    ReadBody(std::io::Error),
}

pub fn ping_beacon(base_url: &str, beacon_id: &str) -> Result<BeaconPing, BeaconPingError>
```

## Behavioral Rules

1. **Construct the URL:** Combine `base_url` and `beacon_id` to form the path `/beacons/{beacon_id}`.
2. **Send a GET request** using `ureq::get`.
3. **Read the response body** as a `String`. An I/O failure reading the body maps to `BeaconPingError::ReadBody`. Invalid UTF-8 in the body also maps to `BeaconPingError::ReadBody`.
4. **Return `BeaconPing`** with:
   - `endpoint`: the full URL string that was requested
   - `status`: the HTTP response status code
   - `body`: the response body text
5. **Map errors:** Any transport or HTTP-level failure from ureq maps to `BeaconPingError::Request`.

## Concepts Practiced

- **ureq:** Minimal sync HTTP client—no async required.
- **String formatting:** Constructing a URL from parts.
- **Result:** Propagating two distinct error kinds.
- **Error mapping:** Converting library errors into your own enum variants.

## Setup Notes

Tests use a local **wiremock** mock server spun up by the test harness. No internet connection or API key is required. The mock server binds on an ephemeral port on `127.0.0.1` and is torn down after each test.

## Edge Cases

- `beacon_id` with a trailing slash in `base_url`.
- A server that responds with a non-2xx status (the function returns `Err(BeaconPingError::Request(...))` carrying the ureq error, which preserves the status code in `ureq::Error::Status`).
- A body that arrives in multiple chunks (let ureq handle this).

## How to Work on This Quest

**From inside `nix develop path:.`:**

```bash
# Check your work
learn check ex19-beacon-ping

# Get a hint if stuck
learn hint ex19-beacon-ping

# Jump to a specific hint level
learn hint ex19-beacon-ping --level 2
```

**Omit the exercise ID to use your current recommendation:**

```bash
learn check
learn hint
```

## XP Reward

**320 XP** for first completion.

## Prerequisites

Complete **Lifetime Observer** (ex18).

## Success Criteria

- `ping_beacon` sends a GET request to the correct URL.
- Returned `BeaconPing.endpoint` matches the URL constructed from `base_url` and `beacon_id`.
- Returned `BeaconPing.status` matches the HTTP response status.
- Returned `BeaconPing.body` contains the response body text.
- Transport errors produce `BeaconPingError::Request`; body read errors produce `BeaconPingError::ReadBody`.

## Next Steps

Complete this quest to unlock **Packet Decode** (ex20), where you'll deserialize the JSON payloads arriving over those beacon connections.
