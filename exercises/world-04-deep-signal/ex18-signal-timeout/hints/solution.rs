use serde::Deserialize;
use std::{
    error::Error as StdError,
    io::{self, Read},
    time::Duration,
};
use ureq::{AgentBuilder, Error as UreqError, ErrorKind};

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
    let agent = AgentBuilder::new().timeout(timeout).build();
    let url = format!("{}/signal", base_url.trim_end_matches('/'));

    let response = agent.get(&url).call().map_err(|err| match err {
        UreqError::Status(code, _) => SignalTimeoutError::HttpStatus(code),
        UreqError::Transport(t) => {
            let is_timeout = t.kind() == ErrorKind::Io
                && StdError::source(&t)
                    .and_then(|src| src.downcast_ref::<io::Error>())
                    .is_some_and(|e| e.kind() == io::ErrorKind::TimedOut);
            if is_timeout {
                SignalTimeoutError::TimedOut
            } else {
                SignalTimeoutError::Transport(t)
            }
        }
    })?;

    let mut bytes = Vec::new();
    response
        .into_reader()
        .read_to_end(&mut bytes)
        .map_err(SignalTimeoutError::ReadBody)?;
    let body = String::from_utf8(bytes)
        .map_err(|e| SignalTimeoutError::ReadBody(io::Error::new(io::ErrorKind::InvalidData, e)))?;
    serde_json::from_str(&body).map_err(SignalTimeoutError::Decode)
}
