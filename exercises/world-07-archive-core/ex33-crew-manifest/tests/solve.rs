use ex33_crew_manifest::{
    insert_crew_member, load_crew_manifest, CrewManifestEntry, CrewManifestError,
};
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

fn entry(crew_id: &str, role: &str, rank: u8) -> CrewManifestEntry {
    CrewManifestEntry {
        crew_id: crew_id.to_string(),
        role: role.to_string(),
        rank,
    }
}

fn create_manifest_table(conn: &Connection) {
    conn.execute_batch(
        "
        CREATE TABLE crew_manifest (
            crew_id TEXT PRIMARY KEY,
            role TEXT NOT NULL,
            rank INTEGER NOT NULL CHECK(rank BETWEEN 0 AND 255)
        );
        ",
    )
    .expect("manifest schema setup should succeed");
}

#[test]
fn load_empty_manifest_returns_empty_vec() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");
    create_manifest_table(&conn);

    let loaded = call_or_hint!("ex33", "load_crew_manifest", load_crew_manifest(&conn))
        .expect("loading from empty table should succeed");

    assert!(loaded.is_empty());
}

#[test]
fn insert_single_member_round_trips_through_load() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");
    create_manifest_table(&conn);

    let nova = entry("nova", "pilot", 7);
    call_or_hint!(
        "ex33",
        "insert_crew_member",
        insert_crew_member(&conn, &nova)
    )
    .expect("insert should succeed");

    let loaded = call_or_hint!("ex33", "load_crew_manifest", load_crew_manifest(&conn))
        .expect("load should succeed");

    assert_eq!(loaded, vec![nova]);
}

#[test]
fn load_manifest_returns_rows_in_stable_crew_id_order() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");
    create_manifest_table(&conn);

    for member in [
        entry("zeta", "engineer", 4),
        entry("alpha", "scout", 9),
        entry("mira", "analyst", 6),
    ] {
        call_or_hint!(
            "ex33",
            "insert_crew_member",
            insert_crew_member(&conn, &member)
        )
        .expect("insert should succeed");
    }

    let loaded = call_or_hint!("ex33", "load_crew_manifest", load_crew_manifest(&conn))
        .expect("load should succeed");

    let ids: Vec<&str> = loaded
        .iter()
        .map(|member| member.crew_id.as_str())
        .collect();
    assert_eq!(ids, vec!["alpha", "mira", "zeta"]);
}

#[test]
fn duplicate_primary_key_returns_sql_error() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");
    create_manifest_table(&conn);

    let member = entry("nova", "pilot", 7);
    call_or_hint!(
        "ex33",
        "insert_crew_member",
        insert_crew_member(&conn, &member)
    )
    .expect("first insert should succeed");

    let second = call_or_hint!(
        "ex33",
        "insert_crew_member",
        insert_crew_member(&conn, &member)
    );
    assert!(
        matches!(second, Err(CrewManifestError::Sql(_))),
        "duplicate crew_id should surface SQL constraint errors"
    );
}

#[test]
fn injection_shaped_crew_id_is_stored_as_data() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");
    create_manifest_table(&conn);

    let injected = entry("x' OR 1=1 --", "operator", 3);
    call_or_hint!(
        "ex33",
        "insert_crew_member",
        insert_crew_member(&conn, &injected)
    )
    .expect("parameterized inserts should treat SQL-like text as plain data");

    let loaded = call_or_hint!("ex33", "load_crew_manifest", load_crew_manifest(&conn))
        .expect("load should succeed");

    assert_eq!(loaded, vec![injected]);
}

#[test]
fn rank_boundaries_round_trip_correctly() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");
    create_manifest_table(&conn);

    let low = entry("low", "cadet", 0);
    let high = entry("high", "admiral", 255);

    call_or_hint!(
        "ex33",
        "insert_crew_member",
        insert_crew_member(&conn, &low)
    )
    .expect("insert low rank should succeed");
    call_or_hint!(
        "ex33",
        "insert_crew_member",
        insert_crew_member(&conn, &high)
    )
    .expect("insert high rank should succeed");

    let loaded = call_or_hint!("ex33", "load_crew_manifest", load_crew_manifest(&conn))
        .expect("load should succeed");

    assert_eq!(loaded, vec![high, low]);
}

#[test]
fn insert_without_schema_returns_sql_error() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");

    let result = call_or_hint!(
        "ex33",
        "insert_crew_member",
        insert_crew_member(&conn, &entry("nova", "pilot", 7))
    );

    assert!(
        matches!(result, Err(CrewManifestError::Sql(_))),
        "missing table should be surfaced as SQL error"
    );
}

#[test]
fn load_without_schema_returns_sql_error() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");

    let result = call_or_hint!("ex33", "load_crew_manifest", load_crew_manifest(&conn));

    assert!(
        matches!(result, Err(CrewManifestError::Sql(_))),
        "missing table should be surfaced as SQL error"
    );
}
