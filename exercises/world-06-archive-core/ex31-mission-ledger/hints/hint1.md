## Hint 1: Conceptual Question

A transaction makes a group of database changes atomic: either all succeed or none are applied. Before coding: what does it mean to "begin" a transaction? What are the two possible endings—commit and rollback? How do you track the running balance across multiple loop iterations? At what point should you decide to roll back instead of continuing?

**Spoiler threshold:** Low—asks you to understand the transaction lifecycle before coding.
