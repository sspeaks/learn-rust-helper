use std::io;

use ureq::get;

use crate::BeaconPingError::ReadBody;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BeaconPing {
    pub endpoint: String,
    pub status: u16,
    pub body: String,
}

#[derive(Debug)]
pub enum BeaconPingError {
    Request(ureq::Error),
    ReadBody(std::io::Error),
}

pub fn ping_beacon(base_url: &str, beacon_id: &str) -> Result<BeaconPing, BeaconPingError> {
    let url = format!("{}/beacons/{beacon_id}", base_url.trim_end_matches("/"));

    let res = get(&url)
        .call()
        .map_err(|ureq_err| BeaconPingError::Request(ureq_err))?;
    let mut bytes = Vec::new();
    let code = res.status();
    res.into_reader()
        .read_to_end(&mut bytes)
        .map_err(BeaconPingError::ReadBody)?;

    let body = String::from_utf8(bytes).map_err(|error| {
        BeaconPingError::ReadBody(io::Error::new(io::ErrorKind::InvalidData, error))
    })?;

    Ok(BeaconPing {
        endpoint: url,
        status: code,
        body: body,
    })
}
