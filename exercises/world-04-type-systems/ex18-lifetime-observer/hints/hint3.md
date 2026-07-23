## Hint 3: Algorithm Outline

```
longer_label(left, right):
    if left.len() >= right.len():
        return left
    else:
        return right

mission_view(code, captain):
    return MissionView { code, captain }

clipped_prefix(text, max_len):
    if max_len >= text.len():
        return text
    else:
        return &text[..max_len]
```

**Spoiler threshold:** High—full control flow, still not copy-paste Rust.
