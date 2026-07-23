## Hint 1: Conceptual Question

You need to turn a JSON string into a typed Rust struct and then validate one field. Think: how does Rust know which JSON fields map to which struct fields? What crate bridges JSON and Rust types? After successful parsing, how do you inspect a field and return an error if it fails your check?

**Spoiler threshold:** Low—asks you to identify the tools before using them.
