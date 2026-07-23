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
    use AsyncHandshakeError::*;
    let req_url: String = format!("{}/handshake/{}", base_url.trim_end_matches("/"), call_sign);
    let r = reqwest::get(req_url).await.map_err(Request)?;

    let status = r.status();
    if !r.status().is_success() {
        return Err(InvalidStatus(status));
    }

    r.json::<HandshakeReceipt>().await.map_err(Decode)
}
