use ex13_mission_lookup::{active_reward_for_code, reward_or_default, Mission};

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

fn mission(code: &str, reward: u32, active: bool) -> Mission {
    Mission {
        code: code.to_string(),
        reward,
        active,
    }
}

// ── active_reward_for_code ────────────────────────────────────────────────────

#[test]
fn active_mission_returns_some_reward() {
    let missions = vec![mission("ALPHA", 500, true)];
    let result = call_or_hint!(
        "ex13",
        "active_reward_for_code",
        active_reward_for_code(&missions, "ALPHA")
    );
    assert_eq!(result, Some(500), "active mission must return Some(reward)");
}

#[test]
fn inactive_mission_returns_none() {
    let missions = vec![mission("BETA", 300, false)];
    let result = call_or_hint!(
        "ex13",
        "active_reward_for_code",
        active_reward_for_code(&missions, "BETA")
    );
    assert_eq!(result, None, "inactive mission must return None");
}

#[test]
fn missing_code_returns_none() {
    let missions = vec![mission("GAMMA", 200, true)];
    let result = call_or_hint!(
        "ex13",
        "active_reward_for_code",
        active_reward_for_code(&missions, "DELTA")
    );
    assert_eq!(result, None, "non-existent code must return None");
}

#[test]
fn empty_missions_returns_none() {
    let result = call_or_hint!(
        "ex13",
        "active_reward_for_code",
        active_reward_for_code(&[], "ALPHA")
    );
    assert_eq!(result, None, "empty slice always returns None");
}

#[test]
fn multiple_missions_correct_one_matched() {
    let missions = vec![
        mission("ALPHA", 100, true),
        mission("BETA", 200, true),
        mission("GAMMA", 300, false),
    ];
    let result = call_or_hint!(
        "ex13",
        "active_reward_for_code",
        active_reward_for_code(&missions, "BETA")
    );
    assert_eq!(
        result,
        Some(200),
        "must return reward for the matching code"
    );
}

#[test]
fn only_active_flag_determines_result_not_just_presence() {
    let missions = vec![
        mission("OMEGA", 999, false), // exists but inactive
    ];
    let result = call_or_hint!(
        "ex13",
        "active_reward_for_code",
        active_reward_for_code(&missions, "OMEGA")
    );
    assert_eq!(
        result, None,
        "inactive mission (active=false) must return None even if code matches"
    );
}

// ── reward_or_default ─────────────────────────────────────────────────────────

#[test]
fn reward_or_default_returns_reward_when_active() {
    let missions = vec![mission("ALPHA", 500, true)];
    let val = call_or_hint!(
        "ex13",
        "active_reward_for_code",
        reward_or_default(&missions, "ALPHA", 0)
    );
    assert_eq!(val, 500);
}

#[test]
fn reward_or_default_returns_default_when_inactive() {
    let missions = vec![mission("BETA", 300, false)];
    let val = call_or_hint!(
        "ex13",
        "active_reward_for_code",
        reward_or_default(&missions, "BETA", 42)
    );
    assert_eq!(val, 42, "inactive mission → default_reward is returned");
}

#[test]
fn reward_or_default_returns_default_when_missing() {
    let val = call_or_hint!(
        "ex13",
        "active_reward_for_code",
        reward_or_default(&[], "GHOST", 99)
    );
    assert_eq!(val, 99, "missing code → default_reward is returned");
}
