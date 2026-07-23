## Hint 1: Conceptual Question

You've done synchronous HTTP—now the function must be `async`. Think about the difference before coding: with ureq, the call blocks the thread until done. With reqwest, you get a *future* that you must `.await`. What does it mean to `.await` something? When does the async function actually do work? And how is the error type different from ureq's?

**Spoiler threshold:** Low—conceptual framing for the sync-to-async mental shift.
