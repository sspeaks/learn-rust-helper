#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mission {
    pub code: String,
    pub reward: u32,
    pub active: bool,
}

pub fn active_reward_for_code(missions: &[Mission], code: &str) -> Option<u32> {
    let mission = missions.iter().find(|m| m.code == code)?;
    if mission.active {
        Some(mission.reward)
    } else {
        None
    }
}

pub fn reward_or_default(missions: &[Mission], code: &str, default_reward: u32) -> u32 {
    active_reward_for_code(missions, code).unwrap_or(default_reward)
}
