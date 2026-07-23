## Hint 2: Tools & Types

- Compare lengths with `left.len()` and `right.len()`.
- Struct construction can directly reuse borrowed parameters: `MissionView { code, captain }`.
- For clipping:
  - `if max_len >= text.len() { text }`
  - else `&text[..max_len]`
- `&text[..0]` is a valid empty slice.

**Spoiler threshold:** Medium—gives direct std operations used in each function.
