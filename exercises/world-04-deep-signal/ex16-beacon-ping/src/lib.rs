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
    // ══════════════════════════════════════════════════════════════
    // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
    // ══════════════════════════════════════════════════════════════
    todo!("Send a sync GET request to {base_url}/beacons/{beacon_id} and return BeaconPing")
}
