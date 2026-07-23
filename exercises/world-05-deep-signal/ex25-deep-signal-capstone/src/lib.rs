use serde::Deserialize;

use crate::DeepSignalError::*;

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

async fn fetch_signal(url: String, node: &str) -> Result<DeepSignalSample, DeepSignalError> {
    let req_url = format!("{}/deep-signals/{}", url, node);
    reqwest::get(req_url)
        .await
        .map_err(|err| Request {
            node: node.to_string(),
            source: err,
        })
        .and_then(|res| {
            if !res.status().is_success() {
                return Err(InvalidStatus {
                    node: node.to_string(),
                    status: res.status(),
                });
            } else {
                return Ok(res);
            }
        })?
        .json::<DeepSignalSample>()
        .await
        .map_err(|e| Decode {
            node: node.to_string(),
            source: e,
        })
}

pub async fn gather_deep_signals(
    base_url: &str,
    nodes: &[String],
) -> Result<Vec<DeepSignalSample>, DeepSignalError> {
    use tokio::task::*;
    if nodes.is_empty() {
        return Err(EmptyNodeList);
    }

    let mut join_set = JoinSet::new();

    for (index, node) in nodes.iter().cloned().enumerate() {
        let url = base_url.to_string();
        join_set.spawn(async move { (index, fetch_signal(url, &node).await) });
    }

    let mut result: Vec<Option<DeepSignalSample>> = vec![None; nodes.len()];
    while let Some(joined) = join_set.join_next().await {
        let (index, v) = joined.expect("Something something");
        result[index] = Some(v?);
    }

    Ok(result
        .into_iter()
        .map(|item| item.expect("if the item exists in the array, it didn't error"))
        .collect())
}

pub fn build_deep_signal_report(
    samples: &[DeepSignalSample],
) -> Result<DeepSignalReport, DeepSignalError> {
    if samples.is_empty() {
        return Err(EmptyNodeList);
    }

    let mut count = 0;
    let mut total: f64 = 0.0;

    let (healthy, degraded): (Vec<&DeepSignalSample>, Vec<&DeepSignalSample>) = samples
        .into_iter()
        .inspect(|signal| {
            count += 1;
            total += signal.strength as f64;
        })
        .partition(|signal| signal.healthy);

    Ok(DeepSignalReport {
        healthy_nodes: healthy.iter().map(|s| s.node.clone()).collect(),
        degraded_nodes: degraded.iter().map(|s| s.node.clone()).collect(),
        average_strength: total / (count as f64),
    })
}

pub async fn run_deep_signal_pipeline(
    base_url: &str,
    nodes: &[String],
) -> Result<DeepSignalReport, DeepSignalError> {
    build_deep_signal_report(&gather_deep_signals(base_url, nodes).await?)
}
