## Hint 3: Algorithm Outline

```
function parse_server_config(input):
    Step 1: Initialize a HashMap to track seen keys
    Step 2: Initialize variables for host, port, retry_limit (use Option or let them unset)
    Step 3: For each line in input:
        Step 3a: Split on '=' to get key and value
        Step 3b: Check if key was already seen (return DuplicateKey if so)
        Step 3c: Match on key:
            "host" → store value as String
            "port" → parse to u16, return InvalidNumber if it fails
            "retry_limit" → parse to u8, return InvalidNumber if it fails
            else → return UnknownKey
        Step 3d: Mark key as seen
    Step 4: Check that all three fields were set; return MissingField if any missing
    Step 5: Return Ok(ServerConfig { host, port, retry_limit })
```

**Note:** Port range: 0-65535 (u16 naturally fits). Retry limit: 0-255 (u8 naturally fits).

**Spoiler threshold:** High—clear algorithm and validation order, but not the exact Rust code.
