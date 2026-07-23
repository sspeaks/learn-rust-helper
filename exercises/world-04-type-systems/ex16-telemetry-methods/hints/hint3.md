## Hint 3: Algorithm Outline

```
with_capacity(label, capacity):
    return TelemetryBuffer { label, capacity, readings: vec![] }

record(&mut self, reading):
    if readings.len() == capacity:
        return false
    push reading
    return true

average(&self):
    if readings is empty:
        return None
    total = sum(readings)
    return Some(total as f64 / readings.len() as f64)

into_report(self):
    if readings is empty:
        return "label:0@n/a"
    avg = average of readings
    return "label:count@avg_with_one_decimal"
```

**Spoiler threshold:** High—full structure without exact Rust syntax.
