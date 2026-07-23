## Hint 2: Tools & Types

- `window.samples.last()` gives `Option<&T>`.
- `.cloned()` can convert `Option<&T>` to `Option<T>` when `T: Clone`.
- Iterate with `for &item in &window.samples` when `T: Copy`.
- Compare with `if item > current_best` to keep first max on ties.
- `to_string()` on each sample works when `T: Display`.
- `Vec<String>::join(", ")` helps build the list text.

**Spoiler threshold:** Medium—names concrete std helpers and iteration patterns.
