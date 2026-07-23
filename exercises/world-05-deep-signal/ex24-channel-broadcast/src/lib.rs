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
    use ChannelBroadcastError::*;

    let req_url: String = format!(
        "{}/broadcast/{}",
        base_url.trim_end_matches("/"),
        target.channel
    );

    let res = reqwest::get(req_url).await.map_err(Request)?;
    let status = res.status();
    if !status.is_success() {
        return Err(InvalidStatus {
            channel: target.channel.clone(),
            status,
        });
    }

    res.json::<BroadcastReceipt>().await.map_err(Decode)
}

pub async fn broadcast_channels(
    base_url: &str,
    targets: &[BroadcastTarget],
) -> Result<Vec<BroadcastReceipt>, ChannelBroadcastError> {
    let futures: Vec<_> = targets
        .iter()
        .map(|targ| fetch_broadcast_receipt(base_url, targ))
        .collect();
    let mut res: Vec<Result<BroadcastReceipt, ChannelBroadcastError>> = Vec::new();
    for future in futures {
        let val = future.await;
        if val.is_err() {
            return Err(val.unwrap_err());
        }
        res.push(val);
    }

    res.into_iter().collect()
}
