## Hint 2: Tools & Types

- **`serde_json::from_str::<Packet>(json_payload)`:** Deserializes the JSON string into a `Packet`. Returns `Result<Packet, serde_json::Error>`.
- **`.map_err(PacketDecodeError::InvalidJson)`:** Convert the `serde_json::Error` into your error type.
- **`packet.id.is_empty()`:** Check whether the `id` field is zero-length after deserialization.
- **`?` operator:** Use after `map_err` to return early on deserialize failures.
- **For the batch:** Iterate over the slice, call `decode_packet` on each element, and use `?` to propagate the first error. Collect successes into a `Vec`.

**Spoiler threshold:** Medium—names the key functions and methods.
