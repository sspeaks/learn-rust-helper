## Hint 3: Algorithm Outline

```
function calibrate_reactor(base, drift, efficiency):
    Step 1: Add drift to base output
    Step 2: Multiply the result by efficiency_percent
    Step 3: Divide by 100 (integer division)
    Step 4: Return the final value
```

**Note:** Watch your types. `efficiency_percent` is u8, but you're multiplying by an i32. You may need to convert types. Rust's type system will tell you if something is wrong.

**Spoiler threshold:** High—tells you the steps, but not the Rust syntax.
