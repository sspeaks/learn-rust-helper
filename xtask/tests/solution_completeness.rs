//! Metadata-driven solution completeness tests.
//!
//! **Tier 1** — fast, run as part of `cargo test --package xtask`:
//!   * [`every_exercise_has_nonempty_solution`] — each exercise in the
//!     production `campaign.toml` must have a non-empty `hints/solution.rs`.
//!   * [`learn_solution_command_path_succeeds`] — `learn solution <id>` must
//!     exit 0 and print the exercise title for every exercise whose solution
//!     file currently exists.
//!
//! **Tier 2** — full compile/behavior gate (run separately via CI):
//!   `./scripts/validate-solutions.sh`
//!
//! No exercise IDs, world IDs, counts, or paths are hardcoded here; all are
//! driven by the production `campaign.toml` via `xtask::load_campaign`.

mod support;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use support::learn_bin;

// ── helpers ───────────────────────────────────────────────────────────────

/// Returns the real workspace root (the parent of the xtask crate directory).
fn real_workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask crate lives inside the workspace root")
        .to_path_buf()
}

/// Full path to `hints/solution.rs` for a given world+exercise pair.
fn solution_path(root: &Path, world_id: &str, exercise_id: &str) -> PathBuf {
    root.join("exercises")
        .join(world_id)
        .join(exercise_id)
        .join("hints")
        .join("solution.rs")
}

// ── Tier 1: inventory ─────────────────────────────────────────────────────

/// Every exercise declared in `campaign.toml` must have a non-empty
/// `hints/solution.rs`.
///
/// Failures are collected and reported together so a single run reveals the
/// full gap.  The set of exercises is read from `campaign.toml` at test time,
/// so adding new exercises automatically extends coverage without any manual
/// update to this file.
#[test]
fn every_exercise_has_nonempty_solution() {
    let root = real_workspace_root();
    let campaign = xtask::load_campaign(&root)
        .expect("production campaign.toml must parse and pass validation");

    let mut failures: Vec<String> = Vec::new();

    for world in &campaign.worlds {
        for exercise in &world.exercises {
            let path = solution_path(&root, &world.id, &exercise.id);

            if !path.is_file() {
                failures.push(format!(
                    "MISSING  exercises/{}/{}/hints/solution.rs",
                    world.id, exercise.id
                ));
            } else {
                let content = fs::read_to_string(&path)
                    .unwrap_or_else(|e| panic!("I/O error reading {}: {e}", path.display()));
                if content.trim().is_empty() {
                    failures.push(format!(
                        "EMPTY    exercises/{}/{}/hints/solution.rs",
                        world.id, exercise.id
                    ));
                }
            }
        }
    }

    assert!(
        failures.is_empty(),
        "{} exercise(s) with missing or empty hints/solution.rs:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

// ── Tier 1: `learn solution <id>` CLI path ────────────────────────────────

/// `learn solution <id>` must exit 0 and print the exercise title for every
/// exercise whose `hints/solution.rs` currently exists.
///
/// A temporary workspace is built from the real `campaign.toml` so the full
/// `cmd_solution` code path (campaign load → progress check → file read →
/// stdout print) is exercised without touching any learner working-tree file.
///
/// Exercises that do not yet have a solution file are skipped here;
/// [`every_exercise_has_nonempty_solution`] enforces their eventual presence.
#[test]
fn learn_solution_command_path_succeeds() {
    use tempfile::TempDir;

    let src_root = real_workspace_root();
    let campaign = xtask::load_campaign(&src_root)
        .expect("production campaign.toml must parse and pass validation");

    // ── Build isolated temp workspace ─────────────────────────────────

    let tmpdir = TempDir::new().expect("create temp dir");
    let tmp = tmpdir.path();

    // Workspace Cargo.toml — discover_workspace_root requires a [workspace]
    // table whose members list includes "xtask".
    fs::write(
        tmp.join("Cargo.toml"),
        "[workspace]\nresolver = \"2\"\nmembers = [\"xtask\"]\n",
    )
    .expect("write workspace Cargo.toml");

    // campaign.toml — verbatim copy of the real metadata.
    fs::copy(src_root.join("campaign.toml"), tmp.join("campaign.toml"))
        .expect("copy campaign.toml");

    // Minimal xtask stub so discover_workspace_root can verify the [workspace]
    // members entry, even though the binary under test is the real compiled one.
    fs::create_dir_all(tmp.join("xtask")).expect("create xtask stub dir");
    fs::write(
        tmp.join("xtask").join("Cargo.toml"),
        "[package]\nname = \"xtask\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    )
    .expect("write xtask stub Cargo.toml");

    // Progress file — every exercise marked completed so cmd_solution does not
    // return ExerciseIncomplete.
    let completed_toml: String = {
        let ids: Vec<String> = campaign
            .worlds
            .iter()
            .flat_map(|w| w.exercises.iter().map(|e| format!("\"{}\"", e.id)))
            .collect();
        let total_xp: u32 = campaign
            .worlds
            .iter()
            .flat_map(|w| w.exercises.iter().map(|e| e.xp))
            .sum();
        format!(
            "schema_version = 1\nearned_xp = {}\ncompleted = [{}]\n",
            total_xp,
            ids.join(", ")
        )
    };
    let lr_dir = tmp.join(".learn-rust");
    fs::create_dir_all(&lr_dir).expect("create .learn-rust dir");
    fs::write(lr_dir.join("progress.toml"), &completed_toml).expect("write progress.toml");

    // Exercise stubs — validate_campaign checks that each exercise has a
    // Cargo.toml.  Also copy any existing solution.rs into the temp workspace.
    let mut exercises_with_solutions: Vec<(String, String)> = Vec::new();

    for world in &campaign.worlds {
        for exercise in &world.exercises {
            let ex_dir = tmp.join("exercises").join(&world.id).join(&exercise.id);
            let hints_dir = ex_dir.join("hints");
            fs::create_dir_all(&hints_dir).expect("create exercise hints dir");

            // Minimal Cargo.toml — validate_campaign only checks file existence.
            fs::write(
                ex_dir.join("Cargo.toml"),
                format!(
                    "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
                    exercise.id
                ),
            )
            .expect("write exercise Cargo.toml stub");

            let real_sol = solution_path(&src_root, &world.id, &exercise.id);
            if real_sol.is_file() {
                fs::copy(&real_sol, hints_dir.join("solution.rs"))
                    .expect("copy solution.rs into temp workspace");
                exercises_with_solutions.push((exercise.id.clone(), exercise.title.clone()));
            }
        }
    }

    // ── Run `learn solution <id>` for each present solution ───────────

    let mut failures: Vec<String> = Vec::new();

    for (ex_id, title) in &exercises_with_solutions {
        let out = Command::new(learn_bin())
            .args(["solution", ex_id.as_str()])
            .current_dir(tmp)
            .output()
            .unwrap_or_else(|e| panic!("failed to spawn learn binary: {e}"));

        let stdout = String::from_utf8_lossy(&out.stdout);
        let stderr = String::from_utf8_lossy(&out.stderr);

        if !out.status.success() {
            failures.push(format!(
                "{ex_id}: exited {:?}\n  stderr: {}",
                out.status.code(),
                stderr.trim()
            ));
        } else if !stdout.contains(title.as_str()) {
            failures.push(format!(
                "{ex_id}: stdout did not contain title {title:?}\n  stdout: {}",
                stdout.trim()
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "{} exercise(s) failed `learn solution` path:\n\n{}",
        failures.len(),
        failures.join("\n\n")
    );
}
