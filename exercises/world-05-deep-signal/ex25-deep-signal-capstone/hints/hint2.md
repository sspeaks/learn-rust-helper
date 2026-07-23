## Hint 2: Tools & Types

- **`gather_deep_signals`:**
  - Same join-all pattern as ex24: build un-awaited futures per node, `join_all(...).await`, collect results.
  - Map request/status/decode errors to the node-carrying variants with the node name cloned into the error.
  - Error variants: `DeepSignalError::Request { node, source }`, `InvalidStatus { node, status }`, `Decode { node, source }`.

- **`build_deep_signal_report`:**
  - `.iter().filter(...).map(...).collect()` — partition into two Vecs.
  - `samples.iter().map(|s| s.strength as f64).sum::<f64>() / samples.len() as f64` — compute mean.

- **`run_deep_signal_pipeline`:**
  - Call `gather_deep_signals(base_url, nodes).await?` then `build_deep_signal_report(&samples)?`.

**Spoiler threshold:** Medium—names the key methods and patterns for each function.
