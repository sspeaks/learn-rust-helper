use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const CAMPAIGN_SCHEMA_VERSION: u32 = 1;
const PROGRESS_SCHEMA_VERSION: u32 = 1;

pub fn run() -> Result<(), XtaskError> {
    let cli = Cli::parse();
    let cwd = std::env::current_dir().map_err(XtaskError::Io)?;
    let root = discover_workspace_root(&cwd)?;

    match cli.command {
        None => cmd_dashboard(&root),
        Some(Commands::Check { id }) => cmd_check(&root, id.as_deref()),
        Some(Commands::Verify { id }) => cmd_check(&root, Some(&id)),
        Some(Commands::Status) => cmd_status(&root),
        Some(Commands::Next) => cmd_next(&root),
        Some(Commands::Hint { id, level }) => cmd_hint(&root, id.as_deref(), level),
    }
}

#[derive(Debug, Parser)]
#[command(name = "learn")]
#[command(about = "Guided runner for the learn-rust campaign")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Run tests for an exercise and update progress on success.
    Check {
        /// Exercise ID (defaults to the current recommended exercise).
        id: Option<String>,
    },
    /// Legacy alias for check: run tests for an exercise.
    Verify {
        /// Exercise ID.
        id: String,
    },
    /// Show XP, rank, and world-by-world progress.
    Status,
    /// Print the next incomplete exercise ID (script-friendly).
    Next,
    /// Show a hint; auto-advances to the next unseen level unless --level is given.
    Hint {
        /// Exercise ID (defaults to the current recommended exercise).
        id: Option<String>,
        /// Hint level 1–3 (overrides auto-advance when provided).
        #[arg(long)]
        level: Option<u8>,
    },
}

#[derive(Debug)]
pub enum XtaskError {
    Io(std::io::Error),
    MissingWorkspace(PathBuf),
    MissingCampaign(PathBuf),
    CampaignParse(toml::de::Error),
    CampaignSerialize(toml::ser::Error),
    InvalidCampaign(String),
    UnknownExercise(String),
    InvalidHintLevel(u8),
    MissingHint(PathBuf),
    SubprocessFailed {
        command: String,
        code: Option<i32>,
    },
    ProgressParse(toml::de::Error),
    /// Tests ran and failed; output already shown — just exit 1 silently.
    CheckFailed,
}

impl Display for XtaskError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            XtaskError::Io(err) => write!(f, "I/O error: {err}"),
            XtaskError::MissingWorkspace(start) => {
                write!(f, "could not find workspace root from {}", start.display())
            }
            XtaskError::MissingCampaign(path) => {
                write!(f, "campaign metadata is missing: {}", path.display())
            }
            XtaskError::CampaignParse(err) => write!(f, "failed to parse campaign.toml: {err}"),
            XtaskError::CampaignSerialize(err) => {
                write!(f, "failed to serialize progress file: {err}")
            }
            XtaskError::InvalidCampaign(message) => {
                write!(f, "invalid campaign metadata: {message}")
            }
            XtaskError::UnknownExercise(id) => write!(f, "unknown exercise id: {id}"),
            XtaskError::InvalidHintLevel(level) => {
                write!(f, "hint level must be 1, 2, or 3 (got {level})")
            }
            XtaskError::MissingHint(path) => {
                write!(f, "hint file does not exist: {}", path.display())
            }
            XtaskError::SubprocessFailed { command, code } => match code {
                Some(code) => write!(f, "command failed with exit code {code}: {command}"),
                None => write!(f, "command terminated by signal: {command}"),
            },
            XtaskError::ProgressParse(err) => {
                write!(f, "failed to parse .learn-rust/progress.toml: {err}")
            }
            XtaskError::CheckFailed => write!(f, "tests failed"),
        }
    }
}

impl std::error::Error for XtaskError {}

