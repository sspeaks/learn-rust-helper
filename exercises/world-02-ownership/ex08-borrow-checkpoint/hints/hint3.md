## Hint 3: Algorithm Outline

```
function rebalance_turrets(turrets, emergency_boost):
    For each turret in turrets (mutably):
        Step 1: Add emergency_boost to turret.charge
        Step 2: If turret.charge > 100:
            Set turret.charge = 100
            Set turret.overheated = true
        Step 3: Else:
            Set turret.overheated = false
```

**Note:** Use `for turret in turrets.iter_mut()` to get mutable references. Then modify `turret.charge` and `turret.overheated` directly.

**Spoiler threshold:** High—clear algorithm, but not the Rust syntax.
