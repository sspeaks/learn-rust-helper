## Hint 1: Conceptual Question

You need to send data outbound rather than just receiving it. Before coding: how do you turn a Rust struct into a JSON string? Once you have a string, how do you attach it as the body of a POST request? What header does the server need to know the body is JSON? Think through the full round trip: serialize → send → receive → deserialize.

**Spoiler threshold:** Low—asks you to think through the steps before coding.
