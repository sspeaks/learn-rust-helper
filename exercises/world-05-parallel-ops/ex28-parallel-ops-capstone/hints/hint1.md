## Hint 1: Conceptual Question

You need three tools working together: threads for concurrency, a channel for communication, and a mutex for the checksum accumulator. Before coding, sketch the pipeline: what does Stage 1 produce? Where does it send that output? Who consumes it and what does the consumer do with each result? How do you ensure outputs are in a consistent order at the end?

**Spoiler threshold:** Low—asks you to design the pipeline architecture before writing code.
