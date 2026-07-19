## Hint 2: Tools & Types

- **`match` on enums:** `match command { Variant1 { field } => ..., Variant2(data) => ..., ... }`
- **Destructuring in patterns:** Extract associated data directly in the pattern.
- **`format!()` macro:** Build routing messages by interpolating the extracted data.
- **Exhaustiveness:** Rust compiler ensures all variants are handled.
- **Ownership:** Match consumes the enum (you own its data).

Each variant pattern lets you access the associated value(s).

**Spoiler threshold:** Medium—names the tools and syntax, not the exact messages.
