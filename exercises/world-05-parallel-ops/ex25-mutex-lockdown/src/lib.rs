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
    // ══════════════════════════════════════════════════════════════
    // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
    // ══════════════════════════════════════════════════════════════
    todo!("Create Arc<Mutex<LockdownState>> with processed initialized to 0")
}

pub fn process_lockdown_batch(
    state: Arc<Mutex<LockdownState>>,
    events: Vec<LockdownEvent>,
) -> Result<LockdownState, MutexLockdownError> {
    // ══════════════════════════════════════════════════════════════
    // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
    // ══════════════════════════════════════════════════════════════
    todo!("Apply events from multiple threads and return the final shared state")
}
