use ex36_data_migration::{current_schema_version, migrate_schema, DataMigrationError};
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

#[test]
fn current_schema_version_defaults_to_zero() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");

    let version = call_or_hint!(
        "ex36",
        "current_schema_version",
        current_schema_version(&conn)
    )
    .expect("reading version should succeed");

    assert_eq!(version, 0);
}

#[test]
fn migrate_to_zero_returns_no_steps() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");

    let steps = call_or_hint!("ex36", "migrate_schema", migrate_schema(&conn, 0))
        .expect("migrating to current version should succeed");

    assert!(steps.is_empty());
}

#[test]
fn negative_target_version_is_rejected() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");

    let result = call_or_hint!("ex36", "migrate_schema", migrate_schema(&conn, -1));

    assert!(
        matches!(result, Err(DataMigrationError::UnsupportedVersion(-1))),
        "negative targets should be reported as unsupported"
    );
}

#[test]
fn migrate_to_one_sets_version_and_returns_one_step() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");

    let steps = call_or_hint!("ex36", "migrate_schema", migrate_schema(&conn, 1))
        .expect("migration to v1 should succeed");

    assert_eq!(steps.len(), 1, "0 -> 1 should require one step");
    assert_eq!(steps[0].from_version, 0);
    assert_eq!(steps[0].to_version, 1);
    assert!(
        !steps[0].description.trim().is_empty(),
        "migration descriptions should be non-empty"
    );

    let version = call_or_hint!(
        "ex36",
        "current_schema_version",
        current_schema_version(&conn)
    )
    .expect("reading version should succeed");
    assert_eq!(version, 1);
}

#[test]
fn migrate_to_same_version_is_idempotent() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");

    call_or_hint!("ex36", "migrate_schema", migrate_schema(&conn, 1))
        .expect("first migration should succeed");

    let again = call_or_hint!("ex36", "migrate_schema", migrate_schema(&conn, 1))
        .expect("second migration to same version should succeed");

    assert!(
        again.is_empty(),
        "idempotent rerun should produce zero steps"
    );
}

#[test]
fn migrate_to_two_from_zero_returns_contiguous_steps() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");

    let steps = call_or_hint!("ex36", "migrate_schema", migrate_schema(&conn, 2))
        .expect("migration to v2 should succeed");

    assert_eq!(steps.len(), 2, "0 -> 2 should include two ordered steps");
    assert_eq!(steps[0].from_version, 0);
    assert_eq!(steps[0].to_version, 1);
    assert_eq!(steps[1].from_version, 1);
    assert_eq!(steps[1].to_version, 2);

    let version = call_or_hint!(
        "ex36",
        "current_schema_version",
        current_schema_version(&conn)
    )
    .expect("reading version should succeed");
    assert_eq!(version, 2);
}

#[test]
fn migrate_from_one_to_two_returns_single_step() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");

    call_or_hint!("ex36", "migrate_schema", migrate_schema(&conn, 1))
        .expect("migration to v1 should succeed");

    let steps = call_or_hint!("ex36", "migrate_schema", migrate_schema(&conn, 2))
        .expect("migration from v1 to v2 should succeed");

    assert_eq!(steps.len(), 1);
    assert_eq!(steps[0].from_version, 1);
    assert_eq!(steps[0].to_version, 2);
}

#[test]
fn unsupported_high_target_does_not_change_current_version() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");

    call_or_hint!("ex36", "migrate_schema", migrate_schema(&conn, 1))
        .expect("migration to v1 should succeed");

    let result = call_or_hint!("ex36", "migrate_schema", migrate_schema(&conn, 999));
    assert!(
        matches!(result, Err(DataMigrationError::UnsupportedVersion(999))),
        "unsupported targets should return UnsupportedVersion"
    );

    let version = call_or_hint!(
        "ex36",
        "current_schema_version",
        current_schema_version(&conn)
    )
    .expect("reading version should succeed");
    assert_eq!(
        version, 1,
        "failed migration should not mutate stored version"
    );
}
