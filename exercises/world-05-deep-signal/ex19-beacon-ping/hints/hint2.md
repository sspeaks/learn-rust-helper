## Hint 2: Tools & Types

- **`format!`:** Build the full URL: `format!("{}/beacons/{}", base_url, beacon_id)`.
- **`ureq::get(&url).call()`:** Returns `Result<ureq::Response, ureq::Error>`. Map the error to `BeaconPingError::Request`.
- **`response.status()`:** Returns the HTTP status code as `u16`.
- **`response.into_string()`:** Reads the body as `String`, returns `Result<String, std::io::Error>`. Map the error to `BeaconPingError::ReadBody`.
- **`?` operator:** Use it to propagate errors after mapping them with `.map_err(...)`.

The structure: build URL → call → check error → read status → read body → return struct.

**Spoiler threshold:** Medium—names the key methods, not the logic connecting them.
