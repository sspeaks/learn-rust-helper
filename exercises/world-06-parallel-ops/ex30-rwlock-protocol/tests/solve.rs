use ex30_rwlock_protocol::{
    apply_protocol_updates, shared_protocol_state, ProtocolUpdate, RwLockProtocolError,
};
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

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

fn update(key: &str, value: u32) -> ProtocolUpdate {
    ProtocolUpdate {
        key: key.to_string(),
        value,
    }
}

fn map(entries: &[(&str, u32)]) -> BTreeMap<String, u32> {
    entries
        .iter()
        .map(|(key, value)| ((*key).to_string(), *value))
        .collect()
}

fn poison_rwlock(state: &Arc<RwLock<BTreeMap<String, u32>>>) {
    let poisoned = Arc::clone(state);
    let _ = std::thread::spawn(move || {
        let _guard = poisoned
            .write()
            .expect("poison setup should acquire write lock");
        panic!("intentional poison for test");
    })
    .join();
}

#[test]
fn shared_state_keeps_initial_values() {
    let initial = map(&[("alpha", 1), ("beta", 2)]);
    let state = call_or_hint!(
        "ex30",
        "shared_protocol_state",
        shared_protocol_state(initial.clone())
    );

    let snapshot = state
        .read()
        .expect("fresh rwlock should be readable")
        .clone();
    assert_eq!(snapshot, initial);
}

#[test]
fn empty_update_list_returns_current_state() {
    let initial = map(&[("alpha", 10)]);
    let state = call_or_hint!(
        "ex30",
        "shared_protocol_state",
        shared_protocol_state(initial.clone())
    );

    let updated = call_or_hint!(
        "ex30",
        "apply_protocol_updates",
        apply_protocol_updates(Arc::clone(&state), vec![])
    )
    .expect("empty updates should be a no-op");

    assert_eq!(updated, initial);
}

#[test]
fn single_update_inserts_new_key() {
    let state = call_or_hint!(
        "ex30",
        "shared_protocol_state",
        shared_protocol_state(BTreeMap::new())
    );

    let updated = call_or_hint!(
        "ex30",
        "apply_protocol_updates",
        apply_protocol_updates(Arc::clone(&state), vec![update("gamma", 42)])
    )
    .expect("single update should apply");

    assert_eq!(updated.get("gamma"), Some(&42));
}

#[test]
fn updates_overwrite_existing_keys() {
    let state = call_or_hint!(
        "ex30",
        "shared_protocol_state",
        shared_protocol_state(map(&[("alpha", 1)]))
    );

    let updated = call_or_hint!(
        "ex30",
        "apply_protocol_updates",
        apply_protocol_updates(
            Arc::clone(&state),
            vec![update("alpha", 5), update("alpha", 9)],
        )
    )
    .expect("later updates should replace earlier values");

    assert_eq!(updated.get("alpha"), Some(&9));
}

#[test]
fn multiple_updates_apply_and_keep_unrelated_entries() {
    let state = call_or_hint!(
        "ex30",
        "shared_protocol_state",
        shared_protocol_state(map(&[("a", 1), ("z", 99)]))
    );

    let updated = call_or_hint!(
        "ex30",
        "apply_protocol_updates",
        apply_protocol_updates(Arc::clone(&state), vec![update("b", 2), update("c", 3)])
    )
    .expect("new keys should be added without dropping existing ones");

    assert_eq!(updated.get("a"), Some(&1));
    assert_eq!(updated.get("b"), Some(&2));
    assert_eq!(updated.get("c"), Some(&3));
    assert_eq!(updated.get("z"), Some(&99));
}

#[test]
fn returned_map_has_stable_sorted_key_order() {
    let state = call_or_hint!(
        "ex30",
        "shared_protocol_state",
        shared_protocol_state(BTreeMap::new())
    );

    let updated = call_or_hint!(
        "ex30",
        "apply_protocol_updates",
        apply_protocol_updates(
            Arc::clone(&state),
            vec![update("gamma", 1), update("alpha", 2), update("beta", 3)],
        )
    )
    .expect("updates should apply");

    let keys: Vec<&str> = updated.keys().map(|key| key.as_str()).collect();
    assert_eq!(keys, vec!["alpha", "beta", "gamma"]);
}

#[test]
fn consecutive_calls_accumulate_shared_state() {
    let state = call_or_hint!(
        "ex30",
        "shared_protocol_state",
        shared_protocol_state(BTreeMap::new())
    );

    call_or_hint!(
        "ex30",
        "apply_protocol_updates",
        apply_protocol_updates(Arc::clone(&state), vec![update("alpha", 10)])
    )
    .expect("first update batch should succeed");

    let second = call_or_hint!(
        "ex30",
        "apply_protocol_updates",
        apply_protocol_updates(Arc::clone(&state), vec![update("beta", 20)])
    )
    .expect("second update batch should succeed");

    assert_eq!(second.get("alpha"), Some(&10));
    assert_eq!(second.get("beta"), Some(&20));
}

#[test]
fn poisoned_rwlock_returns_lock_poisoned_error() {
    let state = call_or_hint!(
        "ex30",
        "shared_protocol_state",
        shared_protocol_state(BTreeMap::new())
    );
    poison_rwlock(&state);

    let result = call_or_hint!(
        "ex30",
        "apply_protocol_updates",
        apply_protocol_updates(Arc::clone(&state), vec![update("alpha", 1)])
    );

    assert!(
        matches!(result, Err(RwLockProtocolError::LockPoisoned)),
        "poisoned state should map to LockPoisoned"
    );
}
