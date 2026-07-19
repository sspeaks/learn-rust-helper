use ex04_energy_loop::{mission_harvest_report, total_harvest};

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

// ── total_harvest ────────────────────────────────────────────────────────────

#[test]
fn single_element_range() {
    let total = call_or_hint!("ex04", "total_harvest", total_harvest(5..=5));
    assert_eq!(total, 5, "range 5..=5 accumulates to 5");
}

#[test]
fn single_zero_element() {
    let total = call_or_hint!("ex04", "total_harvest", total_harvest(0..=0));
    assert_eq!(total, 0, "range 0..=0 accumulates to 0");
}

#[test]
fn small_range_inclusive() {
    // 1 + 2 + 3 = 6
    let total = call_or_hint!("ex04", "total_harvest", total_harvest(1..=3));
    assert_eq!(
        total, 6,
        "range 1..=3 must include both endpoints and all in between"
    );
}

#[test]
fn standard_range_one_to_ten() {
    // 1+2+…+10 = 55
    let total = call_or_hint!("ex04", "total_harvest", total_harvest(1..=10));
    assert_eq!(total, 55, "sum 1..=10 = 55");
}

#[test]
fn range_starting_at_nonzero() {
    // 5+6+7 = 18
    let total = call_or_hint!("ex04", "total_harvest", total_harvest(5..=7));
    assert_eq!(total, 18, "range 5..=7 should include 5, 6, and 7");
}

#[test]
fn larger_range() {
    // 1+…+100 = 5050
    let total = call_or_hint!("ex04", "total_harvest", total_harvest(1..=100));
    assert_eq!(total, 5050);
}

// ── mission_harvest_report ────────────────────────────────────────────────────

#[test]
fn report_maps_each_range() {
    let missions = vec![1..=3, 5..=5, 1..=10];
    let report = call_or_hint!("ex04", "total_harvest", mission_harvest_report(&missions));
    assert_eq!(report, vec![6, 5, 55]);
}

#[test]
fn report_empty_slice() {
    let report = call_or_hint!("ex04", "total_harvest", mission_harvest_report(&[]));
    assert!(
        report.is_empty(),
        "empty mission list produces empty report"
    );
}
