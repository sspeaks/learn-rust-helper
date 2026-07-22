use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SensorReading {
    pub sensor_id: String,
    pub raw_value: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalibratedReading {
    pub sensor_id: String,
    pub calibrated_value: i64,
}

pub fn calibrate_readings_parallel(
    readings: &[SensorReading],
    offset: i64,
) -> Vec<CalibratedReading> {
    // ══════════════════════════════════════════════════════════════
    // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
    // ══════════════════════════════════════════════════════════════
    todo!("Use rayon parallel iteration to calibrate each reading")
}

pub fn total_calibrated_power_parallel(readings: &[CalibratedReading]) -> i64 {
    // ══════════════════════════════════════════════════════════════
    // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
    // ══════════════════════════════════════════════════════════════
    todo!("Use rayon to sum calibrated values in parallel")
}
