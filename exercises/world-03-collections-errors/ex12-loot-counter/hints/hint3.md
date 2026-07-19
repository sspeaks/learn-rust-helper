## Hint 3: Algorithm Outline

```
function count_loot(items):
    Step 1: Create a new HashMap<String, usize>
    Step 2: For each item name in items:
        Step 2a: Convert &str to owned String (or use .into())
        Step 2b: Use .entry(name).or_insert(0) to get or initialize to 0
        Step 2c: Increment the count (*entry += 1)
    Step 3: Return the HashMap
```

**Note:** The entry API is efficient: you access the bucket once and either get the existing value or insert a default. No duplicate lookups.

**Spoiler threshold:** High—clear algorithm and tools, but not the exact Rust idioms.