#[derive(Debug, Clone, Deserialize)]
pub struct Campaign {
    pub schema_version: u32,
    pub title: String,
    pub ranks: Vec<Rank>,
    pub worlds: Vec<World>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Rank {
    pub id: String,
    pub name: String,
    pub min_xp: u32,
    pub badge: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct World {
    pub id: String,
    pub name: String,
    pub theme: String,
    pub unlock_text: String,
    pub exercises: Vec<Exercise>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Exercise {
    pub id: String,
    pub package: String,
    pub title: String,
    pub skill: String,
    pub xp: u32,
    #[serde(default)]
    pub prerequisites: Vec<String>,
    #[serde(default)]
    pub unlocks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressFile {
    pub schema_version: u32,
    pub earned_xp: u32,
    pub completed: Vec<String>,
    /// Per-exercise highest hint level seen (1–3).  Added additively; absent means none viewed.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub hints_viewed: HashMap<String, u8>,
}

impl Default for ProgressFile {
    fn default() -> Self {
        Self {
            schema_version: PROGRESS_SCHEMA_VERSION,
            earned_xp: 0,
            completed: Vec::new(),
            hints_viewed: HashMap::new(),
        }
    }
}

impl ProgressFile {
    fn normalize(&mut self) {
        self.completed.sort();
        self.completed.dedup();
    }

    fn completed_set(&self) -> BTreeSet<String> {
        self.completed.iter().cloned().collect()
    }

    fn mark_completed(&mut self, id: &str, xp: u32) -> bool {
        if self.completed.iter().any(|existing| existing == id) {
            return false;
        }

        self.completed.push(id.to_owned());
        self.normalize();
        self.earned_xp = self.earned_xp.saturating_add(xp);
        true
    }
}

pub fn discover_workspace_root(start: &Path) -> Result<PathBuf, XtaskError> {
    for dir in start.ancestors() {
        let candidate = dir.join("Cargo.toml");
        if !candidate.exists() {
            continue;
        }

        let Ok(content) = fs::read_to_string(&candidate) else {
            continue;
        };

        let Ok(value) = toml::from_str::<toml::Value>(&content) else {
            continue;
        };

        let members = value
            .get("workspace")
            .and_then(|workspace| workspace.get("members"))
            .and_then(|members| members.as_array());

        if let Some(members) = members {
            let has_xtask = members
                .iter()
                .filter_map(toml::Value::as_str)
                .any(|member| member == "xtask");
            if has_xtask {
                return Ok(dir.to_path_buf());
            }
        }
    }

    Err(XtaskError::MissingWorkspace(start.to_path_buf()))
}

pub fn load_campaign(root: &Path) -> Result<Campaign, XtaskError> {
    let path = root.join("campaign.toml");
    if !path.exists() {
        return Err(XtaskError::MissingCampaign(path));
    }

    let content = fs::read_to_string(path).map_err(XtaskError::Io)?;
    let campaign: Campaign = toml::from_str(&content).map_err(XtaskError::CampaignParse)?;
    validate_campaign(root, &campaign)?;
    Ok(campaign)
}

fn validate_campaign(root: &Path, campaign: &Campaign) -> Result<(), XtaskError> {
    if campaign.schema_version != CAMPAIGN_SCHEMA_VERSION {
        return Err(XtaskError::InvalidCampaign(format!(
            "schema_version must be {CAMPAIGN_SCHEMA_VERSION}"
        )));
    }

    if campaign.title.trim().is_empty() {
        return Err(XtaskError::InvalidCampaign(
            "title must not be empty".to_owned(),
        ));
    }

    if campaign.worlds.len() != 3 {
        return Err(XtaskError::InvalidCampaign(
            "expected exactly 3 worlds".to_owned(),
        ));
    }

    if campaign.ranks.is_empty() {
        return Err(XtaskError::InvalidCampaign(
            "ranks must not be empty".to_owned(),
        ));
    }

    let mut seen_rank_ids = HashSet::new();
    let mut previous_min_xp = None;
    for rank in &campaign.ranks {
        if !seen_rank_ids.insert(rank.id.clone()) {
            return Err(XtaskError::InvalidCampaign(format!(
                "duplicate rank id: {}",
                rank.id
            )));
        }

        if rank.name.trim().is_empty() || rank.badge.trim().is_empty() {
            return Err(XtaskError::InvalidCampaign(format!(
                "rank {} must include non-empty name and badge",
                rank.id
            )));
        }

        if let Some(previous) = previous_min_xp {
            if rank.min_xp <= previous {
                return Err(XtaskError::InvalidCampaign(
                    "ranks must be sorted by strictly increasing min_xp".to_owned(),
                ));
            }
        }
        previous_min_xp = Some(rank.min_xp);
    }

    if campaign.ranks.first().map(|rank| rank.min_xp) != Some(0) {
        return Err(XtaskError::InvalidCampaign(
            "first rank must start at min_xp = 0".to_owned(),
        ));
    }

    let mut seen_world_ids = HashSet::new();
    let mut seen_exercise_ids = HashSet::new();
    let mut seen_packages = HashSet::new();
    let mut id_to_world = HashMap::new();

    for world in &campaign.worlds {
        if !is_valid_world_id(&world.id) {
            return Err(XtaskError::InvalidCampaign(format!(
                "world id must match world-NN-name: {}",
                world.id
            )));
        }

        if !seen_world_ids.insert(world.id.clone()) {
            return Err(XtaskError::InvalidCampaign(format!(
                "duplicate world id: {}",
                world.id
            )));
        }

        if world.exercises.len() != 5 {
            return Err(XtaskError::InvalidCampaign(format!(
                "world {} must contain exactly 5 exercises",
                world.id
            )));
        }

        for exercise in &world.exercises {
            if !is_valid_exercise_id(&exercise.id) {
                return Err(XtaskError::InvalidCampaign(format!(
                    "exercise id must match exNN-kebab-name: {}",
                    exercise.id
                )));
            }

            if exercise.xp == 0 {
                return Err(XtaskError::InvalidCampaign(format!(
                    "exercise {} must have xp > 0",
                    exercise.id
                )));
            }

            if exercise.title.trim().is_empty() || exercise.skill.trim().is_empty() {
                return Err(XtaskError::InvalidCampaign(format!(
                    "exercise {} must include non-empty title and skill",
                    exercise.id
                )));
            }

            if !seen_exercise_ids.insert(exercise.id.clone()) {
                return Err(XtaskError::InvalidCampaign(format!(
                    "duplicate exercise id: {}",
                    exercise.id
                )));
            }

            if !seen_packages.insert(exercise.package.clone()) {
                return Err(XtaskError::InvalidCampaign(format!(
                    "duplicate package name: {}",
                    exercise.package
                )));
            }

            let expected_package = exercise.id.as_str();
            if exercise.package != expected_package {
                return Err(XtaskError::InvalidCampaign(format!(
                    "exercise {} package must match id (expected {expected_package})",
                    exercise.id
                )));
            }

            id_to_world.insert(exercise.id.clone(), world.id.clone());

            let cargo_toml = root
                .join("exercises")
                .join(&world.id)
                .join(&exercise.id)
                .join("Cargo.toml");
            if !cargo_toml.exists() {
                return Err(XtaskError::InvalidCampaign(format!(
                    "exercise {} is missing Cargo.toml at {}",
                    exercise.id,
                    cargo_toml.display()
                )));
            }
        }
    }

    for exercise_ref in campaign.exercises_in_order() {
        for prereq in &exercise_ref.exercise.prerequisites {
            if !seen_exercise_ids.contains(prereq) {
                return Err(XtaskError::InvalidCampaign(format!(
                    "exercise {} has unknown prerequisite {}",
                    exercise_ref.exercise.id, prereq
                )));
            }

            if prereq == &exercise_ref.exercise.id {
                return Err(XtaskError::InvalidCampaign(format!(
                    "exercise {} cannot depend on itself",
                    exercise_ref.exercise.id
                )));
            }
        }

        for unlocked in &exercise_ref.exercise.unlocks {
            if !seen_exercise_ids.contains(unlocked) {
                return Err(XtaskError::InvalidCampaign(format!(
                    "exercise {} references unknown unlock {}",
                    exercise_ref.exercise.id, unlocked
                )));
            }

            if id_to_world.get(unlocked) == Some(&exercise_ref.world.id)
                && unlocked == &exercise_ref.exercise.id
            {
                return Err(XtaskError::InvalidCampaign(format!(
                    "exercise {} cannot unlock itself",
                    exercise_ref.exercise.id
                )));
            }
        }
    }

    if seen_exercise_ids.len() != 15 {
        return Err(XtaskError::InvalidCampaign(
            "expected exactly 15 exercises".to_owned(),
        ));
    }

    Ok(())
}

fn is_valid_world_id(id: &str) -> bool {
    let mut parts = id.splitn(3, '-');
    matches!(
        (parts.next(), parts.next(), parts.next()),
        (Some("world"), Some(number), Some(name))
            if number.len() == 2
                && number.chars().all(|c| c.is_ascii_digit())
                && !name.is_empty()
                && name.chars().all(|c| c.is_ascii_lowercase() || c == '-')
    )
}

fn is_valid_exercise_id(id: &str) -> bool {
    let mut parts = id.splitn(2, '-');
    match (parts.next(), parts.next()) {
        (Some(prefix), Some(name)) => {
            prefix.len() == 4
                && prefix.starts_with("ex")
                && prefix[2..].chars().all(|c| c.is_ascii_digit())
                && !name.is_empty()
                && name
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        }
        _ => false,
    }
}

pub struct ExerciseRef<'a> {
    pub world: &'a World,
    pub exercise: &'a Exercise,
}

impl Campaign {
    pub fn exercises_in_order(&self) -> Vec<ExerciseRef<'_>> {
        let mut output = Vec::new();
        for world in &self.worlds {
            for exercise in &world.exercises {
                output.push(ExerciseRef { world, exercise });
            }
        }
        output
    }

    pub fn resolve_exercise(&self, id: &str) -> Option<ExerciseRef<'_>> {
        self.exercises_in_order()
            .into_iter()
            .find(|exercise| exercise.exercise.id == id)
    }
}

pub fn load_progress(root: &Path) -> Result<ProgressFile, XtaskError> {
    let path = progress_path(root);
    if !path.exists() {
        return Ok(ProgressFile::default());
    }

    let content = fs::read_to_string(path).map_err(XtaskError::Io)?;
    let mut progress: ProgressFile = toml::from_str(&content).map_err(XtaskError::ProgressParse)?;
    if progress.schema_version != PROGRESS_SCHEMA_VERSION {
        return Err(XtaskError::InvalidCampaign(format!(
            "progress schema_version must be {PROGRESS_SCHEMA_VERSION}"
        )));
    }
    progress.normalize();
    Ok(progress)
}

pub fn save_progress(root: &Path, progress: &ProgressFile) -> Result<(), XtaskError> {
    let path = progress_path(root);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(XtaskError::Io)?;
    }
    let text = toml::to_string_pretty(progress).map_err(XtaskError::CampaignSerialize)?;
    fs::write(path, text).map_err(XtaskError::Io)
}

