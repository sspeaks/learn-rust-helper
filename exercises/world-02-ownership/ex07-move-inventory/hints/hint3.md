## Hint 3: Algorithm Outline

```
function move_crate_to_shuttle(manifest, crate_id):
    Step 1: Iterate through manifest to find the index of a crate with matching id
    Step 2: If found, remove it using Vec::remove(index) and return Some(crate)
    Step 3: If not found, return None (manifest remains unchanged)
```

**Note:** `.iter().enumerate()` lets you pair indices with crates. Use `.find()` with a condition like `|(_idx, crate)| crate.id == crate_id`. Once you have the index, `.remove()` takes it and returns the owned crate.

**Spoiler threshold:** High—structure and methods, but not the full Rust code.
