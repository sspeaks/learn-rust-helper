## Hint 2: Tools & Types

- **`Arc::new(Mutex::new(value))`:** The standard wrapping for shared mutable state across threads.
- **`Arc::clone(&state)`:** Creates a new `Arc` pointing to the same `Mutex`. Do this once per thread before spawning.
- **`state.lock().unwrap_or_else(|e| e.into_inner())`:** Or `.map_err(|_| MutexLockdownError::LockPoisoned)?` — acquires the lock and returns `MutexGuard<LockdownState>`.
- **Mutating through the guard:** Dereference the guard (`*guard`) to access and mutate the inner value.
- **Lock scope:** The guard holds the lock; it is released when the guard drops (end of block or explicit `drop`).
- **Reading the final value:** After all threads join, lock the `Arc<Mutex<_>>` once more and `.clone()` the inner value to return it.

**Spoiler threshold:** Medium—names every relevant type and method.
