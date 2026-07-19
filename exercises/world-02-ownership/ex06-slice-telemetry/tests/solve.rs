use ex06_slice_telemetry::{first_sentence, trailing_readings};

// ── Stub detection helpers ──────────────────────────────────────────────────

fn is_stub_panic(e: &Box<dyn std::any::Any + Send>) -> bool {
    e.downcast_ref::<&str>()
        .map_or(false, |s| s.contains("not yet implemented"))
        || e.downcast_ref::<String>()
            .map_or(false, |s| s.contains("not yet implemented"))
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

// ── first_sentence ────────────────────────────────────────────────────────────

#[test]
fn first_sentence_up_to_first_period_inclusive() {
    let report = "Hull breach detected. Seal the port hatch.";
    let sentence = call_or_hint!("ex06", "first_sentence", first_sentence(report));
    assert_eq!(
        sentence, "Hull breach detected.",
        "must stop at the first period, inclusive"
    );
}

#[test]
fn first_sentence_only_one_period() {
    let report = "All systems nominal.";
    let sentence = call_or_hint!("ex06", "first_sentence", first_sentence(report));
    assert_eq!(sentence, "All systems nominal.");
}

#[test]
fn first_sentence_no_period_returns_whole_input() {
    let report = "No anomalies detected";
    let sentence = call_or_hint!("ex06", "first_sentence", first_sentence(report));
    assert_eq!(
        sentence, "No anomalies detected",
        "no period → return the entire input"
    );
}

#[test]
fn first_sentence_empty_string() {
    let sentence = call_or_hint!("ex06", "first_sentence", first_sentence(""));
    assert_eq!(sentence, "", "empty input returns empty slice");
}

#[test]
fn first_sentence_period_only() {
    let sentence = call_or_hint!("ex06", "first_sentence", first_sentence("."));
    assert_eq!(sentence, ".", "a lone period is a complete sentence");
}

/// Returned slice must be a subslice of the input, not a copy.
#[test]
fn first_sentence_is_subslice_of_input() {
    let report = "Reactor stable. Begin harvest sequence.";
    let sentence = call_or_hint!("ex06", "first_sentence", first_sentence(report));
    // The returned slice's pointer must point into report's buffer.
    assert_eq!(
        sentence.as_ptr(),
        report.as_ptr(),
        "first_sentence must return a subslice — no allocation"
    );
    assert_eq!(sentence, "Reactor stable.");
}

// ── trailing_readings ─────────────────────────────────────────────────────────

#[test]
fn trailing_last_three_of_five() {
    let readings = [10, 20, 30, 40, 50];
    let tail = call_or_hint!("ex06", "trailing_readings", trailing_readings(&readings, 3));
    assert_eq!(tail, &[30, 40, 50]);
}

#[test]
fn trailing_count_equals_length() {
    let readings = [1, 2, 3];
    let tail = call_or_hint!("ex06", "trailing_readings", trailing_readings(&readings, 3));
    assert_eq!(tail, &[1, 2, 3]);
}

#[test]
fn trailing_count_exceeds_length_returns_all() {
    let readings = [7, 8, 9];
    let tail = call_or_hint!(
        "ex06",
        "trailing_readings",
        trailing_readings(&readings, 100)
    );
    assert_eq!(
        tail,
        &[7, 8, 9],
        "count larger than slice → return entire slice"
    );
}

#[test]
fn trailing_count_zero_returns_empty() {
    let readings = [1, 2, 3, 4, 5];
    let tail = call_or_hint!("ex06", "trailing_readings", trailing_readings(&readings, 0));
    assert_eq!(tail, &[] as &[i32], "count=0 must return empty slice");
}

#[test]
fn trailing_empty_slice_returns_empty() {
    let tail = call_or_hint!("ex06", "trailing_readings", trailing_readings(&[], 5));
    assert_eq!(tail, &[] as &[i32], "empty input always returns empty");
}

/// Returned slice must be a subslice of the input — no allocation.
#[test]
fn trailing_readings_is_subslice_of_input() {
    let readings = [1, 2, 3, 4, 5];
    let tail = call_or_hint!("ex06", "trailing_readings", trailing_readings(&readings, 3));
    // Must point at element index 2 of readings.
    assert_eq!(
        tail.as_ptr(),
        readings[2..].as_ptr(),
        "trailing_readings must return a subslice — no allocation"
    );
}
