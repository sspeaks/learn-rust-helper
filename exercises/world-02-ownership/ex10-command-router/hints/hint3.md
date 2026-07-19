## Hint 3: Algorithm Outline

```
function route_command(command):
    match command:
        Dock { bay } → return "Routing to Bay {bay}"
        Launch { window } → return "Launch window {window} locked" (or similar)
        Broadcast(msg) → return "Broadcasting: {msg}"
        Abort → return "Abort signal received"
```

**Note:** Each pattern extracts the associated data. For named variants like `Dock { bay }`, you get the field directly. For tuple variants like `Broadcast(msg)`, you bind the value. Use `format!()` to build the strings.

**Spoiler threshold:** High—structure and patterns, but not exact messages (check the spec).
