use ex08_borrow_checkpoint::{rebalance_turrets, Turret};

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

fn make_turret(callsign: &str, charge: i32, overheated: bool) -> Turret {
    Turret {
        callsign: callsign.to_string(),
        charge,
        overheated,
    }
}

// ── rebalance_turrets ─────────────────────────────────────────────────────────
//
// Behavioral contract (derived from docstring + handoff):
//   • Each turret's charge increases by emergency_boost.
//   • overheated is set to true when the resulting charge > 100, false otherwise.

#[test]
fn empty_slice_completes_without_error() {
    let mut turrets: Vec<Turret> = vec![];
    call_or_hint!(
        "ex08",
        "rebalance_turrets",
        rebalance_turrets(&mut turrets, 10)
    );
    // no assertion needed — just must not panic
}

#[test]
fn boost_increases_charge_in_place() {
    let mut turrets = vec![make_turret("T1", 50, false)];
    call_or_hint!(
        "ex08",
        "rebalance_turrets",
        rebalance_turrets(&mut turrets, 30)
    );
    assert_eq!(
        turrets[0].charge, 80,
        "charge should increase by emergency_boost (50 + 30 = 80)"
    );
}

#[test]
fn boost_applied_to_every_turret() {
    let mut turrets = vec![
        make_turret("T1", 10, false),
        make_turret("T2", 40, false),
        make_turret("T3", 70, false),
    ];
    call_or_hint!(
        "ex08",
        "rebalance_turrets",
        rebalance_turrets(&mut turrets, 20)
    );
    assert_eq!(turrets[0].charge, 30, "T1: 10 + 20");
    assert_eq!(turrets[1].charge, 60, "T2: 40 + 20");
    assert_eq!(turrets[2].charge, 90, "T3: 70 + 20");
}

#[test]
fn overheat_triggered_above_100() {
    // 95 + 11 = 106 would exceed 100 → charge is capped at 100, overheated = true
    let mut turrets = vec![make_turret("Hot", 95, false)];
    call_or_hint!(
        "ex08",
        "rebalance_turrets",
        rebalance_turrets(&mut turrets, 11)
    );
    assert!(
        turrets[0].overheated,
        "charge > 100 must set overheated = true"
    );
    assert_eq!(
        turrets[0].charge, 100,
        "charge must be capped at 100 when boost would exceed threshold"
    );
}

#[test]
fn overheat_not_triggered_at_exactly_100() {
    // charge goes from 90 to 100 — boundary: not overheated
    let mut turrets = vec![make_turret("Edge", 90, false)];
    call_or_hint!(
        "ex08",
        "rebalance_turrets",
        rebalance_turrets(&mut turrets, 10)
    );
    assert!(
        !turrets[0].overheated,
        "charge == 100 must NOT set overheated (threshold is strictly > 100)"
    );
    assert_eq!(turrets[0].charge, 100);
}

#[test]
fn overheat_cleared_when_charge_drops_to_safe() {
    // turret was overheated; negative boost brings it back below threshold
    let mut turrets = vec![make_turret("Cool", 110, true)];
    call_or_hint!(
        "ex08",
        "rebalance_turrets",
        rebalance_turrets(&mut turrets, -20)
    );
    assert!(
        !turrets[0].overheated,
        "charge drop below or equal to 100 must clear overheated"
    );
    assert_eq!(turrets[0].charge, 90);
}

#[test]
fn zero_boost_leaves_charges_unchanged() {
    let mut turrets = vec![make_turret("Idle", 50, false)];
    call_or_hint!(
        "ex08",
        "rebalance_turrets",
        rebalance_turrets(&mut turrets, 0)
    );
    assert_eq!(turrets[0].charge, 50, "zero boost must not change charge");
    assert!(
        !turrets[0].overheated,
        "zero boost must not change overheat state"
    );
}