fn progress_path(root: &Path) -> PathBuf {
    root.join(".learn-rust").join("progress.toml")
}

fn run_exercise_tests(root: &Path, package: &str) -> Result<(), XtaskError> {
    // Honour the CARGO env var so integration tests can inject a fake binary.
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status = Command::new(&cargo)
        .arg("test")
        .arg("--package")
        .arg(package)
        .current_dir(root)
        .status()
        .map_err(XtaskError::Io)?;

    if status.success() {
        Ok(())
    } else {
        Err(XtaskError::SubprocessFailed {
            command: format!("cargo test --package {package}"),
            code: status.code(),
        })
    }
}

fn cmd_dashboard(root: &Path) -> Result<(), XtaskError> {
    let campaign = load_campaign(root)?;
    let progress = load_progress(root)?;
    let completed = progress.completed_set();
    let rank = current_rank(&campaign, progress.earned_xp);
    let total_xp: u32 = campaign
        .exercises_in_order()
        .iter()
        .map(|e| e.exercise.xp)
        .sum();

    println!("{}", campaign.title);
    println!(
        "{}  {}  ·  {} / {} XP",
        rank.badge, rank.name, progress.earned_xp, total_xp
    );
    println!();

    for world in &campaign.worlds {
        let done = world
            .exercises
            .iter()
            .filter(|e| completed.contains(&e.id))
            .count();
        let bar: String = world
            .exercises
            .iter()
            .map(|e| {
                if completed.contains(&e.id) {
                    '█'
                } else {
                    '░'
                }
            })
            .collect();
        println!(
            "  {:<28} {}/{}  {}",
            world.name,
            done,
            world.exercises.len(),
            bar
        );
    }

    println!();

    match choose_next_exercise(&campaign, &completed) {
        Some(next) => {
            println!("▶ Next: {} — {}", next.exercise.id, next.exercise.title);
            let edit_path = root
                .join("exercises")
                .join(&next.world.id)
                .join(&next.exercise.id)
                .join("src")
                .join("lib.rs");
            println!("  📂 Edit: {}", edit_path.display());
            println!();
            println!("  learn check          verify your solution");
            println!("  learn hint           get a hint (auto-advances each call)");
            println!("  learn status         detailed progress");
            println!("  learn next           next exercise ID only");
        }
        None => {
            println!("🎉 All exercises complete — great work!");
        }
    }

    Ok(())
}

