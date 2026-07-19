## Hint 2: Tools & Types

- **`&mut [T]` slice:** Allows iteration and modification of elements.
- **`for turret in turrets.iter_mut()`:** Mutable iteration over slice elements.
- **Field mutation:** `turret.charge += emergency_boost;` modifies in place.
- **Conditional logic:** Check if charge > 100 and adjust overheated flag accordingly.

Mutable iteration lets you access and modify each element without cloning.

**Spoiler threshold:** Medium—names the tools, not the full logic.
