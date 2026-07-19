## Hint 2: Tools & Types

Look at these Rust tools:

- **`format!()` macro:** Lets you build strings with formatting rules, e.g., `format!("{:02}", 5)` → `"05"`
- **Format specifiers:**
  - `{:02}` → zero-padded to 2 digits
  - `{:04}` → zero-padded to 4 digits
  - `{:+04}` → zero-padded to 4, with explicit `+` or `-` sign
  - `{}` → no padding, just convert to string
- **`String` type:** Owned, mutable text. Built by `format!()` or `to_string()`.

Read the Rust book's ["Formatting" section](https://doc.rust-lang.org/std/fmt/) or just experiment in `cargo test` until the format is right.

**Spoiler threshold:** Medium—now you know which tool to reach for.
