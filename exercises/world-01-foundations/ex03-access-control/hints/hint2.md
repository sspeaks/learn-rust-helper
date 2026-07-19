## Hint 2: Tools & Types

- **`match` expression:** Lets you pattern-match on enums. E.g., `match clearance { Visitor => ..., Engineer => ..., Captain => ... }`
- **Nested matching:** You can match on both the enum and the boolean together.
- **`&'static str`:** A string literal (like `"Welcome"`) has type `&'static str`.
- **Static string references:** Don't use `format!()` here; just return literal strings.

Experiment with `match` to cover all combinations. The compiler will tell you if you missed a case.

**Spoiler threshold:** Medium—tells you about `match`, but not the six messages.
