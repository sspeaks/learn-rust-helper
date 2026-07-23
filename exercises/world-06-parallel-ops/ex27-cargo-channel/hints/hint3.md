## Hint 3: Algorithm Outline

```
function dispatch_cargo_jobs(jobs):
    Step 1: Create mpsc channel → (tx, rx)

    Step 2: For each job (into_iter):
            Clone tx into tx_worker
            Spawn thread with move closure:
                → build CargoReceipt { cargo_id: job.cargo_id, delivered_to: job.destination }
                → tx_worker.send(receipt)  ← map send error to CargoChannelError::SendFailed
            Collect JoinHandles

    Step 3: Drop the original tx
            ← critical: channel closes when all tx clones are dropped
            ← without this drop, rx.iter() blocks forever

    Step 4: Collect receipts: rx.iter().collect()
            ← blocks until all senders are dropped

    Step 5: Join all handles
            → on panic, return CargoChannelError::WorkerPanicked

    Step 6: Return Ok(receipts)
```

**Note:** Step 3 (dropping `tx`) is the most commonly missed step. If the original sender stays alive, the receiver never knows the channel is closed and `.iter()` loops forever.

**Spoiler threshold:** High—algorithm with the critical ownership insight explained.
