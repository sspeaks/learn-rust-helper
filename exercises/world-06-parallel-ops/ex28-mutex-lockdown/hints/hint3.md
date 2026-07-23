## Hint 3: Algorithm Outline

```
function shared_lockdown_state(initial_stability):
    Step 1: Create LockdownState { stability: initial_stability, processed: 0 }
    Step 2: Return Arc::new(Mutex::new(state))

function process_lockdown_batch(state, events):
    Step 1: Create Vec<JoinHandle<_>> for handles

    Step 2: For each event (into_iter):
            Clone the Arc: state_clone = Arc::clone(&state)
            Spawn thread with move closure:
                → lock state_clone → get guard
                  (map PoisonError to MutexLockdownError::LockPoisoned)
                → guard.stability += event.delta
                → guard.processed += 1
                → guard drops, lock released
            Push JoinHandle into handles

    Step 3: Join all handles
            → on panic, return MutexLockdownError::WorkerPanicked

    Step 4: Lock state one final time
            → clone the inner LockdownState value
            → return Ok(cloned_state)
```

**Note:** The `MutexGuard` is released automatically when it goes out of scope. You don't need to call any unlock function—just let the guard drop.

**Spoiler threshold:** High—complete algorithm without Rust syntax.
