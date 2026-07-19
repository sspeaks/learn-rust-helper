## Hint 2: Tools & Types

- **`.iter()`:** Borrow each element from the slice.
- **`.filter(|e| condition)`:** Keep elements where condition is true.
- **`.map(|e| transform)`:** Transform each remaining element.
- **`.take(n)`:** Take only the first n elements.
- **`.collect::<Vec<_>>()`:** Gather results into a Vec.
- **Closures:** `|event| event.success == false` is a closure predicate.

Chain them: `.iter().filter(...).map(...).take(...).collect()`

**Spoiler threshold:** Medium—names iterator methods, not the exact chain.
