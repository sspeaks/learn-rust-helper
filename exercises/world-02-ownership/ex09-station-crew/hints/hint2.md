## Hint 2: Tools & Types

- **`fn new(...) -> Self`:** Constructor returning a new instance.
- **`&self`:** Immutable borrow; read-only methods like `badge()`.
- **`&mut self`:** Mutable borrow; methods that modify, like `promote()`.
- **`impl Into<String>`:** Lets the caller pass `&str` or `String`; Rust auto-converts.
- **Field access:** `self.name`, `self.role`, `self.level` inside methods.
- **Format string:** `format!("[L{:02}] {} — {}", self.level, self.name, self.role)`
  — the `—` between name and role is the **em dash** character (U+2014), not a plain ASCII hyphen `-`. Copy it directly from this hint or use `"\u{2014}"` in your string literal.

Methods are defined in an `impl CrewMember { ... }` block.

**Spoiler threshold:** Medium—names tools and receiver types, not the full implementation.
