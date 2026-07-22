## Hint 1: Conceptual Question

Before writing any SQL, think about what the schema needs: which tables, which columns, which types? SQLite uses a small set of type affinities (INTEGER, TEXT, REAL, BLOB, NUMERIC). How do you create a table only if it doesn't already exist? How does SQLite store a schema version number—is there a built-in mechanism, or do you need a table for it?

**Spoiler threshold:** Low—asks you to plan the schema before writing DDL.
