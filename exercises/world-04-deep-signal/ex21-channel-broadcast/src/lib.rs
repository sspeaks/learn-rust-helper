use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BroadcastTarget {
    pub channel: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct BroadcastReceipt {
    pub channel: String,
    pub acknowledged: bool,
}

#[derive(Debug)]
pub enum ChannelBroadcastError {
    Request(reqwest::Error),
    InvalidStatus {
        channel: String,
        status: reqwest::StatusCode,
    },
    Decode(reqwest::Error),
}

pub async fn fetch_broadcast_receipt(
    base_url: &str,
    target: &BroadcastTarget,
) -> Result<BroadcastReceipt, ChannelBroadcastError> {
    // ══════════════════════════════════════════════════════════════
    // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
    // ══════════════════════════════════════════════════════════════
    todo!("Request one channel receipt from the broadcast endpoint")
}

pub async fn broadcast_channels(
    base_url: &str,
    targets: &[BroadcastTarget],
) -> Result<Vec<BroadcastReceipt>, ChannelBroadcastError> {
    // ══════════════════════════════════════════════════════════════
    // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
    // ══════════════════════════════════════════════════════════════
    todo!("Fetch all channel receipts concurrently with structured concurrency")
}
