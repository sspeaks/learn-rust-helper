## Hint 1: Conceptual Question

This capstone combines two distinct runtimes: an async HTTP client and a synchronous database connection. Before coding, think about the constraint: why can't `&Connection` exist inside an `async fn`? What does it mean for a type to be `!Send`? The staged design separates these concerns—Stage 1 is fully async (no database), Stage 2 is fully synchronous (no async). How does this separation solve the `!Send` problem?

**Spoiler threshold:** Low—asks you to understand the async/sync constraint before writing any code.
