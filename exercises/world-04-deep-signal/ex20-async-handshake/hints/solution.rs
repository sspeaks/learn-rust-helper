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
    let endpoint = format!("{}/handshake/{call_sign}", base_url.trim_end_matches('/'));

    let response = reqwest::get(&endpoint)
        .await
        .map_err(AsyncHandshakeError::Request)?;

    let status = response.status();
    if !status.is_success() {
        return Err(AsyncHandshakeError::InvalidStatus(status));
    }

    response
        .json::<HandshakeReceipt>()
        .await
        .map_err(AsyncHandshakeError::Decode)
}
