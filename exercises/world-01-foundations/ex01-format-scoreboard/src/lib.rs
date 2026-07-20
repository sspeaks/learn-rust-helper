#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScoreEntry {
    pub player: String,
    pub score: i32,
    pub streak: u32,
}

impl ScoreEntry {
    pub fn new(player: impl Into<String>, score: i32, streak: u32) -> Self {
        Self {
            player: player.into(),
            score,
            streak,
        }
    }
}

pub fn format_scoreboard_line(player: &str, score: i32, rank: usize) -> String {
    // ════════════════════════════════════════════════════════════════════════════
    // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
    // ════════════════════════════════════════════════════════════════════════════
    todo!("Format a scoreboard row like '#01 | PlayerName | +0042'")
}

pub fn render_scoreboard(entries: &[ScoreEntry]) -> String {
    entries
        .iter()
        .enumerate()
        .map(|(idx, entry)| format_scoreboard_line(&entry.player, entry.score, idx + 1))
        .collect::<Vec<_>>()
        .join("\n")
}
