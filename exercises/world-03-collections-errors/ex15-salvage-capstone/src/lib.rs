#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SalvageItem {
    pub name: String,
    pub mass: u32,
    pub priority: u8,
    pub fragile: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SalvagePlan {
    pub total_mass: u32,
    pub fragile_count: usize,
    pub top_targets: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SalvageError {
    EmptyManifest,
    InvalidLine { line: usize, reason: String },
    OverCapacity { capacity: u32, total_mass: u32 },
}

pub fn build_salvage_plan(
    manifest_lines: &[&str],
    capacity: u32,
) -> Result<SalvagePlan, SalvageError> {
    todo!("Parse manifest lines, aggregate metrics, and enforce capacity with explicit errors")
}
