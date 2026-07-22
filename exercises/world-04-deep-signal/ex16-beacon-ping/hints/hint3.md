## Hint 3: Algorithm Outline

```
function ping_beacon(base_url, beacon_id):
    Step 1: Build the endpoint URL string from base_url and beacon_id
            → format as "{base_url}/beacons/{beacon_id}"

    Step 2: Send GET request using ureq::get(&endpoint).call()
            → on error, wrap in BeaconPingError::Request and return

    Step 3: Read the HTTP status code from the response

    Step 4: Consume the response to get the body string via into_string()
            → on error, wrap in BeaconPingError::ReadBody and return

    Step 5: Return Ok(BeaconPing { endpoint, status, body })
```

**Note:** You must read the status *before* calling `into_string()` because `into_string()` consumes the response. Save the status in a local variable first.

**Spoiler threshold:** High—describes the algorithm clearly, but not the exact Rust syntax.
