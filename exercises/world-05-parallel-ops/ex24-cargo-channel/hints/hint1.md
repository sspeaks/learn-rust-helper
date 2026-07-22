## Hint 1: Conceptual Question

Channels separate producers (workers) from the consumer (main thread). Think through the ownership model: who owns the sender? How do multiple threads each get their own copy? What tells the receiver that no more messages are coming? And why is it important to drop the original sender before you start receiving?

**Spoiler threshold:** Low—asks you to reason about channel ownership before coding.
