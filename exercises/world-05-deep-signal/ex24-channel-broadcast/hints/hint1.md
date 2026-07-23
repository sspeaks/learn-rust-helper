## Hint 1: Conceptual Question

You know how to make one async request. Now you need to make many at the same time. What's the difference between calling `.await` inside a loop versus collecting the futures first and then waiting for all of them? If you `.await` in a loop, is it actually concurrent? What Rust tool lets you launch multiple futures and wait for all of them at once?

**Spoiler threshold:** Low—guides you to identify the concurrent vs. sequential distinction.
