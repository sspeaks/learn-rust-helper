use ex18_signal_timeout::{fetch_signal_with_timeout, SignalTimeoutError};
use std::future::Future;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};
use std::time::Duration;
use wiremock::matchers::method;
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

#[test]
fn fetch_signal_successfully_decodes_json_envelope() {
    let server = block_on(MockServer::start());
    block_on(
        Mock::given(method("GET"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(r#"{"node_id":"node-a","status":"healthy","latency_ms":14}"#),
            )
            .expect(1)
            .mount(&server),
    );

    let base_url = server.uri();
    let envelope = call_or_hint!(
        "ex18",
        "fetch_signal_with_timeout",
        fetch_signal_with_timeout(&base_url, Duration::from_secs(1))
    )
    .expect("200 response should decode into SignalEnvelope");

    assert_eq!(envelope.node_id, "node-a");
    assert_eq!(envelope.status, "healthy");
    assert_eq!(envelope.latency_ms, 14);
}

#[test]
fn fetch_signal_non_success_status_maps_to_http_status_error() {
    let server = block_on(MockServer::start());
    block_on(
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(504))
            .expect(1)
            .mount(&server),
    );

    let base_url = server.uri();
    let result = call_or_hint!(
        "ex18",
        "fetch_signal_with_timeout",
        fetch_signal_with_timeout(&base_url, Duration::from_secs(1))
    );

    match result {
        Err(SignalTimeoutError::HttpStatus(status)) => {
            assert_eq!(status, 504, "HTTP status code should be preserved")
        }
        other => panic!("expected HttpStatus(504), got {other:?}"),
    }
}

#[test]
fn fetch_signal_timeout_maps_to_timed_out_error() {
    let server = block_on(MockServer::start());
    block_on(
        Mock::given(method("GET"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_delay(Duration::from_millis(80))
                    .set_body_string(r#"{"node_id":"node-a","status":"healthy","latency_ms":14}"#),
            )
            .expect(1)
            .mount(&server),
    );

    let base_url = server.uri();
    let result = call_or_hint!(
        "ex18",
        "fetch_signal_with_timeout",
        fetch_signal_with_timeout(&base_url, Duration::from_millis(5))
    );

    assert!(
        matches!(result, Err(SignalTimeoutError::TimedOut)),
        "request exceeding timeout must map to SignalTimeoutError::TimedOut"
    );
}

#[test]
fn fetch_signal_connection_failure_maps_to_transport_error() {
    let result = call_or_hint!(
        "ex18",
        "fetch_signal_with_timeout",
        fetch_signal_with_timeout("http://127.0.0.1:9", Duration::from_millis(50))
    );

    assert!(
        matches!(result, Err(SignalTimeoutError::Transport(_))),
        "unreachable host should map to transport errors"
    );
}

#[test]
fn fetch_signal_invalid_json_maps_to_decode_error() {
    let server = block_on(MockServer::start());
    block_on(
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_string("not-json"))
            .expect(1)
            .mount(&server),
    );

    let base_url = server.uri();
    let result = call_or_hint!(
        "ex18",
        "fetch_signal_with_timeout",
        fetch_signal_with_timeout(&base_url, Duration::from_secs(1))
    );

    assert!(
        matches!(result, Err(SignalTimeoutError::Decode(_))),
        "invalid JSON body should map to decode errors"
    );
}

#[test]
fn fetch_signal_invalid_utf8_maps_to_read_body_error() {
    let server = block_on(MockServer::start());
    block_on(
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(vec![0xff, 0xfe]))
            .expect(1)
            .mount(&server),
    );

    let base_url = server.uri();
    let result = call_or_hint!(
        "ex18",
        "fetch_signal_with_timeout",
        fetch_signal_with_timeout(&base_url, Duration::from_secs(1))
    );

    assert!(
        matches!(result, Err(SignalTimeoutError::ReadBody(_))),
        "non-UTF8 response body should map to ReadBody"
    );
}

#[test]
fn fetch_signal_preserves_zero_latency_value() {
    let server = block_on(MockServer::start());
    block_on(
        Mock::given(method("GET"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(r#"{"node_id":"node-z","status":"idle","latency_ms":0}"#),
            )
            .expect(1)
            .mount(&server),
    );

    let base_url = server.uri();
    let envelope = call_or_hint!(
        "ex18",
        "fetch_signal_with_timeout",
        fetch_signal_with_timeout(&base_url, Duration::from_secs(1))
    )
    .expect("valid JSON with latency 0 should decode");

    assert_eq!(envelope.latency_ms, 0);
}

#[test]
fn fetch_signal_requires_expected_json_shape() {
    let server = block_on(MockServer::start());
    block_on(
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_string(r#"{"node_id":"n1"}"#))
            .expect(1)
            .mount(&server),
    );

    let base_url = server.uri();
    let result = call_or_hint!(
        "ex18",
        "fetch_signal_with_timeout",
        fetch_signal_with_timeout(&base_url, Duration::from_secs(1))
    );

    assert!(
        matches!(result, Err(SignalTimeoutError::Decode(_))),
        "missing status/latency fields should fail decode"
    );
}
