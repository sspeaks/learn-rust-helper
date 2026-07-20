#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Turret {
    pub callsign: String,
    pub charge: i32,
    pub overheated: bool,
}

pub fn rebalance_turrets(turrets: &mut [Turret], emergency_boost: i32) {
    // ════════════════════════════════════════════════════════════════════════════
    // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
    // ════════════════════════════════════════════════════════════════════════════
    todo!("Mutably borrow each turret and adjust charge/overheat state in place")
}
