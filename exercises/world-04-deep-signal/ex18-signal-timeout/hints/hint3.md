## Hint 3: Algorithm Outline

```
function fetch_signal_with_timeout(base_url, timeout):
    Step 1: Build a ureq Agent with the given timeout

    Step 2: Send GET to "{base_url}/signal" using the agent

    Step 3: Match on the Result:
            Err(ureq::Error::Status(code, _)) →
                return SignalTimeoutError::HttpStatus(code)
            Err(ureq::Error::Transport(transport)) →
                inspect transport to check if it is an I/O timeout
                if yes → return SignalTimeoutError::TimedOut
                if no  → return SignalTimeoutError::Transport(transport)
            Ok(response) → continue

    Step 4: Read the body string from response.into_string()
            → on error, return SignalTimeoutError::ReadBody

    Step 5: Deserialize the body into SignalEnvelope with serde_json::from_str
            → on error, return SignalTimeoutError::Decode

    Step 6: Return Ok(envelope)
```

**Note:** You must match on both error variants before handling the success case. The transport timeout detection requires inspecting the inner `std::io::Error` inside the transport error.

**Spoiler threshold:** High—provides the decision tree without exact Rust code.
