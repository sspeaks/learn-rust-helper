## Hint 3: Algorithm Outline

```
function normalize_call_sign(input):
    Step 1: Trim leading and trailing whitespace
    Step 2: Split on whitespace to get individual words
    Step 3: Filter out empty strings (in case of multiple spaces)
    Step 4: Join words back together with a single space
    Step 5: Convert to uppercase
    Step 6: Return the owned String
```

**Note:** Use `trim()` first, then `split_whitespace()` (which already handles multiple spaces), then `collect()` to gather, then `join(" ")`, then `to_uppercase()`.

**Spoiler threshold:** High—gives the steps, but not the exact Rust code.
