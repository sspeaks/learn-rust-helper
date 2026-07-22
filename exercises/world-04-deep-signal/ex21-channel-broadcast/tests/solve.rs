use ex21_channel_broadcast::{
    broadcast_channels, fetch_broadcast_receipt, BroadcastTarget, ChannelBroadcastError,
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

fn target(channel: &str) -> BroadcastTarget {
    BroadcastTarget {
        channel: channel.to_string(),
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn fetch_single_receipt_success() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/broadcast/alpha"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(r#"{"channel":"alpha","acknowledged":true}"#),
        )
        .expect(1)
        .mount(&server)
        .await;

    let base_url = server.uri();
    let receipt = async_call_or_hint!("ex21", "fetch_broadcast_receipt", {
        let base_url = base_url.clone();
        let target = target("alpha");
        fetch_broadcast_receipt(&base_url, &target).await
    })
    .expect("successful receipt request should decode");

    assert_eq!(receipt.channel, "alpha");
    assert!(receipt.acknowledged);
}

#[tokio::test(flavor = "multi_thread")]
async fn fetch_invalid_status_includes_channel_context() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/broadcast/red"))
        .respond_with(ResponseTemplate::new(429))
        .expect(1)
        .mount(&server)
        .await;

    let base_url = server.uri();
    let result = async_call_or_hint!("ex21", "fetch_broadcast_receipt", {
        let base_url = base_url.clone();
        let target = target("red");
        fetch_broadcast_receipt(&base_url, &target).await
    });

    match result {
        Err(ChannelBroadcastError::InvalidStatus { channel, status }) => {
            assert_eq!(channel, "red");
            assert_eq!(status.as_u16(), 429);
        }
        other => panic!("expected InvalidStatus with channel context, got {other:?}"),
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn fetch_invalid_json_maps_to_decode_error() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/broadcast/bad-json"))
        .respond_with(ResponseTemplate::new(200).set_body_string("not-json"))
        .expect(1)
        .mount(&server)
        .await;

    let base_url = server.uri();
    let result = async_call_or_hint!("ex21", "fetch_broadcast_receipt", {
        let base_url = base_url.clone();
        let target = target("bad-json");
        fetch_broadcast_receipt(&base_url, &target).await
    });

    assert!(
        matches!(result, Err(ChannelBroadcastError::Decode(_))),
        "invalid receipt JSON should map to decode errors"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn fetch_unreachable_host_maps_to_request_error() {
    let result = async_call_or_hint!("ex21", "fetch_broadcast_receipt", {
        let target = target("offline");
        fetch_broadcast_receipt("http://127.0.0.1:9", &target).await
    });

    assert!(
        matches!(result, Err(ChannelBroadcastError::Request(_))),
        "connection failures should map to request errors"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn broadcast_empty_targets_returns_empty_vec() {
    let result = async_call_or_hint!("ex21", "broadcast_channels", {
        broadcast_channels("http://127.0.0.1:9", &[]).await
    })
    .expect("empty target list should complete without errors");

    assert!(result.is_empty());
}

#[tokio::test(flavor = "multi_thread")]
async fn broadcast_single_target_success() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/broadcast/solo"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(r#"{"channel":"solo","acknowledged":true}"#),
        )
        .expect(1)
        .mount(&server)
        .await;

    let targets = vec![target("solo")];
    let base_url = server.uri();
    let receipts = async_call_or_hint!("ex21", "broadcast_channels", {
        let base_url = base_url.clone();
        let targets = targets.clone();
        broadcast_channels(&base_url, &targets).await
    })
    .expect("single target broadcast should succeed");

    assert_eq!(receipts.len(), 1);
    assert_eq!(receipts[0].channel, "solo");
    assert!(receipts[0].acknowledged);
}

#[tokio::test(flavor = "multi_thread")]
async fn broadcast_multiple_targets_preserves_input_order() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/broadcast/alpha"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_delay(Duration::from_millis(30))
                .set_body_string(r#"{"channel":"alpha","acknowledged":true}"#),
        )
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/broadcast/beta"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(r#"{"channel":"beta","acknowledged":true}"#),
        )
        .expect(1)
        .mount(&server)
        .await;

    let targets = vec![target("alpha"), target("beta")];
    let base_url = server.uri();
    let receipts = async_call_or_hint!("ex21", "broadcast_channels", {
        let base_url = base_url.clone();
        let targets = targets.clone();
        broadcast_channels(&base_url, &targets).await
    })
    .expect("all targets should succeed");

    let channels: Vec<&str> = receipts
        .iter()
        .map(|receipt| receipt.channel.as_str())
        .collect();
    assert_eq!(
        channels,
        vec!["alpha", "beta"],
        "output order must align with input target order"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn broadcast_surfaces_first_invalid_status_with_channel() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/broadcast/alpha"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(r#"{"channel":"alpha","acknowledged":true}"#),
        )
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/broadcast/beta"))
        .respond_with(ResponseTemplate::new(503))
        .expect(1)
        .mount(&server)
        .await;

    let targets = vec![target("alpha"), target("beta")];
    let base_url = server.uri();
    let result = async_call_or_hint!("ex21", "broadcast_channels", {
        let base_url = base_url.clone();
        let targets = targets.clone();
        broadcast_channels(&base_url, &targets).await
    });

    match result {
        Err(ChannelBroadcastError::InvalidStatus { channel, status }) => {
            assert_eq!(channel, "beta");
            assert_eq!(status.as_u16(), 503);
        }
        other => panic!("expected InvalidStatus for failing channel, got {other:?}"),
    }
}
