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
    if manifest_lines.is_empty() {
        return Err(SalvageError::EmptyManifest);
    }

    let mut items = Vec::new();
    for (idx, &line) in manifest_lines.iter().enumerate() {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() != 4 {
            return Err(SalvageError::InvalidLine {
                line: idx,
                reason: format!("expected 4 fields, got {}", parts.len()),
            });
        }
        let name = parts[0].to_string();
        let mass = parts[1]
            .parse::<u32>()
            .map_err(|_| SalvageError::InvalidLine {
                line: idx,
                reason: format!("invalid mass: {}", parts[1]),
            })?;
        let priority = parts[2]
            .parse::<u8>()
            .map_err(|_| SalvageError::InvalidLine {
                line: idx,
                reason: format!("invalid priority: {}", parts[2]),
            })?;
        let fragile = parts[3]
            .parse::<bool>()
            .map_err(|_| SalvageError::InvalidLine {
                line: idx,
                reason: format!("invalid fragile flag: {}", parts[3]),
            })?;
        items.push(SalvageItem {
            name,
            mass,
            priority,
            fragile,
        });
    }

    let total_mass: u32 = items.iter().map(|i| i.mass).sum();
    let fragile_count = items.iter().filter(|i| i.fragile).count();

    if total_mass > capacity {
        return Err(SalvageError::OverCapacity {
            capacity,
            total_mass,
        });
    }

    items.sort_by(|a, b| b.priority.cmp(&a.priority));
    let top_targets = items.into_iter().map(|i| i.name).collect();

    Ok(SalvagePlan {
        total_mass,
        fragile_count,
        top_targets,
    })
}
