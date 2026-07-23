#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TelemetryBuffer {
    pub label: String,
    pub capacity: usize,
    pub readings: Vec<i32>,
}

impl TelemetryBuffer {
    pub fn with_capacity(label: impl Into<String>, capacity: usize) -> Self {
        // ══════════════════════════════════════════════════════════════
        // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
        // ══════════════════════════════════════════════════════════════
        todo!("Build a new TelemetryBuffer with an empty readings list")
    }

    pub fn record(&mut self, reading: i32) -> bool {
        // ══════════════════════════════════════════════════════════════
        // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
        // ══════════════════════════════════════════════════════════════
        todo!("Append reading when capacity allows and report success")
    }

    pub fn average(&self) -> Option<f64> {
        // ══════════════════════════════════════════════════════════════
        // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
        // ══════════════════════════════════════════════════════════════
        todo!("Return None for empty buffers, otherwise return the arithmetic mean")
    }

    pub fn into_report(self) -> String {
        // ══════════════════════════════════════════════════════════════
        // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
        // ══════════════════════════════════════════════════════════════
        todo!("Consume the buffer and return 'label:count@average' or 'label:0@n/a'")
    }
}
