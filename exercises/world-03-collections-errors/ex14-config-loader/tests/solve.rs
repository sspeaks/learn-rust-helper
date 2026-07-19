use ex14_config_loader::{parse_server_config, ConfigError};

// ── Stub detection helpers ──────────────────────────────────────────────────

fn is_stub_panic(e: &Box<dyn std::any::Any + Send>) -> bool {
    e.downcast_ref::<&str>()
        .map_or(false, |s| s.contains("not yet implemented"))
        || e.downcast_ref::<String>()
            .map_or(false, |s| s.contains("not yet implemented"))
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

// ── parse_server_config: happy path ──────────────────────────────────────────

#[test]
fn valid_input_parses_all_fields() {
    let input = "host=localhost\nport=8080\nretry_limit=3";
    let result = call_or_hint!("ex14", "parse_server_config", parse_server_config(input));
    let cfg = result.expect("valid input must parse successfully");
    assert_eq!(cfg.host, "localhost");
    assert_eq!(cfg.port, 8080);
    assert_eq!(cfg.retry_limit, 3);
}

#[test]
fn valid_input_different_values() {
    let input = "host=192.168.1.1\nport=443\nretry_limit=10";
    let result = call_or_hint!("ex14", "parse_server_config", parse_server_config(input));
    let cfg = result.expect("valid config should parse");
    assert_eq!(cfg.host, "192.168.1.1");
    assert_eq!(cfg.port, 443);
    assert_eq!(cfg.retry_limit, 10);
}

// ── parse_server_config: MissingField ────────────────────────────────────────

#[test]
fn missing_host_returns_missing_field_host() {
    let input = "port=8080\nretry_limit=3";
    let result = call_or_hint!("ex14", "parse_server_config", parse_server_config(input));
    assert_eq!(
        result,
        Err(ConfigError::MissingField("host")),
        "absent host key → MissingField(\"host\")"
    );
}

#[test]
fn missing_port_returns_missing_field_port() {
    let input = "host=localhost\nretry_limit=3";
    let result = call_or_hint!("ex14", "parse_server_config", parse_server_config(input));
    assert_eq!(result, Err(ConfigError::MissingField("port")));
}

#[test]
fn missing_retry_limit_returns_missing_field_retry_limit() {
    let input = "host=localhost\nport=8080";
    let result = call_or_hint!("ex14", "parse_server_config", parse_server_config(input));
    assert_eq!(result, Err(ConfigError::MissingField("retry_limit")));
}

// ── parse_server_config: DuplicateKey ────────────────────────────────────────

#[test]
fn duplicate_host_returns_duplicate_key() {
    let input = "host=localhost\nhost=remotehost\nport=8080\nretry_limit=3";
    let result = call_or_hint!("ex14", "parse_server_config", parse_server_config(input));
    assert_eq!(
        result,
        Err(ConfigError::DuplicateKey("host".to_string())),
        "duplicate host key → DuplicateKey(\"host\")"
    );
}

#[test]
fn duplicate_port_returns_duplicate_key() {
    let input = "host=localhost\nport=8080\nport=9090\nretry_limit=3";
    let result = call_or_hint!("ex14", "parse_server_config", parse_server_config(input));
    assert_eq!(result, Err(ConfigError::DuplicateKey("port".to_string())));
}

// ── parse_server_config: UnknownKey ──────────────────────────────────────────

#[test]
fn unknown_key_returns_unknown_key() {
    let input = "host=localhost\nport=8080\nretry_limit=3\ntimeout=30";
    let result = call_or_hint!("ex14", "parse_server_config", parse_server_config(input));
    assert_eq!(
        result,
        Err(ConfigError::UnknownKey("timeout".to_string())),
        "unrecognized key → UnknownKey"
    );
}

#[test]
fn another_unknown_key() {
    let input = "host=localhost\nport=8080\nretry_limit=3\nmode=debug";
    let result = call_or_hint!("ex14", "parse_server_config", parse_server_config(input));
    assert_eq!(result, Err(ConfigError::UnknownKey("mode".to_string())));
}

// ── parse_server_config: InvalidNumber ───────────────────────────────────────

#[test]
fn invalid_port_value_returns_invalid_number() {
    let input = "host=localhost\nport=notanumber\nretry_limit=3";
    let result = call_or_hint!("ex14", "parse_server_config", parse_server_config(input));
    assert_eq!(
        result,
        Err(ConfigError::InvalidNumber {
            field: "port",
            value: "notanumber".to_string()
        }),
        "non-numeric port → InvalidNumber with field=port and the bad value"
    );
}

#[test]
fn invalid_retry_limit_value_returns_invalid_number() {
    let input = "host=localhost\nport=8080\nretry_limit=many";
    let result = call_or_hint!("ex14", "parse_server_config", parse_server_config(input));
    assert_eq!(
        result,
        Err(ConfigError::InvalidNumber {
            field: "retry_limit",
            value: "many".to_string()
        })
    );
}
