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

### Enter the Development Environment & Start

```bash
# Enter the dev shell
nix develop path:.

# Run learn with no arguments to see your dashboard and next quest
learn
```

That's it. After you run `learn`, you'll see:
- Your current rank and XP
- Your progress through each world
- The recommended quest to work on next

### The Learner's Loop

Once you're in the dev shell:

1. **See your quest:** `learn` (shows dashboard and current recommendation)
2. **Work on it:** Edit `exercises/worldN/exNN-name/src/lib.rs` using your editor
3. **Check your work:** `learn check` (or `learn check <id>` for a specific exercise)
4. **Need a nudge?** `learn hint` (or `learn hint <id>` to specify which one)

That's the whole workflow. No Cargo commands to memorize.

### Alternative: Run Commands Without Entering the Shell

If you prefer one-liners, you can run any command with `nix develop path:. -c`:

```bash
nix develop path:. -c learn status
nix develop path:. -c learn check ex01-format-scoreboard
```

### (Optional) Auto-Load the Dev Shell

Use direnv to automatically enter the shell when you `cd` into this repo:

```bash
echo "use flake" > .envrc
direnv allow
```

Then just `cd learn-rust` and the shell loads automatically.

## The `learn` Command Reference

**Inside the dev shell** (`nix develop path:.`), use these commands:

| Command | Purpose |
|---------|---------|
| `learn` | Show dashboard: rank, XP, progress, and next quest |
| `learn status` | Same as `learn` (show progress) |
| `learn check [id]` | Run tests for a specific exercise (or current) and update progress |
| `learn next` | Get a recommendation for the next uncompleted exercise |
| `learn hint [id]` | Show the next hint for an exercise (or current); use `--level N` to jump to hint 1, 2, or 3 |

**Examples:**

```bash
# Show your current progress and recommended next quest
learn

# Check your work on the current (recommended) exercise
learn check

# Check a specific exercise by name
learn check ex01-format-scoreboard

# Get a hint for the current exercise
learn hint

# Jump to hint level 3 for a specific exercise
learn hint ex02-reactor-calibration --level 3

# Get the next recommendation without checking progress
learn next
```

**ID defaults:** When you omit an exercise ID (e.g., `learn check` instead of `learn check ex01-format-scoreboard`), the `learn` command uses your currently recommended exercise. This keeps commands short and focused.

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

When you clone this repo, each exercise compiles but has **intentional test failures**. When you run `learn check` on a fresh exercise, you'll see a failure message—this is by design. Starter stubs use `todo!()` and intentionally panic. Fill in the stubs, and tests pass.

### Editing

Each exercise has a **single stub file**: `exercises/worldN/exNN-name/src/lib.rs`. You edit only this file. All Cargo.toml, test files, and supporting code are locked—you can't break the build by accident.

### Verification

Use the `learn` command to check your work:

```bash
# Check the current (recommended) exercise
learn check

# Check a specific exercise by name
learn check ex01-format-scoreboard
```

`learn check` runs the tests and, on success, saves your progress and awards XP. It's the primary way to verify your implementation.

### Progress Tracking

Your progress lives in `.learn-rust/progress.toml` (gitignored—local only). Your history is yours, never synced or shared. To reset and start fresh:

```bash
rm .learn-rust/progress.toml
learn status  # Starts over
```

### Hints

Each exercise has **three progressive hints**. The first is a question or concept nudge. The second names tools and types. The third gives pseudocode or a structural outline—never valid Rust, never a completed solution.

```bash
# Show the next unhinted level for the current exercise
learn hint

# Show a hint for a specific exercise
learn hint ex01-format-scoreboard

# Jump directly to hint level 3
learn hint ex01-format-scoreboard --level 3
```

### Exercise Order

The order in which exercises appear is a **recommendation**, not a hard requirement. You can skip ahead if you like, but each exercise builds on concepts from earlier ones. Use `learn next` to get a personalized recommendation based on your completion and XP.

## Workspace Verification

After installing Nix and enabling flakes, verify the workspace:

```bash
# Check that Rust and Cargo compile without errors
nix develop path:. -c cargo check --workspace

# Verify workspace compilation and internal tool tests
nix flake check path:.
```

Workspace tests pass. Exercise tests intentionally fail on starter stubs until you implement them (see "Expected Starter Behavior" below).

## Expected Starter Behavior

On a fresh clone, when you run `learn check` on an exercise, you'll see:

```
running 1 test
test tests::test_format_line ... FAILED

---- tests::test_format_line stdout ----
thread 'tests::test_format_line' panicked at 'not yet implemented: Format a scoreboard row...'
    (expected failure; implement the stub to proceed)
```

This is normal. Starter stubs use `todo!()` and intentionally panic. Fill in the stub in `src/lib.rs`, then run `learn check` again to verify your implementation.

## Advanced: Under the Hood (Cargo Compatibility)

The `learn` command is your primary interface. However, if you want to work directly with Cargo (for example, in an IDE, in a CI/CD pipeline, or for debugging), these commands are equivalent and always available:

| Learn | Cargo equivalent | Purpose |
|-------|------------------|---------|
| `learn check ex01-format-scoreboard` | `cargo test -p ex01-format-scoreboard` | Run tests directly (does not save progress or award XP) |
| `learn status` | `cargo xtask status` | Show XP and rank |
| `learn next` | `cargo xtask next` | Get next recommendation |
| `learn hint ex01-format-scoreboard` | `cargo xtask hint ex01-format-scoreboard` | Show hint for exercise |

Use `cargo xtask verify <id>` if you need tests to run **and** save progress/XP (equivalent to `learn check` with side effects).

**Workspace compilation:**

```bash
# Check that everything compiles (from inside shell or with nix develop path:. -c)
cargo check --workspace
```

**Direct Cargo example** (from inside the shell):

```bash
cargo test -p ex01-format-scoreboard
cargo check --workspace
cargo xtask status
```

This section exists for compatibility and advanced use. **For learning, use `learn`.**

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
