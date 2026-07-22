use ex32_archive_query::{query_archive, ArchiveFilter, ArchiveQueryError};
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

fn create_archive_table(conn: &Connection) {
    conn.execute_batch(
        "
        CREATE TABLE archive_records (
            id INTEGER PRIMARY KEY,
            mission_code TEXT NOT NULL,
            priority INTEGER NOT NULL,
            archived INTEGER NOT NULL CHECK (archived IN (0, 1))
        );
        ",
    )
    .expect("archive table setup should succeed");
}

fn seed_archive(conn: &Connection) {
    let rows = [
        (1_i64, "ALPHA-1", 2_u8, 0_i64),
        (2_i64, "ALPHA-2", 4_u8, 1_i64),
        (3_i64, "BETA-1", 5_u8, 0_i64),
        (4_i64, "GAMMA-1", 1_u8, 0_i64),
    ];

    for (id, mission_code, priority, archived) in rows {
        conn.execute(
            "INSERT INTO archive_records (id, mission_code, priority, archived) VALUES (?1, ?2, ?3, ?4)",
            params![id, mission_code, priority, archived],
        )
        .expect("seed insert should succeed");
    }
}

fn filter(min_priority: u8, mission_prefix: &str, include_archived: bool) -> ArchiveFilter {
    ArchiveFilter {
        min_priority,
        mission_prefix: mission_prefix.to_string(),
        include_archived,
    }
}

#[test]
fn query_missing_table_returns_sql_error() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");
    let result = call_or_hint!(
        "ex32",
        "query_archive",
        query_archive(&conn, &filter(0, "", true))
    );

    assert!(
        matches!(result, Err(ArchiveQueryError::Sql(_))),
        "missing schema should surface SQL errors"
    );
}

#[test]
fn include_archived_false_excludes_archived_rows() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");
    create_archive_table(&conn);
    seed_archive(&conn);

    let records = call_or_hint!(
        "ex32",
        "query_archive",
        query_archive(&conn, &filter(0, "", false))
    )
    .expect("query should succeed");

    assert!(records.iter().all(|record| !record.archived));
    assert_eq!(records.len(), 3);
}

#[test]
fn include_archived_true_includes_all_matching_rows() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");
    create_archive_table(&conn);
    seed_archive(&conn);

    let records = call_or_hint!(
        "ex32",
        "query_archive",
        query_archive(&conn, &filter(0, "", true))
    )
    .expect("query should succeed");

    assert_eq!(records.len(), 4);
}

#[test]
fn min_priority_filter_is_inclusive() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");
    create_archive_table(&conn);
    seed_archive(&conn);

    let records = call_or_hint!(
        "ex32",
        "query_archive",
        query_archive(&conn, &filter(4, "", true))
    )
    .expect("query should succeed");

    let priorities: Vec<u8> = records.iter().map(|record| record.priority).collect();
    assert_eq!(priorities, vec![4, 5]);
}

#[test]
fn mission_prefix_filter_matches_prefix_only() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");
    create_archive_table(&conn);
    seed_archive(&conn);

    let records = call_or_hint!(
        "ex32",
        "query_archive",
        query_archive(&conn, &filter(0, "ALPHA", true))
    )
    .expect("query should succeed");

    let codes: Vec<&str> = records
        .iter()
        .map(|record| record.mission_code.as_str())
        .collect();
    assert_eq!(codes, vec!["ALPHA-1", "ALPHA-2"]);
}

#[test]
fn combined_filters_apply_all_constraints() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");
    create_archive_table(&conn);
    seed_archive(&conn);

    let records = call_or_hint!(
        "ex32",
        "query_archive",
        query_archive(&conn, &filter(3, "ALPHA", false))
    )
    .expect("query should succeed");

    assert!(
        records.is_empty(),
        "ALPHA row with priority>=3 is archived and excluded"
    );
}

#[test]
fn query_results_are_sorted_by_id_ascending() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");
    create_archive_table(&conn);
    seed_archive(&conn);

    let records = call_or_hint!(
        "ex32",
        "query_archive",
        query_archive(&conn, &filter(0, "", true))
    )
    .expect("query should succeed");

    let ids: Vec<i64> = records.iter().map(|record| record.id).collect();
    assert_eq!(ids, vec![1, 2, 3, 4]);
}

#[test]
fn injection_shaped_prefix_is_treated_as_literal_data() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");
    create_archive_table(&conn);
    seed_archive(&conn);

    let records = call_or_hint!(
        "ex32",
        "query_archive",
        query_archive(&conn, &filter(0, "ALPHA' OR 1=1 --", true))
    )
    .expect("query should succeed");

    assert!(
        records.is_empty(),
        "parameterized queries should not treat prefix text as executable SQL"
    );
}
