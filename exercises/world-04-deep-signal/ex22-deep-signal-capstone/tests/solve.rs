use ex22_deep_signal_capstone::{
    build_deep_signal_report, gather_deep_signals, run_deep_signal_pipeline, DeepSignalError,
    DeepSignalSample,
};
use std::time::Duration;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn is_stub_panic(e: &Box<dyn std::any::Any + Send>) -> bool {
    e.downcast_ref::<&str>()
        .is_some_and(|s| s.contains("not yet implemented"))
        || e.downcast_ref::<String>()
            .is_some_and(|s| s.contains("not yet implemented"))
}

macro_rules! call_or_hint {
    ($ex:expr, $fn:expr, $body:expr) => {{
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| $body)) {
            Ok(v) => v,
            Err(e) => {
                if is_stub_panic(&e) {
                    panic!(
                        "\n\n  ✖  {} '{}' not started — fill in src/lib.rs\n",
                        $ex, $fn
                    );
                }
                std::panic::resume_unwind(e)
            }
        }
    }};
}

macro_rules! async_call_or_hint {
    ($ex:expr, $fn:expr, $body:expr) => {{
        match tokio::task::spawn(async move { $body }).await {
            Ok(v) => v,
            Err(join_error) => {
                if join_error.is_panic() {
                    let panic_payload = join_error.into_panic();
                    if is_stub_panic(&panic_payload) {
                        panic!(
                            "\n\n  ✖  {} '{}' not started — fill in src/lib.rs\n",
                            $ex, $fn
                        );
                    }
                    std::panic::resume_unwind(panic_payload);
                }
                panic!("\n\n  ✖  {} '{}' task cancelled unexpectedly\n", $ex, $fn);
            }
        }
    }};
}

#[tokio::test(flavor = "multi_thread")]
async fn gather_deep_signals_empty_nodes_returns_error() {
    let result = async_call_or_hint!("ex22", "gather_deep_signals", {
        gather_deep_signals("http://127.0.0.1:9", &[]).await
    });

    assert!(
        matches!(result, Err(DeepSignalError::EmptyNodeList)),
        "empty node list should be rejected immediately"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn gather_deep_signals_success_preserves_node_order() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/deep-signals/node-a"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_delay(Duration::from_millis(25))
                .set_body_string(r#"{"node":"node-a","strength":70,"healthy":true}"#),
        )
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/deep-signals/node-b"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(r#"{"node":"node-b","strength":40,"healthy":false}"#),
        )
        .expect(1)
        .mount(&server)
        .await;

    let nodes = vec!["node-a".to_string(), "node-b".to_string()];
    let base_url = server.uri();
    let samples = async_call_or_hint!("ex22", "gather_deep_signals", {
        let base_url = base_url.clone();
        let nodes = nodes.clone();
        gather_deep_signals(&base_url, &nodes).await
    })
    .expect("all nodes should gather successfully");

    assert_eq!(samples.len(), 2);
    assert_eq!(samples[0].node, "node-a");
    assert_eq!(samples[1].node, "node-b");
}

