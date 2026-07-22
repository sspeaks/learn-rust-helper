use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct HandshakeReceipt {
    pub session_id: String,
    pub protocol: String,
    pub motd: String,
}

#[derive(Debug)]
pub enum AsyncHandshakeError {
    Request(reqwest::Error),
    InvalidStatus(reqwest::StatusCode),
    Decode(reqwest::Error),
}

pub async fn perform_async_handshake(
    base_url: &str,
    call_sign: &str,
) -> Result<HandshakeReceipt, AsyncHandshakeError> {
    // ══════════════════════════════════════════════════════════════
    // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
    // ══════════════════════════════════════════════════════════════
    todo!("Send an async handshake request and decode HandshakeReceipt")
}
