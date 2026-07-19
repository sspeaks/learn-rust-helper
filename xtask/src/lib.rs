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
        Commands::Verify { id } => cmd_verify(&root, &id),
        Commands::Status => cmd_status(&root),
        Commands::Next => cmd_next(&root),
        Commands::Hint { id, level } => cmd_hint(&root, &id, level),
    }
}

#[derive(Debug, Parser)]
#[command(name = "cargo xtask")]
#[command(about = "Local automation for the Rust learning campaign")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Run tests for one exercise and update progress on success.
    Verify { id: String },
    /// Show XP, rank, and world-by-world progress.
    Status,
    /// Recommend the next incomplete exercise.
    Next,
    /// Print a hint level for an exercise.
    Hint {
        id: String,
        #[arg(long, default_value_t = 1)]
        level: u8,
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
    SubprocessFailed { command: String, code: Option<i32> },
    ProgressParse(toml::de::Error),
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
}

impl Default for ProgressFile {
    fn default() -> Self {
        Self {
            schema_version: PROGRESS_SCHEMA_VERSION,
            earned_xp: 0,
            completed: Vec::new(),
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
    let status = Command::new("cargo")
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

fn cmd_verify(root: &Path, id: &str) -> Result<(), XtaskError> {
    let campaign = load_campaign(root)?;
    let exercise = campaign
        .resolve_exercise(id)
        .ok_or_else(|| XtaskError::UnknownExercise(id.to_owned()))?;

    run_exercise_tests(root, &exercise.exercise.package)?;

    let mut progress = load_progress(root)?;
    if progress.mark_completed(id, exercise.exercise.xp) {
        save_progress(root, &progress)?;
        println!(
            "✅ {} verified. +{} XP (total: {}).",
            id, exercise.exercise.xp, progress.earned_xp
        );
    } else {
        println!(
            "✅ {} verified again. XP unchanged (total: {}).",
            id, progress.earned_xp
        );
    }

    Ok(())
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

fn cmd_hint(root: &Path, id: &str, level: u8) -> Result<(), XtaskError> {
    if !(1..=3).contains(&level) {
        return Err(XtaskError::InvalidHintLevel(level));
    }

    let campaign = load_campaign(root)?;
    let exercise = campaign
        .resolve_exercise(id)
        .ok_or_else(|| XtaskError::UnknownExercise(id.to_owned()))?;

    let hint_path = root
        .join("exercises")
        .join(&exercise.world.id)
        .join(&exercise.exercise.id)
        .join("hints")
        .join(format!("hint{level}.md"));

    if !hint_path.exists() {
        return Err(XtaskError::MissingHint(hint_path));
    }

    let hint = fs::read_to_string(hint_path).map_err(XtaskError::Io)?;
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
}
