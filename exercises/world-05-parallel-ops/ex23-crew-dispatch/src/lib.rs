#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrewTask {
    pub crew_id: String,
    pub units: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrewTaskResult {
    pub crew_id: String,
    pub delivered_units: u32,
}

#[derive(Debug)]
pub enum CrewDispatchError {
    WorkerPanicked,
}

pub fn run_crew_dispatch(tasks: Vec<CrewTask>) -> Result<Vec<CrewTaskResult>, CrewDispatchError> {
    // ══════════════════════════════════════════════════════════════
    // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
    // ══════════════════════════════════════════════════════════════
    todo!("Spawn one thread per task, join them, and return deterministic output order")
}
