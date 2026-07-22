# Quest 25: Mutex Lockdown

**🎮 Quest:** The station's stability monitor tracks a shared counter that dozens of concurrent threads must update. Without coordination, updates would race and corrupt the data. Protect the shared state with a Mutex and let the threads work safely in parallel.

## Objective

Implement `shared_lockdown_state` (create the shared value) and `process_lockdown_batch` (apply events across threads using `Arc<Mutex<T>>`). This exercise teaches Rust's interior mutability pattern for thread-safe shared state.

## Public API

```rust
pub struct LockdownEvent {
    pub zone: String,
    pub delta: i32,
}

pub struct LockdownState {
    pub stability: i32,
    pub processed: usize,
}

pub enum MutexLockdownError {
    LockPoisoned,
    WorkerPanicked,
}

pub fn shared_lockdown_state(initial_stability: i32) -> Arc<Mutex<LockdownState>>

pub fn process_lockdown_batch(
    state: Arc<Mutex<LockdownState>>,
    events: Vec<LockdownEvent>,
) -> Result<LockdownState, MutexLockdownError>
```

## Behavioral Rules

### `shared_lockdown_state`
1. Construct a `LockdownState` with `stability = initial_stability` and `processed = 0`.
2. Wrap it in `Arc::new(Mutex::new(...))` and return it.

### `process_lockdown_batch`
1. Spawn one thread per event, passing a clone of the `Arc` into each thread.
2. Each thread locks the mutex, applies `event.delta` to `state.stability`, increments `state.processed` by 1, then unlocks.
3. A poisoned lock returns `MutexLockdownError::LockPoisoned`.
4. A panicking worker returns `MutexLockdownError::WorkerPanicked`.
5. Join all threads after spawning.
6. Return the final `LockdownState` by locking the mutex one last time and cloning the value.

## Concepts Practiced

- **`Arc<T>`:** Atomically reference-counted pointer for shared ownership across threads.
- **`Mutex<T>`:** Mutual exclusion—only one thread can hold the lock at a time.
- **`Arc::clone(&arc)`:** Cheaply creates a new reference to the same allocation.
- **`mutex.lock()`:** Acquires the lock, returns `MutexGuard<T>`. Lock is released when the guard drops.
- **Poison detection:** If a thread panics while holding the lock, the mutex is "poisoned". Subsequent `lock()` calls return `Err(PoisonError)`.
- **`Arc` + `Mutex` together:** The canonical pattern for shared mutable state across threads.

## Setup Notes

Concurrency tests are deterministic. The final state depends only on the sum of all deltas applied, regardless of thread scheduling. Tests verify the final `stability` and `processed` count, not intermediate states.

## Edge Cases

- Empty event list (no threads spawned; final state equals initial state with `processed = 0`).
- Multiple deltas that sum to zero.
- Negative deltas reducing stability below zero (allowed—no floor enforcement).

## How to Work on This Quest

**From inside `nix develop path:.`:**

```bash
# Check your work
learn check ex25-mutex-lockdown

# Get a hint if stuck
learn hint ex25-mutex-lockdown

# Jump to a specific hint level
learn hint ex25-mutex-lockdown --level 2
```

**Omit the exercise ID to use your current recommendation:**

```bash
learn check
learn hint
```

## XP Reward

**410 XP** for first completion.

## Prerequisites

Complete **Cargo Channel** (ex24).

## Success Criteria

- `shared_lockdown_state` returns a correctly initialized `Arc<Mutex<LockdownState>>`.
- Each event is applied in a separate thread via mutex lock.
- Final `stability` equals initial value plus sum of all deltas.
- Final `processed` equals the number of events.
- Poisoned lock returns `MutexLockdownError::LockPoisoned`.
- Panicking worker returns `MutexLockdownError::WorkerPanicked`.

## Next Steps

Complete this quest to unlock **Sensor Array** (ex26), where you'll use rayon to process data in parallel without managing threads manually.
