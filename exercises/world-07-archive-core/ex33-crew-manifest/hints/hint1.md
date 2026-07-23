## Hint 1: Conceptual Question

You need to write a row to a table and then read rows back. Before coding: how does rusqlite distinguish between statements that modify data (INSERT/UPDATE/DELETE) and queries that return rows (SELECT)? For INSERT, what is the rusqlite method and how do you safely bind field values? For SELECT, how do you iterate over rows and map each to a Rust struct?

**Spoiler threshold:** Low—asks you to identify the two different rusqlite execution paths.
