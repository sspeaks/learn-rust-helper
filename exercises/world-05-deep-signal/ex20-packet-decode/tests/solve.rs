use ex20_packet_decode::{decode_packet, decode_packet_batch, Packet, PacketDecodeError};

fn is_stub_panic(e: &Box<dyn std::any::Any + Send>) -> bool {
    e.downcast_ref::<&str>()
        .is_some_and(|s| s.contains("not yet implemented"))
        || e.downcast_ref::<String>()
            .is_some_and(|s| s.contains("not yet implemented"))
}

macro_rules! call_or_hint {
    ($ex:expr, $fn:expr, $body:expr) => {{
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| $body)) {
            Ok(v) => v,
            Err(e) => {
                if is_stub_panic(&e) {
                    panic!(
                        "\n\n  ✖  {} '{}' not started — fill in src/lib.rs\n",
                        $ex, $fn
                    );
                }
                std::panic::resume_unwind(e)
            }
        }
    }};
}

#[test]
fn decode_packet_parses_valid_payload() {
    let payload = r#"{"id":"pkt-01","signal":42,"active":true}"#;
    let packet = call_or_hint!("ex20", "decode_packet", decode_packet(payload))
        .expect("valid JSON packet should parse");

    assert_eq!(
        packet,
        Packet {
            id: "pkt-01".to_string(),
            signal: 42,
            active: true,
        }
    );
}

#[test]
fn decode_packet_supports_negative_signal_values() {
    let payload = r#"{"id":"pkt-neg","signal":-7,"active":false}"#;
    let packet = call_or_hint!("ex20", "decode_packet", decode_packet(payload))
        .expect("negative signals are valid i32 values");

    assert_eq!(packet.signal, -7);
    assert!(!packet.active);
}

#[test]
fn decode_packet_rejects_empty_packet_id() {
    let payload = r#"{"id":"","signal":9,"active":true}"#;
    let result = call_or_hint!("ex20", "decode_packet", decode_packet(payload));

    assert!(
        matches!(result, Err(PacketDecodeError::EmptyPacketId)),
        "empty id must map to PacketDecodeError::EmptyPacketId"
    );
}

#[test]
fn decode_packet_reports_invalid_json() {
    let result = call_or_hint!("ex20", "decode_packet", decode_packet("{not-json}"));

    assert!(
        matches!(result, Err(PacketDecodeError::InvalidJson(_))),
        "invalid JSON syntax must map to InvalidJson"
    );
}

#[test]
fn decode_packet_reports_missing_required_field() {
    let payload = r#"{"id":"pkt-02","active":true}"#;
    let result = call_or_hint!("ex20", "decode_packet", decode_packet(payload));

    assert!(
        matches!(result, Err(PacketDecodeError::InvalidJson(_))),
        "missing required fields should fail during deserialization"
    );
}

#[test]
fn decode_packet_batch_empty_input_returns_empty_vec() {
    let decoded = call_or_hint!("ex20", "decode_packet_batch", decode_packet_batch(&[]))
        .expect("empty batch should decode as empty vec");

    assert!(decoded.is_empty());
}

#[test]
fn decode_packet_batch_preserves_input_order() {
    let inputs = [
        r#"{"id":"pkt-a","signal":1,"active":true}"#,
        r#"{"id":"pkt-b","signal":2,"active":false}"#,
        r#"{"id":"pkt-c","signal":3,"active":true}"#,
    ];

    let decoded = call_or_hint!("ex20", "decode_packet_batch", decode_packet_batch(&inputs))
        .expect("all valid packets should decode");

    let ids: Vec<&str> = decoded.iter().map(|packet| packet.id.as_str()).collect();
    assert_eq!(ids, vec!["pkt-a", "pkt-b", "pkt-c"]);
}

#[test]
fn decode_packet_batch_stops_on_first_error() {
    let inputs = [
        r#"{"id":"pkt-ok","signal":10,"active":true}"#,
        r#"{"id":"","signal":0,"active":false}"#,
        r#"{"id":"pkt-late","signal":99,"active":true}"#,
    ];

    let result = call_or_hint!("ex20", "decode_packet_batch", decode_packet_batch(&inputs));

    assert!(
        matches!(result, Err(PacketDecodeError::EmptyPacketId)),
        "batch decoding should return the first packet error encountered"
    );
}
