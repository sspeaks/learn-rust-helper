## Hint 2: Tools & Types

- **`Vec::iter().position()`:** Finds the index of an element matching a condition.
- **`Vec::remove(index)`:** Removes and returns the element at `index`, mutating the vector.
- **`Option` type:** Return `Some(crate)` if found, `None` otherwise.
- **`&mut Vec<T>` reference:** Allows you to remove elements (requires mutability).
- **Ownership transfer:** When you remove, you own the element; return it to the caller.

Chain `.position()` to find, then `.remove()` to extract.

**Spoiler threshold:** Medium—names the methods, not the exact sequence.
