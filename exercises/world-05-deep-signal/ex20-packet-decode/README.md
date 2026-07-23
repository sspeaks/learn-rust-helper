# Quest 20: Packet Decode

**🎮 Quest:** The beacon network is alive—but signals arrive as raw JSON strings, and the relay station can't act on raw text. You need a reliable decoder that turns JSON payloads into structured `Packet` values and rejects anything malformed.

## Objective

Implement `decode_packet` and `decode_packet_batch` to deserialize JSON into a `Packet` struct, rejecting payloads with empty `id` fields. This exercise teaches `serde` deserialization, `serde_json`, and batch error propagation—no network required.

## Public API

```rust
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Packet {
    pub id: String,
    pub signal: i32,
    pub active: bool,
}

pub enum PacketDecodeError {
    InvalidJson(serde_json::Error),
    EmptyPacketId,
}

pub fn decode_packet(json_payload: &str) -> Result<Packet, PacketDecodeError>

pub fn decode_packet_batch(json_payloads: &[&str]) -> Result<Vec<Packet>, PacketDecodeError>
```

## Behavioral Rules

1. **`decode_packet`:**
   - Deserialize `json_payload` into a `Packet`. A parse failure returns `PacketDecodeError::InvalidJson`.
   - If the deserialized `Packet.id` is empty (zero-length), return `PacketDecodeError::EmptyPacketId`.
   - On success, return `Ok(Packet { ... })`.

2. **`decode_packet_batch`:**
   - Call `decode_packet` for each payload in order.
   - Return the first error encountered (fail fast).
   - On complete success, return `Ok(Vec<Packet>)` preserving input order.

## Concepts Practiced

- **`serde::Deserialize`:** Auto-derive deserialization for a struct.
- **`serde_json::from_str`:** Parse a JSON string into a typed Rust value.
- **Custom validation:** Post-deserialization checks that the library cannot express.
- **Batch error propagation:** Stopping at the first failure in a collection.

## Setup Notes

This exercise involves no network calls. The test harness passes raw JSON strings directly. No mock server, no internet, no external process required.

## Edge Cases

- JSON missing a required field (`id`, `signal`, or `active`).
- JSON with extra unknown fields (serde ignores these by default).
- An `id` field that is present but is an empty string `""`.
- A whitespace-only `id` is **not** empty (only zero-length is rejected).
- Mixed batch where valid packets precede an invalid one.

## How to Work on This Quest

**From inside `nix develop path:.`:**

```bash
# Check your work
learn check ex20-packet-decode

# Get a hint if stuck
learn hint ex20-packet-decode

# Jump to a specific hint level
learn hint ex20-packet-decode --level 2
```

**Omit the exercise ID to use your current recommendation:**

```bash
learn check
learn hint
```

## XP Reward

**350 XP** for first completion.

## Prerequisites

Complete **Beacon Ping** (ex19).

## Success Criteria

- `decode_packet` correctly deserializes valid JSON into a `Packet`.
- `decode_packet` returns `EmptyPacketId` when `id` is `""`.
- `decode_packet` returns `InvalidJson` for malformed JSON or missing fields.
- `decode_packet_batch` returns `Ok` only when all payloads are valid.
- `decode_packet_batch` stops at the first error and returns it.
- Output order matches input order on success.

## Next Steps

Complete this quest to unlock **Signal Timeout** (ex21), where you'll configure timeouts and map the full spectrum of HTTP errors.
