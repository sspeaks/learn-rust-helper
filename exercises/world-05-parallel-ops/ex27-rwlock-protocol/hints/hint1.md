## Hint 1: Conceptual Question

You already know `Mutex<T>`. `RwLock<T>` has two kinds of locks—when do you use each? A read lock can be held by many threads at once. A write lock is exclusive. For this exercise, which kind do worker threads need when inserting into the map? Which kind is safe for reading the final snapshot on the main thread?

**Spoiler threshold:** Low—asks you to reason about read vs. write access before coding.
