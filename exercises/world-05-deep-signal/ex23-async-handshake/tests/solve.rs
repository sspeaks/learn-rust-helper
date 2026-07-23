use ex23_async_handshake::{perform_async_handshake, AsyncHandshakeError, HandshakeReceipt};
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

fn sample_receipt() -> HandshakeReceipt {
    HandshakeReceipt {
        session_id: "sess-42".to_string(),
        protocol: "v2".to_string(),
        motd: "welcome aboard".to_string(),
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn perform_handshake_success_decodes_receipt() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/handshake/orca-1"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(
                r#"{"session_id":"sess-42","protocol":"v2","motd":"welcome aboard"}"#,
            ),
        )
        .expect(1)
        .mount(&server)
        .await;

    let base_url = server.uri();
    let receipt = async_call_or_hint!("ex23", "perform_async_handshake", {
        let base_url = base_url.clone();
        perform_async_handshake(&base_url, "orca-1").await
    })
    .expect("successful handshake should decode receipt");

    assert_eq!(receipt, sample_receipt());
}

#[tokio::test(flavor = "multi_thread")]
async fn perform_handshake_supports_different_call_signs() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/handshake/zenith"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(
                r#"{"session_id":"sess-zenith","protocol":"v2","motd":"locked in"}"#,
            ),
        )
        .expect(1)
        .mount(&server)
        .await;

    let base_url = server.uri();
    let receipt = async_call_or_hint!("ex23", "perform_async_handshake", {
        let base_url = base_url.clone();
        perform_async_handshake(&base_url, "zenith").await
    })
    .expect("valid alternate call_sign should succeed");

    assert_eq!(receipt.session_id, "sess-zenith");
    assert_eq!(receipt.protocol, "v2");
}

#[tokio::test(flavor = "multi_thread")]
async fn perform_handshake_preserves_unicode_motd() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/handshake/aurora"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(r#"{"session_id":"sess-u","protocol":"v3","motd":"grüße 🚀"}"#),
        )
        .expect(1)
        .mount(&server)
        .await;

    let base_url = server.uri();
    let receipt = async_call_or_hint!("ex23", "perform_async_handshake", {
        let base_url = base_url.clone();
        perform_async_handshake(&base_url, "aurora").await
    })
    .expect("UTF-8 JSON payload should decode");

    assert_eq!(receipt.motd, "grüße 🚀");
}

#[tokio::test(flavor = "multi_thread")]
async fn perform_handshake_invalid_status_maps_to_invalid_status_error() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(503))
        .expect(1)
        .mount(&server)
        .await;

    let base_url = server.uri();
    let result = async_call_or_hint!("ex23", "perform_async_handshake", {
        let base_url = base_url.clone();
        perform_async_handshake(&base_url, "delta").await
    });

    match result {
        Err(AsyncHandshakeError::InvalidStatus(status)) => {
            assert_eq!(status.as_u16(), 503, "status code should be preserved")
        }
        other => panic!("expected InvalidStatus(503), got {other:?}"),
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn perform_handshake_unreachable_host_maps_to_request_error() {
    let result = async_call_or_hint!("ex23", "perform_async_handshake", {
        perform_async_handshake("http://127.0.0.1:9", "lost").await
    });

    assert!(
        matches!(result, Err(AsyncHandshakeError::Request(_))),
        "transport/request failures should map to AsyncHandshakeError::Request"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn perform_handshake_invalid_json_maps_to_decode_error() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_string("not-json"))
        .expect(1)
        .mount(&server)
        .await;

    let base_url = server.uri();
    let result = async_call_or_hint!("ex23", "perform_async_handshake", {
        let base_url = base_url.clone();
        perform_async_handshake(&base_url, "decode").await
    });

    assert!(
        matches!(result, Err(AsyncHandshakeError::Decode(_))),
        "invalid JSON should map to decode errors"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn perform_handshake_missing_fields_maps_to_decode_error() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_string(r#"{"session_id":"s"}"#))
        .expect(1)
        .mount(&server)
        .await;

    let base_url = server.uri();
    let result = async_call_or_hint!("ex23", "perform_async_handshake", {
        let base_url = base_url.clone();
        perform_async_handshake(&base_url, "missing").await
    });

    assert!(
        matches!(result, Err(AsyncHandshakeError::Decode(_))),
        "JSON with missing fields should not decode successfully"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn perform_handshake_malformed_base_url_maps_to_request_error() {
    let result = async_call_or_hint!("ex23", "perform_async_handshake", {
        perform_async_handshake("not-a-url", "echo").await
    });

    assert!(
        matches!(result, Err(AsyncHandshakeError::Request(_))),
        "invalid base URLs should map to request construction/sending errors"
    );
}
