use serde::Deserialize;
use ureq::Error::{Status, Transport};
use std::{ time::Duration};


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

     let res =   ureq::get(base_url)
        .timeout(timeout)
        .call()
        .map_err(|err| {
            match err {
                Status(a,_b) => if a == 408 {
                    SignalTimeoutError::TimedOut
                } else {
                    SignalTimeoutError::HttpStatus(a)
                },
                Transport(a) => SignalTimeoutError::Transport(a)
            }
        })?;
    let json_raw: String = res.into_string().map_err(SignalTimeoutError::ReadBody)?;
    let json: SignalEnvelope = serde_json::from_str(&json_raw).map_err(SignalTimeoutError::Decode)?;
    Ok(json)
}
