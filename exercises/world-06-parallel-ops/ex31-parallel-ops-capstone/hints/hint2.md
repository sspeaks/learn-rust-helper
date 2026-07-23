## Hint 2: Tools & Types

- **Channel:** `mpsc::channel::<PipelineStageOutput>()` — workers send, main thread receives.
- **Shared checksum:** `Arc::new(Mutex::new(0i32))` — accumulates `stage_two` values.
- **Worker computation:** `stage_one = payload * 2`, `stage_two = payload * 3`.
- **Main thread loop:** After dropping the original sender, use `rx.iter()` to drain all outputs.
  - For each output: lock checksum mutex, add `output.stage_two`.
  - Push output into a `Vec<PipelineStageOutput>`.
- **Sorting:** After collecting all outputs, sort by `job_id` with `.sort_by(|a, b| a.job_id.cmp(&b.job_id))`.
- **Final checksum:** Lock the mutex one last time and dereference to get the `i32`.

**Spoiler threshold:** Medium—names all three mechanisms and their connection points.
