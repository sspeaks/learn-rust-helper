use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SampleWindow<T> {
    pub source: String,
    pub samples: Vec<T>,
}

pub fn newest_sample<T: Clone>(window: &SampleWindow<T>) -> Option<T> {
    // ══════════════════════════════════════════════════════════════
    // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
    // ══════════════════════════════════════════════════════════════
    todo!("Return a cloned copy of the last sample, or None when empty")
}

pub fn strongest_sample<T: PartialOrd + Copy>(window: &SampleWindow<T>) -> Option<T> {
    // ══════════════════════════════════════════════════════════════
    // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
    // ══════════════════════════════════════════════════════════════
    todo!("Return the strongest sample, choosing the first value when tied")
}

pub fn format_window<T: Display>(window: &SampleWindow<T>) -> String {
    // ══════════════════════════════════════════════════════════════
    // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
    // ══════════════════════════════════════════════════════════════
    todo!("Format as '<source> [v1, v2, ...]' and '<source> []' when empty")
}
