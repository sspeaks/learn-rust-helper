## Hint 3: Algorithm Outline

```
function apply_ledger_transaction(conn, events):
    Step 1: Read current balance from the ledger table (0 if no rows yet)
            → map SQL error to MissionLedgerError::Sql

    Step 2: Begin a transaction on conn
            → map SQL error to MissionLedgerError::Sql

    Step 3: Let balance = current_balance, applied = 0

    Step 4: For each event in events:
            balance += event.delta
            If balance < 0:
                → rollback the transaction
                → return MissionLedgerError::NegativeBalance { attempted_balance: balance }
            Execute INSERT into ledger (mission_code, delta, balance)
                → map SQL error to MissionLedgerError::Sql
            applied += 1

    Step 5: Commit the transaction
            → map SQL error to MissionLedgerError::Sql

    Step 6: Return Ok(LedgerOutcome { applied, final_balance: balance })
```

**Note:** Dropping the `Transaction` object without calling `.commit()` automatically triggers a rollback. You can also call `.rollback()` explicitly for clarity. Either approach is correct.

**Spoiler threshold:** High—complete algorithm without Rust syntax.
