use ex31_parallel_ops_capstone::{run_parallel_pipeline, PipelineJob};

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

fn job(job_id: &str, payload: i32) -> PipelineJob {
    PipelineJob {
        job_id: job_id.to_string(),
        payload,
    }
}

#[test]
fn empty_pipeline_returns_empty_outputs_and_zero_checksum() {
    let report = call_or_hint!(
        "ex31",
        "run_parallel_pipeline",
        run_parallel_pipeline(vec![])
    )
    .expect("empty pipeline input should succeed");

    assert!(report.outputs.is_empty());
    assert_eq!(report.checksum, 0);
}

#[test]
fn single_job_output_keeps_job_identity() {
    let report = call_or_hint!(
        "ex31",
        "run_parallel_pipeline",
        run_parallel_pipeline(vec![job("job-1", 7)])
    )
    .expect("single job should process successfully");

    assert_eq!(report.outputs.len(), 1);
    assert_eq!(report.outputs[0].job_id, "job-1");
}

#[test]
fn output_count_matches_input_count() {
    let jobs = vec![job("a", 1), job("b", 2), job("c", 3), job("d", 4)];
    let report = call_or_hint!("ex31", "run_parallel_pipeline", run_parallel_pipeline(jobs))
        .expect("all jobs should process");

    assert_eq!(report.outputs.len(), 4);
}

#[test]
fn outputs_preserve_input_order() {
    let jobs = vec![job("alpha", 10), job("beta", 20), job("gamma", 30)];
    let report = call_or_hint!("ex31", "run_parallel_pipeline", run_parallel_pipeline(jobs))
        .expect("pipeline should return deterministic order");

    let ids: Vec<&str> = report
        .outputs
        .iter()
        .map(|output| output.job_id.as_str())
        .collect();
    assert_eq!(ids, vec!["alpha", "beta", "gamma"]);
}

#[test]
fn checksum_equals_sum_of_stage_two_outputs() {
    let jobs = vec![job("a", 5), job("b", 6), job("c", 7)];
    let report = call_or_hint!("ex31", "run_parallel_pipeline", run_parallel_pipeline(jobs))
        .expect("pipeline should produce stage outputs");

    let summed_stage_two: i32 = report.outputs.iter().map(|output| output.stage_two).sum();
    assert_eq!(report.checksum, summed_stage_two);
}

#[test]
fn repeated_runs_with_same_jobs_are_deterministic() {
    let jobs = vec![job("a", 2), job("b", 4), job("c", 6)];

    let first = call_or_hint!(
        "ex31",
        "run_parallel_pipeline",
        run_parallel_pipeline(jobs.clone())
    )
    .expect("first run should succeed");

    let second = call_or_hint!("ex31", "run_parallel_pipeline", run_parallel_pipeline(jobs))
        .expect("second run should succeed");

    assert_eq!(
        first, second,
        "pipeline report should be stable across runs"
    );
}

#[test]
fn duplicate_job_ids_are_supported_as_distinct_positions() {
    let jobs = vec![job("dup", 1), job("dup", 2), job("dup", 3)];
    let report = call_or_hint!("ex31", "run_parallel_pipeline", run_parallel_pipeline(jobs))
        .expect("duplicate IDs should not cause dropped outputs");

    assert_eq!(report.outputs.len(), 3);
    let payloads: Vec<i32> = report
        .outputs
        .iter()
        .map(|output| output.stage_one)
        .collect();
    assert_eq!(payloads.len(), 3);
}

#[test]
fn negative_payloads_are_handled_deterministically() {
    let jobs = vec![job("neg-a", -4), job("neg-b", -8)];

    let first = call_or_hint!(
        "ex31",
        "run_parallel_pipeline",
        run_parallel_pipeline(jobs.clone())
    )
    .expect("negative payloads should be processable");

    let second = call_or_hint!("ex31", "run_parallel_pipeline", run_parallel_pipeline(jobs))
        .expect("second run should also succeed");

    assert_eq!(first, second);
}
