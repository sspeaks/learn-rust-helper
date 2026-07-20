#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Dock { bay: u8 },
    Launch { window: u8 },
    Broadcast(String),
    Abort,
}

pub fn route_command(command: Command) -> String {
    match command {
        Command::Dock { bay } => format!("Routing to docking bay {bay}"),
        Command::Launch { window } => format!("Launch window {window} confirmed"),
        Command::Broadcast(msg) => format!("Broadcasting: {msg}"),
        Command::Abort => "ABORT: all operations halted".to_string(),
    }
}

pub fn route_batch(commands: Vec<Command>) -> Vec<String> {
    commands.into_iter().map(route_command).collect()
}
