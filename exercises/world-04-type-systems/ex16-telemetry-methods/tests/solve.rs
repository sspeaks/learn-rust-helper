use ex16_telemetry_methods::TelemetryBuffer;

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
fn with_capacity_sets_metadata_and_empty_readings() {
    let buffer = call_or_hint!(
        "ex16",
        "TelemetryBuffer::with_capacity",
        TelemetryBuffer::with_capacity("alpha", 3)
    );

    assert_eq!(buffer.label, "alpha");
    assert_eq!(buffer.capacity, 3);
    assert!(buffer.readings.is_empty());
}

#[test]
fn record_accepts_values_until_capacity() {
    let mut buffer = call_or_hint!(
        "ex16",
        "TelemetryBuffer::with_capacity",
        TelemetryBuffer::with_capacity("beta", 2)
    );

    assert!(call_or_hint!(
        "ex16",
        "TelemetryBuffer::record",
        buffer.record(10)
    ));
    assert!(call_or_hint!(
        "ex16",
        "TelemetryBuffer::record",
        buffer.record(20)
    ));
    assert!(!call_or_hint!(
        "ex16",
        "TelemetryBuffer::record",
        buffer.record(30)
    ));
    assert_eq!(buffer.readings, vec![10, 20]);
}

#[test]
fn average_returns_none_when_empty() {
    let buffer = call_or_hint!(
        "ex16",
        "TelemetryBuffer::with_capacity",
        TelemetryBuffer::with_capacity("gamma", 4)
    );

    let avg = call_or_hint!("ex16", "TelemetryBuffer::average", buffer.average());
    assert_eq!(avg, None);
}

#[test]
fn average_handles_negative_and_positive_values() {
    let mut buffer = call_or_hint!(
        "ex16",
        "TelemetryBuffer::with_capacity",
        TelemetryBuffer::with_capacity("delta", 4)
    );
    call_or_hint!("ex16", "TelemetryBuffer::record", buffer.record(-5));
    call_or_hint!("ex16", "TelemetryBuffer::record", buffer.record(15));
    call_or_hint!("ex16", "TelemetryBuffer::record", buffer.record(0));

    let avg = call_or_hint!("ex16", "TelemetryBuffer::average", buffer.average())
        .expect("average should exist after at least one reading");

    assert!((avg - 3.333_333_333).abs() < 1e-9);
}

#[test]
fn into_report_formats_empty_buffer() {
    let buffer = call_or_hint!(
        "ex16",
        "TelemetryBuffer::with_capacity",
        TelemetryBuffer::with_capacity("echo", 5)
    );

    let report = call_or_hint!("ex16", "TelemetryBuffer::into_report", buffer.into_report());
    assert_eq!(report, "echo:0@n/a");
}

#[test]
fn into_report_consumes_buffer_and_formats_average_to_one_decimal() {
    let mut buffer = call_or_hint!(
        "ex16",
        "TelemetryBuffer::with_capacity",
        TelemetryBuffer::with_capacity("foxtrot", 3)
    );
    call_or_hint!("ex16", "TelemetryBuffer::record", buffer.record(2));
    call_or_hint!("ex16", "TelemetryBuffer::record", buffer.record(3));
    call_or_hint!("ex16", "TelemetryBuffer::record", buffer.record(4));

    let report = call_or_hint!("ex16", "TelemetryBuffer::into_report", buffer.into_report());

    assert_eq!(report, "foxtrot:3@3.0");
}
