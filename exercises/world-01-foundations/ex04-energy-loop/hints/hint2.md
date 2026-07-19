## Hint 2: Tools & Types

- **`for` loop:** Iterate over a range with `for cycle in cycles { ... }`
- **Accumulation:** Use a mutable variable to track the sum (e.g., `let mut total = 0;`)
- **Ranges:** `RangeInclusive` from `std::ops` can be iterated directly.
- **Borrowing:** You receive `cycles` by value, so you own it and can iterate.

Start with `let mut total = 0u32;`, then loop and add each cycle number.

**Spoiler threshold:** Medium—names the tools, but not the implementation.
