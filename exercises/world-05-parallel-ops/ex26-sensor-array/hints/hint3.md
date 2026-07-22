## Hint 3: Algorithm Outline

```
function calibrate_readings_parallel(readings, offset):
    Step 1: Call readings.par_iter() to get a parallel iterator

    Step 2: Map each SensorReading to CalibratedReading:
            calibrated_value = raw_value + offset
            sensor_id = sensor_id cloned

    Step 3: Collect into Vec<CalibratedReading>
            (rayon preserves order for slice-based par_iter)

    Step 4: Return the Vec

function total_calibrated_power_parallel(readings):
    Step 1: Call readings.par_iter() to get a parallel iterator

    Step 2: Map each CalibratedReading to its calibrated_value

    Step 3: Sum all values using .sum::<i64>()

    Step 4: Return the sum
```

**Note:** The sequential versions would use `.iter()` instead of `.par_iter()`—that's the entire difference. Rayon handles thread creation, work distribution, and result collection automatically.

**Spoiler threshold:** High—algorithm that makes the rayon switch explicit.
