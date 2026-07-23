## Hint 3: Algorithm Outline

```
async function perform_async_handshake(base_url, call_sign):
    Step 1: Build the URL: "{base_url}/handshake/{call_sign}"

    Step 2: Send GET request and await the response
            → on reqwest error, return AsyncHandshakeError::Request

    Step 3: Check response.status()
            → if not 2xx, return AsyncHandshakeError::InvalidStatus(status)

    Step 4: Decode the response body as HandshakeReceipt via .json().await
            → on decode error, return AsyncHandshakeError::Decode

    Step 5: Return Ok(receipt)
```

**Note:** The key difference from ureq is that every I/O operation has `.await` appended. The `async fn` keyword marks the function as returning a future; the caller must `.await` it too. The Tokio test runtime drives the future to completion automatically in tests.

**Spoiler threshold:** High—algorithm with async-specific note, no Rust syntax.
