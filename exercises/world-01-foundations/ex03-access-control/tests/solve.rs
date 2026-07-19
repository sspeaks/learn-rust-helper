use ex03_access_control::{access_message, gate_announcement, Clearance};

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

// ── access_message: all 6 clearance × duty combinations ────────────────────

#[test]
fn all_six_combinations_return_nonempty() {
    let combos = [
        (Clearance::Visitor, false),
        (Clearance::Visitor, true),
        (Clearance::Engineer, false),
        (Clearance::Engineer, true),
        (Clearance::Captain, false),
        (Clearance::Captain, true),
    ];
    for (clearance, on_duty) in combos {
        let msg = call_or_hint!("ex03", "access_message", access_message(clearance, on_duty));
        assert!(
            !msg.is_empty(),
            "access_message({clearance:?}, {on_duty}) must return a non-empty string"
        );
    }
}

#[test]
fn different_clearances_produce_different_messages_on_duty() {
    let visitor = call_or_hint!(
        "ex03",
        "access_message",
        access_message(Clearance::Visitor, true)
    );
    let engineer = call_or_hint!(
        "ex03",
        "access_message",
        access_message(Clearance::Engineer, true)
    );
    let captain = call_or_hint!(
        "ex03",
        "access_message",
        access_message(Clearance::Captain, true)
    );
    assert_ne!(
        visitor, engineer,
        "Visitor on-duty and Engineer on-duty must have different messages"
    );
    assert_ne!(
        engineer, captain,
        "Engineer on-duty and Captain on-duty must have different messages"
    );
}

#[test]
fn different_clearances_produce_different_messages_off_duty() {
    let visitor = call_or_hint!(
        "ex03",
        "access_message",
        access_message(Clearance::Visitor, false)
    );
    let captain = call_or_hint!(
        "ex03",
        "access_message",
        access_message(Clearance::Captain, false)
    );
    assert_ne!(
        visitor, captain,
        "Visitor off-duty and Captain off-duty must have different messages"
    );
}

#[test]
fn duty_status_affects_message_for_each_clearance() {
    for clearance in [Clearance::Visitor, Clearance::Engineer, Clearance::Captain] {
        let on = call_or_hint!("ex03", "access_message", access_message(clearance, true));
        let off = call_or_hint!("ex03", "access_message", access_message(clearance, false));
        assert_ne!(
            on, off,
            "{clearance:?}: on-duty and off-duty must produce different messages"
        );
    }
}

// ── gate_announcement ────────────────────────────────────────────────────────

#[test]
fn gate_announcement_prefixes_name_with_colon_space() {
    let ann = call_or_hint!(
        "ex03",
        "access_message",
        gate_announcement("Commander Vex", Clearance::Captain, true)
    );
    let msg = call_or_hint!(
        "ex03",
        "access_message",
        access_message(Clearance::Captain, true)
    );
    let expected = format!("Commander Vex: {msg}");
    assert_eq!(
        ann, expected,
        "gate_announcement should be '{{name}}: {{access_message}}'"
    );
}

#[test]
fn gate_announcement_visitor_off_duty() {
    let ann = call_or_hint!(
        "ex03",
        "access_message",
        gate_announcement("Scout", Clearance::Visitor, false)
    );
    let msg = call_or_hint!(
        "ex03",
        "access_message",
        access_message(Clearance::Visitor, false)
    );
    assert_eq!(ann, format!("Scout: {msg}"));
}
