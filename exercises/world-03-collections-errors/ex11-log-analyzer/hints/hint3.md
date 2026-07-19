## Hint 3: Algorithm Outline

```
function summarize_failures(events, max_items):
    Step 1: Iterate over events
    Step 2: Filter to keep only where success == false
    Step 3: Map each failure to format "{system}: error code {code}"
    Step 4: Take the first max_items results
    Step 5: Collect into a Vec<String>
    Step 6: Return the Vec
```

**Note:** Use iterator adapters chained together. The closure in `.filter()` checks `!e.success` or `e.success == false`. The closure in `.map()` builds the summary string using `format!()`.

**Spoiler threshold:** High—clear algorithm and tools, but not the exact Rust syntax.
