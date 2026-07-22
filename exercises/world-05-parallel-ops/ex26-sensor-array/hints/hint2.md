## Hint 2: Tools & Types

- **`use rayon::prelude::*`** (already in `src/lib.rs`): Brings parallel iterator traits into scope.
- **`readings.par_iter()`:** Returns a `rayon::ParallelIterator` over `&SensorReading`. Drop-in replacement for `.iter()`.
- **`.map(|r| CalibratedReading { sensor_id: r.sensor_id.clone(), calibrated_value: r.raw_value + offset })`:** Transform each element.
- **`.collect::<Vec<_>>()`:** Rayon's collect preserves input order when using `par_iter()` on a slice.
- **`.par_iter().map(|r| r.calibrated_value).sum::<i64>()`:** Parallel sum—rayon's parallel sum works like the standard iterator `sum()`.

**Spoiler threshold:** Medium—gives the exact chain for both functions.
