## Hint 2: Tools & Types

- **`std::thread::spawn(move || { ... })`:** Spawns a new thread; the closure must be `move` to take ownership of captured data. Returns a `JoinHandle<T>`.
- **`handle.join()`:** Blocks until the thread completes. Returns `Result<T, Box<dyn Any + Send>>`. `Err(...)` means the thread panicked—map this to `CrewDispatchError::WorkerPanicked`.
- **Two-pass pattern:**
  1. First loop: spawn all threads, collect `Vec<JoinHandle<CrewTaskResult>>`.
  2. Second loop: join each handle in order, collect results.
- **Moving into closures:** Each task must be moved into its thread closure. Iterate with `.into_iter()` to move owned values.

**Spoiler threshold:** Medium—names the key types and the two-pass pattern.
