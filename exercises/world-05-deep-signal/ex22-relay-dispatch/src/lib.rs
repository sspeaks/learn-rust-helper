use std::io;

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
    use RelayDispatchError::*;
    let req_url: String = format!("{}/relay/dispatch", base_url.trim_end_matches("/"));
    let json_raw: String = serde_json::to_string(request).map_err(Serialize)?;

    let mut bytes: Vec<u8> = Vec::new();
    ureq::post(&req_url)
        .set("Content-Type", "application/json")
        .send_string(&json_raw)
        .map_err(Request)?
        .into_reader()
        .read_to_end(&mut bytes)
        .map_err(ReadBody)?;
    let resp: String = String::from_utf8(bytes)
        .map_err(|err| ReadBody(io::Error::new(io::ErrorKind::InvalidData, err)))?;
    serde_json::from_str::<RelayDispatchReceipt>(&resp).map_err(Decode)
}
