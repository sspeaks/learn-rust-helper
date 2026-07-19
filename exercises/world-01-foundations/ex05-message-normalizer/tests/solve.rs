use ex05_message_normalizer::{normalize_call_sign, normalize_manifest};

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

// ── normalize_call_sign ──────────────────────────────────────────────────────
//
// Call-sign normalization contract (from docstring):
//   • Trim leading and trailing whitespace.
//   • Collapse runs of internal whitespace to a single space.
//   • Normalize casing to UPPERCASE (call signs are conventionally uppercase).

#[test]
fn empty_input_returns_empty() {
    let result = call_or_hint!("ex05", "normalize_call_sign", normalize_call_sign(""));
    assert_eq!(result, "", "empty input must produce empty output");
}

#[test]
fn all_whitespace_returns_empty() {
    let result = call_or_hint!("ex05", "normalize_call_sign", normalize_call_sign("   "));
    assert_eq!(result, "", "all-whitespace input must produce empty output");
}

#[test]
fn trims_leading_and_trailing_spaces() {
    let result = call_or_hint!(
        "ex05",
        "normalize_call_sign",
        normalize_call_sign("  ALPHA  ")
    );
    assert_eq!(
        result, "ALPHA",
        "leading and trailing spaces must be trimmed"
    );
}

#[test]
fn collapses_internal_spaces() {
    let result = call_or_hint!(
        "ex05",
        "normalize_call_sign",
        normalize_call_sign("ALPHA  SEVEN")
    );
    assert_eq!(
        result, "ALPHA SEVEN",
        "multiple internal spaces collapse to one"
    );
}

#[test]
fn lowercased_input_is_uppercased() {
    let result = call_or_hint!("ex05", "normalize_call_sign", normalize_call_sign("echo"));
    assert_eq!(result, "ECHO", "lowercase letters must be uppercased");
}

#[test]
fn mixed_case_and_extra_spaces() {
    let result = call_or_hint!(
        "ex05",
        "normalize_call_sign",
        normalize_call_sign("  delta  bravo  ")
    );
    assert_eq!(
        result, "DELTA BRAVO",
        "trim + collapse spaces + uppercase must all apply"
    );
}

#[test]
fn already_normalized_unchanged() {
    let result = call_or_hint!(
        "ex05",
        "normalize_call_sign",
        normalize_call_sign("SIERRA TANGO")
    );
    assert_eq!(
        result, "SIERRA TANGO",
        "already-normalized input is preserved"
    );
}

// ── normalize_manifest ────────────────────────────────────────────────────────

#[test]
fn manifest_normalizes_each_entry() {
    let input = ["  alpha  ", "BRAVO", "  charlie delta  "];
    let result = call_or_hint!("ex05", "normalize_call_sign", normalize_manifest(&input));
    assert_eq!(result, vec!["ALPHA", "BRAVO", "CHARLIE DELTA"]);
}

#[test]
fn manifest_empty_slice() {
    let result = call_or_hint!("ex05", "normalize_call_sign", normalize_manifest(&[]));
    assert!(result.is_empty());
}
