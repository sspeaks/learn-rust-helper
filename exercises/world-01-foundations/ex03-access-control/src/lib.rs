#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Clearance {
    Visitor,
    Engineer,
    Captain,
}

pub fn access_message(clearance: Clearance, is_on_duty: bool) -> &'static str {
    // ════════════════════════════════════════════════════════════════════════════
    // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
    // ════════════════════════════════════════════════════════════════════════════
    todo!("Use if and match to map each clearance/duty pair to a gate message")
}

pub fn gate_announcement(name: &str, clearance: Clearance, is_on_duty: bool) -> String {
    format!("{name}: {}", access_message(clearance, is_on_duty))
}
