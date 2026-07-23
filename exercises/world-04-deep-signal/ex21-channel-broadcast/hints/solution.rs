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
    let endpoint = format!(
        "{}/broadcast/{}",
        base_url.trim_end_matches('/'),
        target.channel
    );

    let response = reqwest::get(&endpoint)
        .await
        .map_err(ChannelBroadcastError::Request)?;

    let status = response.status();
    if !status.is_success() {
        return Err(ChannelBroadcastError::InvalidStatus {
            channel: target.channel.clone(),
            status,
        });
    }

    response
        .json::<BroadcastReceipt>()
        .await
        .map_err(ChannelBroadcastError::Decode)
}

pub async fn broadcast_channels(
    base_url: &str,
    targets: &[BroadcastTarget],
) -> Result<Vec<BroadcastReceipt>, ChannelBroadcastError> {
    let mut join_set = tokio::task::JoinSet::new();

    for (index, target) in targets.iter().cloned().enumerate() {
        let base_url = base_url.to_string();
        join_set.spawn(async move {
            (
                index,
                fetch_broadcast_receipt(base_url.as_str(), &target).await,
            )
        });
    }

    let mut ordered: Vec<Option<BroadcastReceipt>> = vec![None; targets.len()];
    while let Some(joined) = join_set.join_next().await {
        let (index, receipt) = joined.expect("broadcast worker task should not panic");
        ordered[index] = Some(receipt?);
    }

    Ok(ordered
        .into_iter()
        .map(|receipt| receipt.expect("every target should produce one receipt"))
        .collect())
}