#[tokio::test(flavor = "multi_thread")]
async fn gather_deep_signals_invalid_status_maps_node_context() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/deep-signals/node-x"))
        .respond_with(ResponseTemplate::new(502))
        .expect(1)
        .mount(&server)
        .await;

    let nodes = vec!["node-x".to_string()];
    let base_url = server.uri();
    let result = async_call_or_hint!("ex22", "gather_deep_signals", {
        let base_url = base_url.clone();
        let nodes = nodes.clone();
        gather_deep_signals(&base_url, &nodes).await
    });

    match result {
        Err(DeepSignalError::InvalidStatus { node, status }) => {
            assert_eq!(node, "node-x");
            assert_eq!(status.as_u16(), 502);
        }
        other => panic!("expected InvalidStatus with node context, got {other:?}"),
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn gather_deep_signals_invalid_json_maps_decode_error_with_node() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/deep-signals/node-bad"))
        .respond_with(ResponseTemplate::new(200).set_body_string("not-json"))
        .expect(1)
        .mount(&server)
        .await;

    let nodes = vec!["node-bad".to_string()];
    let base_url = server.uri();
    let result = async_call_or_hint!("ex22", "gather_deep_signals", {
        let base_url = base_url.clone();
        let nodes = nodes.clone();
        gather_deep_signals(&base_url, &nodes).await
    });

    match result {
        Err(DeepSignalError::Decode { node, .. }) => assert_eq!(node, "node-bad"),
        other => panic!("expected Decode error with node context, got {other:?}"),
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn gather_deep_signals_unreachable_host_maps_request_error_with_node() {
    let nodes = vec!["isolated".to_string()];
    let result = async_call_or_hint!("ex22", "gather_deep_signals", {
        let nodes = nodes.clone();
        gather_deep_signals("http://127.0.0.1:9", &nodes).await
    });

    match result {
        Err(DeepSignalError::Request { node, .. }) => assert_eq!(node, "isolated"),
        other => panic!("expected Request error with node context, got {other:?}"),
    }
}

#[test]
fn build_report_partitions_nodes_and_computes_average_strength() {
    let samples = vec![
        DeepSignalSample {
            node: "node-a".to_string(),
            strength: 90,
            healthy: true,
        },
        DeepSignalSample {
            node: "node-b".to_string(),
            strength: 30,
            healthy: false,
        },
        DeepSignalSample {
            node: "node-c".to_string(),
            strength: 60,
            healthy: true,
        },
    ];

    let report = call_or_hint!(
        "ex22",
        "build_deep_signal_report",
        build_deep_signal_report(&samples)
    )
    .expect("non-empty samples should produce a report");

    assert_eq!(report.healthy_nodes, vec!["node-a", "node-c"]);
    assert_eq!(report.degraded_nodes, vec!["node-b"]);
    assert!((report.average_strength - 60.0).abs() < f64::EPSILON);
}

#[test]
fn build_report_supports_fractional_average_strength() {
    let samples = vec![
        DeepSignalSample {
            node: "n1".to_string(),
            strength: 1,
            healthy: true,
        },
        DeepSignalSample {
            node: "n2".to_string(),
            strength: 2,
            healthy: true,
        },
        DeepSignalSample {
            node: "n3".to_string(),
            strength: 2,
            healthy: false,
        },
    ];

    let report = call_or_hint!(
        "ex22",
        "build_deep_signal_report",
        build_deep_signal_report(&samples)
    )
    .expect("valid samples should produce average");

    assert!((report.average_strength - (5.0 / 3.0)).abs() < 1e-9);
}

#[test]
fn build_report_empty_samples_returns_empty_node_list_error() {
    let result = call_or_hint!(
        "ex22",
        "build_deep_signal_report",
        build_deep_signal_report(&[])
    );

    assert!(
        matches!(result, Err(DeepSignalError::EmptyNodeList)),
        "empty sample sets should not produce reports"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn run_pipeline_success_returns_report() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/deep-signals/one"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(r#"{"node":"one","strength":80,"healthy":true}"#),
        )
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/deep-signals/two"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(r#"{"node":"two","strength":20,"healthy":false}"#),
        )
        .expect(1)
        .mount(&server)
        .await;

    let nodes = vec!["one".to_string(), "two".to_string()];
    let base_url = server.uri();
    let report = async_call_or_hint!("ex22", "run_deep_signal_pipeline", {
        let base_url = base_url.clone();
        let nodes = nodes.clone();
        run_deep_signal_pipeline(&base_url, &nodes).await
    })
    .expect("pipeline should gather and report");

    assert_eq!(report.healthy_nodes, vec!["one"]);
    assert_eq!(report.degraded_nodes, vec!["two"]);
    assert!((report.average_strength - 50.0).abs() < f64::EPSILON);
}

#[tokio::test(flavor = "multi_thread")]
async fn run_pipeline_propagates_gather_errors() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/deep-signals/down"))
        .respond_with(ResponseTemplate::new(500))
        .expect(1)
        .mount(&server)
        .await;

    let nodes = vec!["down".to_string()];
    let base_url = server.uri();
    let result = async_call_or_hint!("ex22", "run_deep_signal_pipeline", {
        let base_url = base_url.clone();
        let nodes = nodes.clone();
        run_deep_signal_pipeline(&base_url, &nodes).await
    });

    assert!(
        matches!(result, Err(DeepSignalError::InvalidStatus { .. })),
        "pipeline should forward gather-stage failures"
    );
}
