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
    Arc::new(RwLock::new(initial))
}

pub fn apply_protocol_updates(
    state: Arc<RwLock<BTreeMap<String, u32>>>,
    updates: Vec<ProtocolUpdate>,
) -> Result<BTreeMap<String, u32>, RwLockProtocolError> {
    for update in updates {
        let state = Arc::clone(&state);
        std::thread::scope(|s| {
            s.spawn(move || {
                let mut guard = state
                    .write()
                    .map_err(|_| RwLockProtocolError::LockPoisoned)?;
                guard.insert(update.key, update.value);
                Ok::<(), RwLockProtocolError>(())
            });
        });
    }

    let snapshot = state
        .read()
        .map_err(|_| RwLockProtocolError::LockPoisoned)?
        .clone();

    Ok(snapshot)
}
