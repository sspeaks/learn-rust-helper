use ex01_format_scoreboard::{format_scoreboard_line, render_scoreboard, ScoreEntry};

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

// ── format_scoreboard_line ──────────────────────────────────────────────────

#[test]
fn line_rank_one_positive_score() {
    let line = call_or_hint!(
        "ex01",
        "format_scoreboard_line",
        format_scoreboard_line("Nova", 42, 1)
    );
    assert_eq!(
        line, "#01 | Nova | +0042",
        "rank=1, score=42 should produce '#01 | Nova | +0042'"
    );
}

#[test]
fn line_negative_score() {
    let line = call_or_hint!(
        "ex01",
        "format_scoreboard_line",
        format_scoreboard_line("Zeta", -7, 2)
    );
    assert_eq!(
        line, "#02 | Zeta | -0007",
        "negative score should use '-' sign with zero-padding"
    );
}

#[test]
fn line_zero_score() {
    let line = call_or_hint!(
        "ex01",
        "format_scoreboard_line",
        format_scoreboard_line("Alpha", 0, 3)
    );
    assert_eq!(
        line, "#03 | Alpha | +0000",
        "zero score should display as '+0000'"
    );
}

#[test]
fn line_rank_double_digit() {
    let line = call_or_hint!(
        "ex01",
        "format_scoreboard_line",
        format_scoreboard_line("Rex", 100, 10)
    );
    assert_eq!(
        line, "#10 | Rex | +0100",
        "double-digit rank should not be padded further"
    );
}

#[test]
fn line_large_score() {
    let line = call_or_hint!(
        "ex01",
        "format_scoreboard_line",
        format_scoreboard_line("Iris", 9999, 1)
    );
    assert_eq!(line, "#01 | Iris | +9999");
}

#[test]
fn line_negative_large_score() {
    let line = call_or_hint!(
        "ex01",
        "format_scoreboard_line",
        format_scoreboard_line("Kira", -9999, 5)
    );
    assert_eq!(line, "#05 | Kira | -9999");
}

// ── render_scoreboard ───────────────────────────────────────────────────────

#[test]
fn render_two_entries_joined_by_newline() {
    let entries = vec![
        ScoreEntry::new("Nova", 42, 3),
        ScoreEntry::new("Zeta", -7, 1),
    ];
    let board = call_or_hint!(
        "ex01",
        "format_scoreboard_line",
        render_scoreboard(&entries)
    );
    let lines: Vec<&str> = board.split('\n').collect();
    assert_eq!(lines.len(), 2, "two entries should produce two lines");
    assert_eq!(lines[0], "#01 | Nova | +0042", "first entry is rank 1");
    assert_eq!(lines[1], "#02 | Zeta | -0007", "second entry is rank 2");
}

#[test]
fn render_single_entry() {
    let entries = vec![ScoreEntry::new("Solo", 1, 0)];
    let board = call_or_hint!(
        "ex01",
        "format_scoreboard_line",
        render_scoreboard(&entries)
    );
    assert!(!board.contains('\n'), "single entry should have no newline");
    assert_eq!(board, "#01 | Solo | +0001");
}

#[test]
fn render_empty_slice_produces_empty_string() {
    let board = call_or_hint!("ex01", "format_scoreboard_line", render_scoreboard(&[]));
    assert_eq!(board, "", "empty slice renders empty string");
}