fn cmd_check(root: &Path, id: Option<&str>) -> Result<(), XtaskError> {
    let campaign = load_campaign(root)?;
    let progress = load_progress(root)?;
    let completed = progress.completed_set();

    let exercise_ref = match id {
        Some(id) => campaign
            .resolve_exercise(id)
            .ok_or_else(|| XtaskError::UnknownExercise(id.to_owned()))?,
        None => match choose_next_exercise(&campaign, &completed) {
            Some(e) => e,
            None => {
                println!("🎉 All exercises complete — nothing left to check!");
                return Ok(());
            }
        },
    };

    let ex_id = exercise_ref.exercise.id.clone();
    let ex_xp = exercise_ref.exercise.xp;
    let ex_package = exercise_ref.exercise.package.clone();

    match run_exercise_tests(root, &ex_package) {
        Ok(()) => {
            let mut progress = load_progress(root)?;
            if progress.mark_completed(&ex_id, ex_xp) {
                save_progress(root, &progress)?;
                println!(
                    "✅ {} verified. +{} XP (total: {}).",
                    ex_id, ex_xp, progress.earned_xp
                );
            } else {
                println!(
                    "✅ {} verified again. XP unchanged (total: {}).",
                    ex_id, progress.earned_xp
                );
            }
            Ok(())
        }
        Err(_) => {
            eprintln!();
            eprintln!("💡 Tip: run `learn hint {}` for a nudge.", ex_id);
            Err(XtaskError::CheckFailed)
        }
    }
}

