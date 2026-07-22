# Quest 27: RwLock Protocol

**🎮 Quest:** The protocol registry is updated rarely but read constantly. Using a `Mutex` means every reader blocks every other reader—even though they don't conflict. A `RwLock` lets many readers proceed simultaneously while still protecting writes. Upgrade the protocol.

## Objective

Implement `shared_protocol_state` (create the shared registry) and `apply_protocol_updates` (apply key/value writes across threads). This exercise teaches `Arc<RwLock<T>>`: the difference between read locks and write locks, and when RwLock outperforms Mutex.

## Public API

```rust
pub struct ProtocolUpdate {
    pub key: String,
    pub value: u32,
}

pub enum RwLockProtocolError {
    LockPoisoned,
    WorkerPanicked,
}

pub fn shared_protocol_state(
    initial: BTreeMap<String, u32>,
) -> Arc<RwLock<BTreeMap<String, u32>>>

pub fn apply_protocol_updates(
    state: Arc<RwLock<BTreeMap<String, u32>>>,
    updates: Vec<ProtocolUpdate>,
) -> Result<BTreeMap<String, u32>, RwLockProtocolError>
```

## Behavioral Rules

### `shared_protocol_state`
1. Wrap the `initial` `BTreeMap` in `Arc::new(RwLock::new(...))` and return it.

### `apply_protocol_updates`
1. Spawn one thread per update, cloning the `Arc` for each.
2. Each thread acquires a **write lock** and inserts/updates its key/value pair.
3. A poisoned write lock returns `RwLockProtocolError::LockPoisoned`.
4. A panicking worker returns `RwLockProtocolError::WorkerPanicked`.
5. Join all threads.
6. Acquire a **read lock** on the final state, clone the `BTreeMap`, and return it.

## Concepts Practiced

- **`RwLock<T>`:** Allows many concurrent readers OR one exclusive writer.
- **`rwlock.read()`:** Returns `RwLockReadGuard<T>`—many threads can hold this simultaneously.
- **`rwlock.write()`:** Returns `RwLockWriteGuard<T>`—exclusive access; blocks until all readers and writers release.
- **`RwLock` vs. `Mutex`:** Mutex is simpler; RwLock is better when reads outnumber writes. Same poison semantics.
- **`BTreeMap`:** An ordered map—keys are always sorted, making the final state deterministic in tests.

## Setup Notes

Concurrency tests are deterministic because `BTreeMap` iterates in sorted key order. Tests verify the final state snapshot after all updates, not intermediate values. No `thread::sleep` is used.

## Edge Cases

- Empty `updates` slice (no threads spawned; final state equals initial map).
- Updates with duplicate keys (last write wins within the ordering imposed by thread scheduling; tests use unique keys).
- Initial map entries that are not covered by updates (preserved in the output).

## How to Work on This Quest

**From inside `nix develop path:.`:**

```bash
# Check your work
learn check ex27-rwlock-protocol

# Get a hint if stuck
learn hint ex27-rwlock-protocol

# Jump to a specific hint level
learn hint ex27-rwlock-protocol --level 2
```

**Omit the exercise ID to use your current recommendation:**

```bash
learn check
learn hint
```

## XP Reward

**460 XP** for first completion.

## Prerequisites

Complete **Sensor Array** (ex26).

## Success Criteria

- `shared_protocol_state` returns a correctly wrapped `Arc<RwLock<BTreeMap<_>>>`.
- Each update is applied with a write lock in its own thread.
- Poisoned write lock returns `RwLockProtocolError::LockPoisoned`.
- Panicking worker returns `RwLockProtocolError::WorkerPanicked`.
- Final state is read with a read lock and returned as a cloned `BTreeMap`.

## Next Steps

Complete this quest to unlock **Parallel Ops Capstone** (ex28), the World 5 finale combining all concurrency tools in one pipeline.
