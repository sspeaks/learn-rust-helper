## Hint 2: Tools & Types

- **`ureq::AgentBuilder::new().timeout(duration).build()`:** Creates an agent with a request timeout.
- **`agent.get(&url).call()`:** Returns `Result<ureq::Response, ureq::Error>`.
- **Matching on `ureq::Error`:**
  - `ureq::Error::Status(code, _response)` — HTTP error with a status code.
  - `ureq::Error::Transport(transport)` — network-level error.
- **Detecting a timeout from a `Transport` error:**
  - `transport.kind()` returns `ureq::ErrorKind`; check for `ureq::ErrorKind::Io`.
  - Cast to `std::io::Error` via `transport.source()` and check `err.kind() == std::io::ErrorKind::TimedOut`.
- **`response.into_string()`:** Reads the body; returns `Result<String, std::io::Error>`.
- **`serde_json::from_str::<SignalEnvelope>(&body)`:** Deserializes the body string.

**Spoiler threshold:** Medium—names the types and match structure without filling in the logic.
