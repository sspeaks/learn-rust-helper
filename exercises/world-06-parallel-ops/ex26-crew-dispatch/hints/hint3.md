## Hint 3: Algorithm Outline

```
function run_crew_dispatch(tasks):
    Step 1: Create an empty Vec<JoinHandle<CrewTaskResult>> called handles

    Step 2: For each task (consuming the Vec with into_iter):
            Spawn a thread that:
                → creates CrewTaskResult { crew_id: task.crew_id, delivered_units: task.units }
                → returns it
            Push the returned JoinHandle into handles

    Step 3: Create an empty Vec<CrewTaskResult> called results

    Step 4: For each handle in handles (in order):
            Call handle.join()
            → if Err(panic), return CrewDispatchError::WorkerPanicked
            → if Ok(result), push result to results

    Step 5: Return Ok(results)
```

**Note:** The two-pass approach (spawn all, then join all) is critical. Joining inside the spawn loop would serialize the threads—you'd get threads finishing one at a time instead of running concurrently.

**Spoiler threshold:** High—explicit algorithm with the concurrency insight explained.
