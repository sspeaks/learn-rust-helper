#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrewMember {
    pub name: String,
    pub role: String,
    pub level: u8,
}

impl CrewMember {
    pub fn new(name: impl Into<String>, role: impl Into<String>, level: u8) -> Self {
        Self {
            name: name.into(),
            role: role.into(),
            level,
        }
    }

    pub fn promote(&mut self, new_role: impl Into<String>) {
        self.role = new_role.into();
        self.level = (self.level + 1).min(99);
    }

    pub fn badge(&self) -> String {
        format!("[L{:02}] {} \u{2014} {}", self.level, self.name, self.role)
    }
}
