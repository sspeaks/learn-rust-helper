# Quest 19: Relay Dispatch

**🎮 Quest:** Listening to the network is only half the battle. Now the relay station needs to send priority instructions outbound. Dispatch a relay request by POSTing JSON to a remote endpoint and parsing the relay receipt that comes back.

## Objective

Implement `dispatch_relay` to serialize a `RelayDispatchRequest` to JSON, POST it to a remote endpoint, and deserialize the `RelayDispatchReceipt` from the response body. This exercise teaches outbound HTTP: serializing request bodies, setting content type headers, and decoding structured responses.

## Public API

```rust
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RelayDispatchRequest {
    pub route: String,
    pub priority: u8,
    pub payload: serde_json::Value,  // arbitrary JSON value
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct RelayDispatchReceipt {
    pub accepted: bool,
    pub relay_id: String,
    pub queued_at: String,
}

pub enum RelayDispatchError {
    Serialize(serde_json::Error),
    Request(ureq::Error),
    ReadBody(std::io::Error),
    Decode(serde_json::Error),
}

pub fn dispatch_relay(
    base_url: &str,
    request: &RelayDispatchRequest,
) -> Result<RelayDispatchReceipt, RelayDispatchError>
```

## Behavioral Rules

1. **Serialize the request:** Convert `request` to a JSON string with `serde_json::to_string`. A serialization failure returns `RelayDispatchError::Serialize`.
2. **POST to `{base_url}/relay/dispatch`:** Construct the endpoint by appending `/relay/dispatch` to `base_url` (which has no trailing slash). Send the JSON string as the body with content type `application/json`.
3. **Map request errors:** Any transport or HTTP-level failure from ureq returns `RelayDispatchError::Request`.
4. **Read the response body:** Call `into_string()` on the response. I/O failure returns `RelayDispatchError::ReadBody`.
5. **Decode the receipt:** Parse the body string into `RelayDispatchReceipt` with `serde_json`. Failure returns `RelayDispatchError::Decode`.
6. **Return** `Ok(RelayDispatchReceipt { ... })` on success.

## Concepts Practiced

- **`serde::Serialize`:** Converting a Rust struct to JSON.
- **HTTP POST:** Sending a request body with a content-type header.
- **ureq `.send_string`:** Sending a raw string body with specified content type.
- **Round-trip JSON:** Serialize outbound, deserialize inbound.
- **`serde_json::Value`:** Working with arbitrary JSON payloads in a struct field.

## Setup Notes

Tests use a local **wiremock** mock server. The mock validates that the POST body is valid JSON and returns a canned receipt. No internet connection is required.

## Edge Cases

- `payload` field is a nested JSON object or array.
- Server returns an unexpected 201 status (treat as success if ureq doesn't error).
- Request JSON is serialized correctly even when `payload` is `serde_json::Value::Null`.

## How to Work on This Quest

**From inside `nix develop path:.`:**

```bash
# Check your work
learn check ex19-relay-dispatch

# Get a hint if stuck
learn hint ex19-relay-dispatch

# Jump to a specific hint level
learn hint ex19-relay-dispatch --level 2
```

**Omit the exercise ID to use your current recommendation:**

```bash
learn check
learn hint
```

## XP Reward

**390 XP** for first completion.

## Prerequisites

Complete **Signal Timeout** (ex18).

## Success Criteria

- Request body is serialized as valid JSON before being sent.
- POST is sent to `{base_url}/relay/dispatch` with `Content-Type: application/json`.
- Serialization failures return `RelayDispatchError::Serialize`.
- Transport/HTTP failures return `RelayDispatchError::Request`.
- Body read failures return `RelayDispatchError::ReadBody`.
- Decode failures return `RelayDispatchError::Decode`.
- A successful round trip returns `Ok(RelayDispatchReceipt { ... })`.

## Next Steps

Complete this quest to unlock **Async Handshake** (ex20), where the station switches from synchronous HTTP to Tokio-powered async requests.
