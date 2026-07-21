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
    /*
       PARSE PHASE
    */
    let mut plans: Vec<SalvageItem> = Vec::new();
    for (ind, &line) in manifest_lines.iter().enumerate() {
        let parts: Vec<&str> = line.split("|").collect();
        let name = parts
            .get(0)
            .ok_or(SalvageError::InvalidLine {
                line: ind,
                reason: String::from("name isn't there"),
            })
            .map(|&v| v.to_string())?;
        let mass = parts
            .get(1)
            .ok_or(SalvageError::InvalidLine {
                line: ind,
                reason: String::from("mass isn't there"),
            })
            .and_then(|&v| {
                v.parse::<u32>().map_err(|_| SalvageError::InvalidLine {
                    line: ind,
                    reason: format!("mass couldn't be parsed"),
                })
            })?;
        let priority = parts
            .get(2)
            .ok_or(SalvageError::InvalidLine {
                line: ind,
                reason: String::from("priority isn't there"),
            })
            .and_then(|&v| {
                v.parse::<u8>().map_err(|_| SalvageError::InvalidLine {
                    line: ind,
                    reason: format!("priority couldn't be parsed"),
                })
            })?;
        let fragile: bool = parts
            .get(3)
            .ok_or(SalvageError::InvalidLine {
                line: ind,
                reason: String::from("fragile isn't there"),
            })
            .and_then(|&v| {
                v.parse::<bool>().map_err(|_| SalvageError::InvalidLine {
                    line: ind,
                    reason: format!("Fragile couldn't be parsed"),
                })
            })?;
        plans.push(SalvageItem {
            name: name,
            mass: mass,
            priority: priority,
            fragile: fragile,
        });
    }

    if (plans.is_empty()) {
        return Err(SalvageError::EmptyManifest);
    }

    let total_mass = plans.iter().fold(0, |acc, i| acc + i.mass);

    if total_mass > capacity {
        return Err(SalvageError::OverCapacity {
            capacity,
            total_mass,
        });
    }

    let fragile_count = plans.iter().filter(|x| x.fragile).count();
    plans.sort_by(|a, b| b.priority.cmp(&a.priority));
    let top_targets: Vec<String> = plans.into_iter().map(|x| x.name).collect();

    return Ok(SalvagePlan {
        total_mass,
        fragile_count,
        top_targets,
    });
}
