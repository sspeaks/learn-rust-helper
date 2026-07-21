#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrewMember {
    pub name: String,
    pub role: String,
    pub level: u8,
}

impl CrewMember {
    pub fn new(name: impl Into<String>, role: impl Into<String>, level: u8) -> Self {
        CrewMember { name: name.into(), role: role.into(), level }
    }

    pub fn promote(&mut self, new_role: impl Into<String>) {
        self.level = self.level + 1;
        if self.level > 99 {
            self.level = 99;
        }
        self.role = new_role.into();
    }

    pub fn badge(&self) -> String {
        format!("[L{:02}] {} — {}", self.level, self.name, self.role)
    }
}
