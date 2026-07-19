use ex02_reactor_calibration::{calibrate_batch, calibrate_reactor, CalibrationInput};

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

// ── calibrate_reactor ───────────────────────────────────────────────────────
//
// Formula: (base_output + drift) * efficiency_percent / 100  (integer division)

#[test]
fn full_efficiency_no_drift() {
    // (100 + 0) * 100 / 100 = 100
    let out = call_or_hint!("ex02", "calibrate_reactor", calibrate_reactor(100, 0, 100));
    assert_eq!(out, 100, "100% efficiency, zero drift → same output");
}

#[test]
fn half_efficiency_no_drift() {
    // (100 + 0) * 50 / 100 = 50
    let out = call_or_hint!("ex02", "calibrate_reactor", calibrate_reactor(100, 0, 50));
    assert_eq!(out, 50, "50% efficiency halves output");
}

#[test]
fn zero_efficiency_returns_zero() {
    let out = call_or_hint!("ex02", "calibrate_reactor", calibrate_reactor(100, 50, 0));
    assert_eq!(out, 0, "0% efficiency always yields 0");
}

#[test]
fn positive_drift_adds_to_base() {
    // (100 + 10) * 100 / 100 = 110
    let out = call_or_hint!("ex02", "calibrate_reactor", calibrate_reactor(100, 10, 100));
    assert_eq!(
        out, 110,
        "positive drift increases output at full efficiency"
    );
}

#[test]
fn negative_drift_reduces_base() {
    // (100 + (-10)) * 100 / 100 = 90
    let out = call_or_hint!(
        "ex02",
        "calibrate_reactor",
        calibrate_reactor(100, -10, 100)
    );
    assert_eq!(out, 90, "negative drift reduces output");
}

#[test]
fn drift_and_partial_efficiency() {
    // (200 + 50) * 75 / 100 = 250 * 75 / 100 = 187 (integer division)
    let out = call_or_hint!("ex02", "calibrate_reactor", calibrate_reactor(200, 50, 75));
    assert_eq!(out, 187, "drift + 75% efficiency with integer truncation");
}

#[test]
fn negative_drift_with_efficiency() {
    // (100 + (-30)) * 50 / 100 = 70 * 50 / 100 = 35
    let out = call_or_hint!("ex02", "calibrate_reactor", calibrate_reactor(100, -30, 50));
    assert_eq!(out, 35, "negative drift combined with partial efficiency");
}

// ── calibrate_batch ─────────────────────────────────────────────────────────

#[test]
fn batch_processes_each_input() {
    let inputs = vec![
        CalibrationInput {
            base_output: 100,
            drift: 0,
            efficiency_percent: 100,
        },
        CalibrationInput {
            base_output: 100,
            drift: 0,
            efficiency_percent: 0,
        },
        CalibrationInput {
            base_output: 200,
            drift: 50,
            efficiency_percent: 75,
        },
    ];
    let results = call_or_hint!("ex02", "calibrate_reactor", calibrate_batch(&inputs));
    assert_eq!(results, vec![100, 0, 187]);
}

#[test]
fn batch_empty_slice_returns_empty() {
    let results = call_or_hint!("ex02", "calibrate_reactor", calibrate_batch(&[]));
    assert!(results.is_empty(), "empty batch produces empty results");
}
