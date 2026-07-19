## Hint 2: Tools & Types

- **`trim()`:** Remove leading/trailing whitespace from a `&str`.
- **`split_whitespace()`:** Split on whitespace; returns an iterator.
- **`collect()`:** Gather iterator results into a collection (e.g., `Vec<&str>`).
- **`join()`:** Combine a collection of strings with a separator.
- **`to_uppercase()`:** Convert to uppercase; returns a `String`.
- **`to_string()`:** Convert a `&str` to an owned `String`.

Chain these methods together: trim → split → collect → join → uppercase.

**Spoiler threshold:** Medium—names the methods, not the exact order or combination.
