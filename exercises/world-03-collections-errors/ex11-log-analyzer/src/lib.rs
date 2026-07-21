#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogEvent {
    pub system: String,
    pub success: bool,
    pub code: u16,
}

pub fn summarize_failures(events: &[LogEvent], max_items: usize) -> Vec<String> {
    events.iter()
        .filter(|log| !log.success)
        .take(max_items)
        .map(|log| format!("{}: error code {}", log.system, log.code))
        .collect()
}
