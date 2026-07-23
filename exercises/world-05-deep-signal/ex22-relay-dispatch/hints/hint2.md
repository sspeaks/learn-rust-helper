## Hint 2: Tools & Types

- **`serde_json::to_string(request)`:** Serializes the request to a JSON string. Returns `Result<String, serde_json::Error>`.
- **`ureq::post(&url).set("Content-Type", "application/json").send_string(&json_body)`:** Sends a POST with a string body and explicit content-type header. Returns `Result<ureq::Response, ureq::Error>`. Build `url` by appending `/relay/dispatch` to `base_url`.
- **`response.into_string()`:** Reads the response body. Returns `Result<String, std::io::Error>`.
- **`serde_json::from_str::<RelayDispatchReceipt>(&body)`:** Deserializes the response body.
- Each step has its own error variant: `Serialize`, `Request`, `ReadBody`, `Decode`.

**Spoiler threshold:** Medium—names the key methods in the right order.
