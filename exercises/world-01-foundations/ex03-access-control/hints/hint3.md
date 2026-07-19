## Hint 3: Algorithm Outline

```
function access_message(clearance, is_on_duty):
    Match on (clearance, is_on_duty):
        (Visitor, true) → return the "on duty" visitor message
        (Visitor, false) → return the "off duty" visitor message
        (Engineer, true) → return the "on duty" engineer message
        (Engineer, false) → return the "off duty" engineer message
        (Captain, true) → return the "on duty" captain message
        (Captain, false) → return the "off duty" captain message
```

**Note:** Each branch returns a `&'static str` literal. The exact messages are in the spec. Use `match` with a tuple pattern `(clearance, is_on_duty)` to match all six cases.

**Spoiler threshold:** High—gives the structure, but not the exact messages (which are in the quest spec above).
