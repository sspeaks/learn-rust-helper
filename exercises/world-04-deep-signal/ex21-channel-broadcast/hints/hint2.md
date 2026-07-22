## Hint 2: Tools & Types

- **`futures::future::join_all(futures_vec)`:** Takes a `Vec<impl Future>` and returns a future that resolves to a `Vec` of all results. This is your primary tool.
- **Building the futures vec:** Map each target to a future by calling `fetch_broadcast_receipt(base_url, target)` *without* `.await`. Store the un-awaited futures in a `Vec`.
- **`.await` the join:** `join_all(futures).await` gives you `Vec<Result<BroadcastReceipt, ChannelBroadcastError>>`.
- **`results.into_iter().collect::<Result<Vec<_>, _>>()`:** Converts `Vec<Result<T, E>>` into `Result<Vec<T>, E>`, short-circuiting on the first `Err`.

**Spoiler threshold:** Medium—names the key functions and collection idiom.
