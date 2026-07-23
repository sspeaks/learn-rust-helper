## Hint 1: Conceptual Question

You need to filter results based on three independent conditions. Before writing SQL: how do you bind a `u8` value to a `>=` comparison? How do you match a prefix in SQL—what operator works for "starts with"? For the `include_archived` flag, does the SQL query change based on whether it's `true` or `false`? How do you handle the case where the flag is `false` without using a Rust `if` to build two separate SQL strings?

**Spoiler threshold:** Low—asks you to plan the query shape before writing it.
