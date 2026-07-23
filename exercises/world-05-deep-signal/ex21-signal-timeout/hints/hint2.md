## Hint 2: Tools & Types

- **`ureq::AgentBuilder::new().timeout(duration).build()`:** Creates an agent with a request timeout.
- **`agent.get(&url).call()`:** Returns `Result<ureq::Response, ureq::Error>`.
- **Matching on `ureq::Error`:**
  - `ureq::Error::Status(code, _response)` — HTTP error with a status code.
  - `ureq::Error::Transport(transport)` — network-level error.
- **Detecting a timeout from a `Transport` error:**
  - `transport.kind()` returns `ureq::ErrorKind`; check for `ureq::ErrorKind::Io`.
  - Cast to `std::io::Error` via `transport.source()` and check `err.kind() == std::io::ErrorKind::TimedOut`.
- **`response.into_reader()`:** Returns a `Read`-implementing reader over the raw response bytes. Use `Read::read_to_end` to collect the bytes into a `Vec<u8>`; propagate any `io::Error` as `SignalTimeoutError::ReadBody`.
- **`String::from_utf8(bytes)`:** Strictly decodes a byte vector to a UTF-8 `String`. Returns `Err` for invalid UTF-8—map that `Err` into `SignalTimeoutError::ReadBody` (wrap it in an `io::Error` with `io::ErrorKind::InvalidData`).
- **`serde_json::from_str::<SignalEnvelope>(&body)`:** Deserializes the body string.

**Spoiler threshold:** Medium—names the types and match structure without filling in the logic.
