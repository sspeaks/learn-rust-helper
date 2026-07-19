## Hint 2: Tools & Types

- **`str::lines()`:** Split input into lines.
- **`str::split('=')`:** Split each line on the key-value delimiter.
- **`.parse::<u16>()`:** Try parsing a string to a number; returns `Result`.
- **`match` on Result:** Handle `Ok(val)` and `Err(_)` for parse errors.
- **HashMap for tracking keys:** Detect duplicates by checking if a key was already seen.
- **Return early on error:** Return `Err(ConfigError::...)` as soon as an issue is found.

Validate: all required fields present, numbers parse and are in range, no duplicates, no unknown keys.

**Spoiler threshold:** Medium—names the parsing methods, not the full logic.
