## Hint 1: Conceptual Question

You need to make an HTTP request that honors a time limit and maps each possible failure to a specific error variant. Before writing code: how do you attach a timeout to a ureq request? Where do you inspect the error to tell apart a timeout from a server error or a transport failure? What is the difference between a transport error and an HTTP status error in ureq?

**Spoiler threshold:** Low—asks you to think about the tool model before coding.
