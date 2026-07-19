## Hint 1: Conceptual Question

You're implementing three methods on a struct. `new()` creates an instance; `promote()` modifies it; `badge()` reads it for display. What do the receiver types (`self`, `&self`, `&mut self`) mean, and when would you use each? How does `impl Into<String>` let callers pass either `&str` or `String`?

**Spoiler threshold:** Low—asks you to think about method types.
