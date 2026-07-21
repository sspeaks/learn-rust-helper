#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Turret {
    pub callsign: String,
    pub charge: i32,
    pub overheated: bool,
}

pub fn rebalance_turrets(turrets: &mut [Turret], emergency_boost: i32) {
    for turret in turrets {
        let new_sum = turret.charge + emergency_boost;
        turret.charge = if new_sum > 100 {
            turret.overheated = true;
            100
        } else {
            turret.overheated = false;
            new_sum
        };
    }
}
