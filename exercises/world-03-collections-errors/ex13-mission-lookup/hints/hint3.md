## Hint 3: Algorithm Outline

**Approach 1 (using find + filter + map):**
```
Step 1: Find the mission with matching code
Step 2: If found, check if active
Step 3: If active, return Some(reward)
Step 4: Otherwise, return None
```

**Approach 2 (using match):**
```
Step 1: Find the mission with matching code (returns Option)
Step 2: Match on the Option:
    Some(mission) → if mission.active, return Some(mission.reward), else None
    None → return None
```

**Note:** Use `.find()` to search, then chain with `.and_then()` or use a match to apply the active check and extract reward.

**Spoiler threshold:** High—gives both approaches, but not the exact syntax.
