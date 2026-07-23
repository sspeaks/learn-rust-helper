use std::io::{self, Read};

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
    let endpoint = format!("{}/relay/dispatch", base_url.trim_end_matches('/'));
    let payload = serde_json::to_string(request).map_err(RelayDispatchError::Serialize)?;

    let response = ureq::post(&endpoint)
        .set("Content-Type", "application/json")
        .send_string(&payload)
        .map_err(RelayDispatchError::Request)?;

    let mut bytes = Vec::new();
    response
        .into_reader()
        .read_to_end(&mut bytes)
        .map_err(RelayDispatchError::ReadBody)?;
    let body = String::from_utf8(bytes).map_err(|error| {
        RelayDispatchError::ReadBody(io::Error::new(io::ErrorKind::InvalidData, error))
    })?;
    serde_json::from_str::<RelayDispatchReceipt>(&body).map_err(RelayDispatchError::Decode)
}
