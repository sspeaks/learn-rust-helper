## Hint 1: Conceptual Question

Which method receiver should own the data, and which should borrow it? Think in terms of intent:
- constructing new state,
- mutating existing state,
- reading state,
- producing a final value while consuming the struct.

**Spoiler threshold:** Low—focuses on method design choices.
