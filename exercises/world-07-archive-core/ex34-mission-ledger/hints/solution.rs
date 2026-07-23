use rusqlite::{params, Connection};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerEvent {
    pub mission_code: String,
    pub delta: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerOutcome {
    pub applied: usize,
    pub final_balance: i64,
}

#[derive(Debug)]
pub enum MissionLedgerError {
    Sql(rusqlite::Error),
    NegativeBalance { attempted_balance: i64 },
}

pub fn apply_ledger_transaction(
    conn: &Connection,
    events: &[LedgerEvent],
) -> Result<LedgerOutcome, MissionLedgerError> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS mission_ledger (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            mission_code TEXT NOT NULL,
            delta INTEGER NOT NULL,
            balance INTEGER NOT NULL
        );
        ",
    )
    .map_err(MissionLedgerError::Sql)?;

    let current_balance: i64 = conn
        .query_row(
            "SELECT COALESCE((SELECT balance FROM mission_ledger ORDER BY id DESC LIMIT 1), 0)",
            [],
            |row| row.get(0),
        )
        .map_err(MissionLedgerError::Sql)?;

    let tx = conn
        .unchecked_transaction()
        .map_err(MissionLedgerError::Sql)?;

    let mut running_balance = current_balance;
    let mut applied = 0usize;

    for event in events {
        running_balance += event.delta;
        if running_balance < 0 {
            let _ = tx.rollback();
            return Err(MissionLedgerError::NegativeBalance {
                attempted_balance: running_balance,
            });
        }

        tx.execute(
            "INSERT INTO mission_ledger (mission_code, delta, balance) VALUES (?1, ?2, ?3)",
            params![event.mission_code, event.delta, running_balance],
        )
        .map_err(MissionLedgerError::Sql)?;

        applied += 1;
    }

    tx.commit().map_err(MissionLedgerError::Sql)?;

    Ok(LedgerOutcome {
        applied,
        final_balance: running_balance,
    })
}
