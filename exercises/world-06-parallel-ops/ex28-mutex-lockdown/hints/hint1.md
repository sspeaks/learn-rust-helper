## Hint 1: Conceptual Question

You have shared data that multiple threads must mutate. Rust's ownership system normally prevents two threads from mutating the same value simultaneously. What types let you share ownership across threads? What type provides mutual exclusion—ensuring only one thread mutates at a time? How do you safely access the value inside?

**Spoiler threshold:** Low—asks you to identify the types before using them.
