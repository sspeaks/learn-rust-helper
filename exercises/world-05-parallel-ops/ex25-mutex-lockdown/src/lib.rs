use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LockdownEvent {
    pub zone: String,
    pub delta: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LockdownState {
    pub stability: i32,
    pub processed: usize,
}

#[derive(Debug)]
pub enum MutexLockdownError {
    LockPoisoned,
    WorkerPanicked,
}

pub fn shared_lockdown_state(initial_stability: i32) -> Arc<Mutex<LockdownState>> {
    Arc::new(Mutex::new(LockdownState {
        stability: initial_stability,
        processed: 0,
    }))
}

pub fn process_lockdown_batch(
    state: Arc<Mutex<LockdownState>>,
    events: Vec<LockdownEvent>,
) -> Result<LockdownState, MutexLockdownError> {
    std::thread::scope(|s| {
        for event in events {
            let state = Arc::clone(&state);
            s.spawn(move || {
                let mut guard = state.lock().map_err(|_| MutexLockdownError::LockPoisoned)?;
                guard.stability += event.delta;
                guard.processed += 1;
                Ok::<(), MutexLockdownError>(())
            });
        }
        // All threads automatically join here
    });

    let final_state = state
        .lock()
        .map_err(|_| MutexLockdownError::LockPoisoned)?
        .clone();

    Ok(final_state)
}
