use ex32_schema_bootstrap::{bootstrap_archive_schema, ensure_schema_version};
use rusqlite::{params, Connection};

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

fn user_version(conn: &Connection) -> i64 {
    conn.query_row("PRAGMA user_version", [], |row| row.get(0))
        .expect("PRAGMA user_version should always be readable")
}

fn count_user_objects(conn: &Connection, object_type: &str) -> i64 {
    conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type = ?1 AND name NOT LIKE 'sqlite_%'",
        params![object_type],
        |row| row.get(0),
    )
    .expect("sqlite_master should be queryable")
}

#[test]
fn bootstrap_schema_succeeds_on_fresh_in_memory_db() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");

    call_or_hint!(
        "ex32",
        "bootstrap_archive_schema",
        bootstrap_archive_schema(&conn)
    )
    .expect("schema bootstrap should succeed on fresh DB");
}

#[test]
fn bootstrap_schema_is_idempotent() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");

    call_or_hint!(
        "ex32",
        "bootstrap_archive_schema",
        bootstrap_archive_schema(&conn)
    )
    .expect("first bootstrap should succeed");

    call_or_hint!(
        "ex32",
        "bootstrap_archive_schema",
        bootstrap_archive_schema(&conn)
    )
    .expect("second bootstrap should also succeed without conflicts");
}

#[test]
fn bootstrap_schema_creates_at_least_one_user_table() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");

    call_or_hint!(
        "ex32",
        "bootstrap_archive_schema",
        bootstrap_archive_schema(&conn)
    )
    .expect("bootstrap should create schema objects");

    assert!(
        count_user_objects(&conn, "table") > 0,
        "bootstrap should create non-system tables"
    );
}

#[test]
fn bootstrap_schema_creates_at_least_one_user_index() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");

    call_or_hint!(
        "ex32",
        "bootstrap_archive_schema",
        bootstrap_archive_schema(&conn)
    )
    .expect("bootstrap should create schema objects");

    assert!(
        count_user_objects(&conn, "index") > 0,
        "bootstrap should create non-system indexes"
    );
}

#[test]
fn ensure_schema_version_sets_user_version() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");

    call_or_hint!(
        "ex32",
        "ensure_schema_version",
        ensure_schema_version(&conn, 3)
    )
    .expect("setting schema version should succeed");

    assert_eq!(user_version(&conn), 3);
}

#[test]
fn ensure_schema_version_overwrites_existing_value() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");

    call_or_hint!(
        "ex32",
        "ensure_schema_version",
        ensure_schema_version(&conn, 1)
    )
    .expect("initial version set should succeed");

    call_or_hint!(
        "ex32",
        "ensure_schema_version",
        ensure_schema_version(&conn, 7)
    )
    .expect("updating version should succeed");

    assert_eq!(user_version(&conn), 7);
}

#[test]
fn ensure_schema_version_accepts_zero() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");

    call_or_hint!(
        "ex32",
        "ensure_schema_version",
        ensure_schema_version(&conn, 0)
    )
    .expect("version zero should be allowed");

    assert_eq!(user_version(&conn), 0);
}

#[test]
fn ensure_schema_version_after_bootstrap_keeps_schema_objects() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");

    call_or_hint!(
        "ex32",
        "bootstrap_archive_schema",
        bootstrap_archive_schema(&conn)
    )
    .expect("bootstrap should succeed");
    let table_count_before = count_user_objects(&conn, "table");

    call_or_hint!(
        "ex32",
        "ensure_schema_version",
        ensure_schema_version(&conn, 2)
    )
    .expect("setting version after bootstrap should succeed");

    assert_eq!(count_user_objects(&conn, "table"), table_count_before);
}
