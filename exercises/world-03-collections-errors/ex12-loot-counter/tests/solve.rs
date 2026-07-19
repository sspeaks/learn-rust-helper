use ex12_loot_counter::{count_loot, total_items};
use std::collections::HashMap;

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

// ── count_loot ────────────────────────────────────────────────────────────────

#[test]
fn empty_input_returns_empty_map() {
    let counts = call_or_hint!("ex12", "count_loot", count_loot(&[]));
    assert!(counts.is_empty(), "empty input must produce an empty map");
}

#[test]
fn single_item_has_count_one() {
    let counts = call_or_hint!("ex12", "count_loot", count_loot(&["sword"]));
    assert_eq!(counts.get("sword"), Some(&1));
}

#[test]
fn duplicate_items_counted_correctly() {
    let items = ["shield", "shield", "shield"];
    let counts = call_or_hint!("ex12", "count_loot", count_loot(&items));
    assert_eq!(counts.get("shield"), Some(&3));
}

#[test]
fn multiple_distinct_items() {
    let items = ["sword", "bow", "sword", "staff", "bow", "bow"];
    let counts = call_or_hint!("ex12", "count_loot", count_loot(&items));
    assert_eq!(counts.get("sword"), Some(&2), "sword appears twice");
    assert_eq!(counts.get("bow"), Some(&3), "bow appears three times");
    assert_eq!(counts.get("staff"), Some(&1), "staff appears once");
    assert_eq!(counts.len(), 3, "three distinct items");
}

#[test]
fn keys_are_owned_strings() {
    let items = ["helm"];
    let counts = call_or_hint!("ex12", "count_loot", count_loot(&items));
    // HashMap<String, usize> — key lookup by &str must work
    assert_eq!(counts.get("helm"), Some(&1));
}

#[test]
fn count_is_case_sensitive() {
    // "Sword" and "sword" are distinct keys
    let items = ["Sword", "sword", "SWORD"];
    let counts = call_or_hint!("ex12", "count_loot", count_loot(&items));
    assert_eq!(counts.get("Sword"), Some(&1));
    assert_eq!(counts.get("sword"), Some(&1));
    assert_eq!(counts.get("SWORD"), Some(&1));
    assert_eq!(counts.len(), 3, "case-sensitive: three distinct keys");
}

// ── total_items ────────────────────────────────────────────────────────────────

#[test]
fn total_items_sums_all_counts() {
    let items = ["a", "b", "a", "c", "a"];
    let counts = call_or_hint!("ex12", "count_loot", count_loot(&items));
    let total = total_items(&counts);
    assert_eq!(total, 5, "total_items must sum all counts (3+1+1=5)");
}

#[test]
fn total_items_empty_map_is_zero() {
    let empty: HashMap<String, usize> = HashMap::new();
    let total = total_items(&empty);
    assert_eq!(total, 0);
}

#[test]
fn total_items_single_entry() {
    let mut map = HashMap::new();
    map.insert("gem".to_string(), 7_usize);
    let total = total_items(&map);
    assert_eq!(total, 7);
}
