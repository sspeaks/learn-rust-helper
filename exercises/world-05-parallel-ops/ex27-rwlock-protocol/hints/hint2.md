## Hint 2: Tools & Types

- **`Arc::new(RwLock::new(initial))`:** Wrap the `BTreeMap` for shared access.
- **`Arc::clone(&state)`:** Clone the Arc for each thread—cheap reference count bump.
- **`state_clone.write().map_err(|_| RwLockProtocolError::LockPoisoned)?`:** Acquire write lock. Returns `RwLockWriteGuard<BTreeMap<String, u32>>`.
- **`guard.insert(key, value)`:** Insert through the guard—the guard dereferences to `&mut BTreeMap`.
- **`state.read().map_err(|_| RwLockProtocolError::LockPoisoned)?`:** Acquire read lock after joining threads. Returns `RwLockReadGuard<BTreeMap<...>>`.
- **`guard.clone()`:** Clone the `BTreeMap` out of the read guard to return an owned copy.

**Spoiler threshold:** Medium—names every method and guard type.
