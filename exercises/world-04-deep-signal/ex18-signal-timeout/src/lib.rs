use serde::Deserialize;
use std::{io, time::Duration};
use ureq::{
    Error::{Status, Transport},
    ErrorKind,
};

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
    let res = ureq::get(format!("{}/signal", base_url.trim_end_matches("/")).as_str())
        .timeout(timeout)
        .call()
        .map_err(|err| match err {
            Status(a, _b) => SignalTimeoutError::HttpStatus(a),
            Transport(a) => {
                if a.kind() == ErrorKind::Io {
                    SignalTimeoutError::TimedOut
                } else {
                    SignalTimeoutError::Transport(a)
                }
            }
        })?;
    let mut bytes = Vec::new();
    res.into_reader()
        .read_to_end(&mut bytes)
        .map_err(SignalTimeoutError::ReadBody)?;
    let json_raw = String::from_utf8(bytes).map_err(|error| {
        SignalTimeoutError::ReadBody(io::Error::new(io::ErrorKind::InvalidData, error))
    })?;
    let json: SignalEnvelope =
        serde_json::from_str(&json_raw).map_err(SignalTimeoutError::Decode)?;
    Ok(json)
}
