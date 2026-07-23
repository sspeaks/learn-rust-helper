use ex26_crew_dispatch::{run_crew_dispatch, CrewTask, CrewTaskResult};

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

fn task(crew_id: &str, units: u32) -> CrewTask {
    CrewTask {
        crew_id: crew_id.to_string(),
        units,
    }
}

#[test]
fn empty_dispatch_returns_empty_results() {
    let results = call_or_hint!("ex26", "run_crew_dispatch", run_crew_dispatch(vec![]))
        .expect("empty input should succeed");

    assert!(results.is_empty());
}

#[test]
fn single_task_round_trips_into_single_result() {
    let results = call_or_hint!(
        "ex26",
        "run_crew_dispatch",
        run_crew_dispatch(vec![task("crew-a", 8)])
    )
    .expect("single task should dispatch successfully");

    assert_eq!(
        results,
        vec![CrewTaskResult {
            crew_id: "crew-a".to_string(),
            delivered_units: 8,
        }]
    );
}

#[test]
fn multiple_tasks_preserve_input_order() {
    let tasks = vec![task("crew-a", 2), task("crew-b", 4), task("crew-c", 6)];
    let results = call_or_hint!("ex26", "run_crew_dispatch", run_crew_dispatch(tasks))
        .expect("valid tasks should all dispatch");

    let ids: Vec<&str> = results
        .iter()
        .map(|result| result.crew_id.as_str())
        .collect();
    assert_eq!(ids, vec!["crew-a", "crew-b", "crew-c"]);
}

#[test]
fn duplicate_crew_ids_are_preserved_by_position() {
    let tasks = vec![task("crew-a", 1), task("crew-a", 3), task("crew-a", 5)];
    let results = call_or_hint!("ex26", "run_crew_dispatch", run_crew_dispatch(tasks))
        .expect("duplicate ids are valid task data");

    let units: Vec<u32> = results
        .iter()
        .map(|result| result.delivered_units)
        .collect();
    assert_eq!(units, vec![1, 3, 5]);
}

#[test]
fn zero_unit_tasks_are_handled_without_special_cases() {
    let tasks = vec![task("crew-zero", 0)];
    let results = call_or_hint!("ex26", "run_crew_dispatch", run_crew_dispatch(tasks))
        .expect("zero-unit tasks should still produce receipts");

    assert_eq!(results[0].delivered_units, 0);
}

#[test]
fn large_unit_values_are_preserved() {
    let tasks = vec![task("crew-max", u32::MAX)];
    let results = call_or_hint!("ex26", "run_crew_dispatch", run_crew_dispatch(tasks))
        .expect("large values should not be truncated");

    assert_eq!(results[0].delivered_units, u32::MAX);
}

#[test]
fn repeated_runs_are_deterministic_for_same_input() {
    let tasks = vec![task("crew-a", 7), task("crew-b", 9), task("crew-c", 11)];

    let first = call_or_hint!(
        "ex26",
        "run_crew_dispatch",
        run_crew_dispatch(tasks.clone())
    )
    .expect("first run should succeed");

    let second = call_or_hint!("ex26", "run_crew_dispatch", run_crew_dispatch(tasks))
        .expect("second run should succeed");

    assert_eq!(first, second, "dispatch output must be deterministic");
}

#[test]
fn all_inputs_receive_exactly_one_output() {
    let tasks = (0..12)
        .map(|index| task(&format!("crew-{index}"), index))
        .collect::<Vec<_>>();

    let results = call_or_hint!("ex26", "run_crew_dispatch", run_crew_dispatch(tasks))
        .expect("batch dispatch should succeed");

    assert_eq!(results.len(), 12, "each task should yield one result");
}
