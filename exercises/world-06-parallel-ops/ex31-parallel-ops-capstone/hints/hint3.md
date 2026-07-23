## Hint 3: Algorithm Outline

```
function run_parallel_pipeline(jobs):
    Step 1: Create mpsc channel (tx, rx) for PipelineStageOutput
    Step 2: Create Arc<Mutex<i32>> checksum initialized to 0

    Step 3: For each job (into_iter):
            Clone tx → tx_worker
            Spawn thread with move closure:
                → stage_one = job.payload * 2
                → stage_two = job.payload * 3
                → send PipelineStageOutput { job_id, stage_one, stage_two }
                  via tx_worker (map error → ChannelClosed)
            Push JoinHandle

    Step 4: Drop original tx (so rx.iter() terminates)

    Step 5: Collect outputs from the channel:
            For each output from rx.iter():
                → lock checksum mutex (map PoisonError → LockPoisoned)
                → *guard += output.stage_two
                → push output into results Vec

    Step 6: Join all JoinHandles
            → on panic, return WorkerPanicked

    Step 7: Sort results by job_id ascending

    Step 8: Read final checksum (lock mutex, dereference)

    Step 9: Return Ok(ParallelPipelineReport { outputs: results, checksum })
```

**Spoiler threshold:** High—full pipeline algorithm without Rust syntax.