fn current_rank<'a>(campaign: &'a Campaign, xp: u32) -> &'a Rank {
    campaign
        .ranks
        .iter()
        .rev()
        .find(|rank| xp >= rank.min_xp)
        .unwrap_or(&campaign.ranks[0])
}

fn cmd_status(root: &Path) -> Result<(), XtaskError> {
    let campaign = load_campaign(root)?;
    let progress = load_progress(root)?;
    let completed = progress.completed_set();
    let rank = current_rank(&campaign, progress.earned_xp);

    println!("Campaign: {}", campaign.title);
    println!("XP: {}", progress.earned_xp);
    println!("Rank: {} {}", rank.badge, rank.name);
    println!();

    for world in &campaign.worlds {
        let done = world
            .exercises
            .iter()
            .filter(|exercise| completed.contains(&exercise.id))
            .count();
        println!(
            "{}: {}/{} complete",
            world.name,
            done,
            world.exercises.len()
        );
    }

    if let Some(next) = choose_next_exercise(&campaign, &completed) {
        println!();
        println!(
            "Recommended next: {} — {}",
            next.exercise.id, next.exercise.title
        );
    }

    Ok(())
}

fn choose_next_exercise<'a>(
    campaign: &'a Campaign,
    completed: &BTreeSet<String>,
) -> Option<ExerciseRef<'a>> {
    let all = campaign.exercises_in_order();

    if let Some(unlocked) = all.iter().find(|candidate| {
        !completed.contains(&candidate.exercise.id)
            && candidate
                .exercise
                .prerequisites
                .iter()
                .all(|prereq| completed.contains(prereq))
    }) {
        return Some(ExerciseRef {
            world: unlocked.world,
            exercise: unlocked.exercise,
        });
    }

    all.into_iter()
        .find(|candidate| !completed.contains(&candidate.exercise.id))
}

