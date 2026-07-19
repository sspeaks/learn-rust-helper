use ex07_move_inventory::{move_crate_to_shuttle, SupplyCrate};

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

fn make_crate(id: &str, contents: &[&str]) -> SupplyCrate {
    SupplyCrate {
        id: id.to_string(),
        contents: contents.iter().map(|s| s.to_string()).collect(),
    }
}

// ── move_crate_to_shuttle: found ──────────────────────────────────────────────

#[test]
fn found_crate_returned_as_some() {
    let mut manifest = vec![
        make_crate("A1", &["oxygen", "water"]),
        make_crate("B2", &["fuel"]),
    ];
    let result = call_or_hint!(
        "ex07",
        "move_crate_to_shuttle",
        move_crate_to_shuttle(&mut manifest, "A1")
    );
    let found = result.expect("known crate id should return Some");
    assert_eq!(found.id, "A1");
    assert_eq!(found.contents, vec!["oxygen", "water"]);
}

#[test]
fn manifest_shrinks_after_removal() {
    let mut manifest = vec![make_crate("A1", &["oxygen"]), make_crate("B2", &["fuel"])];
    call_or_hint!(
        "ex07",
        "move_crate_to_shuttle",
        move_crate_to_shuttle(&mut manifest, "A1")
    );
    assert_eq!(
        manifest.len(),
        1,
        "manifest must shrink by one after removing a crate"
    );
}

#[test]
fn removed_crate_no_longer_in_manifest() {
    let mut manifest = vec![
        make_crate("X9", &["sample"]),
        make_crate("Y7", &["probe"]),
        make_crate("Z3", &["spare"]),
    ];
    call_or_hint!(
        "ex07",
        "move_crate_to_shuttle",
        move_crate_to_shuttle(&mut manifest, "Y7")
    );
    assert!(
        manifest.iter().all(|c| c.id != "Y7"),
        "removed crate must not remain in the manifest"
    );
    assert_eq!(manifest.len(), 2, "two crates remain");
}

#[test]
fn correct_crate_removed_when_multiple_present() {
    let mut manifest = vec![
        make_crate("A1", &["alpha"]),
        make_crate("B2", &["beta"]),
        make_crate("C3", &["gamma"]),
    ];
    let result = call_or_hint!(
        "ex07",
        "move_crate_to_shuttle",
        move_crate_to_shuttle(&mut manifest, "B2")
    );
    let found = result.unwrap();
    assert_eq!(found.id, "B2");
    let remaining: Vec<&str> = manifest.iter().map(|c| c.id.as_str()).collect();
    assert_eq!(remaining, vec!["A1", "C3"]);
}

// ── move_crate_to_shuttle: not found ─────────────────────────────────────────

#[test]
fn missing_id_returns_none() {
    let mut manifest = vec![make_crate("A1", &["oxygen"])];
    let result = call_or_hint!(
        "ex07",
        "move_crate_to_shuttle",
        move_crate_to_shuttle(&mut manifest, "GHOST")
    );
    assert!(result.is_none(), "unknown crate id must return None");
}

#[test]
fn manifest_unchanged_when_not_found() {
    let mut manifest = vec![make_crate("A1", &["oxygen"]), make_crate("B2", &["fuel"])];
    call_or_hint!(
        "ex07",
        "move_crate_to_shuttle",
        move_crate_to_shuttle(&mut manifest, "GHOST")
    );
    assert_eq!(
        manifest.len(),
        2,
        "manifest must be unchanged when the id is not found"
    );
}

#[test]
fn empty_manifest_returns_none() {
    let mut manifest: Vec<SupplyCrate> = vec![];
    let result = call_or_hint!(
        "ex07",
        "move_crate_to_shuttle",
        move_crate_to_shuttle(&mut manifest, "A1")
    );
    assert!(result.is_none(), "empty manifest always returns None");
}
