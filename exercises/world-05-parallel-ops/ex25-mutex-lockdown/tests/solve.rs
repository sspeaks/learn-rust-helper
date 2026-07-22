use ex25_mutex_lockdown::{
    process_lockdown_batch, shared_lockdown_state, LockdownEvent, MutexLockdownError,
};
use std::sync::{Arc, Mutex};

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

fn event(zone: &str, delta: i32) -> LockdownEvent {
    LockdownEvent {
        zone: zone.to_string(),
        delta,
    }
}

fn poison_mutex(state: &Arc<Mutex<ex25_mutex_lockdown::LockdownState>>) {
    let poisoned = Arc::clone(state);
    let _ = std::thread::spawn(move || {
        let _guard = poisoned.lock().expect("poison setup should acquire lock");
        panic!("intentional poison for test");
    })
    .join();
}

#[test]
fn shared_state_initializes_with_given_stability_and_zero_processed() {
    let state = call_or_hint!("ex25", "shared_lockdown_state", shared_lockdown_state(25));
    let snapshot = state.lock().expect("fresh mutex should lock");

    assert_eq!(snapshot.stability, 25);
    assert_eq!(snapshot.processed, 0);
}

#[test]
fn processing_empty_batch_keeps_state_unchanged() {
    let state = call_or_hint!("ex25", "shared_lockdown_state", shared_lockdown_state(100));
    let outcome = call_or_hint!(
        "ex25",
        "process_lockdown_batch",
        process_lockdown_batch(Arc::clone(&state), vec![])
    )
    .expect("empty batches should succeed");

    assert_eq!(outcome.stability, 100);
    assert_eq!(outcome.processed, 0);
}

#[test]
fn single_event_updates_stability_and_processed_count() {
    let state = call_or_hint!("ex25", "shared_lockdown_state", shared_lockdown_state(40));
    let outcome = call_or_hint!(
        "ex25",
        "process_lockdown_batch",
        process_lockdown_batch(Arc::clone(&state), vec![event("a", 5)])
    )
    .expect("single event should apply");

    assert_eq!(outcome.stability, 45);
    assert_eq!(outcome.processed, 1);
}

#[test]
fn multiple_events_apply_all_deltas() {
    let state = call_or_hint!("ex25", "shared_lockdown_state", shared_lockdown_state(10));
    let outcome = call_or_hint!(
        "ex25",
        "process_lockdown_batch",
        process_lockdown_batch(
            Arc::clone(&state),
            vec![event("a", 3), event("b", -4), event("c", 9), event("d", -1)],
        )
    )
    .expect("all events should apply");

    assert_eq!(outcome.stability, 17);
    assert_eq!(outcome.processed, 4);
}

#[test]
fn zero_delta_events_still_increment_processed_counter() {
    let state = call_or_hint!("ex25", "shared_lockdown_state", shared_lockdown_state(55));
    let outcome = call_or_hint!(
        "ex25",
        "process_lockdown_batch",
        process_lockdown_batch(
            Arc::clone(&state),
            vec![event("idle", 0), event("idle2", 0)]
        )
    )
    .expect("zero-delta events are still processed events");

    assert_eq!(outcome.stability, 55);
    assert_eq!(outcome.processed, 2);
}

#[test]
fn consecutive_batches_accumulate_on_shared_state() {
    let state = call_or_hint!("ex25", "shared_lockdown_state", shared_lockdown_state(0));

    call_or_hint!(
        "ex25",
        "process_lockdown_batch",
        process_lockdown_batch(Arc::clone(&state), vec![event("a", 4), event("b", 6)])
    )
    .expect("first batch should apply");

    let second = call_or_hint!(
        "ex25",
        "process_lockdown_batch",
        process_lockdown_batch(Arc::clone(&state), vec![event("c", -3)])
    )
    .expect("second batch should also apply");

    assert_eq!(second.stability, 7);
    assert_eq!(second.processed, 3);
}

#[test]
fn poisoned_state_returns_lock_poisoned_error() {
    let state = call_or_hint!("ex25", "shared_lockdown_state", shared_lockdown_state(10));
    poison_mutex(&state);

    let result = call_or_hint!(
        "ex25",
        "process_lockdown_batch",
        process_lockdown_batch(Arc::clone(&state), vec![event("a", 1)])
    );

    assert!(
        matches!(result, Err(MutexLockdownError::LockPoisoned)),
        "poisoned mutexes must surface LockPoisoned errors"
    );
}

#[test]
fn negative_deltas_can_reduce_stability_below_zero() {
    let state = call_or_hint!("ex25", "shared_lockdown_state", shared_lockdown_state(3));
    let outcome = call_or_hint!(
        "ex25",
        "process_lockdown_batch",
        process_lockdown_batch(Arc::clone(&state), vec![event("x", -10)])
    )
    .expect("negative totals are still valid integer outcomes");

    assert_eq!(outcome.stability, -7);
    assert_eq!(outcome.processed, 1);
}
