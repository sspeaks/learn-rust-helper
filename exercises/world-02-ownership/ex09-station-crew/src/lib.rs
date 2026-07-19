#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrewMember {
    pub name: String,
    pub role: String,
    pub level: u8,
}

impl CrewMember {
    pub fn new(name: impl Into<String>, role: impl Into<String>, level: u8) -> Self {
        todo!("Construct a CrewMember from inputs")
    }

    pub fn promote(&mut self, new_role: impl Into<String>) {
        todo!("Update role and increase level, capped at 99")
    }

    pub fn badge(&self) -> String {
        todo!("Build a display badge like '[L05] Nova — Navigator'")
    }
}
