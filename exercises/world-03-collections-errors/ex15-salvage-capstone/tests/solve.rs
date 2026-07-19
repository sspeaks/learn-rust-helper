use ex15_salvage_capstone::{build_salvage_plan, SalvageError};

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

// ── Manifest line format ───────────────────────────────────────────────────
//
// Each line: "name|mass|priority|fragile"
// Example:   "engine|500|3|false"
//            "cryo-pod|200|5|true"

// ── build_salvage_plan: EmptyManifest ─────────────────────────────────────

#[test]
fn empty_manifest_returns_empty_manifest_error() {
    let result = call_or_hint!("ex15", "build_salvage_plan", build_salvage_plan(&[], 1000));
    assert_eq!(
        result,
        Err(SalvageError::EmptyManifest),
        "empty manifest must return SalvageError::EmptyManifest"
    );
}

// ── build_salvage_plan: happy path ────────────────────────────────────────

#[test]
fn single_item_plan_aggregates_correctly() {
    let lines = ["engine|500|3|false"];
    let result = call_or_hint!(
        "ex15",
        "build_salvage_plan",
        build_salvage_plan(&lines, 1000)
    );
    let plan = result.expect("valid single-item manifest should succeed");
    assert_eq!(plan.total_mass, 500, "total_mass must equal item mass");
    assert_eq!(plan.fragile_count, 0, "non-fragile item: fragile_count = 0");
    assert_eq!(plan.top_targets, vec!["engine"]);
}

#[test]
fn fragile_item_counted() {
    let lines = ["cryo-pod|200|5|true"];
    let result = call_or_hint!(
        "ex15",
        "build_salvage_plan",
        build_salvage_plan(&lines, 1000)
    );
    let plan = result.expect("valid manifest");
    assert_eq!(
        plan.fragile_count, 1,
        "one fragile item → fragile_count = 1"
    );
    assert_eq!(plan.total_mass, 200);
}

#[test]
fn multiple_items_aggregate_total_mass_and_fragile_count() {
    let lines = [
        "engine|500|3|false",
        "cryo-pod|200|5|true",
        "fuel-cell|100|2|false",
        "sensor|50|4|true",
    ];
    let result = call_or_hint!(
        "ex15",
        "build_salvage_plan",
        build_salvage_plan(&lines, 2000)
    );
    let plan = result.expect("valid multi-item manifest");
    assert_eq!(plan.total_mass, 850, "total_mass = 500+200+100+50 = 850");
    assert_eq!(
        plan.fragile_count, 2,
        "two fragile items (cryo-pod and sensor)"
    );
}

#[test]
fn top_targets_ordered_by_priority_descending() {
    // priorities: 3, 5, 2, 4 → sorted desc: cryo-pod(5), sensor(4), engine(3), fuel-cell(2)
    let lines = [
        "engine|500|3|false",
        "cryo-pod|200|5|true",
        "fuel-cell|100|2|false",
        "sensor|50|4|false",
    ];
    let result = call_or_hint!(
        "ex15",
        "build_salvage_plan",
        build_salvage_plan(&lines, 2000)
    );
    let plan = result.expect("valid manifest");
    assert_eq!(
        plan.top_targets,
        vec!["cryo-pod", "sensor", "engine", "fuel-cell"],
        "top_targets must be names sorted by priority descending"
    );
}

// ── build_salvage_plan: OverCapacity ──────────────────────────────────────

#[test]
fn over_capacity_returns_error_with_correct_fields() {
    let lines = ["engine|500|3|false", "hull|700|2|false"];
    let result = call_or_hint!(
        "ex15",
        "build_salvage_plan",
        build_salvage_plan(&lines, 1000)
    );
    assert_eq!(
        result,
        Err(SalvageError::OverCapacity {
            capacity: 1000,
            total_mass: 1200
        }),
        "total_mass(1200) > capacity(1000) must return OverCapacity"
    );
}

#[test]
fn exact_capacity_match_succeeds() {
    let lines = ["engine|500|3|false", "hull|500|2|false"];
    let result = call_or_hint!(
        "ex15",
        "build_salvage_plan",
        build_salvage_plan(&lines, 1000)
    );
    assert!(
        result.is_ok(),
        "total_mass == capacity must succeed (not OverCapacity)"
    );
}

// ── build_salvage_plan: InvalidLine ──────────────────────────────────────

#[test]
fn invalid_line_returns_error_with_line_index() {
    // Line 0 is malformed (not enough fields)
    let lines = ["bad-line"];
    let result = call_or_hint!(
        "ex15",
        "build_salvage_plan",
        build_salvage_plan(&lines, 1000)
    );
    match result {
        Err(SalvageError::InvalidLine { line, reason }) => {
            assert_eq!(line, 0, "first line (index 0) is malformed");
            assert!(!reason.is_empty(), "reason must be non-empty");
        }
        other => panic!("expected InvalidLine, got {other:?}"),
    }
}

#[test]
fn invalid_mass_value_returns_invalid_line() {
    let lines = [
        "engine|500|3|false",
        "hull|notanumber|2|false", // line index 1 is bad
    ];
    let result = call_or_hint!(
        "ex15",
        "build_salvage_plan",
        build_salvage_plan(&lines, 2000)
    );
    match result {
        Err(SalvageError::InvalidLine { line, reason }) => {
            assert_eq!(line, 1, "second line (index 1) has invalid mass");
            assert!(!reason.is_empty());
        }
        other => panic!("expected InvalidLine, got {other:?}"),
    }
}

#[test]
fn invalid_line_before_valid_lines_reported_first() {
    let lines = [
        "BROKEN",             // index 0 — bad
        "engine|500|3|false", // index 1 — fine
    ];
    let result = call_or_hint!(
        "ex15",
        "build_salvage_plan",
        build_salvage_plan(&lines, 2000)
    );
    match result {
        Err(SalvageError::InvalidLine { line, .. }) => {
            assert_eq!(line, 0, "earliest invalid line should be reported");
        }
        other => panic!("expected InvalidLine, got {other:?}"),
    }
}
