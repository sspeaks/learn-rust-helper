#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Turret {
    pub callsign: String,
    pub charge: i32,
    pub overheated: bool,
}

pub fn rebalance_turrets(turrets: &mut [Turret], emergency_boost: i32) {
    for turret in turrets.iter_mut() {
        turret.charge += emergency_boost;
        if turret.charge > 100 {
            turret.overheated = true;
            turret.charge = 100;
        } else {
            turret.overheated = false;
        }
    }
}
