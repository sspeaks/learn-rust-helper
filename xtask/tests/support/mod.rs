use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use tempfile::TempDir;

/// A self-contained temporary workspace for integration tests.
///
/// Contains a valid workspace Cargo.toml, a complete test campaign.toml,
/// minimal exercise Cargo.toml stubs for all 15 exercises, and hint files
/// for ex01-alpha.  The real `.learn-rust/progress.toml` is never touched.
pub struct TempWorkspace {
    dir: TempDir,
    pub fake_cargo: PathBuf,
}

impl TempWorkspace {
    /// Create a new isolated workspace under a system temp directory.
    pub fn new() -> Self {
        let dir = TempDir::new().expect("failed to create temp dir");
        let root = dir.path();

        // Workspace Cargo.toml — only needs to satisfy discover_workspace_root.
        fs::write(
            root.join("Cargo.toml"),
            r#"[workspace]
resolver = "2"
members = ["xtask"]
"#,
        )
        .expect("write Cargo.toml");

        // Campaign metadata.
        let fixture_campaign =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/campaign.toml");
        fs::copy(&fixture_campaign, root.join("campaign.toml")).expect("copy campaign.toml");

        // Exercise stub Cargo.toml files.
        let exercise_dirs: &[(&str, &str)] = &[
            ("world-01-alpha", "ex01-alpha"),
            ("world-01-alpha", "ex02-beta"),
            ("world-01-alpha", "ex03-gamma"),
            ("world-01-alpha", "ex04-delta"),
            ("world-01-alpha", "ex05-epsilon"),
            ("world-02-beta", "ex06-zeta"),
            ("world-02-beta", "ex07-eta"),
            ("world-02-beta", "ex08-theta"),
            ("world-02-beta", "ex09-iota"),
            ("world-02-beta", "ex10-kappa"),
            ("world-03-gamma", "ex11-lambda"),
            ("world-03-gamma", "ex12-mu"),
            ("world-03-gamma", "ex13-nu"),
            ("world-03-gamma", "ex14-xi"),
            ("world-03-gamma", "ex15-omicron"),
        ];

        for (world, exercise) in exercise_dirs {
            let ex_dir = root.join("exercises").join(world).join(exercise);
            fs::create_dir_all(&ex_dir).expect("create exercise dir");
            fs::write(
                ex_dir.join("Cargo.toml"),
                format!(
                    "[package]\nname = \"{exercise}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n"
                ),
            )
            .expect("write exercise Cargo.toml");
        }

        // Hint files for ex01-alpha (used by hint tests).
        let hints_dir = root
            .join("exercises")
            .join("world-01-alpha")
            .join("ex01-alpha")
            .join("hints");
        fs::create_dir_all(&hints_dir).expect("create hints dir");
        fs::write(hints_dir.join("hint1.md"), "# Hint 1\nFirst nudge.\n").expect("write hint1");
        fs::write(hints_dir.join("hint2.md"), "# Hint 2\nSecond nudge.\n").expect("write hint2");
        fs::write(hints_dir.join("hint3.md"), "# Hint 3\nThird nudge.\n").expect("write hint3");

        // Fake cargo script (controllable via FAKE_CARGO_EXIT env var).
        let bin_dir = root.join("bin");
        fs::create_dir_all(&bin_dir).expect("create bin dir");
        let fake_cargo = bin_dir.join("fake_cargo.sh");
        let fixture_script =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/fake_cargo.sh");
        fs::copy(&fixture_script, &fake_cargo).expect("copy fake_cargo.sh");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&fake_cargo, fs::Permissions::from_mode(0o755))
                .expect("chmod fake_cargo");
        }

        TempWorkspace { dir, fake_cargo }
    }

    pub fn root(&self) -> &Path {
        self.dir.path()
    }

    /// Write a `.learn-rust/progress.toml` with the given TOML content.
    pub fn set_progress(&self, toml_content: &str) {
        let progress_dir = self.dir.path().join(".learn-rust");
        fs::create_dir_all(&progress_dir).expect("create .learn-rust");
        fs::write(progress_dir.join("progress.toml"), toml_content).expect("write progress.toml");
    }

    /// Return the path to the progress file (may or may not exist).
    pub fn progress_path(&self) -> PathBuf {
        self.dir.path().join(".learn-rust").join("progress.toml")
    }

    /// Write a `hints/solution.rs` for the given world/exercise in this workspace.
    /// The `hints/` directory is created if it does not already exist.
    pub fn set_solution(&self, world: &str, exercise: &str, content: &str) {
        let hints_dir = self
            .dir
            .path()
            .join("exercises")
            .join(world)
            .join(exercise)
            .join("hints");
        fs::create_dir_all(&hints_dir).expect("create hints dir for solution");
        fs::write(hints_dir.join("solution.rs"), content).expect("write solution.rs");
    }

    /// Run the `learn` binary with the given args in this workspace.
    /// Returns the raw `Output` so tests can inspect stdout, stderr, and status.
    pub fn run(&self, args: &[&str]) -> Output {
        Command::new(learn_bin())
            .args(args)
            .current_dir(self.dir.path())
            .output()
            .expect("failed to run learn binary")
    }

    /// Run `learn` with the fake cargo substituted for the real one.
    /// `fake_exit` controls whether the fake cargo exits 0 (success) or 1 (failure).
    pub fn run_with_fake_cargo(&self, args: &[&str], fake_exit: u8) -> Output {
        Command::new(learn_bin())
            .args(args)
            .env("CARGO", &self.fake_cargo)
            .env("FAKE_CARGO_EXIT", fake_exit.to_string())
            .current_dir(self.dir.path())
            .output()
            .expect("failed to run learn binary with fake cargo")
    }
}

/// Path to the compiled `learn` binary under test.
pub fn learn_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_learn"))
}

/// Assert that `haystack` contains `needle` as a substring.
#[track_caller]
pub fn assert_contains(haystack: &str, needle: &str) {
    assert!(
        haystack.contains(needle),
        "expected output to contain {:?}\n--- actual ---\n{haystack}",
        needle
    );
}

/// Assert that `haystack` does NOT contain `needle`.
#[track_caller]
pub fn assert_not_contains(haystack: &str, needle: &str) {
    assert!(
        !haystack.contains(needle),
        "expected output NOT to contain {:?}\n--- actual ---\n{haystack}",
        needle
    );
}
