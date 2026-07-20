#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CalibrationInput {
    pub base_output: i32,
    pub drift: i32,
    pub efficiency_percent: u8,
}

pub fn calibrate_reactor(base_output: i32, drift: i32, efficiency_percent: u8) -> i32 {
    // ════════════════════════════════════════════════════════════════════════════
    // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
    // ════════════════════════════════════════════════════════════════════════════
    todo!("Apply drift and percentage efficiency to produce final reactor output")
}

pub fn calibrate_batch(inputs: &[CalibrationInput]) -> Vec<i32> {
    inputs
        .iter()
        .map(|input| calibrate_reactor(input.base_output, input.drift, input.efficiency_percent))
        .collect()
}
