## Hint 2: Tools & Types

- **`let (tx, rx) = std::sync::mpsc::channel::<CargoReceipt>()`:** Creates a sender `tx` and receiver `rx`.
- **`tx.clone()`:** `Sender<T>` is `Clone`. Clone the sender once per thread and move the clone into the thread closure.
- **`drop(tx)`:** After cloning for all workers, drop the original sender. This ensures the channel closes when all worker senders are dropped.
- **`tx_clone.send(receipt).map_err(|_| CargoChannelError::SendFailed)?`:** Send from inside the worker thread.
- **`rx.iter().collect::<Vec<_>>()`:** On the main thread, drain all messages until the channel closes. Returns `Vec<CargoReceipt>`.
- **`handle.join().map_err(|_| CargoChannelError::WorkerPanicked)?`:** Join handles after collecting.

**Spoiler threshold:** Medium—names every key call in the correct order.
