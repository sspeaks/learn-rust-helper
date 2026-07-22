use ex31_mission_ledger::{apply_ledger_transaction, LedgerEvent, MissionLedgerError};
use rusqlite::Connection;

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

fn event(mission_code: &str, delta: i64) -> LedgerEvent {
    LedgerEvent {
        mission_code: mission_code.to_string(),
        delta,
    }
}

#[test]
fn empty_transaction_applies_zero_events_and_zero_balance() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");

    let outcome = call_or_hint!(
        "ex31",
        "apply_ledger_transaction",
        apply_ledger_transaction(&conn, &[])
    )
    .expect("empty transactions should succeed");

    assert_eq!(outcome.applied, 0);
    assert_eq!(outcome.final_balance, 0);
}

#[test]
fn single_positive_event_updates_balance() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");
    let events = [event("M-1", 7)];

    let outcome = call_or_hint!(
        "ex31",
        "apply_ledger_transaction",
        apply_ledger_transaction(&conn, &events)
    )
    .expect("positive delta should apply");

    assert_eq!(outcome.applied, 1);
    assert_eq!(outcome.final_balance, 7);
}

#[test]
fn multiple_events_accumulate_balance_in_order() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");
    let events = [event("M-1", 10), event("M-2", -3), event("M-3", 4)];

    let outcome = call_or_hint!(
        "ex31",
        "apply_ledger_transaction",
        apply_ledger_transaction(&conn, &events)
    )
    .expect("valid mixed deltas should apply");

    assert_eq!(outcome.applied, 3);
    assert_eq!(outcome.final_balance, 11);
}

#[test]
fn negative_first_event_is_rejected_with_attempted_balance() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");
    let events = [event("M-1", -1)];

    let result = call_or_hint!(
        "ex31",
        "apply_ledger_transaction",
        apply_ledger_transaction(&conn, &events)
    );

    match result {
        Err(MissionLedgerError::NegativeBalance { attempted_balance }) => {
            assert_eq!(attempted_balance, -1)
        }
        other => panic!("expected NegativeBalance(-1), got {other:?}"),
    }
}

#[test]
fn exact_zero_balance_after_deductions_is_allowed() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");
    let events = [event("M-1", 5), event("M-1", -5)];

    let outcome = call_or_hint!(
        "ex31",
        "apply_ledger_transaction",
        apply_ledger_transaction(&conn, &events)
    )
    .expect("balance equal to zero should remain valid");

    assert_eq!(outcome.final_balance, 0);
    assert_eq!(outcome.applied, 2);
}

#[test]
fn rollback_keeps_previous_balance_when_later_event_would_go_negative() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");

    call_or_hint!(
        "ex31",
        "apply_ledger_transaction",
        apply_ledger_transaction(&conn, &[event("seed", 10)])
    )
    .expect("seed transaction should succeed");

    let failing = call_or_hint!(
        "ex31",
        "apply_ledger_transaction",
        apply_ledger_transaction(&conn, &[event("drop", -3), event("drop", -20)])
    );

    match failing {
        Err(MissionLedgerError::NegativeBalance { attempted_balance }) => {
            assert_eq!(attempted_balance, -13)
        }
        other => panic!("expected NegativeBalance(-13), got {other:?}"),
    }

    let after = call_or_hint!(
        "ex31",
        "apply_ledger_transaction",
        apply_ledger_transaction(&conn, &[])
    )
    .expect("state readback transaction should succeed");

    assert_eq!(
        after.final_balance, 10,
        "failed transaction should roll back and preserve previous balance"
    );
}

#[test]
fn applied_count_matches_number_of_successful_events() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");
    let events = [
        event("M-1", 1),
        event("M-2", 2),
        event("M-3", 3),
        event("M-4", 4),
    ];

    let outcome = call_or_hint!(
        "ex31",
        "apply_ledger_transaction",
        apply_ledger_transaction(&conn, &events)
    )
    .expect("all events should apply");

    assert_eq!(outcome.applied, events.len());
}

#[test]
fn intermediate_negative_balance_reports_exact_attempted_value() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");
    let events = [event("M-1", 2), event("M-2", -5)];

    let result = call_or_hint!(
        "ex31",
        "apply_ledger_transaction",
        apply_ledger_transaction(&conn, &events)
    );

    match result {
        Err(MissionLedgerError::NegativeBalance { attempted_balance }) => {
            assert_eq!(attempted_balance, -3)
        }
        other => panic!("expected NegativeBalance(-3), got {other:?}"),
    }
}
