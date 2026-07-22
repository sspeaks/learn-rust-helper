## Hint 3: Algorithm Outline

```
function shared_protocol_state(initial):
    Step 1: Return Arc::new(RwLock::new(initial))

function apply_protocol_updates(state, updates):
    Step 1: Create Vec<JoinHandle<_>>

    Step 2: For each update (into_iter):
            state_clone = Arc::clone(&state)
            Spawn thread with move closure:
                → acquire WRITE lock on state_clone
                  (map PoisonError → LockPoisoned)
                → insert update.key, update.value into the map
                → guard drops, write lock released
            Push JoinHandle

    Step 3: Join all handles
            → on panic, return RwLockProtocolError::WorkerPanicked

    Step 4: Acquire READ lock on state
            (map PoisonError → LockPoisoned)

    Step 5: Clone the BTreeMap out of the read guard

    Step 6: Return Ok(cloned_map)
```

**Note:** The final read uses `.read()` rather than `.write()` because no mutation occurs. Using a read lock here also documents intent clearly: "I'm only observing the final result."

**Spoiler threshold:** High—algorithm distinguishing write and read lock usage.
