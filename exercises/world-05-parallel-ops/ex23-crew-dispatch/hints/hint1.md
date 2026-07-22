## Hint 1: Conceptual Question

You need to run work in parallel and collect results. Think through the lifecycle: when you spawn a thread, what do you get back? How do you wait for it to finish? If you want results in the same order as your inputs, how must you structure the spawn-and-join process? What happens if you join in a different order than you spawned?

**Spoiler threshold:** Low—asks you to reason about the thread lifecycle before coding.
