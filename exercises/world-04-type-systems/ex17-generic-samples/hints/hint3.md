## Hint 3: Algorithm Outline

```
newest_sample(window):
    return window.samples.last().cloned()

strongest_sample(window):
    take first sample as current best, or return None if empty
    scan remaining samples:
        if sample > best: update best
    return Some(best)

format_window(window):
    if samples empty: return "source []"
    stringify each sample and join with ", "
    return "source [joined_values]"
```

**Spoiler threshold:** High—complete algorithm shape without exact final code.
