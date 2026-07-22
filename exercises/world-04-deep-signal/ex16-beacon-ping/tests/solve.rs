use ex16_beacon_ping::{ping_beacon, BeaconPingError};
use std::future::Future;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};
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
fn ping_success_returns_endpoint_status_and_body() {
    let server = block_on(MockServer::start());
    block_on(
        Mock::given(method("GET"))
            .and(path("/beacons/alpha-7"))
            .respond_with(ResponseTemplate::new(200).set_body_string("online"))
            .expect(1)
            .mount(&server),
    );

    let base_url = server.uri();
    let ping = call_or_hint!("ex16", "ping_beacon", ping_beacon(&base_url, "alpha-7"))
        .expect("successful 200 ping should decode");

    assert_eq!(ping.endpoint, format!("{base_url}/beacons/alpha-7"));
    assert_eq!(ping.status, 200);
    assert_eq!(ping.body, "online");
}

#[test]
fn ping_handles_not_found_as_request_error() {
    let server = block_on(MockServer::start());
    block_on(
        Mock::given(method("GET"))
            .and(path("/beacons/missing"))
            .respond_with(ResponseTemplate::new(404).set_body_string("missing"))
            .expect(1)
            .mount(&server),
    );

    let base_url = server.uri();
    let result = call_or_hint!("ex16", "ping_beacon", ping_beacon(&base_url, "missing"));

    match result {
        Err(BeaconPingError::Request(ureq::Error::Status(code, _))) => {
            assert_eq!(
                code, 404,
                "HTTP 404 should be preserved in ureq::Error::Status"
            )
        }
        other => panic!("expected request status error, got {other:?}"),
    }
}

#[test]
fn ping_handles_server_error_as_request_error() {
    let server = block_on(MockServer::start());
    block_on(
        Mock::given(method("GET"))
            .and(path("/beacons/unstable"))
            .respond_with(ResponseTemplate::new(503).set_body_string("retry later"))
            .expect(1)
            .mount(&server),
    );

    let base_url = server.uri();
    let result = call_or_hint!("ex16", "ping_beacon", ping_beacon(&base_url, "unstable"));

    match result {
        Err(BeaconPingError::Request(ureq::Error::Status(code, _))) => {
            assert_eq!(code, 503, "HTTP 503 should map to request status errors")
        }
        other => panic!("expected request status error, got {other:?}"),
    }
}

#[test]
fn ping_connection_failure_maps_to_transport_request_error() {
    let result = call_or_hint!(
        "ex16",
        "ping_beacon",
        ping_beacon("http://127.0.0.1:9", "offline")
    );

    match result {
        Err(BeaconPingError::Request(ureq::Error::Transport(_))) => {}
        other => panic!("expected transport request error, got {other:?}"),
    }
}

#[test]
fn ping_invalid_utf8_body_maps_to_read_body_error() {
    let server = block_on(MockServer::start());
    block_on(
        Mock::given(method("GET"))
            .and(path("/beacons/raw"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(vec![0xff, 0xfe, 0xfd]))
            .expect(1)
            .mount(&server),
    );

    let base_url = server.uri();
    let result = call_or_hint!("ex16", "ping_beacon", ping_beacon(&base_url, "raw"));

    match result {
        Err(BeaconPingError::ReadBody(_)) => {}
        other => panic!("expected body-read error for invalid UTF-8, got {other:?}"),
    }
}

#[test]
fn ping_supports_empty_204_response_body() {
    let server = block_on(MockServer::start());
    block_on(
        Mock::given(method("GET"))
            .and(path("/beacons/empty"))
            .respond_with(ResponseTemplate::new(204))
            .expect(1)
            .mount(&server),
    );

    let base_url = server.uri();
    let ping = call_or_hint!("ex16", "ping_beacon", ping_beacon(&base_url, "empty"))
        .expect("204 response should still produce BeaconPing");

    assert_eq!(ping.status, 204);
    assert_eq!(ping.body, "");
}

#[test]
fn ping_preserves_multiline_body_text() {
    let server = block_on(MockServer::start());
    block_on(
        Mock::given(method("GET"))
            .and(path("/beacons/logs"))
            .respond_with(ResponseTemplate::new(200).set_body_string("line-1\nline-2"))
            .expect(1)
            .mount(&server),
    );

    let base_url = server.uri();
    let ping = call_or_hint!("ex16", "ping_beacon", ping_beacon(&base_url, "logs"))
        .expect("200 response with text body should succeed");

    assert_eq!(ping.body, "line-1\nline-2");
}

#[test]
fn ping_allows_alphanumeric_beacon_ids() {
    let server = block_on(MockServer::start());
    block_on(
        Mock::given(method("GET"))
            .and(path("/beacons/BETA42"))
            .respond_with(ResponseTemplate::new(200).set_body_string("ok"))
            .expect(1)
            .mount(&server),
    );

    let base_url = server.uri();
    let ping = call_or_hint!("ex16", "ping_beacon", ping_beacon(&base_url, "BETA42"))
        .expect("alphanumeric IDs should be routable");

    assert_eq!(ping.endpoint, format!("{base_url}/beacons/BETA42"));
}
