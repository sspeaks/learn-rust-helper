## Hint 3: Algorithm Outline

```
function build_salvage_plan(manifest_lines, capacity):
    Step 1: Check if manifest_lines is empty; return EmptyManifest if so
    Step 2: Parse each line:
        For each line:
            Split on pipe ('|') to get name, mass, priority, fragile
            Convert mass to u32, priority to u8, fragile to bool
            If parse fails, return InvalidLine with line number and reason
            Construct SalvageItem and add to items Vec
    Step 3: Calculate total_mass by summing all masses
    Step 4: Check if total_mass > capacity; return OverCapacity if so
    Step 5: Count fragile items (fragile == true)
    Step 6: Sort all items by priority descending; collect all names into top_targets
    Step 7: Return Ok(SalvagePlan { total_mass, fragile_count, top_targets })
```

**Note:** Line indexes are 0-based, matching Rust indexing and the public error contract. The `InvalidLine` error includes the 0-based line index and a reason string. Sort all items by priority descending; `top_targets` contains all item names in that order.

**Spoiler threshold:** High—clear algorithm, but not the exact Rust code (parsing, sorting, collecting).
