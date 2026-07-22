use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RelayDispatchRequest {
    pub route: String,
    pub priority: u8,
    pub payload: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct RelayDispatchReceipt {
    pub accepted: bool,
    pub relay_id: String,
    pub queued_at: String,
}

#[derive(Debug)]
pub enum RelayDispatchError {
    Serialize(serde_json::Error),
    Request(ureq::Error),
    ReadBody(std::io::Error),
    Decode(serde_json::Error),
}

pub fn dispatch_relay(
    base_url: &str,
    request: &RelayDispatchRequest,
) -> Result<RelayDispatchReceipt, RelayDispatchError> {
    // ══════════════════════════════════════════════════════════════
    // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
    // ══════════════════════════════════════════════════════════════
    todo!("POST request as JSON and decode the relay receipt")
}
