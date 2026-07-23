use rusqlite::Connection;

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
    // ══════════════════════════════════════════════════════════════
    // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
    // ══════════════════════════════════════════════════════════════
    todo!("Use a transaction and roll back when any event would create a negative balance")
}
