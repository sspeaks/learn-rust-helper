use std::collections::HashMap;

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
    let mut fields: HashMap<&str, &str> = HashMap::new();

    for line in input.lines() {
        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| ConfigError::UnknownKey(line.to_string()))?;
        match key {
            "host" | "port" | "retry_limit" => {}
            _ => return Err(ConfigError::UnknownKey(key.to_string())),
        }
        if fields.contains_key(key) {
            return Err(ConfigError::DuplicateKey(key.to_string()));
        }
        fields.insert(key, value);
    }

    let host = fields
        .get("host")
        .ok_or(ConfigError::MissingField("host"))?
        .to_string();

    let port_str = *fields
        .get("port")
        .ok_or(ConfigError::MissingField("port"))?;
    let port = port_str
        .parse::<u16>()
        .map_err(|_| ConfigError::InvalidNumber {
            field: "port",
            value: port_str.to_string(),
        })?;

    let retry_str = *fields
        .get("retry_limit")
        .ok_or(ConfigError::MissingField("retry_limit"))?;
    let retry_limit = retry_str
        .parse::<u8>()
        .map_err(|_| ConfigError::InvalidNumber {
            field: "retry_limit",
            value: retry_str.to_string(),
        })?;

    Ok(ServerConfig {
        host,
        port,
        retry_limit,
    })
}
