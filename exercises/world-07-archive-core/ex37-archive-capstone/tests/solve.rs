use ex37_archive_capstone::{
    fetch_archive_batch, load_archive_preview, persist_archive_batch, ArchiveBatch,
    ArchiveCapstoneError, RemoteArchiveRecord,
};
use rusqlite::{params, Connection};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

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

macro_rules! async_call_or_hint {
    ($ex:expr, $fn:expr, $body:expr) => {{
        match tokio::task::spawn(async move { $body }).await {
            Ok(v) => v,
            Err(join_error) => {
                if join_error.is_panic() {
                    let panic_payload = join_error.into_panic();
                    if is_stub_panic(&panic_payload) {
                        panic!(
                            "\n\n  ✖  {} '{}' not started — fill in src/lib.rs\n",
                            $ex, $fn
                        );
                    }
                    std::panic::resume_unwind(panic_payload);
                }
                panic!("\n\n  ✖  {} '{}' task cancelled unexpectedly\n", $ex, $fn);
            }
        }
    }};
}

fn record(mission_code: &str, artifact: &str, priority: u8) -> RemoteArchiveRecord {
    RemoteArchiveRecord {
        mission_code: mission_code.to_string(),
        artifact: artifact.to_string(),
        priority,
    }
}

fn create_archive_table(conn: &Connection, with_priority_check: bool) {
    let ddl = if with_priority_check {
        "
        CREATE TABLE archive_records (
            mission_code TEXT NOT NULL,
            artifact TEXT NOT NULL,
            priority INTEGER NOT NULL CHECK(priority <= 5),
            UNIQUE(mission_code, artifact)
        );
        "
    } else {
        "
        CREATE TABLE archive_records (
            mission_code TEXT NOT NULL,
            artifact TEXT NOT NULL,
            priority INTEGER NOT NULL,
            UNIQUE(mission_code, artifact)
        );
        "
    };

    conn.execute_batch(ddl)
        .expect("archive table setup should succeed");
}

fn row_count(conn: &Connection) -> i64 {
    conn.query_row("SELECT COUNT(*) FROM archive_records", [], |row| row.get(0))
        .expect("count query should succeed")
}

#[tokio::test(flavor = "multi_thread")]
async fn fetch_archive_batch_successfully_decodes_payload() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/archive/M-42"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"{
                "fetched_at":"2026-07-21T15:53:52.323-07:00",
                "records":[
                    {"mission_code":"M-42","artifact":"core","priority":3},
                    {"mission_code":"M-42","artifact":"map","priority":1}
                ]
            }"#,
        ))
        .expect(1)
        .mount(&server)
        .await;

    let base_url = server.uri();
    let batch = async_call_or_hint!("ex37", "fetch_archive_batch", {
        let base_url = base_url.clone();
        fetch_archive_batch(&base_url, "M-42").await
    })
    .expect("valid remote payload should decode");

    assert_eq!(batch.fetched_at, "2026-07-21T15:53:52.323-07:00");
    assert_eq!(batch.records.len(), 2);
    assert_eq!(batch.records[0].artifact, "core");
}

#[tokio::test(flavor = "multi_thread")]
async fn fetch_archive_batch_invalid_status_maps_to_error() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/archive/M-500"))
        .respond_with(ResponseTemplate::new(500))
        .expect(1)
        .mount(&server)
        .await;

    let base_url = server.uri();
    let result = async_call_or_hint!("ex37", "fetch_archive_batch", {
        let base_url = base_url.clone();
        fetch_archive_batch(&base_url, "M-500").await
    });

    match result {
        Err(ArchiveCapstoneError::InvalidStatus(status)) => {
            assert_eq!(status.as_u16(), 500);
        }
        other => panic!("expected InvalidStatus(500), got {other:?}"),
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn fetch_archive_batch_invalid_json_maps_to_decode_error() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/archive/M-bad"))
        .respond_with(ResponseTemplate::new(200).set_body_string("not-json"))
        .expect(1)
        .mount(&server)
        .await;

    let base_url = server.uri();
    let result = async_call_or_hint!("ex37", "fetch_archive_batch", {
        let base_url = base_url.clone();
        fetch_archive_batch(&base_url, "M-bad").await
    });

    assert!(
        matches!(result, Err(ArchiveCapstoneError::Decode(_))),
        "malformed JSON should map to Decode"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn fetch_archive_batch_unreachable_host_maps_to_request_error() {
    let result = async_call_or_hint!("ex37", "fetch_archive_batch", {
        fetch_archive_batch("http://127.0.0.1:9", "M-offline").await
    });

    assert!(
        matches!(result, Err(ArchiveCapstoneError::Request(_))),
        "transport failures should map to Request"
    );
}

#[test]
fn persist_archive_batch_inserts_new_rows() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");
    create_archive_table(&conn, false);

    let batch = ArchiveBatch {
        fetched_at: "now".to_string(),
        records: vec![record("M-1", "core", 3), record("M-1", "map", 2)],
    };

    let report = call_or_hint!(
        "ex37",
        "persist_archive_batch",
        persist_archive_batch(&conn, &batch)
    )
    .expect("persist should succeed");

    assert_eq!(report.inserted, 2);
    assert_eq!(report.skipped, 0);
    assert_eq!(row_count(&conn), 2);
}

#[test]
fn persist_archive_batch_skips_duplicate_records() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");
    create_archive_table(&conn, false);

    conn.execute(
        "INSERT INTO archive_records (mission_code, artifact, priority) VALUES (?1, ?2, ?3)",
        params!["M-1", "core", 3_u8],
    )
    .expect("seed insert should succeed");

    let batch = ArchiveBatch {
        fetched_at: "now".to_string(),
        records: vec![record("M-1", "core", 3), record("M-1", "map", 2)],
    };

    let report = call_or_hint!(
        "ex37",
        "persist_archive_batch",
        persist_archive_batch(&conn, &batch)
    )
    .expect("persist should succeed with duplicate skip");

    assert_eq!(report.inserted, 1);
    assert_eq!(report.skipped, 1);
    assert_eq!(row_count(&conn), 2);
}

