## Hint 2: Tools & Types

- **`&str::find()`:** Returns `Option<usize>` with the index of a substring (e.g., `"."` for period).
- **Slicing syntax:** `&s[0..n]` creates a slice from index 0 to n (exclusive), or `&s[..n]` for short.
- **`Option` handling:** Use `match` or `.map()` to handle `Some(idx)` and `None`.
- **Array slicing:** `&arr[start..end]` or `&arr[len - count..]` for trailing elements.

Combine these to extract portions without allocating new memory.

**Spoiler threshold:** Medium—names the tools, not the exact combination.
