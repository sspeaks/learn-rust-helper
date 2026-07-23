#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TelemetryBuffer {
    pub label: String,
    pub capacity: usize,
    pub readings: Vec<i32>,
}

impl TelemetryBuffer {
    pub fn with_capacity(label: impl Into<String>, capacity: usize) -> Self {
        Self {
            label: label.into(),
            capacity,
            readings: Vec::new(),
        }
    }

    pub fn record(&mut self, reading: i32) -> bool {
        if self.readings.len() >= self.capacity {
            return false;
        }

        self.readings.push(reading);
        true
    }

    pub fn average(&self) -> Option<f64> {
        if self.readings.is_empty() {
            return None;
        }

        let total: i32 = self.readings.iter().sum();
        Some(total as f64 / self.readings.len() as f64)
    }

    pub fn into_report(self) -> String {
        if self.readings.is_empty() {
            return format!("{}:0@n/a", self.label);
        }

        let total: i32 = self.readings.iter().sum();
        let avg = total as f64 / self.readings.len() as f64;
        format!("{}:{}@{avg:.1}", self.label, self.readings.len())
    }
}
