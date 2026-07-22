## Hint 1: Conceptual Question

Rayon lets you write parallel code that looks almost identical to sequential code. Think about the difference: with a normal iterator, `.map(...).collect()` runs in a single thread. With rayon, the same operations run across all CPU cores. What single method call converts a sequential slice iterator into a rayon parallel iterator? Where does rayon's prelude need to be imported?

**Spoiler threshold:** Low—asks you to identify the one key API change.
