use ex22_relay_dispatch::{
    dispatch_relay, RelayDispatchError, RelayDispatchReceipt, RelayDispatchRequest,
};
use serde_json::{json, Value};
use std::future::Future;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};
use wiremock::matchers::{body_json, method, path};
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

struct ThreadWaker(std::thread::Thread);

impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.0.unpark();
    }
}

fn block_on<F: Future>(future: F) -> F::Output {
    let waker = Waker::from(Arc::new(ThreadWaker(std::thread::current())));
    let mut context = Context::from_waker(&waker);
    let mut future = Box::pin(future);

    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(value) => return value,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}

fn sample_request(payload: Value) -> RelayDispatchRequest {
    RelayDispatchRequest {
        route: "relay-7".to_string(),
        priority: 3,
        payload,
    }
}

fn success_receipt() -> RelayDispatchReceipt {
    RelayDispatchReceipt {
        accepted: true,
        relay_id: "relay-7".to_string(),
        queued_at: "2026-07-21T15:35:48.817-07:00".to_string(),
    }
}

#[test]
fn dispatch_success_posts_json_and_decodes_receipt() {
    let server = block_on(MockServer::start());
    let request = sample_request(json!({"artifact":"core","count":2}));

    block_on(
        Mock::given(method("POST"))
            .and(path("/relay/dispatch"))
            .and(body_json(json!({
                "route": "relay-7",
                "priority": 3,
                "payload": {"artifact": "core", "count": 2}
            })))
            .respond_with(ResponseTemplate::new(200).set_body_string(
                r#"{"accepted":true,"relay_id":"relay-7","queued_at":"2026-07-21T15:35:48.817-07:00"}"#,
            ))
            .expect(1)
            .mount(&server),
    );

    let base_url = server.uri();
    let receipt = call_or_hint!(
        "ex22",
        "dispatch_relay",
        dispatch_relay(&base_url, &request)
    )
    .expect("valid relay request should decode receipt");

    assert_eq!(receipt, success_receipt());
}

#[test]
fn dispatch_supports_nested_payload_values() {
    let server = block_on(MockServer::start());
    let request = sample_request(json!({"nested": {"flag": true, "tags": ["a", "b"]}}));

    block_on(
        Mock::given(method("POST"))
            .and(path("/relay/dispatch"))
            .and(body_json(json!({
                "route": "relay-7",
                "priority": 3,
                "payload": {"nested": {"flag": true, "tags": ["a", "b"]}}
            })))
            .respond_with(ResponseTemplate::new(200).set_body_string(
                r#"{"accepted":true,"relay_id":"relay-7","queued_at":"2026-07-21T15:35:48.817-07:00"}"#,
            ))
            .expect(1)
            .mount(&server),
    );

    let base_url = server.uri();
    let receipt = call_or_hint!(
        "ex22",
        "dispatch_relay",
        dispatch_relay(&base_url, &request)
    )
    .expect("nested JSON payload should serialize");

    assert!(receipt.accepted);
}

#[test]
fn dispatch_preserves_false_accepted_receipt_field() {
    let server = block_on(MockServer::start());
    let request = sample_request(json!({"artifact":"core"}));

    block_on(
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "accepted": false,
                "relay_id": "relay-7",
                "queued_at": "queued"
            })))
            .expect(1)
            .mount(&server),
    );

    let base_url = server.uri();
    let receipt = call_or_hint!(
        "ex22",
        "dispatch_relay",
        dispatch_relay(&base_url, &request)
    )
    .expect("valid JSON receipt should decode even when not accepted");

    assert!(!receipt.accepted);
    assert_eq!(receipt.relay_id, "relay-7");
}

#[test]
fn dispatch_status_errors_map_to_request_error() {
    let server = block_on(MockServer::start());
    let request = sample_request(json!({"artifact":"core"}));

    block_on(
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(500).set_body_string("fail"))
            .expect(1)
            .mount(&server),
    );

    let base_url = server.uri();
    let result = call_or_hint!(
        "ex22",
        "dispatch_relay",
        dispatch_relay(&base_url, &request)
    );

    match result {
        Err(RelayDispatchError::Request(ureq::Error::Status(code, _))) => {
            assert_eq!(code, 500, "HTTP status should be preserved")
        }
        other => panic!("expected RelayDispatchError::Request(Status), got {other:?}"),
    }
}

#[test]
fn dispatch_connection_failure_maps_to_request_transport_error() {
    let request = sample_request(json!({"artifact":"core"}));
    let result = call_or_hint!(
        "ex22",
        "dispatch_relay",
        dispatch_relay("http://127.0.0.1:9", &request)
    );

    assert!(
        matches!(
            result,
            Err(RelayDispatchError::Request(ureq::Error::Transport(_)))
        ),
        "unreachable host should map to request transport errors"
    );
}

#[test]
fn dispatch_invalid_utf8_response_maps_to_read_body_error() {
    let server = block_on(MockServer::start());
    let request = sample_request(json!({"artifact":"core"}));

    block_on(
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(vec![0xff, 0xfe, 0xfd]))
            .expect(1)
            .mount(&server),
    );

    let base_url = server.uri();
    let result = call_or_hint!(
        "ex22",
        "dispatch_relay",
        dispatch_relay(&base_url, &request)
    );

    assert!(
        matches!(result, Err(RelayDispatchError::ReadBody(_))),
        "invalid UTF-8 in response body should map to ReadBody"
    );
}

#[test]
fn dispatch_invalid_json_response_maps_to_decode_error() {
    let server = block_on(MockServer::start());
    let request = sample_request(json!({"artifact":"core"}));

    block_on(
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_string("not-json"))
            .expect(1)
            .mount(&server),
    );

    let base_url = server.uri();
    let result = call_or_hint!(
        "ex22",
        "dispatch_relay",
        dispatch_relay(&base_url, &request)
    );

    assert!(
        matches!(result, Err(RelayDispatchError::Decode(_))),
        "invalid JSON response should map to Decode"
    );
}

#[test]
fn dispatch_requires_receipt_json_shape() {
    let server = block_on(MockServer::start());
    let request = sample_request(json!({"artifact":"core"}));

    block_on(
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_string(r#"{"accepted":true}"#))
            .expect(1)
            .mount(&server),
    );

    let base_url = server.uri();
    let result = call_or_hint!(
        "ex22",
        "dispatch_relay",
        dispatch_relay(&base_url, &request)
    );

    assert!(
        matches!(result, Err(RelayDispatchError::Decode(_))),
        "missing receipt fields should be reported as decode errors"
    );
}
