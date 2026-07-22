mod support;

use std::path::Path;
use std::process::Command;

use support::{assert_contains, assert_not_contains, TempWorkspace};

// ── 1. no_args_dashboard ────────────────────────────────────────────────────

#[test]
fn no_args_dashboard() {
    let ws = TempWorkspace::new();
    let out = ws.run(&[]);
    assert!(out.status.success(), "exit code should be 0");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_contains(&stdout, "Test Campaign");
    assert_contains(&stdout, "Cadet");
    assert_contains(&stdout, "XP");
    assert_contains(&stdout, "Alpha World");
    assert_contains(&stdout, "Beta World");
    assert_contains(&stdout, "Gamma World");
    assert_contains(&stdout, "learn check");
    assert_contains(&stdout, "learn hint");
    assert_contains(&stdout, "learn solution");
}

// ── 2. status_fresh ─────────────────────────────────────────────────────────

#[test]
fn status_fresh() {
    let ws = TempWorkspace::new();
    let out = ws.run(&["status"]);
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_contains(&stdout, "XP: 0");
    assert_contains(&stdout, "Cadet");
    assert_contains(&stdout, "Alpha World");
    assert_contains(&stdout, "0/5 complete");
}

// ── 3. next_fresh ───────────────────────────────────────────────────────────

#[test]
fn next_fresh() {
    let ws = TempWorkspace::new();
    let out = ws.run(&["next"]);
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    // First exercise with no prerequisites should be recommended.
    assert_contains(&stdout, "ex01-alpha");
}

// ── 4. check_success ────────────────────────────────────────────────────────

