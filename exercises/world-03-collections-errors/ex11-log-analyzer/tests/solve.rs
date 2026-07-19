use ex11_log_analyzer::{summarize_failures, LogEvent};

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

fn event(system: &str, success: bool, code: u16) -> LogEvent {
    LogEvent {
        system: system.to_string(),
        success,
        code,
    }
}

// ── summarize_failures ────────────────────────────────────────────────────────

#[test]
fn empty_events_returns_empty() {
    let result = call_or_hint!("ex11", "summarize_failures", summarize_failures(&[], 10));
    assert!(result.is_empty(), "no events → no failure summaries");
}

#[test]
fn all_successes_returns_empty() {
    let events = vec![
        event("reactor", true, 200),
        event("nav", true, 200),
        event("comms", true, 200),
    ];
    let result = call_or_hint!(
        "ex11",
        "summarize_failures",
        summarize_failures(&events, 10)
    );
    assert!(result.is_empty(), "all-success events produce no failures");
}

#[test]
fn only_failure_events_included() {
    let events = vec![
        event("reactor", false, 503),
        event("nav", true, 200),
        event("comms", false, 404),
    ];
    let result = call_or_hint!(
        "ex11",
        "summarize_failures",
        summarize_failures(&events, 10)
    );
    assert_eq!(
        result.len(),
        2,
        "only failed events should appear in the summary"
    );
}

#[test]
fn failure_summaries_contain_system_name() {
    let events = vec![event("engines", false, 500)];
    let result = call_or_hint!(
        "ex11",
        "summarize_failures",
        summarize_failures(&events, 10)
    );
    assert_eq!(result.len(), 1);
    assert!(
        result[0].contains("engines"),
        "failure summary must contain the system name"
    );
}

#[test]
fn failure_summaries_contain_error_code() {
    let events = vec![event("hull", false, 503)];
    let result = call_or_hint!(
        "ex11",
        "summarize_failures",
        summarize_failures(&events, 10)
    );
    assert_eq!(result.len(), 1);
    assert!(
        result[0].contains("503"),
        "failure summary must contain the error code"
    );
}

#[test]
fn max_items_limits_output() {
    let events: Vec<LogEvent> = (0..10)
        .map(|i| event(&format!("sys{i}"), false, 500))
        .collect();
    let result = call_or_hint!("ex11", "summarize_failures", summarize_failures(&events, 3));
    assert_eq!(
        result.len(),
        3,
        "max_items=3 must return at most 3 summaries"
    );
}

#[test]
fn max_items_zero_returns_empty() {
    let events = vec![event("reactor", false, 500)];
    let result = call_or_hint!("ex11", "summarize_failures", summarize_failures(&events, 0));
    assert!(result.is_empty(), "max_items=0 must return empty");
}

#[test]
fn max_items_larger_than_failure_count_returns_all_failures() {
    let events = vec![
        event("a", false, 500),
        event("b", true, 200),
        event("c", false, 503),
    ];
    let result = call_or_hint!(
        "ex11",
        "summarize_failures",
        summarize_failures(&events, 100)
    );
    assert_eq!(
        result.len(),
        2,
        "returns all failures when max_items > failure count"
    );
}

#[test]
fn order_is_stable_matches_input_order() {
    let events = vec![
        event("alpha", false, 501),
        event("beta", false, 502),
        event("gamma", false, 503),
    ];
    let result = call_or_hint!(
        "ex11",
        "summarize_failures",
        summarize_failures(&events, 10)
    );
    assert_eq!(result.len(), 3);
    // The first failure summary should mention "alpha", and the third "gamma".
    assert!(
        result[0].contains("alpha"),
        "first summary must correspond to the first failure in input order"
    );
    assert!(
        result[2].contains("gamma"),
        "last summary must correspond to the last failure in input order"
    );
}
