#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TelemetryBuffer {
    pub label: String,
    pub capacity: usize,
    pub readings: Vec<i32>,
}

impl TelemetryBuffer {
    pub fn with_capacity(label: impl Into<String>, capacity: usize) -> Self {
        Self { label: label.into(), capacity, readings: Vec::new() }
    }

    pub fn record(&mut self, reading: i32) -> bool {
        if self.readings.len() < self.capacity {
            self.readings.push(reading);
            return true;
        } else { 
            return false;
        }
    }

    pub fn average(&self) -> Option<f64> {
        let len = self.readings.len();
        if len == 0 { return None; }
        if len > 2^53-1 {
            panic!("how did we overflow on len!?!");
        }
        let sum: i32 = self.readings.iter().sum();
        Some(f64::try_from(sum).expect("Sum shouldn't overflow for simple example")
            / len as f64)
    }

    pub fn into_report(self) -> String {
        if self.readings.is_empty() {
            format!("{}:0@n/a", self.label)
        } else {
            format!("{}:{}@{:.1}", self.label, self.readings.len(), self.average().expect("already checked len"))
        }
    }
}