#[test]
fn check_success() {
    let ws = TempWorkspace::new();
    let out = ws.run_with_fake_cargo(&["check", "ex01-alpha"], 0);
    assert!(
        out.status.success(),
        "exit code should be 0 on passing tests"
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_contains(&stdout, "ex01-alpha");
    assert_contains(&stdout, "verified");
    assert_contains(&stdout, "XP");
    assert_contains(&stdout, "learn solution");
    // Progress file should now exist.
    assert!(
        ws.progress_path().exists(),
        "progress file should be created"
    );
}

// ── 5. check_default_id ─────────────────────────────────────────────────────

#[test]
fn check_default_id() {
    let ws = TempWorkspace::new();
    // With no ID, check should default to the recommended next exercise.
    let out = ws.run_with_fake_cargo(&["check"], 0);
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    // ex01-alpha is the first recommended exercise.
    assert_contains(&stdout, "ex01-alpha");
    assert_contains(&stdout, "verified");
}

// ── 6. check_failure ────────────────────────────────────────────────────────

#[test]
fn check_failure() {
    let ws = TempWorkspace::new();
    let out = ws.run_with_fake_cargo(&["check", "ex01-alpha"], 1);
    assert!(
        !out.status.success(),
        "exit code should be nonzero on failing tests"
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    // Actionable hint nudge must appear in stderr.
    assert_contains(&stderr, "learn hint");
    assert_contains(&stderr, "ex01-alpha");
    // No redundant "error:" line should be emitted.
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_not_contains(&stdout, "error: tests failed");
}

// ── 7. check_idempotent ─────────────────────────────────────────────────────

#[test]
fn check_idempotent() {
    let ws = TempWorkspace::new();
    // Pre-mark ex01-alpha as completed.
    ws.set_progress(
        r#"schema_version = 1
earned_xp = 10
completed = ["ex01-alpha"]
"#,
    );
    let out = ws.run_with_fake_cargo(&["check", "ex01-alpha"], 0);
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_contains(&stdout, "verified again");
    assert_contains(&stdout, "XP unchanged");
    // Repeat completion must NOT re-advertise the solution command.
    assert_not_contains(&stdout, "learn solution");
}

// ── 8. hint_default_level ───────────────────────────────────────────────────

#[test]
fn hint_default_level() {
    let ws = TempWorkspace::new();
    // No prior hints viewed → should show hint 1.
    let out = ws.run(&["hint", "ex01-alpha"]);
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_contains(&stdout, "Hint 1");
}

// ── 9. hint_auto_increment ──────────────────────────────────────────────────

#[test]
fn hint_auto_increment() {
    let ws = TempWorkspace::new();
    // First call → hint 1.
    let out1 = ws.run(&["hint", "ex01-alpha"]);
    assert!(out1.status.success());
    let stdout1 = String::from_utf8_lossy(&out1.stdout);
    assert_contains(&stdout1, "Hint 1");

    // Second call → auto-advances to hint 2.
    let out2 = ws.run(&["hint", "ex01-alpha"]);
    assert!(out2.status.success());
    let stdout2 = String::from_utf8_lossy(&out2.stdout);
    assert_contains(&stdout2, "Hint 2");
}

// ── 10. hint_explicit_level ─────────────────────────────────────────────────

#[test]
fn hint_explicit_level() {
    let ws = TempWorkspace::new();
    let out = ws.run(&["hint", "ex01-alpha", "--level", "3"]);
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_contains(&stdout, "Hint 3");
}

// ── 11. hint_unknown_id ─────────────────────────────────────────────────────

#[test]
fn hint_unknown_id() {
    let ws = TempWorkspace::new();
    let out = ws.run(&["hint", "ex99-unknown"]);
    assert!(
        !out.status.success(),
        "exit code should be 1 for unknown ID"
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_contains(&stderr, "unknown exercise id");
    assert_contains(&stderr, "ex99-unknown");
}

// ── 12. hint_invalid_level ──────────────────────────────────────────────────

#[test]
fn hint_invalid_level() {
    let ws = TempWorkspace::new();
    let out = ws.run(&["hint", "ex01-alpha", "--level", "5"]);
    assert!(!out.status.success(), "exit code should be 1 for level > 3");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_contains(&stderr, "hint level must be 1, 2, or 3");
}

// ── 13. missing_campaign ────────────────────────────────────────────────────

#[test]
fn missing_campaign() {
    let ws = TempWorkspace::new();
    // Remove the campaign.toml to trigger the MissingCampaign error.
    std::fs::remove_file(ws.root().join("campaign.toml")).expect("remove campaign.toml");
    let out = ws.run(&["status"]);
    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_contains(&stderr, "campaign metadata is missing");
}

// ── 14. progress_compat ─────────────────────────────────────────────────────

#[test]
fn progress_compat() {
    let ws = TempWorkspace::new();
    // Old-format progress file without hints_viewed field.
    ws.set_progress(
        r#"schema_version = 1
earned_xp = 10
completed = ["ex01-alpha"]
"#,
    );
    // status should succeed without any parse error.
    let out = ws.run(&["status"]);
    assert!(
        out.status.success(),
        "old progress format should deserialize cleanly"
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_contains(&stdout, "XP: 10");
}

// ── 15. mid_campaign_status ─────────────────────────────────────────────────

#[test]
fn mid_campaign_status() {
    let ws = TempWorkspace::new();
    ws.set_progress(
        r#"schema_version = 1
earned_xp = 30
completed = ["ex01-alpha", "ex02-beta", "ex03-gamma"]
"#,
    );
    let out = ws.run(&["status"]);
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_contains(&stdout, "XP: 30");
    // Alpha World should show 3/5 complete.
    assert_contains(&stdout, "3/5 complete");
}

// ── 16. all_complete ────────────────────────────────────────────────────────

#[test]
fn all_complete() {
    let ws = TempWorkspace::new();
    let all_ids = [
        "ex01-alpha",
        "ex02-beta",
        "ex03-gamma",
        "ex04-delta",
        "ex05-epsilon",
        "ex06-zeta",
        "ex07-eta",
        "ex08-theta",
        "ex09-iota",
        "ex10-kappa",
        "ex11-lambda",
        "ex12-mu",
        "ex13-nu",
        "ex14-xi",
        "ex15-omicron",
    ];
    let completed_list = all_ids
        .iter()
        .map(|id| format!("\"{id}\""))
        .collect::<Vec<_>>()
        .join(", ");
    ws.set_progress(&format!(
        "schema_version = 1\nearned_xp = 150\ncompleted = [{completed_list}]\n"
    ));

    // learn next should report all complete.
    let next_out = ws.run(&["next"]);
    assert!(next_out.status.success());
    let next_stdout = String::from_utf8_lossy(&next_out.stdout);
    assert_contains(&next_stdout, "All exercises are complete");

    // learn check (no id) should report nothing left.
    let check_out = ws.run_with_fake_cargo(&["check"], 0);
    assert!(check_out.status.success());
    let check_stdout = String::from_utf8_lossy(&check_out.stdout);
    assert_contains(&check_stdout, "All exercises complete");
}

// ── 17. cargo_xtask_compat ──────────────────────────────────────────────────

#[test]
fn cargo_xtask_compat() {
    let ws = TempWorkspace::new();
    // `learn verify <id>` is the legacy alias for `learn check <id>`.
    let out = ws.run_with_fake_cargo(&["verify", "ex01-alpha"], 0);
    assert!(out.status.success(), "legacy verify command should succeed");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_contains(&stdout, "ex01-alpha");
    assert_contains(&stdout, "verified");
    assert_contains(&stdout, "XP");
}

// ── 18. dashboard_shows_absolute_edit_path ──────────────────────────────────

#[test]
fn dashboard_shows_absolute_edit_path() {
    let ws = TempWorkspace::new();
    let out = ws.run(&[]);
    assert!(out.status.success(), "dashboard should exit 0");
    let stdout = String::from_utf8_lossy(&out.stdout);

    // The fixture campaign's first exercise is ex01-alpha in world-01-alpha.
    // The subprocess calls current_dir() after chdir(), which on macOS returns
    // the physical (symlink-resolved) path.  Canonicalize the temp root to match.
    let expected_path = ws
        .root()
        .canonicalize()
        .expect("temp dir should be canonicalizable")
        .join("exercises")
        .join("world-01-alpha")
        .join("ex01-alpha")
        .join("src")
        .join("lib.rs");
    let expected_line = format!("  📂 Edit: {}", expected_path.display());
    assert_contains(&stdout, &expected_line);
}

// ── 19. solution_completed ───────────────────────────────────────────────────

#[test]
fn solution_completed() {
    let ws = TempWorkspace::new();
    ws.set_progress(
        r#"schema_version = 1
earned_xp = 10
completed = ["ex01-alpha"]
"#,
    );
    ws.set_solution(
        "world-01-alpha",
        "ex01-alpha",
        "fn answer() -> u32 { 42 }\n",
    );
    let out = ws.run(&["solution", "ex01-alpha"]);
    assert!(
        out.status.success(),
        "exit code should be 0 for completed exercise with solution file"
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_contains(&stdout, "📖 Reference Solution");
    assert_contains(&stdout, "ex01-alpha");
    // Structural framing: separator lines must bracket the content.
    assert_contains(&stdout, "────");
    // Exact file bytes are echoed.
    assert_contains(&stdout, "fn answer() -> u32 { 42 }");
    // Footer note.
    assert_contains(&stdout, "Note: This is one idiomatic approach");
}

// ── 20. solution_incomplete ──────────────────────────────────────────────────

#[test]
fn solution_incomplete() {
    let ws = TempWorkspace::new();
    // No progress written → ex01-alpha is not completed.
    let out = ws.run(&["solution", "ex01-alpha"]);
    assert!(
        !out.status.success(),
        "exit code should be nonzero for incomplete exercise"
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_contains(&stderr, "complete ex01-alpha");
    assert_contains(&stderr, "learn check ex01-alpha");
}

// ── 21. solution_unknown_id ──────────────────────────────────────────────────

#[test]
fn solution_unknown_id() {
    let ws = TempWorkspace::new();
    let out = ws.run(&["solution", "ex99-unknown"]);
    assert!(
        !out.status.success(),
        "exit code should be nonzero for unknown exercise ID"
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_contains(&stderr, "unknown exercise id");
    assert_contains(&stderr, "ex99-unknown");
}

// ── 22. solution_missing_file ────────────────────────────────────────────────

#[test]
fn solution_missing_file() {
    let ws = TempWorkspace::new();
    ws.set_progress(
        r#"schema_version = 1
earned_xp = 10
completed = ["ex01-alpha"]
"#,
    );
    // Exercise is completed but no solution.rs has been written.
    let out = ws.run(&["solution", "ex01-alpha"]);
    assert!(
        !out.status.success(),
        "exit code should be nonzero when solution file is missing"
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_contains(&stderr, "reference solution not yet available");
    // Error must show a relative path (not an absolute temp path).
    assert_contains(
        &stderr,
        "exercises/world-01-alpha/ex01-alpha/hints/solution.rs",
    );
}

// ── 23. production_campaign_status_succeeds ──────────────────────────────────

/// Regression guard: the binary compiled from current source must accept the
/// production 6-world/34-exercise campaign without any hard-coded world-count or
/// exercise-count invariant (the stale Nix binary enforced "exactly 3 worlds" and
/// "exactly 15 exercises", which broke `learn check` after the campaign was expanded).
#[test]
fn production_campaign_status_succeeds() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask crate should have workspace parent");

    let out = Command::new(support::learn_bin())
        .arg("status")
        .current_dir(root)
        .output()
        .expect("failed to run learn status against production campaign");

    assert!(
        out.status.success(),
        "learn status must succeed against the production 6-world campaign:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    // World 1 (Foundations, 5 exercises) and world 6 (Archive Core, 6 exercises)
    // must both appear, proving no fixed 3-world ceiling is enforced.
    assert_contains(&stdout, "Foundations");
    assert_contains(&stdout, "Archive Core");
}
