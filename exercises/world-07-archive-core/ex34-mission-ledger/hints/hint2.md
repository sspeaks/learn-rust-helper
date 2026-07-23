## Hint 2: Tools & Types

- **`let tx = conn.unchecked_transaction().map_err(MissionLedgerError::Sql)?`:**
  Begins a transaction. `tx` is a `rusqlite::Transaction<'_>`.
- **`tx.execute("INSERT INTO ...", params![...])`:** Run DML within the transaction.
- **`tx.rollback()`** (or just drop `tx` without committing): Undoes all changes in this transaction.
- **`tx.commit().map_err(MissionLedgerError::Sql)?`:** Commits all changes atomically.
- **`let balance = tx.query_row("SELECT COALESCE(MAX(balance), 0) FROM ledger", [], |r| r.get::<_, i64>(0))`:**
  One pattern for reading the current balance.
- **Running balance in a variable:** Increment `balance += event.delta` in a loop; check `balance < 0` before inserting.

**Spoiler threshold:** Medium—names the key rusqlite transaction methods.
