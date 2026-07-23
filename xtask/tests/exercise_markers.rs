use std::path::{Path, PathBuf};

// ── helpers ──────────────────────────────────────────────────────────────────

/// Collect every `exercises/<world>/<exercise>/src/lib.rs` in the real tree.
fn collect_exercise_lib_rs() -> Vec<PathBuf> {
    let exercises_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask crate has a parent directory (the workspace root)")
        .join("exercises");

    let mut files = Vec::new();
    for world_entry in std::fs::read_dir(&exercises_root).expect("exercises/ directory must exist")
    {
        let world_dir = world_entry.expect("read world dir entry").path();
        if !world_dir.is_dir() {
            continue;
        }
        for ex_entry in std::fs::read_dir(&world_dir).expect("read exercise dir entries") {
            let ex_dir = ex_entry.expect("read exercise entry").path();
            let lib_rs = ex_dir.join("src").join("lib.rs");
            if lib_rs.is_file() {
                files.push(lib_rs);
            }
        }
    }
    files.sort();
    files
}

/// True when `s` (already trimmed) is a banner rule line:
/// `// ` followed by one or more `═` characters and nothing else.
fn is_rule_line(s: &str) -> bool {
    if let Some(rest) = s.strip_prefix("// ") {
        !rest.is_empty() && rest.chars().all(|c| c == '═')
    } else {
        false
    }
}

const MISSION_LINE: &str = "// 🚀 YOUR MISSION: Replace the todo!() below with your solution.";

/// True when the three lines immediately before `lines[todo_idx]` form the
/// complete banner, allowing any leading indentation.
fn has_banner(lines: &[&str], todo_idx: usize) -> bool {
    if todo_idx < 3 {
        return false;
    }
    let l1 = lines[todo_idx - 3].trim();
    let l2 = lines[todo_idx - 2].trim();
    let l3 = lines[todo_idx - 1].trim();
    is_rule_line(l1) && l2 == MISSION_LINE && is_rule_line(l3)
}

// ── test ─────────────────────────────────────────────────────────────────────

/// Every `todo!()` in every exercise `src/lib.rs` must be immediately preceded
/// by the complete three-line YOUR MISSION banner.  Expects exactly 37 files;
/// the number of remaining `todo!()` stubs may be zero as learners make progress.
#[test]
fn every_exercise_has_mission_banner_for_each_todo() {
    let files = collect_exercise_lib_rs();
    assert_eq!(
        files.len(),
        37,
        "expected exactly 37 exercise src/lib.rs files; found {}",
        files.len()
    );

    let mut failures: Vec<String> = Vec::new();

    for file in &files {
        let content = std::fs::read_to_string(file)
            .unwrap_or_else(|e| panic!("failed to read {}: {e}", file.display()));
        let lines: Vec<&str> = content.lines().collect();

        for (i, line) in lines.iter().enumerate() {
            if !line.trim().starts_with("todo!(") {
                continue;
            }
            if !has_banner(&lines, i) {
                failures.push(format!(
                    "{}:{}: todo!() is missing the immediately preceding \
                     3-line YOUR MISSION banner",
                    file.display(),
                    i + 1,
                ));
            }
        }
    }

    if !failures.is_empty() {
        panic!(
            "{} todo!() call(s) missing a complete banner:\n{}",
            failures.len(),
            failures.join("\n"),
        );
    }
}
