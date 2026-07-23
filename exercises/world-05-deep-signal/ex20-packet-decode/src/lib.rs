use serde::Deserialize;

use crate::PacketDecodeError::EmptyPacketId;

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
    serde_json::from_str::<Packet>(json_payload)
        .map_err(PacketDecodeError::InvalidJson)
        .and_then(|pack| {
            if pack.id.is_empty() {
                Err(EmptyPacketId)
            } else {
                Ok(pack)
            }
        })
}

pub fn decode_packet_batch(json_payloads: &[&str]) -> Result<Vec<Packet>, PacketDecodeError> {
    json_payloads
        .iter()
        .map(|payload| decode_packet(payload))
        .collect()
}
