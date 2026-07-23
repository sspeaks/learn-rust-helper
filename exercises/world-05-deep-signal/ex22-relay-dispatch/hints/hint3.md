## Hint 3: Algorithm Outline

```
function dispatch_relay(base_url, request):
    Step 1: Serialize request to JSON string
            → on error, return RelayDispatchError::Serialize

    Step 2: Send POST to "{base_url}/relay/dispatch"
            Set Content-Type to "application/json"
            Attach the serialized JSON as the body
            → on error, return RelayDispatchError::Request

    Step 3: Read the response body with into_string()
            → on error, return RelayDispatchError::ReadBody

    Step 4: Deserialize the body string into RelayDispatchReceipt
            → on error, return RelayDispatchError::Decode

    Step 5: Return Ok(receipt)
```

**Note:** `serde_json::to_string` can fail on types with non-string map keys or un-serializable values, even for simple structs. Always handle the error rather than using `unwrap`.

**Spoiler threshold:** High—sequential steps without Rust-specific syntax.
