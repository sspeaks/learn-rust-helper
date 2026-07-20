#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogEvent {
    pub system: String,
    pub success: bool,
    pub code: u16,
}

pub fn summarize_failures(events: &[LogEvent], max_items: usize) -> Vec<String> {
    events
        .iter()
        .filter(|e| !e.success)
        .take(max_items)
        .map(|e| format!("{}: error {}", e.system, e.code))
        .collect()
}
