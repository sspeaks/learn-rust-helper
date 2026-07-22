use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Packet {
    pub id: String,
    pub signal: i32,
    pub active: bool,
}

#[derive(Debug)]
pub enum PacketDecodeError {
    InvalidJson(serde_json::Error),
    EmptyPacketId,
}

pub fn decode_packet(json_payload: &str) -> Result<Packet, PacketDecodeError> {
    // ══════════════════════════════════════════════════════════════
    // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
    // ══════════════════════════════════════════════════════════════
    todo!("Deserialize one packet and reject empty packet IDs")
}

pub fn decode_packet_batch(json_payloads: &[&str]) -> Result<Vec<Packet>, PacketDecodeError> {
    // ══════════════════════════════════════════════════════════════
    // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
    // ══════════════════════════════════════════════════════════════
    todo!("Decode each payload with decode_packet and preserve order")
}
