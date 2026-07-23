use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct DeepSignalSample {
    pub node: String,
    pub strength: i32,
    pub healthy: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeepSignalReport {
    pub healthy_nodes: Vec<String>,
    pub degraded_nodes: Vec<String>,
    pub average_strength: f64,
}

#[derive(Debug)]
pub enum DeepSignalError {
    EmptyNodeList,
    Request {
        node: String,
        source: reqwest::Error,
    },
    InvalidStatus {
        node: String,
        status: reqwest::StatusCode,
    },
    Decode {
        node: String,
        source: reqwest::Error,
    },
}

pub async fn gather_deep_signals(
    base_url: &str,
    nodes: &[String],
) -> Result<Vec<DeepSignalSample>, DeepSignalError> {
    if nodes.is_empty() {
        return Err(DeepSignalError::EmptyNodeList);
    }

    let mut join_set = tokio::task::JoinSet::new();
    for (index, node) in nodes.iter().cloned().enumerate() {
        let base_url = base_url.to_string();
        join_set.spawn(async move {
            let endpoint = format!("{}/deep-signals/{node}", base_url.trim_end_matches('/'));

            let response =
                reqwest::get(&endpoint)
                    .await
                    .map_err(|source| DeepSignalError::Request {
                        node: node.clone(),
                        source,
                    })?;

            let status = response.status();
            if !status.is_success() {
                return Err(DeepSignalError::InvalidStatus {
                    node: node.clone(),
                    status,
                });
            }

            let sample = response
                .json::<DeepSignalSample>()
                .await
                .map_err(|source| DeepSignalError::Decode {
                    node: node.clone(),
                    source,
                })?;

            Ok((index, sample))
        });
    }

    let mut ordered: Vec<Option<DeepSignalSample>> = vec![None; nodes.len()];
    while let Some(joined) = join_set.join_next().await {
        let (index, sample) = joined.expect("deep-signal worker task should not panic")?;
        ordered[index] = Some(sample);
    }

    Ok(ordered
        .into_iter()
        .map(|sample| sample.expect("every node should produce one sample"))
        .collect())
}

pub fn build_deep_signal_report(
    samples: &[DeepSignalSample],
) -> Result<DeepSignalReport, DeepSignalError> {
    if samples.is_empty() {
        return Err(DeepSignalError::EmptyNodeList);
    }

    let healthy_nodes = samples
        .iter()
        .filter(|sample| sample.healthy)
        .map(|sample| sample.node.clone())
        .collect::<Vec<_>>();

    let degraded_nodes = samples
        .iter()
        .filter(|sample| !sample.healthy)
        .map(|sample| sample.node.clone())
        .collect::<Vec<_>>();

    let total_strength: i64 = samples
        .iter()
        .map(|sample| i64::from(sample.strength))
        .sum();
    let average_strength = total_strength as f64 / samples.len() as f64;

    Ok(DeepSignalReport {
        healthy_nodes,
        degraded_nodes,
        average_strength,
    })
}

pub async fn run_deep_signal_pipeline(
    base_url: &str,
    nodes: &[String],
) -> Result<DeepSignalReport, DeepSignalError> {
    let samples = gather_deep_signals(base_url, nodes).await?;
    build_deep_signal_report(&samples)
}
