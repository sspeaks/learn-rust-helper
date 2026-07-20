# Quest 14: Config Loader

**🎮 Quest:** Parse a config file format and detect errors precisely. Implement a parser that returns a custom `Result` with detailed error information. Practice error handling and `Result`.

## Objective

Implement `parse_server_config` to parse key=value lines into a `ServerConfig` struct, returning custom errors for parse failures. This teaches `Result<T, E>`, custom error types, and error propagation.

## Public API

```rust
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub retry_limit: u8,
}

pub enum ConfigError {
    MissingField(&'static str),
    InvalidNumber { field: &'static str, value: String },
    DuplicateKey(String),
    UnknownKey(String),
}

pub fn parse_server_config(input: &str) -> Result<ServerConfig, ConfigError> {
    // Your implementation
}
```

## Behavioral Rules

From `src/lib.rs`, the function must:

1. Accept a string containing lines in `key=value` format.
2. Parse and extract `host`, `port`, and `retry_limit`.
3. Return errors for:
   - Missing required fields: `MissingField("field_name")`
   - Non-numeric values: `InvalidNumber { field: "port", value: "abc" }`
   - Duplicate keys: `DuplicateKey("host")`
   - Unknown keys: `UnknownKey("timeout")`
4. Return `Ok(ServerConfig { ... })` on success.

## Concepts Practiced

- **Result<T, E>:** Success or failure with associated types.
- **Custom enums for errors:** Describing different error cases.
- **Error propagation:** Returning errors early.
- **Parsing:** Converting strings to structured data.
- **Validation:** Checking constraints (port range, etc.).

## Edge Cases

- Empty input (all fields missing).
- Duplicate key appearing twice.
- Invalid port (negative, too large, non-numeric).
- Unknown keys mixed with valid ones.
- Extra whitespace around key or value.

## How to Work on This Quest

**From inside `nix develop path:.`:**

```bash
# Check your work
learn check ex14-config-loader

# Get a hint if stuck
learn hint ex14-config-loader

# Or see the next hint level
learn hint ex14-config-loader --level 2
```

**Omit the exercise ID to use your current recommendation:**

```bash
learn check    # Checks current exercise
learn hint     # Hints current exercise
```

## XP Reward

**240 XP** for first completion.

## Prerequisites

Complete **Mission Lookup** (ex13).

## Success Criteria

- Correctly parses valid key=value lines.
- Returns appropriate errors with correct variant and field names.
- Detects duplicates and unknown keys.
- Validates numeric ranges (port: 0-65535, retry_limit: 0-255).
- All fields are required.

## Next Steps

Complete this quest to unlock **Salvage Capstone**, the final challenge synthesizing all concepts.
