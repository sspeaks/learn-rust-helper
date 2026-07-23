use ex27_cargo_channel::{dispatch_cargo_jobs, CargoJob, CargoReceipt};

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

fn job(cargo_id: &str, destination: &str) -> CargoJob {
    CargoJob {
        cargo_id: cargo_id.to_string(),
        destination: destination.to_string(),
    }
}

#[test]
fn empty_jobs_return_empty_receipts() {
    let receipts = call_or_hint!("ex27", "dispatch_cargo_jobs", dispatch_cargo_jobs(vec![]))
        .expect("empty cargo list should succeed");

    assert!(receipts.is_empty());
}

#[test]
fn single_job_round_trips_into_receipt() {
    let receipts = call_or_hint!(
        "ex27",
        "dispatch_cargo_jobs",
        dispatch_cargo_jobs(vec![job("cargo-1", "dock-a")])
    )
    .expect("single cargo job should dispatch");

    assert_eq!(
        receipts,
        vec![CargoReceipt {
            cargo_id: "cargo-1".to_string(),
            delivered_to: "dock-a".to_string(),
        }]
    );
}

#[test]
fn multiple_jobs_preserve_submission_order() {
    let jobs = vec![
        job("cargo-1", "dock-a"),
        job("cargo-2", "dock-b"),
        job("cargo-3", "dock-c"),
    ];

    let receipts = call_or_hint!("ex27", "dispatch_cargo_jobs", dispatch_cargo_jobs(jobs))
        .expect("valid cargo jobs should all succeed");

    let ids: Vec<&str> = receipts
        .iter()
        .map(|receipt| receipt.cargo_id.as_str())
        .collect();
    assert_eq!(ids, vec!["cargo-1", "cargo-2", "cargo-3"]);
}

#[test]
fn duplicate_cargo_ids_are_treated_as_data() {
    let jobs = vec![
        job("cargo-x", "dock-a"),
        job("cargo-x", "dock-b"),
        job("cargo-x", "dock-c"),
    ];

    let receipts = call_or_hint!("ex27", "dispatch_cargo_jobs", dispatch_cargo_jobs(jobs))
        .expect("duplicate ids should still dispatch by position");

    let destinations: Vec<&str> = receipts
        .iter()
        .map(|receipt| receipt.delivered_to.as_str())
        .collect();
    assert_eq!(destinations, vec!["dock-a", "dock-b", "dock-c"]);
}

#[test]
fn destinations_with_symbols_are_preserved() {
    let jobs = vec![job("cargo-symbols", "bay-7/sector-β")];
    let receipts = call_or_hint!("ex27", "dispatch_cargo_jobs", dispatch_cargo_jobs(jobs))
        .expect("UTF-8/symbol destinations should round-trip");

    assert_eq!(receipts[0].delivered_to, "bay-7/sector-β");
}

#[test]
fn repeated_runs_return_same_receipts() {
    let jobs = vec![job("cargo-a", "dock-a"), job("cargo-b", "dock-b")];

    let first = call_or_hint!(
        "ex27",
        "dispatch_cargo_jobs",
        dispatch_cargo_jobs(jobs.clone())
    )
    .expect("first run should succeed");

    let second = call_or_hint!("ex27", "dispatch_cargo_jobs", dispatch_cargo_jobs(jobs))
        .expect("second run should succeed");

    assert_eq!(first, second, "channel dispatch should be deterministic");
}

#[test]
fn every_job_produces_one_receipt() {
    let jobs = (0..15)
        .map(|index| job(&format!("cargo-{index}"), &format!("dock-{index}")))
        .collect::<Vec<_>>();

    let receipts = call_or_hint!("ex27", "dispatch_cargo_jobs", dispatch_cargo_jobs(jobs))
        .expect("batch dispatch should succeed");

    assert_eq!(receipts.len(), 15);
}

#[test]
fn empty_strings_are_valid_payload_data() {
    let jobs = vec![job("", "")];
    let receipts = call_or_hint!("ex27", "dispatch_cargo_jobs", dispatch_cargo_jobs(jobs))
        .expect("empty strings should be treated as input data");

    assert_eq!(receipts[0].cargo_id, "");
    assert_eq!(receipts[0].delivered_to, "");
}
