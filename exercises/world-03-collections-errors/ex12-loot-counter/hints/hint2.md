## Hint 2: Tools & Types

- **`HashMap::new()`:** Create a new empty map.
- **`.entry(key).or_insert(default)`:** Get or insert a value for a key (the entry API).
- **`+=` on the returned reference:** Increment the count: `*count += 1;`
- **Type inference:** `HashMap<String, usize>` is often inferred from context.
- **String ownership:** Keys in the map are owned `String` (convert `&str` with `.to_string()` or `.into()`).

The entry API is idiomatic for counting: `*map.entry(name.to_string()).or_insert(0) += 1;`

**Spoiler threshold:** Medium—names the types and methods, not the exact chain.
