# learn-rust — A Gamified Rust Learning Campaign

Welcome to **learn-rust**, a progression-based campaign that teaches Rust through 15 increasingly complex exercises. You'll build real, working programs by completing stubs—not a tutorial, not a textbook, but a series of quests where you write the code that matters.

## Setup & Quick Start

### Prerequisites

- **Nix with flakes enabled** — This project uses a `flake.nix` to provide Rust, Cargo, and all dependencies in an isolated dev environment.
  - [Install Nix](https://nixos.org/download/) (multi-user install recommended)
  - Enable flakes in `~/.config/nix/nix.conf`:
    ```
    experimental-features = nix-command flakes
    ```

### Enter the Development Environment

```bash
# Enter a shell with Rust, Cargo, and tools ready
nix develop path:.

# (Optional) Use direnv to auto-enter the dev shell on cd
# nix flake update  # in this repo, once
# echo "use flake" > .envrc && direnv allow
```

Once inside the dev shell, all standard Cargo commands work. Alternatively, run commands without entering the shell:

```bash
# Check compilation (without entering shell)
nix develop path:. -c cargo check --workspace

# Run xtask commands (without entering shell)
nix develop path:. -c cargo xtask status
```

### Your First Quest

```bash
# Option 1: Enter dev shell, then run commands (recommended during development)
nix develop path:.
cargo check --workspace
cargo xtask verify ex01-format-scoreboard
cargo xtask hint ex01-format-scoreboard

# Option 2: Run commands directly from outside the shell (one-liner style)
nix develop path:. -c cargo xtask verify ex01-format-scoreboard
nix develop path:. -c cargo xtask hint ex01-format-scoreboard

# Both options print the same output. Inside the shell, you can chain commands:
cargo check --workspace
cat exercises/world-01-foundations/ex01-format-scoreboard/README.md
# Edit the stub (exercises/world-01-foundations/ex01-format-scoreboard/src/lib.rs)
cargo test -p ex01-format-scoreboard
cargo xtask status
```

## Command Reference

**Inside the dev shell** (`nix develop path:.`):

| Command | Purpose |
|---------|---------|
| `cargo xtask status` | Display current XP, rank, and progress |
| `cargo xtask next` | Get a recommendation for the next exercise |
| `cargo xtask verify <id>` | Run tests for a specific exercise |
| `cargo xtask hint <id> [--level N]` | Show next hint; optionally force hint N (1–3) |
| `cargo test -p <id>` | Run exercise tests directly with Cargo |
| `cargo check --workspace` | Verify everything still compiles |

**From outside the shell**, prefix commands with `nix develop path:. -c`:

```bash
nix develop path:. -c cargo check --workspace
nix develop path:. -c cargo xtask verify ex01-format-scoreboard
nix develop path:. -c cargo test -p ex01-format-scoreboard
```

**Build artifacts** with Nix:

| Command | Purpose |
|---------|---------|
| `nix develop path:.` | Enter interactive dev shell (dependencies installed) |
| `nix develop path:. -c <cmd>` | Run a command in the dev shell without entering it |
| `nix build path:.` | Build the workspace; produces `result/bin/xtask` binary |
| `nix flake check path:.` | Check compilation and xtask internal tests (does not run unfinished exercise tests) |

## The Campaign Map

### Ranks & Progression

Earn XP for each completed exercise. Progress through ranks as your skill grows:

| Rank | Title | Min XP | Badge |
|------|-------|--------|-------|
| 0 | Cadet | 0 | `◊` |
| 1 | Operator | 300 | `☆` |
| 2 | Specialist | 800 | `◆` |
| 3 | Commander | 1500 | `▲` |
| 4 | Legend | 2300 | `◈` |

XP is awarded for first-time completion. Hints are free; solving without them is its own reward.

### World 1: Foundations (5 exercises, 600 XP)

Learn the core language: functions, variables, types, and control flow. These exercises compile and run from day one.

| # | Quest | Concept | XP |
|---|-------|---------|-----|
| 1 | Format Scoreboard | Functions & formatting | 100 |
| 2 | Reactor Calibration | Variables & arithmetic | 110 |
| 3 | Access Control | Booleans & pattern matching | 120 |
| 4 | Energy Loop | Ranges & iteration | 130 |
| 5 | Message Normalizer | String & &str | 140 |

### World 2: Ownership (5 exercises, 800 XP)

Master Rust's memory model: moves, borrows, mutable references, and the borrow checker. These exercises teach you to write fast, safe code.

| # | Quest | Concept | XP |
|---|-------|---------|-----|
| 6 | Slice Telemetry | References & slices | 150 |
| 7 | Move Inventory | Ownership & moves | 160 |
| 8 | Borrow Checkpoint | Mutable borrowing | 170 |
| 9 | Station Crew | Structs & methods | 180 |
| 10 | Command Router | Enums & exhaustive matching | 190 |

### World 3: Collections & Errors (5 exercises, 900 XP)

Build with real data structures: vectors, hash maps, Option, Result, and custom error types. Capstone synthesizes all three worlds.

| # | Quest | Concept | XP |
|---|-------|---------|-----|
| 11 | Log Analyzer | Vectors & iterators | 200 |
| 12 | Loot Counter | Hash maps | 210 |
| 13 | Mission Lookup | Option type | 220 |
| 14 | Config Loader | Result & custom errors | 240 |
| 15 | Salvage Capstone | All concepts combined | 300 |

## How It Works

### Starter State

When you clone this repo, `cargo check --workspace` passes. Each exercise compiles but has **intentional test failures**—you'll see `#[test]` functions fail with clear error messages as you work. This is by design. There are no ignored tests (no `#[ignore]`).

### Editing

Each exercise has a **single stub file**: `exercises/worldN/exNN-name/src/lib.rs`. You edit only this file. All Cargo.toml, test files, and supporting code are locked—you can't break the build by accident.

### Verification

```bash
# Direct Cargo (always works):
cargo test -p ex01-format-scoreboard

# Or through xtask (tracks XP):
cargo xtask verify ex01-format-scoreboard
```

Both run the same tests. Using `cargo xtask verify` also saves your progress and awards XP on first completion.

### Progress Tracking

Progress lives in `.learn-rust/progress.toml` (gitignored—local only). Your history is yours, never synced or shared. Reset with:

```bash
rm .learn-rust/progress.toml
cargo xtask status  # Starts fresh
```

### Hints

Each exercise has **three progressive hints**. The first is a question or concept nudge. The second names tools and types. The third gives pseudocode or a structural outline—never valid Rust, never a completed solution.

```bash
# Show the next unhinted level for this exercise
cargo xtask hint ex01-format-scoreboard

# Jump directly to hint N (1–3)
cargo xtask hint ex01-format-scoreboard --level 3
```

### Prerequisites & Recommendations

Exercise order is a **recommendation**, not a hard gate. You can skip ahead, but each exercise builds on concepts from earlier ones. `cargo xtask next` suggests what to do next based on XP and completion.

## Workspace Verification

After installing Nix and enabling flakes, verify the workspace:

```bash
# Check that Rust and Cargo compile without errors
nix develop path:. -c cargo check --workspace

# Verify xtask and workspace tests (does NOT run unfinished exercise tests)
nix flake check path:.
```

The xtask internal tests pass. Exercise tests intentionally fail on starter stubs (see "Expected Starter Behavior" below).

## Expected Starter Behavior

On a fresh clone, inside the dev shell or via `nix develop path:. -c`:

```
$ cargo check --workspace
   Compiling ex01-format-scoreboard v0.1.0
   ...
   Finished dev [unoptimized + debuginfo] target(s) in 0.25s

$ cargo test -p ex01-format-scoreboard 2>&1 | head -20
running 1 test
test tests::test_format_line ... FAILED

---- tests::test_format_line stdout ----
thread 'tests::test_format_line' panicked at 'not yet implemented: Format a scoreboard row...'
    (expected failure; implement the stub to proceed)
```

This is normal. Starter stubs use `todo!()` and **intentionally panic**. You're meant to see failures. Fill in the stubs, and tests pass. The `nix flake check` command checks workspace compilation and xtask tests only (not learner exercise tests, which are expected to fail until you implement them).

## API Contracts

Each exercise's **learner API** is documented in its `src/lib.rs`—public functions, structs, and their signatures. These are your spec. Read them. Tests validate that your implementation matches the spec, not how you implement it. Multiple correct implementations will pass.

Example from ex01:

```rust
pub fn format_scoreboard_line(player: &str, score: i32, rank: usize) -> String {
    // Your implementation here
}
```

See the exercise README for full context on inputs, outputs, and examples.

## Content & Tone

This isn't a grind. No streaks, no badges for daily logins, no punitive timeouts. You earn XP when you finish, and hints are free. The campaign is designed to feel like a series of story-driven quests with a coherent theme—completing a mission, not checking boxes.

## File Structure

```
learn-rust/
├── Cargo.toml                    # Workspace root
├── campaign.toml                 # Exercise metadata & progression
├── README.md                     # This file
├── xtask/
│   ├── Cargo.toml
│   └── src/main.rs               # verify, status, next, hint commands
├── exercises/
│   ├── world-01-foundations/
│   │   ├── ex01-format-scoreboard/
│   │   │   ├── Cargo.toml
│   │   │   ├── README.md
│   │   │   ├── src/lib.rs        # Your edit zone
│   │   │   ├── tests/solve.rs    # Tests (read-only)
│   │   │   └── hints/
│   │   │       ├── hint1.md      # Conceptual nudge
│   │   │       ├── hint2.md      # Tools & types
│   │   │       └── hint3.md      # Pseudocode outline
│   │   └── ... (4 more exercises)
│   ├── world-02-ownership/       # (5 exercises)
│   └── world-03-collections-errors/  # (5 exercises)
└── .learn-rust/
    └── progress.toml             # Your local progress (gitignored)
```

## Extending the Campaign

To add more exercises:

1. Copy the structure of an existing exercise into a new world directory or extend an existing one.
2. Add a `Cargo.toml` using the exercise ID as the package name.
3. Write public stubs in `src/lib.rs` using `todo!("description")`.
4. Add tests in `tests/solve.rs`.
5. Write a README and three hints.
6. Update `campaign.toml` with the new exercise metadata and XP value.
7. Update the root `Cargo.toml` workspace members to include the new crate path.

## Troubleshooting

**Nix not found or flakes not enabled:**  
Ensure you have [Nix installed](https://nixos.org/download/) and flakes enabled in `~/.config/nix/nix.conf`:
```
experimental-features = nix-command flakes
```
Then try `nix flake check path:.` to verify setup.

**"command not found: cargo" inside the dev shell:**  
Flake entry was interrupted. Try exiting and re-entering:
```bash
exit
nix develop path:.
```

**Dev shell environment not loading automatically:**  
To auto-load on `cd`, install and set up direnv:
```bash
echo "use flake" > .envrc
direnv allow
```
Then Nix dependencies load automatically when you enter the directory.

**"todo!() panicked" on a completed exercise:**  
Did you accidentally delete the implementation? Redo it or use `git checkout` to restore. Tests always re-run.

**Hint shows the full answer:**  
Report it. Hints should nudge, not spoil. The three-level structure prevents this, but feedback helps.

**Can't compile the workspace:**  
Run `nix develop path:. -c cargo check --workspace` (or `cargo check --workspace` inside the shell) to see the error. Most likely, you edited a non-stub file or introduced a Rust syntax error. Check that you only edited `src/lib.rs` in the exercise you're working on.

**Progress not saving:**  
Ensure `.learn-rust/` is writable. On first run, `xtask` creates `.learn-rust/progress.toml`. If it fails, check file permissions.

**`nix build` fails:**  
Run `nix flake check path:.` first to diagnose. Workspace compilation and xtask tests should pass. If they do and `nix build` still fails, the issue is in the flake configuration, not your code.

## Philosophy

This campaign exists because Rust is learnable, but its mental models (ownership, borrowing, pattern matching) need practice, not lecture. Each exercise is a small, complete program. You write the logic, tests verify correctness. No artificial barriers, no gamification gimmicks—just the satisfaction of solving real problems.

Happy coding. You've got this. 🦀

---

**Last updated:** 2026-07-19  
**Campaign schema:** v1  
**Total exercises:** 15 | **Total XP available:** 2,620 | **Legend threshold:** 2,300
