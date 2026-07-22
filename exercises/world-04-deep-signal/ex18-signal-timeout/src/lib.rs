use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct SignalEnvelope {
    pub node_id: String,
    pub status: String,
    pub latency_ms: u64,
}

#[derive(Debug)]
pub enum SignalTimeoutError {
    TimedOut,
    HttpStatus(u16),
    Transport(ureq::Transport),
    Decode(serde_json::Error),
    ReadBody(std::io::Error),
}

pub fn fetch_signal_with_timeout(
    base_url: &str,
    timeout: Duration,
) -> Result<SignalEnvelope, SignalTimeoutError> {
    // ══════════════════════════════════════════════════════════════
    // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
    // ══════════════════════════════════════════════════════════════
    todo!("Configure a timeout-aware GET request and map failures into SignalTimeoutError")
}