fn cmd_next(root: &Path) -> Result<(), XtaskError> {
    let campaign = load_campaign(root)?;
    let progress = load_progress(root)?;
    let completed = progress.completed_set();

    match choose_next_exercise(&campaign, &completed) {
        Some(candidate) => {
            let missing_prereqs: Vec<_> = candidate
                .exercise
                .prerequisites
                .iter()
                .filter(|prereq| !completed.contains(*prereq))
                .cloned()
                .collect();

            if missing_prereqs.is_empty() {
                println!("{}", candidate.exercise.id);
            } else {
                println!(
                    "{} (recommended path missing prerequisites: {})",
                    candidate.exercise.id,
                    missing_prereqs.join(", ")
                );
            }
            Ok(())
        }
        None => {
            println!("All exercises are complete. Great work!");
            Ok(())
        }
    }
}

fn cmd_hint(root: &Path, id: Option<&str>, level: Option<u8>) -> Result<(), XtaskError> {
    let campaign = load_campaign(root)?;
    let mut progress = load_progress(root)?;
    let completed = progress.completed_set();

    let exercise_ref = match id {
        Some(id) => campaign
            .resolve_exercise(id)
            .ok_or_else(|| XtaskError::UnknownExercise(id.to_owned()))?,
        None => match choose_next_exercise(&campaign, &completed) {
            Some(e) => e,
            None => {
                println!("🎉 All exercises complete — no hints needed!");
                return Ok(());
            }
        },
    };

    let ex_id = exercise_ref.exercise.id.clone();
    let world_id = exercise_ref.world.id.clone();

    let effective_level = match level {
        Some(l) => {
            if !(1..=3).contains(&l) {
                return Err(XtaskError::InvalidHintLevel(l));
            }
            l
        }
        None => {
            let viewed = progress.hints_viewed.get(&ex_id).copied().unwrap_or(0);
            (viewed + 1).min(3)
        }
    };

    let hint_path = root
        .join("exercises")
        .join(&world_id)
        .join(&ex_id)
        .join("hints")
        .join(format!("hint{effective_level}.md"));

    if !hint_path.exists() {
        return Err(XtaskError::MissingHint(hint_path));
    }

    let hint = fs::read_to_string(&hint_path).map_err(XtaskError::Io)?;

    // Persist: record this level if it advances beyond what was seen before.
    let current_max = progress.hints_viewed.get(&ex_id).copied().unwrap_or(0);
    if effective_level > current_max {
        progress.hints_viewed.insert(ex_id.clone(), effective_level);
        save_progress(root, &progress)?;
    }

    print!("{hint}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_campaign() -> Campaign {
        toml::from_str(
            r#"
                schema_version = 1
                title = "Campaign"

                [[ranks]]
                id = "rank-cadet"
                name = "Cadet"
                min_xp = 0
                badge = "🛰️"

                [[ranks]]
                id = "rank-pilot"
                name = "Pilot"
                min_xp = 100
                badge = "🚀"

                [[worlds]]
                id = "world-01-foundations"
                name = "Foundations"
                theme = "Base systems"
                unlock_text = "Start your training"

                  [[worlds.exercises]]
                  id = "ex01-alpha"
                  package = "ex01-alpha"
                  title = "Alpha"
                  skill = "A"
                  xp = 10
                  prerequisites = []

                  [[worlds.exercises]]
                  id = "ex02-beta"
                  package = "ex02-beta"
                  title = "Beta"
                  skill = "B"
                  xp = 10
                  prerequisites = ["ex01-alpha"]

                  [[worlds.exercises]]
                  id = "ex03-gamma"
                  package = "ex03-gamma"
                  title = "Gamma"
                  skill = "C"
                  xp = 10

                  [[worlds.exercises]]
                  id = "ex04-delta"
                  package = "ex04-delta"
                  title = "Delta"
                  skill = "D"
                  xp = 10

                  [[worlds.exercises]]
                  id = "ex05-epsilon"
                  package = "ex05-epsilon"
                  title = "Epsilon"
                  skill = "E"
                  xp = 10

                [[worlds]]
                id = "world-02-ownership"
                name = "Ownership"
                theme = "Flow"
                unlock_text = "Trust the borrow checker"

                  [[worlds.exercises]]
                  id = "ex06-zeta"
                  package = "ex06-zeta"
                  title = "Zeta"
                  skill = "Z"
                  xp = 10

                  [[worlds.exercises]]
                  id = "ex07-eta"
                  package = "ex07-eta"
                  title = "Eta"
                  skill = "Eta"
                  xp = 10

                  [[worlds.exercises]]
                  id = "ex08-theta"
                  package = "ex08-theta"
                  title = "Theta"
                  skill = "Theta"
                  xp = 10

                  [[worlds.exercises]]
                  id = "ex09-iota"
                  package = "ex09-iota"
                  title = "Iota"
                  skill = "Iota"
                  xp = 10

                  [[worlds.exercises]]
                  id = "ex10-kappa"
                  package = "ex10-kappa"
                  title = "Kappa"
                  skill = "Kappa"
                  xp = 10

                [[worlds]]
                id = "world-03-collections-errors"
                name = "Collections + Errors"
                theme = "State"
                unlock_text = "Scale your logic"

                  [[worlds.exercises]]
                  id = "ex11-lambda"
                  package = "ex11-lambda"
                  title = "Lambda"
                  skill = "Lambda"
                  xp = 10

                  [[worlds.exercises]]
                  id = "ex12-mu"
                  package = "ex12-mu"
                  title = "Mu"
                  skill = "Mu"
                  xp = 10

                  [[worlds.exercises]]
                  id = "ex13-nu"
                  package = "ex13-nu"
                  title = "Nu"
                  skill = "Nu"
                  xp = 10

                  [[worlds.exercises]]
                  id = "ex14-xi"
                  package = "ex14-xi"
                  title = "Xi"
                  skill = "Xi"
                  xp = 10

                  [[worlds.exercises]]
                  id = "ex15-omicron"
                  package = "ex15-omicron"
                  title = "Omicron"
                  skill = "Omicron"
                  xp = 10
            "#,
        )
        .expect("sample campaign should parse")
    }

    #[test]
    fn progress_is_idempotent_for_completed_exercises() {
        let mut progress = ProgressFile::default();
        assert!(progress.mark_completed("ex01-alpha", 25));
        assert_eq!(progress.earned_xp, 25);

        assert!(!progress.mark_completed("ex01-alpha", 25));
        assert_eq!(progress.earned_xp, 25);
        assert_eq!(progress.completed, vec!["ex01-alpha".to_owned()]);
    }

    #[test]
    fn choose_next_prefers_unlocked_exercise() {
        let campaign = sample_campaign();
        let completed: BTreeSet<_> = ["ex01-alpha".to_owned()].into_iter().collect();

        let next = choose_next_exercise(&campaign, &completed).expect("exercise expected");
        assert_eq!(next.exercise.id, "ex02-beta");
    }

    #[test]
    fn choose_next_falls_back_to_first_incomplete_when_locked() {
        let mut campaign = sample_campaign();
        for world in &mut campaign.worlds {
            for exercise in &mut world.exercises {
                exercise.prerequisites = vec!["ex15-omicron".to_owned()];
            }
        }

        let completed = BTreeSet::new();
        let next = choose_next_exercise(&campaign, &completed).expect("exercise expected");
        assert_eq!(next.exercise.id, "ex01-alpha");
    }

    #[test]
    fn invalid_exercise_id_is_detected() {
        assert!(!is_valid_exercise_id("exercise-01"));
        assert!(is_valid_exercise_id("ex01-format-scoreboard"));
    }

    #[test]
    fn progress_file_deserializes_without_hints_viewed() {
        let toml = r#"
            schema_version = 1
            earned_xp = 50
            completed = ["ex01-alpha"]
        "#;
        let progress: ProgressFile =
            toml::from_str(toml).expect("should parse without hints_viewed");
        assert_eq!(progress.earned_xp, 50);
        assert!(progress.hints_viewed.is_empty());
    }

    #[test]
    fn hints_viewed_auto_advance_increments_from_zero() {
        let progress = ProgressFile::default();
        let viewed = progress
            .hints_viewed
            .get("ex01-alpha")
            .copied()
            .unwrap_or(0);
        let next_level = (viewed + 1).min(3);
        assert_eq!(next_level, 1);
    }

    #[test]
    fn hints_viewed_auto_advance_caps_at_three() {
        let mut progress = ProgressFile::default();
        progress.hints_viewed.insert("ex01-alpha".to_owned(), 3);
        let viewed = progress
            .hints_viewed
            .get("ex01-alpha")
            .copied()
            .unwrap_or(0);
        let next_level = (viewed + 1).min(3);
        assert_eq!(next_level, 3);
    }
}
