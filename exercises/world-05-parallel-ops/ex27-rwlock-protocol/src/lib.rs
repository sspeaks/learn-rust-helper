use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtocolUpdate {
    pub key: String,
    pub value: u32,
}

#[derive(Debug)]
pub enum RwLockProtocolError {
    LockPoisoned,
    WorkerPanicked,
}

pub fn shared_protocol_state(initial: BTreeMap<String, u32>) -> Arc<RwLock<BTreeMap<String, u32>>> {
    // ══════════════════════════════════════════════════════════════
    // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
    // ══════════════════════════════════════════════════════════════
    todo!("Create shared read-heavy state with Arc<RwLock<_>>")
}

pub fn apply_protocol_updates(
    state: Arc<RwLock<BTreeMap<String, u32>>>,
    updates: Vec<ProtocolUpdate>,
) -> Result<BTreeMap<String, u32>, RwLockProtocolError> {
    // ══════════════════════════════════════════════════════════════
    // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
    // ══════════════════════════════════════════════════════════════
    todo!("Apply updates safely while allowing concurrent read snapshots")
}
