#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Clearance {
    Visitor,
    Engineer,
    Captain,
}

pub fn access_message(clearance: Clearance, is_on_duty: bool) -> &'static str {
    match (clearance, is_on_duty) {
        (Clearance::Visitor, true) => "Welcome Visitor-enjoy your tour.",
        (Clearance::Visitor, false) => "Visitor pass expired.",
        (Clearance::Engineer, true) => "Welcome, Engineer. Proceed to Bay A.",
        (Clearance::Engineer, false) => "Engineer must check in first.",
        (Clearance::Captain, true) => "Welcome back, Captain. All systems yours.",
        (Clearance::Captain, false) => "Captain, command duty awaits.",
    }
}

pub fn gate_announcement(name: &str, clearance: Clearance, is_on_duty: bool) -> String {
    format!("{name}: {}", access_message(clearance, is_on_duty))
}
