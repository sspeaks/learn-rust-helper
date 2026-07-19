use ex09_station_crew::CrewMember;

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

// ── CrewMember::new ──────────────────────────────────────────────────────────

#[test]
fn new_stores_all_fields() {
    let member = call_or_hint!(
        "ex09",
        "CrewMember::new",
        CrewMember::new("Nova", "Navigator", 5)
    );
    assert_eq!(member.name, "Nova");
    assert_eq!(member.role, "Navigator");
    assert_eq!(member.level, 5);
}

#[test]
fn new_with_level_zero() {
    let member = call_or_hint!(
        "ex09",
        "CrewMember::new",
        CrewMember::new("Rex", "Cadet", 0)
    );
    assert_eq!(member.level, 0);
}

#[test]
fn new_accepts_string_and_str() {
    // Both &str and String should be accepted (impl Into<String>)
    let member = call_or_hint!(
        "ex09",
        "CrewMember::new",
        CrewMember::new(String::from("Iris"), String::from("Engineer"), 10)
    );
    assert_eq!(member.name, "Iris");
    assert_eq!(member.role, "Engineer");
}

// ── CrewMember::promote ──────────────────────────────────────────────────────

#[test]
fn promote_updates_role() {
    let mut member = call_or_hint!(
        "ex09",
        "CrewMember::new",
        CrewMember::new("Vex", "Cadet", 3)
    );
    call_or_hint!("ex09", "CrewMember::promote", member.promote("Pilot"));
    assert_eq!(member.role, "Pilot", "role must change after promotion");
}

#[test]
fn promote_increments_level() {
    let mut member = call_or_hint!(
        "ex09",
        "CrewMember::new",
        CrewMember::new("Vex", "Cadet", 3)
    );
    call_or_hint!("ex09", "CrewMember::promote", member.promote("Pilot"));
    assert_eq!(member.level, 4, "level must increase by 1 on promotion");
}

#[test]
fn promote_level_cap_at_99() {
    let mut member = call_or_hint!("ex09", "CrewMember::new", CrewMember::new("Max", "Ace", 99));
    call_or_hint!("ex09", "CrewMember::promote", member.promote("Admiral"));
    assert_eq!(member.level, 99, "level must not exceed 99 (capped)");
}

#[test]
fn promote_from_98_reaches_99_not_100() {
    let mut member = call_or_hint!(
        "ex09",
        "CrewMember::new",
        CrewMember::new("Zara", "Senior", 98)
    );
    call_or_hint!("ex09", "CrewMember::promote", member.promote("Commander"));
    assert_eq!(member.level, 99, "98 → 99 on promotion (not 100)");
}

// ── CrewMember::badge ────────────────────────────────────────────────────────
//
// Contract from docstring: "[L05] Nova — Navigator"
// Format: [L{level:02}] {name} — {role}
// The separator is an em dash (U+2014), not a hyphen.

#[test]
fn badge_format_matches_docstring_example() {
    let member = call_or_hint!(
        "ex09",
        "CrewMember::new",
        CrewMember::new("Nova", "Navigator", 5)
    );
    let badge = call_or_hint!("ex09", "CrewMember::badge", member.badge());
    assert_eq!(
        badge, "[L05] Nova \u{2014} Navigator",
        "badge must match '[L05] Nova \u{2014} Navigator' (em dash U+2014)"
    );
}

#[test]
fn badge_level_zero_padded_to_two_digits() {
    let member = call_or_hint!(
        "ex09",
        "CrewMember::new",
        CrewMember::new("Zeta", "Engineer", 1)
    );
    let badge = call_or_hint!("ex09", "CrewMember::badge", member.badge());
    assert_eq!(badge, "[L01] Zeta \u{2014} Engineer");
}

#[test]
fn badge_double_digit_level() {
    let member = call_or_hint!(
        "ex09",
        "CrewMember::new",
        CrewMember::new("Orion", "Captain", 42)
    );
    let badge = call_or_hint!("ex09", "CrewMember::badge", member.badge());
    assert_eq!(badge, "[L42] Orion \u{2014} Captain");
}

#[test]
fn badge_level_99() {
    let member = call_or_hint!(
        "ex09",
        "CrewMember::new",
        CrewMember::new("Titan", "Admiral", 99)
    );
    let badge = call_or_hint!("ex09", "CrewMember::badge", member.badge());
    assert_eq!(badge, "[L99] Titan \u{2014} Admiral");
}
