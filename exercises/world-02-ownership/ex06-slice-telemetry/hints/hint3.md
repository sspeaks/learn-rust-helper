## Hint 3: Algorithm Outline

For **first_sentence**:
```
Step 1: Use find(".") to locate the period
Step 2: If found at index i, return &report[..=i]  (inclusive slice up to and including index)
Step 3: If not found, return the entire report
```

For **trailing_readings**:
```
Step 1: Calculate how many readings to skip: len - count
Step 2: Return &readings[skip..]  (slice from skip to end)
Step 3: Handle edge case where count >= readings.len() (return entire slice)
```

**Note:** Use inclusive slice syntax `[..=i]` for the period (to include it). For trailing, the `..` syntax naturally handles slicing to the end.

**Spoiler threshold:** High—gives the algorithm and syntax, but not the full solution.
