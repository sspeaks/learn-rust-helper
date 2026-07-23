use std::io::{self, Read};

use ureq::get;

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
    let endpoint = format!("{}/beacons/{beacon_id}", base_url.trim_end_matches('/'));

    let response = get(&endpoint).call().map_err(BeaconPingError::Request)?;
    let status = response.status();
    let mut bytes = Vec::new();
    response
        .into_reader()
        .read_to_end(&mut bytes)
        .map_err(BeaconPingError::ReadBody)?;
    let body = String::from_utf8(bytes).map_err(|error| {
        BeaconPingError::ReadBody(io::Error::new(io::ErrorKind::InvalidData, error))
    })?;

    Ok(BeaconPing {
        endpoint,
        status,
        body,
    })
}
