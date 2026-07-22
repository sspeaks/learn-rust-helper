# Quest 31: Mission Ledger

**🎮 Quest:** The fleet runs on credits. Every mission deposits or withdraws from the ledger. But the balance must never go negative—if any transaction in a batch would overdraw the account, the entire batch must be rolled back, as if it never happened. That's the power of a database transaction.

## Objective

Implement `apply_ledger_transaction` to apply a series of `LedgerEvent` deltas inside a single SQLite transaction. If any event would push the balance below zero, roll back the entire transaction and return an error. This exercise teaches atomic, all-or-nothing database operations.

## Public API

```rust
pub struct LedgerEvent {
    pub mission_code: String,
    pub delta: i64,
}

pub struct LedgerOutcome {
    pub applied: usize,
    pub final_balance: i64,
}

pub enum MissionLedgerError {
    Sql(rusqlite::Error),
    NegativeBalance { attempted_balance: i64 },
}

pub fn apply_ledger_transaction(
    conn: &Connection,
    events: &[LedgerEvent],
) -> Result<LedgerOutcome, MissionLedgerError>
```

## Behavioral Rules

1. **Begin a transaction** before processing any events.
2. **Track a running balance.** Start from the current stored balance (or 0 if no prior ledger row exists).
3. **For each event:**
   - Add `event.delta` to the running balance.
   - If the new balance is negative, roll back the transaction and return `MissionLedgerError::NegativeBalance { attempted_balance }`.
   - Otherwise, insert the event (with `mission_code` and updated balance) into the ledger table.
4. **Commit** only after all events are applied successfully.
5. **Return** `Ok(LedgerOutcome { applied: events.len(), final_balance })` on success.
6. Any SQL error that is not a negative-balance check wraps in `MissionLedgerError::Sql`.

## Concepts Practiced

- **`conn.execute("BEGIN")`** or **`rusqlite::Transaction`:** Wrapping operations in an atomic unit.
- **`conn.execute("ROLLBACK")`** or **`transaction.rollback()`:** Undoing all changes if any step fails.
- **`conn.execute("COMMIT")`** or **`transaction.commit()`:** Making changes permanent.
- **Running balance:** Maintaining a value in memory across loop iterations.
- **Business rule enforcement:** Returning a domain error (not a SQL error) when a constraint is violated.

## Setup Notes

SQLite uses the **bundled** feature—no system installation needed. The first build may take 30–45 seconds. Tests use `Connection::open_in_memory()`. The test harness creates the required ledger table before calling your function.

## Edge Cases

- Empty `events` slice (no changes; return `Ok(LedgerOutcome { applied: 0, final_balance: 0 })` or current balance).
- A batch where the first event would create a negative balance (roll back immediately).
- A batch where a middle event creates a negative balance (roll back all previously applied events in the batch).
- Events with zero delta (allowed; no balance change).

## How to Work on This Quest

**From inside `nix develop path:.`:**

```bash
# Check your work
learn check ex31-mission-ledger

# Get a hint if stuck
learn hint ex31-mission-ledger

# Jump to a specific hint level
learn hint ex31-mission-ledger --level 2
```

**Omit the exercise ID to use your current recommendation:**

```bash
learn check
learn hint
```

## XP Reward

**440 XP** for first completion.

## Prerequisites

Complete **Crew Manifest** (ex30).

## Success Criteria

- All events in a batch are applied atomically (commit) or none are (rollback).
- A negative balance triggers rollback and returns `NegativeBalance`.
- `LedgerOutcome.applied` equals the number of events in the batch.
- `LedgerOutcome.final_balance` reflects the committed balance.
- An empty event batch returns a zero-applied outcome.
- SQL errors wrap in `MissionLedgerError::Sql`.

## Next Steps

Complete this quest to unlock **Archive Query** (ex32), where you'll filter and retrieve records with parameterized WHERE clauses.
