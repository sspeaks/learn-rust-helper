#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogEvent {
    pub system: String,
    pub success: bool,
    pub code: u16,
}

pub fn summarize_failures(events: &[LogEvent], max_items: usize) -> Vec<String> {
    // ════════════════════════════════════════════════════════════════════════════
    // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
    // ════════════════════════════════════════════════════════════════════════════
    todo!("Use iterator adaptors to collect up to max_items failure summaries")
}
