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
    readings
        .par_iter()
        .map(|reading| CalibratedReading {
            sensor_id: reading.sensor_id.clone(),
            calibrated_value: reading.raw_value + offset,
        })
        .collect()
}

pub fn total_calibrated_power_parallel(readings: &[CalibratedReading]) -> i64 {
    readings
        .par_iter()
        .map(|reading| reading.calibrated_value)
        .sum()
}
