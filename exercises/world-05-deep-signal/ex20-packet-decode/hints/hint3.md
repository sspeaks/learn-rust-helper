## Hint 3: Algorithm Outline

```
function decode_packet(json_payload):
    Step 1: Deserialize json_payload into Packet
            → on serde_json error, return PacketDecodeError::InvalidJson

    Step 2: Check if packet.id is empty
            → if so, return PacketDecodeError::EmptyPacketId

    Step 3: Return Ok(packet)

function decode_packet_batch(json_payloads):
    Step 1: Create an empty Vec<Packet> for results

    Step 2: For each payload in json_payloads:
            → call decode_packet(payload)
            → if Err, return that error immediately (use ? or match)
            → if Ok, push the Packet into the results Vec

    Step 3: Return Ok(results)
```

**Note:** `decode_packet_batch` does not collect then validate—it fails on the first error and never processes later payloads. This "fail-fast" pattern is idiomatic when any failure invalidates the whole batch.

**Spoiler threshold:** High—clear algorithm without exact Rust syntax.