#[test]
fn persist_archive_batch_is_atomic_when_sql_error_occurs() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");
    create_archive_table(&conn, true);

    let batch = ArchiveBatch {
        fetched_at: "now".to_string(),
        records: vec![
            record("M-1", "core", 3),
            record("M-1", "invalid-priority", 9),
        ],
    };

    let result = call_or_hint!(
        "ex37",
        "persist_archive_batch",
        persist_archive_batch(&conn, &batch)
    );

    assert!(
        matches!(result, Err(ArchiveCapstoneError::Sql(_))),
        "constraint failures should map to SQL errors"
    );
    assert_eq!(
        row_count(&conn),
        0,
        "failed batch should roll back all inserts in the transaction"
    );
}

#[test]
fn load_archive_preview_respects_limit_and_order() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");
    create_archive_table(&conn, false);

    let batch = ArchiveBatch {
        fetched_at: "now".to_string(),
        records: vec![
            record("M-1", "core", 3),
            record("M-2", "map", 2),
            record("M-3", "beacon", 1),
        ],
    };

    call_or_hint!(
        "ex37",
        "persist_archive_batch",
        persist_archive_batch(&conn, &batch)
    )
    .expect("persist should succeed");

    let preview = call_or_hint!(
        "ex37",
        "load_archive_preview",
        load_archive_preview(&conn, 2)
    )
    .expect("preview should load");

    assert_eq!(preview.len(), 2);
    assert_eq!(preview[0], record("M-1", "core", 3));
    assert_eq!(preview[1], record("M-2", "map", 2));
}

#[test]
fn load_archive_preview_with_zero_limit_returns_empty_vec() {
    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");
    create_archive_table(&conn, false);

    let preview = call_or_hint!(
        "ex37",
        "load_archive_preview",
        load_archive_preview(&conn, 0)
    )
    .expect("preview with limit=0 should succeed");

    assert!(preview.is_empty());
}

#[tokio::test(flavor = "multi_thread")]
async fn staged_fetch_then_sync_persist_then_preview_round_trip() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/archive/M-stage"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"{
                "fetched_at":"2026-07-21T15:53:52.323-07:00",
                "records":[
                    {"mission_code":"M-stage","artifact":"logbook","priority":4},
                    {"mission_code":"M-stage","artifact":"beacon","priority":2}
                ]
            }"#,
        ))
        .expect(1)
        .mount(&server)
        .await;

    let base_url = server.uri();
    let batch = async_call_or_hint!("ex37", "fetch_archive_batch", {
        let base_url = base_url.clone();
        fetch_archive_batch(&base_url, "M-stage").await
    })
    .expect("fetch stage should succeed");

    let conn = Connection::open_in_memory().expect("in-memory sqlite should open");
    create_archive_table(&conn, false);

    let report = call_or_hint!(
        "ex37",
        "persist_archive_batch",
        persist_archive_batch(&conn, &batch)
    )
    .expect("sync persist stage should succeed");
    assert_eq!(report.inserted, 2);
    assert_eq!(report.skipped, 0);

    let preview = call_or_hint!(
        "ex37",
        "load_archive_preview",
        load_archive_preview(&conn, 10)
    )
    .expect("preview load should succeed");

    assert_eq!(preview.len(), 2);
    assert_eq!(preview[0].mission_code, "M-stage");
    assert_eq!(preview[1].mission_code, "M-stage");
}
