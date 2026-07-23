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
    let packet =
        serde_json::from_str::<Packet>(json_payload).map_err(PacketDecodeError::InvalidJson)?;
    if packet.id.is_empty() {
        return Err(PacketDecodeError::EmptyPacketId);
    }

    Ok(packet)
}

pub fn decode_packet_batch(json_payloads: &[&str]) -> Result<Vec<Packet>, PacketDecodeError> {
    json_payloads
        .iter()
        .map(|payload| decode_packet(payload))
        .collect()
}
