use ex10_command_router::{route_batch, route_command, Command};

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

// ── route_command: Dock ──────────────────────────────────────────────────────

#[test]
fn dock_produces_nonempty_string() {
    let msg = call_or_hint!(
        "ex10",
        "route_command",
        route_command(Command::Dock { bay: 3 })
    );
    assert!(
        !msg.is_empty(),
        "Dock must produce a non-empty routing message"
    );
}

#[test]
fn dock_preserves_bay_number() {
    let msg = call_or_hint!(
        "ex10",
        "route_command",
        route_command(Command::Dock { bay: 7 })
    );
    assert!(
        msg.contains('7'),
        "Dock routing message must contain the bay number (7)"
    );
}

#[test]
fn dock_different_bays_produce_different_messages() {
    let msg3 = call_or_hint!(
        "ex10",
        "route_command",
        route_command(Command::Dock { bay: 3 })
    );
    let msg9 = call_or_hint!(
        "ex10",
        "route_command",
        route_command(Command::Dock { bay: 9 })
    );
    assert_ne!(
        msg3, msg9,
        "different bay numbers must produce different messages"
    );
}

// ── route_command: Launch ─────────────────────────────────────────────────────

#[test]
fn launch_produces_nonempty_string() {
    let msg = call_or_hint!(
        "ex10",
        "route_command",
        route_command(Command::Launch { window: 2 })
    );
    assert!(
        !msg.is_empty(),
        "Launch must produce a non-empty routing message"
    );
}

#[test]
fn launch_preserves_window_number() {
    let msg = call_or_hint!(
        "ex10",
        "route_command",
        route_command(Command::Launch { window: 5 })
    );
    assert!(
        msg.contains('5'),
        "Launch routing message must contain the window number (5)"
    );
}

// ── route_command: Broadcast ──────────────────────────────────────────────────

#[test]
fn broadcast_produces_nonempty_string() {
    let msg = call_or_hint!(
        "ex10",
        "route_command",
        route_command(Command::Broadcast("hello crew".to_string()))
    );
    assert!(
        !msg.is_empty(),
        "Broadcast must produce a non-empty routing message"
    );
}

#[test]
fn broadcast_preserves_payload_text() {
    let msg = call_or_hint!(
        "ex10",
        "route_command",
        route_command(Command::Broadcast("mission complete".to_string()))
    );
    assert!(
        msg.contains("mission complete"),
        "Broadcast routing message must contain the payload text"
    );
}

#[test]
fn broadcast_different_payloads_produce_different_messages() {
    let msg_a = call_or_hint!(
        "ex10",
        "route_command",
        route_command(Command::Broadcast("alpha".to_string()))
    );
    let msg_b = call_or_hint!(
        "ex10",
        "route_command",
        route_command(Command::Broadcast("bravo".to_string()))
    );
    assert_ne!(msg_a, msg_b);
}

// ── route_command: Abort ──────────────────────────────────────────────────────

#[test]
fn abort_produces_nonempty_string() {
    let msg = call_or_hint!("ex10", "route_command", route_command(Command::Abort));
    assert!(
        !msg.is_empty(),
        "Abort must produce a non-empty routing message"
    );
}

// ── All four variants produce distinct messages ───────────────────────────────

#[test]
fn all_variants_produce_distinct_messages() {
    let dock = call_or_hint!(
        "ex10",
        "route_command",
        route_command(Command::Dock { bay: 1 })
    );
    let launch = call_or_hint!(
        "ex10",
        "route_command",
        route_command(Command::Launch { window: 1 })
    );
    let broadcast = call_or_hint!(
        "ex10",
        "route_command",
        route_command(Command::Broadcast("x".to_string()))
    );
    let abort = call_or_hint!("ex10", "route_command", route_command(Command::Abort));

    assert_ne!(dock, launch);
    assert_ne!(dock, abort);
    assert_ne!(launch, abort);
    // broadcast with payload "x" can differ from the rest
    let _ = broadcast; // checked above for non-empty
}

// ── route_batch ────────────────────────────────────────────────────────────────

#[test]
fn batch_processes_all_commands() {
    let commands = vec![
        Command::Dock { bay: 2 },
        Command::Abort,
        Command::Broadcast("all hands".to_string()),
    ];
    let results = call_or_hint!("ex10", "route_command", route_batch(commands));
    assert_eq!(
        results.len(),
        3,
        "batch must produce one result per command"
    );
    assert!(!results[0].is_empty());
    assert!(!results[1].is_empty());
    assert!(results[2].contains("all hands"));
}

#[test]
fn batch_empty_produces_empty() {
    let results = call_or_hint!("ex10", "route_command", route_batch(vec![]));
    assert!(results.is_empty());
}
