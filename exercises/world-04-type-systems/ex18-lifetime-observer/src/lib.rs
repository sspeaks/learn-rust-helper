#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MissionView<'a> {
    pub code: &'a str,
    pub captain: &'a str,
}

pub fn longer_label<'a>(left: &'a str, right: &'a str) -> &'a str {
    // ══════════════════════════════════════════════════════════════
    // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
    // ══════════════════════════════════════════════════════════════
    todo!("Return the longer string slice; return left when both lengths match")
}

pub fn mission_view<'a>(code: &'a str, captain: &'a str) -> MissionView<'a> {
    // ══════════════════════════════════════════════════════════════
    // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
    // ══════════════════════════════════════════════════════════════
    todo!("Build a MissionView that borrows both input slices")
}

pub fn clipped_prefix<'a>(text: &'a str, max_len: usize) -> &'a str {
    // ══════════════════════════════════════════════════════════════
    // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
    // ══════════════════════════════════════════════════════════════
    todo!("Return text up to max_len bytes, or all text when max_len is large enough")
}
