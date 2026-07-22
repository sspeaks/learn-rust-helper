## Hint 2: Tools & Types

- **`reqwest::get(&url).await`:** Sends an async GET and waits for the response headers. Returns `Result<reqwest::Response, reqwest::Error>`. Map errors to `AsyncHandshakeError::Request`.
- **`response.status()`:** Returns `reqwest::StatusCode`. Call `.is_success()` to check for 2xx.
- **`response.status().is_success()`:** Returns `true` for 2xx status codes.
- **`response.json::<HandshakeReceipt>().await`:** Deserializes the JSON body asynchronously. Returns `Result<HandshakeReceipt, reqwest::Error>`. Map errors to `AsyncHandshakeError::Decode`.
- **Order matters:** Read the status *before* consuming the response with `.json()`.

**Spoiler threshold:** Medium—names the key async methods in sequence.
