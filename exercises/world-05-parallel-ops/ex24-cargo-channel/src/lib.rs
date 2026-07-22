#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CargoJob {
    pub cargo_id: String,
    pub destination: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CargoReceipt {
    pub cargo_id: String,
    pub delivered_to: String,
}

#[derive(Debug)]
pub enum CargoChannelError {
    SendFailed,
    ReceiveFailed,
    WorkerPanicked,
}

pub fn dispatch_cargo_jobs(jobs: Vec<CargoJob>) -> Result<Vec<CargoReceipt>, CargoChannelError> {
    // ══════════════════════════════════════════════════════════════
    // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
    // ══════════════════════════════════════════════════════════════
    todo!("Use std::sync::mpsc channels to move job results from worker threads")
}
