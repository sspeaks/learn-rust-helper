#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Dock { bay: u8 },
    Launch { window: u8 },
    Broadcast(String),
    Abort,
}

pub fn route_command(command: Command) -> String {
    // ════════════════════════════════════════════════════════════════════════════
    // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
    // ════════════════════════════════════════════════════════════════════════════
    todo!("Pattern-match each command variant into a routing message")
}

pub fn route_batch(commands: Vec<Command>) -> Vec<String> {
    commands.into_iter().map(route_command).collect()
}
