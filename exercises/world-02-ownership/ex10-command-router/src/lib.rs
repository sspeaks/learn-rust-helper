#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Dock { bay: u8 },
    Launch { window: u8 },
    Broadcast(String),
    Abort,
}

pub fn route_command(command: Command) -> String {
    match command {
        Command::Dock {bay} => format!("Routing to Bay {}", bay),
        Command::Launch {window} => format!("Launch window {} locked", window),
        Command::Broadcast(msg) => format!("Broadcasting: {}", msg),
        Command::Abort => "Abort signal received".to_string()
    }
}

pub fn route_batch(commands: Vec<Command>) -> Vec<String> {
    commands.into_iter().map(route_command).collect()
}
