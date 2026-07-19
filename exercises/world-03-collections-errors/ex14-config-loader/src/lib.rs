#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub retry_limit: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigError {
    MissingField(&'static str),
    InvalidNumber { field: &'static str, value: String },
    DuplicateKey(String),
    UnknownKey(String),
}

pub fn parse_server_config(input: &str) -> Result<ServerConfig, ConfigError> {
    todo!("Parse key=value lines into ServerConfig and return detailed ConfigError values")
}
