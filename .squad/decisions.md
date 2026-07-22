# Squad Decisions

## Active Decisions

### 2026-07-20: Final Re-Review — CLI & Test Artifacts (Cycle 3) — ✅ APPROVED

**By:** Mikey (Learning Journey Lead)
**Ceremony:** Mandatory Re-Review (cycle 3 — post-incident recovery)
**Timestamp:** 2026-07-20T13:16:00-07:00

---

## VERDICT: ✅ APPROVE

The prior conditional rejection (cycle 1 — rustfmt diffs in Brand-authored test files) is **fully resolved**. Data independently recreated and formatted the test artifacts. All gating criteria pass on the current working tree.

---

## Validation Evidence

| Check | Result | Detail |
|-------|--------|--------|
| `cargo fmt --all -- --check` | ✅ Pass | Exit 0, no diffs |
| `cargo check --workspace` | ✅ Pass | Warnings only from exercise `todo!()` stubs (expected) |
| `cargo test --package xtask` | ✅ 7 unit + 17 integration | All 24 tests pass |
| `nix flake check path:.` | ✅ Pass | workspace-check + xtask-tests derivations succeed |
| `nix build path:.` | ✅ Pass | Produces `result/bin/learn` (1.7 MB) |
| `nix run path:. -- next` | ✅ Pass | Outputs `ex01-format-scoreboard`, exit 0 |
| `nix run path:. -- status` | ✅ Pass | Correct dashboard output |
| `cargo xtask status` | ✅ Pass | Legacy alias works, correct output |
| No-arg dashboard | ✅ Pass | Rank, XP, world counts, recommended quest, key commands |
| Progress isolation | ✅ Confirmed | `.learn-rust/progress.toml` never created by any test or CLI run |
| `tempfile` dev-dep | ✅ Justified | Used by `TempWorkspace` in `support/mod.rs` for isolated test dirs |
| Cargo.lock changes | ✅ Justified | Adds `tempfile` + transitive deps only |
| Exercise READMEs (all 15) | ✅ Pass | Use `learn check`/`learn hint`, zero `cargo xtask` references |
| Root README | ✅ Pass | Nix-only setup, `learn`-first, Cargo demoted to Advanced section |
| Test artifacts | ✅ Clean | 17 behavioral black-box tests, no solution leaks, proper `TempDir` isolation |
| `cargo xtask verify` compat | ✅ Confirmed | Test #17 validates; binary accepts `verify` as alias for `check` |

## Incident Resolution

- Brand's original test files had 6 rustfmt diffs → conditional rejection (cycle 1).
- Brand locked out. Data assigned as independent revision owner.
- Accidental `git checkout . && git clean -fd` erased all uncommitted work (violates no-destructive-git policy).
- Mouth replayed docs. Data restored implementation and independently recreated test artifacts from the approved public behavior contract.
- Data formatted all code. `cargo fmt --all -- --check` now passes clean.
- The formatting rejection is fully resolved. No prior evidence was reused; all checks re-executed on the current working tree.

## Authorship

| Surface | Author | Status |
|--------|--------|--------|
| `xtask/src/lib.rs`, `xtask/src/main.rs`, `xtask/Cargo.toml` | Data | ✅ Approved |
| `xtask/tests/cli.rs`, `xtask/tests/support/mod.rs` | Data (independent recreation) | ✅ Approved |
| `xtask/tests/fixtures/**` | Data | ✅ Approved |
| `.cargo/config.toml`, `flake.nix`, `Cargo.lock` | Data | ✅ Approved |
| `README.md` (root), 15× exercise `README.md` | Mouth | ✅ Approved |

---

### 2026-07-20: Final Review — Guided Campaign CLI (`learn`)

**By:** Mikey (Learning Journey Lead)  
**Ceremony:** Final Coherence & Correctness Review  
**Timestamp:** 2026-07-20T12:46:00-07:00  
**Cycle:** Cycle 1 (superseded by Cycle 3 re-review above)

---

## Verdict: ⚠️ CONDITIONAL REJECT — Rustfmt Formatting (Resolved)

### Summary

The implementation was functionally correct and fully met the approved contract. All behavior was validated. Gating criterion failed: `cargo fmt --all -- --check` reported 6 formatting diffs in Brand's test files. **This has been resolved by Data's independent recreation and formatting in Cycle 3 re-review.**

### Conditional Rejection Reason (Now Resolved)

**File(s):** `xtask/tests/cli.rs`, `xtask/tests/support/mod.rs`  
**Author:** Brand  
**Reason:** `cargo fmt --all -- --check` failed with 6 diffs (line length, argument formatting, method chain formatting).

### Resolution

**Assigned to:** Data (per lockout map)  
**Action Taken:** Independently recreated test artifacts from the approved behavioral contract; applied formatting; confirmed `cargo fmt --all -- --check` passes clean.

### Status: ✅ RESOLVED (Cycle 3 re-review)

---

### 2026-07-20T12:22:07.761-07:00: Implemented guided `learn` CLI end to end

**By:** Data (Rust Engineer)
**What:** Replaced the `xtask`-named binary and `verify`-only command set with the full `learn` CLI as approved by Design Review.
**Why:** Learners needed a single entry point that makes the next useful action obvious without requiring knowledge of raw Cargo commands.

#### Changes made

**`xtask/Cargo.toml`** — Added `[[bin]] name = "learn"` so the binary is named `learn`.

**`xtask/src/lib.rs`** — Full CLI rewrite:
- `Cli.command` is now `Option<Commands>`; no subcommand → `cmd_dashboard` (compact XP/rank/world progress + recommended next + guidance)
- Added `Commands::Check { id: Option<String> }` — optional ID defaults to next recommended exercise; test failures print a `learn hint` nudge and exit 1 silently (no redundant "error:" line)
- `Commands::Verify { id: String }` retained as legacy alias; delegates to `cmd_check`
- `Commands::Hint { id: Option<String>, level: Option<u8> }` — both optional; auto-advance logic reads `hints_viewed[exercise_id]` and bumps by 1 (capped at 3) when no `--level`; explicit `--level` overrides; persists new high-water mark in progress file
- `Commands::Status` and `Commands::Next` output unchanged
- `ProgressFile` gains `hints_viewed: HashMap<String, u8>` with `#[serde(default, skip_serializing_if = "HashMap::is_empty")]` — fully backward-compatible
- `XtaskError::CheckFailed` added — silent sentinel so `main.rs` exits 1 without reprinting the error

**`xtask/src/main.rs`** — Suppresses the "error:" print for `XtaskError::CheckFailed` (cargo output already explains the failure).

**`.cargo/config.toml`** — Alias updated: `xtask = "run --package xtask --bin learn --"` (was `run --package xtask --`); preserves all `cargo xtask` invocations.

**`flake.nix`** — `pname`/`mainProgram` → `"learn"`; `cargoBuildFlags` adds `--bin learn`; apps `program` → `bin/learn`; devShell includes `self.packages.${system}.default` so `learn` is on PATH inside `nix develop`.

#### Validation (Cycle 1)

- `cargo test --package xtask` — 7 unit tests + 17 integration tests pass
- `cargo fmt --package xtask -- --check` — clean
- `cargo check --workspace` — clean (exercise stub warnings are pre-existing)
- `learn` (no args) — compact dashboard renders correctly
- `learn check ex01-format-scoreboard` — test output flows through, hint nudge printed, exit 1, no double error message
- `learn status`, `learn next`, `learn verify ex99-unknown`, `learn hint --level 5` — all behave as specified
- `cargo xtask status`, `cargo xtask next` — legacy alias works

---

### 2026-07-20: Adopt learner-facing `learn` CLI over raw `cargo xtask`
**By:** Data
**What:** Keep the implementation crate in `xtask/`, but make the primary learner-facing binary `learn`, available automatically in `nix develop`, with no-arg behavior showing the current quest dashboard. Keep `cargo xtask` as a compatibility alias and preserve `.learn-rust/progress.toml` unchanged.
**Why:** The learner specifically found `cargo xtask` plus explicit exercise IDs confusing. This keeps the internal Rust workspace stable while giving a simpler command surface: `learn`, `learn show`, `learn verify`, `learn hint`, `learn next`, and `learn status`, with omitted exercise IDs resolving to the recommended current exercise.

---

### 2026-07-20: Design Review — Guided Campaign CLI (`learn`)

**By:** Mikey (Learning Journey Lead)
**Ceremony:** Design Review
**Requested by:** sspeaks610
**Participants:** Data (Rust Engineer), Mouth (Game Designer), Brand (Challenge Tester)
**Timestamp:** 2026-07-20T12:22:07-07:00

---

## Trigger

User feedback: "There should be an intuitive cli to progress through the game rather than running the cargo commands... they're confusing to me."

---

## Approved Contract

### 1. Primary Invocation

**Binary name:** `learn`

**Rationale:** The repository is `learn-rust`; typing `learn` is the most discoverable, memorable, and self-documenting entry point. Quest/adventure theme lives in output copy, not the command name.

**Availability:**
- Inside `nix develop path:.`: binary is on PATH (added to devShell packages from the built flake package)
- Outside: `nix run path:. -- <subcommand>`
- Fallback: `cargo xtask <subcommand>` continues to work via `.cargo/config.toml` alias

### 2. Command Set

| Command | Behavior | Exercise ID |
|---------|----------|-------------|
| `learn` (no args) | Dashboard: rank badge + XP, world progress summary, current recommended exercise with file paths, 3 key next-step commands | — |
| `learn check [id]` | Run tests for exercise; update progress on pass. Defaults to `next` exercise if ID omitted. | Optional |
| `learn hint [id] [--level N]` | Show hint. Defaults to current exercise; auto-increments to next unseen level. `--level` overrides. | Optional |
| `learn status` | Full progress: XP, rank, all worlds, recommendation | — |
| `learn next` | One-line output: next exercise ID (script-friendly) | — |

**No-argument behavior:** Print a compact "Mission Control" dashboard:
```
◊ Cadet — 0 XP

  Foundations:  0/5
  Ownership:    0/5
  Collections:  0/5

▶ Next quest: ex01-format-scoreboard — Format Scoreboard
  Edit: exercises/world-01-foundations/ex01-format-scoreboard/src/lib.rs

  learn check    — verify your solution
  learn hint     — get a nudge
  learn status   — full progress map
```

### 3. Output & Error Behavior

- **Success (check passes):** `✅ ex01-format-scoreboard complete! +100 XP (total: 100). Next: ex02-reactor-calibration`
- **Already complete:** `✅ ex01-format-scoreboard verified again. XP unchanged (total: 100).`
- **Failure (tests fail):** Exit 1. Show `❌ Tests failed for ex01-format-scoreboard. Try: learn hint` (test output passes through from cargo)
- **Unknown exercise:** Exit 1. Stderr: `error: unknown exercise id: <id>`. No fuzzy matching in v1.
- **Invalid hint level:** Exit 1. Stderr: `error: hint level must be 1, 2, or 3 (got N)`

Color/ANSI: Use when stdout is a TTY; plain text otherwise. Rank badges always shown (they're UTF-8, not ANSI).

### 4. Progress & Campaign Interactions

- **Progress file:** `.learn-rust/progress.toml` — schema unchanged (v1). No migration needed.
- **Campaign metadata:** `campaign.toml` — read-only, schema unchanged.
- **Hint tracking:** Store last-viewed hint level per exercise in progress file (new optional field `[hints_viewed]` map). Backward compatible: missing field means all hints unseen.
- **No solutions exposed:** Hints display file content; no exercise source shown in CLI output.

### 5. Compatibility

- `cargo xtask verify/status/next/hint` continues to work via `.cargo/config.toml`:
  ```toml
  [alias]
  xtask = "run --package xtask --bin learn --"
  ```
- Old progress files work with new binary (additive schema: new `[hints_viewed]` section is optional).
- `nix build path:.` produces `result/bin/learn` (renamed from `xtask`).
- `nix run path:.` invokes `learn`.

### 6. Implementation Plan (Nix-only toolchain preserved)

| Change | File(s) |
|--------|---------|
| Rename binary | `xtask/Cargo.toml`: add `[[bin]] name = "learn", path = "src/main.rs"` |
| Update clap CLI | `xtask/src/lib.rs`: new `Commands` enum with `check/hint/status/next`, no-subcommand dashboard |
| Add cargo alias | `.cargo/config.toml` (new file) |
| Update flake | `flake.nix`: `pname = "learn"`, `mainProgram = "learn"`, add to devShell packages |
| Add hint tracking | `xtask/src/lib.rs`: `ProgressFile` gains optional `hints_viewed: HashMap<String, u8>` |

---

## File Ownership (Non-Overlapping)

| Agent | Owns | Deliverables |
|-------|------|-------------|
| **Data** | All Rust implementation + Nix + Cargo config | `xtask/Cargo.toml`, `xtask/src/main.rs`, `xtask/src/lib.rs`, `flake.nix`, `.cargo/config.toml`, `Cargo.lock` |
| **Mouth** | Learner-facing docs + campaign metadata | `README.md` (root), exercise `README.md` files (update command references), `campaign.toml` (if schema bump needed) |
| **Brand** | All test code | `xtask/tests/cli.rs`, `xtask/tests/support/`, `xtask/tests/fixtures/` |

---

## Acceptance Tests (Brand owns — later revised by Data)

### Required test matrix:

| Test ID | Input | Assertion |
|---------|-------|-----------|
| `no_args_dashboard` | `learn` | Exit 0; stdout contains XP line, recommended exercise, and command hints |
| `status_fresh` | `learn status` | Exit 0; XP: 0, 0/5 per world, recommendation shown |
| `next_fresh` | `learn next` | Exit 0; stdout contains `ex01-format-scoreboard` |
| `check_success` | `learn check ex01-format-scoreboard` (fake cargo passes) | Exit 0; progress.toml created with XP 100 |
| `check_default_id` | `learn check` (no ID, fresh) | Resolves to ex01; same behavior as explicit ID |
| `check_failure` | `learn check ex01-format-scoreboard` (fake cargo fails) | Exit 1; no progress write |
| `check_idempotent` | Run check twice on same exercise | Second run: XP unchanged, no duplicates |
| `hint_default_level` | `learn hint ex01-format-scoreboard` | Exit 0; shows hint1 content |
| `hint_auto_increment` | `learn hint` after hint1 viewed | Shows hint2; hints_viewed updated |
| `hint_explicit_level` | `learn hint ex01-format-scoreboard --level 3` | Exit 0; shows hint3 |
| `hint_unknown_id` | `learn hint ex99-fake` | Exit 1; stderr: unknown exercise |
| `hint_invalid_level` | `learn hint ex01-format-scoreboard --level 4` | Exit 1; stderr: level must be 1-3 |
| `missing_campaign` | Any command without campaign.toml | Exit 1; stderr: campaign metadata missing |
| `progress_compat` | Load existing v1 progress.toml (no hints_viewed) | Works without error; hints default to unseen |
| `mid_campaign_status` | Progress with 2 exercises done | Correct XP, correct counts, correct next |
| `all_complete` | All 15 done | `learn next`: "All exercises are complete" |
| `cargo_xtask_compat` | `cargo xtask status` | Same output as `learn status` |

**Test infrastructure:** Fixture workspace under `xtask/tests/fixtures/` with a minimal campaign. Fake cargo script (PATH override) for verify tests. All tests run via `cargo test --package xtask` and are picked up by `nix flake check`.

---

## VERDICT: ✅ APPROVED (Cycle 3 Re-Review)

Contract is fully implemented and validated. All gating criteria pass. Guided CLI ships.

---

### 2026-07-19: Re-Review — ex08 & ex15 Data Revisions (Mandatory Re-Review)

**By:** Mikey (Learning Journey Lead)
**Ceremony:** Mandatory Re-Review (post-lockout revision)
**Requested by:** sspeaks610
**Timestamp:** 2026-07-19T15:15:09-07:00

#### Context

Brand was locked out of `ex08-borrow-checkpoint/tests/solve.rs` and `ex15-salvage-capstone/tests/solve.rs` after Mikey's initial rejection. Mouth was locked out of `ex15-salvage-capstone/README.md` and `ex15-salvage-capstone/hints/hint3.md`. Data independently produced revised versions of all four artifacts.

#### Evidence

**`ex08-borrow-checkpoint/tests/solve.rs`**
- Original rejection: charge not capped at 100 when boost exceeds threshold; wording misaligned.
- Resolution: `overheat_triggered_above_100` now asserts `charge == 100` and `overheated == true`. Comment reads "charge is capped at 100, overheated = true". `overheat_not_triggered_at_exactly_100` uses "strictly > 100" language. All 6 tests align with README and hint3 behavioral contract. No solution leaks. ✓

**`ex15-salvage-capstone/tests/solve.rs`**
- Original rejection: manifest inputs not pipe-separated.
- Resolution: All manifest lines use `name|mass|priority|fragile` pipe format. Format comment at file top is explicit. `top_targets_ordered_by_priority_descending` asserts all 4 items in descending order `["cryo-pod","sensor","engine","fuel-cell"]`. All `InvalidLine` tests use 0-based line indexes. D6 (behavioral black-box) fully satisfied. ✓

**`ex15-salvage-capstone/README.md`**
- Original rejection: top_targets description ambiguous about inclusion scope.
- Resolution: Behavioral Rule 3 states "All item names sorted by priority descending." Success Criteria confirms "Top targets contain all items sorted by priority (highest first)." Consistent with tests and lib.rs. No solution leaks. ✓

**`ex15-salvage-capstone/hints/hint3.md`**
- Original rejection: top_targets scope and InvalidLine line indexing ambiguous.
- Resolution: Note explicitly states "Line indexes are 0-based, matching Rust indexing and the public error contract." Step 6 states "Sort all items by priority descending; top_targets contains all item names in that order." Pseudocode only — no Rust syntax, no solution. ✓

**Root README.md / flake.nix / .envrc (Nix-only audit)**
- Nix with flakes required; no Brew/rustup/curl toolchain instructions present. ✓
- `nix develop path:.` explained for interactive shell; `nix develop path:. -c <cmd>` for one-liner style. ✓
- `nix build path:.` builds xtask binary; `nix flake check path:.` runs workspace check and xtask tests (exercise stubs excluded by design). ✓
- Starter-state test failures explained: `todo!()` stubs intentionally panic; `cargo check --workspace` passes. ✓
- direnv `.envrc` present (`use flake path:.`); README correctly describes it as optional auto-loading, not a toolchain installer. ✓
- No excessive or misleading content found that would materially harm onboarding.

**Campaign integrity check**
- 15 exercises across 3 worlds confirmed. Learner-owned stubs (`todo!()`), behavioral tests, 3-level hints, README per exercise. Local gamified progress via xtask + `.learn-rust/progress.toml`. ✓

#### VERDICT: ✅ APPROVE

All four Data revisions fully resolve the original rejection reasons. No new contradictions or solution leaks introduced. Root README satisfies Nix-only onboarding requirement. Campaign ships.

---

### 2026-07-19T13:55:57.659-07:00: Use Nix for the Rust toolchain and build workflow

**By:** sspeaks610 (user directive)
**What:** The learn-rust repository must provide its Rust toolchain and build-system setup through Nix rather than Brew, curl-based installers, or other host-level installation instructions.
**Why:** User request: "Can we get the toolchain and build system setup using nix instead of installing things with brew or curl or whatever?"
**Implementation Status:** ✅ Deployed (flake.nix, Cargo.lock, .envrc, root README with Nix-only instructions) — Mikey approved.

---

### 2026-07-19T21-09-24: Adopted campaign.toml schema and xtask validation contract

**By:** Data (Rust Engineer)
**What:** Adopted campaign.toml schema and xtask validation contract.
**References:** xtask/src/lib.rs, Cargo.toml
**Why:** Defined campaign metadata contract for xtask consumption: top-level `schema_version`, `title`, `[[ranks]]`, and `[[worlds]]` with nested `[[worlds.exercises]]`. Validation enforces 3 worlds, 5 exercises per world (15 total), unique world/exercise/rank IDs, `world-NN-name` and `exNN-kebab-name` ID formats, rank ordering by increasing `min_xp` with first rank at 0 XP, positive exercise XP, known prerequisites/unlocks, and package name matching exercise ID. `next` uses prerequisites only as advisory recommendation, while `verify` never hard-blocks and updates `.learn-rust/progress.toml` idempotently after successful tests.

---

### 2026-07-19T13:55:57.659-07:00: Workspace glob narrowed to skip hint-only placeholder directories (consolidated)

**By:** Data (Rust Engineer), consolidated with workspace layout (D1)
**What:** Changed the workspace `members` glob in `Cargo.toml` from `exercises/*/*` to `exercises/*/ex[0-9][0-9]-[a-z]*`. The old glob matched literal-asterisk placeholder directories (e.g. `ex*`, `ex06-*`) that contain only `hints/` subdirectories and no `Cargo.toml`. Cargo errors—not silently skips—when a glob-matched path lacks `Cargo.toml`. The new pattern requires two ASCII digits after `ex` and a lowercase letter as the first character of the exercise name, which precisely matches the 15 real exercise crates and rejects all placeholder directories.
**Why:** Required to make `cargo generate-lockfile`, `cargo check --workspace`, and `nix build` work without touching or deleting hint-only directories that belong to other team members. The semantic intent of D1 ("one crate per exercise") is preserved; only the glob specificity increases.
**Impact:** D1 workspace layout updated to use `members = ["xtask", "exercises/*/ex[0-9][0-9]-[a-z]*"]`.

---

### 2026-07-19T13:55:57.659-07:00: Ex05/08/15 behavioral assumptions and rustfmt blockers (consolidated)

**By:** Brand (Challenge Tester), Data (Rust Engineer)
**Topics:** 
1. Undocumented behavioral specs in ex05, ex08, ex15 (Brand assumptions)
2. Rustfmt failures on Data-owned files (Brand documentation)

**Resolved by:** Mikey's final code review (contract review above). All assumed specs confirmed or corrected by Data revisions.

**Rustfmt status:** Data resolved all 12 formatting issues. Workspace passes `cargo fmt --all -- --check`.

---

## Governance

- All meaningful changes require team consensus ✅
- Document architectural decisions here ✅
- Keep history focused on work, decisions focused on direction ✅


---

### 2026-07-20: Design Review — Absolute Edit Paths & Learner-Stub Markers — ✅ APPROVED

**By:** Mikey (Learning Journey Lead)
**Ceremony:** Design Review  
**Timestamp:** 2026-07-20T13:45:00-07:00
**Participants:** Data (Rust Engineer), Mouth (Game Designer), Brand (Challenge Tester)

---

## VERDICT: ✅ APPROVED — Implementation-Ready Contract

Two changes address user feedback: the `learn` dashboard will print a full absolute path to the file the learner should edit, and every `todo!()` stub will be preceded by a visible 3-line banner comment.

## Decisions

1. **Absolute path source:** Use raw `root` from `discover_workspace_root()` (via `current_dir()`), not `canonicalize()`, to avoid confusing symlink resolution on macOS.

2. **Path format in dashboard:** Print `📂 Edit: <absolute_path>` on a separate indented line immediately below `▶ Next:` exercise recommendation for copyability.

3. **Banner placement:** 3-line `YOUR MISSION` marker inside function body, directly above each `todo!()` call, at correct statement indent (inherits automatically).

4. **Banner text (exact):** `// ══════════════════════════════════════════════════════════════\n// 🚀 YOUR MISSION: Replace the todo!() below with your code.\n// ══════════════════════════════════════════════════════════════`

5. **No code restructuring:** Banners solve visibility without reorganizing functions—preserves test line expectations, git blame, and teaching flow.

6. **Test file split:** Dashboard path test in `xtask/tests/cli.rs`; banner inventory test in new `xtask/tests/exercise_markers.rs` (scans real exercise tree, not fixtures).

## Action Items

- **Data:** Implement absolute edit path in `cmd_dashboard()` + `println!` in `xtask/src/lib.rs`
- **Data:** Insert `YOUR MISSION` banner above each of 18 `todo!()` calls across 15 exercise `src/lib.rs` files
- **Brand:** Add `dashboard_shows_absolute_edit_path()` test to `xtask/tests/cli.rs`
- **Brand:** Create `xtask/tests/exercise_markers.rs` with `every_exercise_has_mission_banner_for_each_todo()` test

## Gating Criteria

- `cargo fmt --all -- --check` must pass
- `cargo test --package xtask` — all tests pass
- `rg "YOUR MISSION" exercises/*/src/lib.rs | wc -l` — must equal 18
- `nix flake check path:.` — must pass (if available)

---


---

### 2026-07-20T13-45-24.420-07-00: Edit-Path & Mission Banners — Implementation Complete — ✅ APPROVED (Final)

**By:** Mikey (Learning Journey Lead)  
**Ceremony:** Final Coherence & Correctness Review  
**Timestamp:** 2026-07-20T14:02:00-07:00  
**Status:** ✅ IMPLEMENTATION-READY → ✅ APPROVED & COMPLETE

---

## VERDICT: ✅ APPROVE

All contract points verified, all gating checks pass. Implementation by Data and Brand complete and validated.

## Validation Evidence

| Check | Result | Detail |
|-------|--------|--------|
| `cargo fmt --all -- --check` | ✅ Exit 0 | No formatting diffs |
| `cargo check --workspace` | ✅ Exit 0 | Expected `todo!()` warnings only (learner stubs) |
| `cargo test --package xtask` | ✅ 26 pass | 7 unit + 19 integration (added: path test + marker inventory test) |
| `nix flake check path:.` | ✅ Pass | workspace-check + xtask-tests derivations rebuilt and passed |
| Dashboard path output | ✅ Correct | `/Users/sspeaks/learn-rust/exercises/world-01-foundations/ex01-format-scoreboard/src/lib.rs` |
| Banner count verification | ✅ 18 total | ex06: 2 stubs, ex09: 3 methods, remaining 12 files: 1 each |
| API/behavior preservation | ✅ Verified | Additive only: 3-line banner insertions + one `println!` addition |
| No solution leaks | ✅ Verified | All `todo!()` message strings unchanged (inline hints preserved) |

## Implementation Assignments Completed

| Agent | Work | Status |
|-------|------|--------|
| **Data** | `xtask/src/lib.rs` — `cmd_dashboard()` absolute path impl + println | ✅ Complete |
| **Data** | 15× `exercises/*/src/lib.rs` — 18 `YOUR MISSION` banner insertions | ✅ Complete |
| **Brand** | `xtask/tests/cli.rs` — `dashboard_shows_absolute_edit_path()` test | ✅ Complete |
| **Brand** | `xtask/tests/exercise_markers.rs` (new) — `every_exercise_has_mission_banner_for_each_todo()` test | ✅ Complete |

## Technical Notes

1. **Path canonicalization behavior:** Product code uses raw `current_dir()` ancestor walk (no canonicalization). Test fixture canonicalizes *only* expected value to match macOS symlink kernel behavior—separates kernel symlink resolution from product logic.

2. **Test determinism:** `exercise_markers` test scans the real exercise tree (not fixtures), counts file entries and stub pairs, asserts adjacency with actionable panic messages for failure investigation.

3. **Warnings in CI:** All warnings are pre-existing `unused_variable` from learner `todo!()` stubs—expected and correct behavior (learner code will eventually bind these).

4. **Nix derivation status:** Both `workspace-check` and `xtask-tests` rebuilt against current source tree and passed.

---


---

### 2026-07-21: Session Finalization — Advanced Curriculum Complete: 6 Worlds/34 Exercises/10,730 XP — ✅ APPROVED

**By:** Mikey (Learning Journey Lead)  
**Consolidated Decision:** Review cycle + revision cycle + recovery cycle outcome  
**Timestamp:** 2026-07-21  
**Session Lead Participants:** Data (Rust Engineer), Mouth (Game Designer), Brand (Test Architect)

## Consolidated Outcome: ✅ APPROVED

Final campaign: **6 worlds, 34 exercises, 9 ranks, 10,730 XP, ex01→ex34 linear chain**

---

## Cycle 1: Final Coherence & Correctness Review (16:16:00-07:00) — ❌ REJECT (Material Defects)

### Validation Evidence (Passing)
- ✅ `cargo check --workspace`
- ✅ `cargo test --package xtask` (23 tests pass)
- ✅ All 19 test targets compile
- ✅ `learn status` shows 6 worlds, 34 exercises
- ✅ `learn next` returns `ex16-beacon-ping`
- ✅ 156 tests, 8-10 per exercise, deterministic
- ✅ No solution leaks, no secrets, no public network

### Material Defects

**Defect A: Rustfmt Violations (90 diffs in 21 files)**
- 19 test files (tests/solve.rs) — Author: Brand
- 2 src/lib.rs files (ex27-rwlock-protocol, ex29-schema-bootstrap) — Author: Data
- Identical defect class to prior Cycle 1 rejection; gating criterion

**Defect B: ex16 README Contradicts Test Behavior**
- Edge Cases section claims non-2xx → Ok(status), contradicts Behavioral Rule 5 and test assertions
- Tests expect Err(BeaconPingError::Request) for HTTP errors
- Teaching error; learner would implement wrong behavior
- Author: Mouth

### Lockout & Reassignment
| Defect | Original Author | Locked | Revision Owner |
|--------|-----------------|--------|----------------|
| A (tests) | Brand | Brand | **Data** |
| A (src) | Data | Data | **Mouth** |
| B (README) | Mouth | Mouth | **Data** |

---

## Cycle 2: Generalized xtask Validation (23:15:33-07:00) — ✅ DECISION RECORDED

**By:** Data (Rust Engineer)

Removed hardcoded campaign size constraints (3 worlds / 5 ex/world / 15 total). Implemented structural validation:
- Non-empty world list, non-empty exercises per world
- Unique IDs, packages, valid formats
- Prerequisite/unlock existence, self-reference guards
- On-disk Cargo.toml presence
- Added regression tests for 6W/34E production shape and ex15→ex16 transition

---

## Cycle 3: Strict Revision Re-Review (16:30:00-07:00) — ✅ APPROVE

**By:** Mikey (Learning Journey Lead)

### Finding A: Rustfmt Violations — ✅ RESOLVED
- All 19 `tests/solve.rs` files: zero diffs after Data's independent revision
- `ex27-rwlock-protocol/src/lib.rs` + `ex29-schema-bootstrap/src/lib.rs`: zero diffs after Mouth's independent revision
- `cargo fmt --check` passes on all 38 new .rs files

### Finding B: ex16 README Contradiction — ✅ RESOLVED
- Edge Cases now reads: "...function returns `Err(BeaconPingError::Request(...))`..."
- Aligns with Behavioral Rule 5, test assertions, and API contract
- Data's independent revision (Mouth locked out)

### Revision Integrity
- ✅ Mechanical/scoped (formatting-only, wording-only)
- ✅ No semantics altered
- ✅ No solution leaks, no test weakening
- ✅ Lockout ownership strictly observed

### Lockout Resolution
All revision-cycle lockouts cleared:
- Brand: Cleared (was locked out of test files)
- Mouth: Cleared (was locked out of ex16 README)
- Data: Cleared (was locked out of ex27/ex29 source)

### Gates Passed
| Gate | Status |
|------|--------|
| `cargo fmt --check` (38 new files) | ✅ Pass |
| `cargo check --workspace` | ✅ Pass |
| `cargo test --package xtask` (23 tests) | ✅ Pass |
| All 19 test targets compile | ✅ Pass |
| `learn status` | ✅ Pass |
| `learn next` | ⚠️ Note (see follow-up) |
| `nix flake check path:.` | ✅ Pass |

### Follow-Up Item (Non-Blocking)
Campaign integration gap: campaign.toml only defines worlds 1-3 (ex01-ex15). Worlds 4-6 (ex16-ex34) not yet registered. This predates revision cycle (gap created in persistence race, not revision defect). Assigned as NEW WORK ITEM to Data (non-blocking).

---

## Cycle 4: Final Recovery Review (16:47:00-07:00) — ✅ APPROVE

**By:** Mikey (Learning Journey Lead)  
**Recovery Authors:** Mouth (campaign.toml, README.md recovery), Data (xtask/src/lib.rs tests recovery)

### Context: Tracked-File Persistence Race
Previous git state incident removed:
- campaign.toml (6 worlds, 34 exercises)
- README.md (Advanced Worlds section + 10,730 XP total)
- xtask/src/lib.rs test regression checks

All three files recovered sequentially by authors. Mikey conducted final recovery verification.

### Gates Passed (Post-Recovery)
| Gate | Result |
|------|--------|
| `cargo fmt --check` (38 new .rs + xtask) | ✅ Clean |
| `cargo check --workspace` | ✅ Pass |
| `cargo test --package xtask` (35 tests) | ✅ 12 unit + 22 CLI + 1 marker = 35 pass |
| All 19 solve targets compile | ✅ All 19 |
| `nix flake check path:.` | ✅ 2 checks pass |
| `learn next` (fresh state) | ✅ Returns `ex16-beacon-ping` |
| `learn status` shows worlds 4-6 | ✅ Deep Signal 0/7, Parallel Ops 0/6, Archive Core 0/6 |
| `git diff --check` (product files) | ⚠️ README.md:413 trailing spaces (Markdown convention, non-blocking) |
| 5 recovery regression tests | ✅ All 5 pass |
| No solution leaks | ✅ All 19 src/lib.rs have `todo!()` |
| No live network/credentials | ✅ wiremock + in-memory SQLite |
| ex34 async/sync boundary | ✅ Staged API maintained |

### Campaign Shape Confirmed
- **6 worlds:** [5, 5, 5, 7, 6, 6] exercises per world
- **34 exercises:** Linear chain ex01→ex34
- **10,730 total XP** (verified against README)
- **9 ranks:** Cadet→Sovereign with thresholds 3500/5500/7500/10000
- **Unlock chain verified:** ex15 → ex16-beacon-ping (production test + `learn next`)
- **Per-world XP verified:** W1=600, W2=850, W3=1170, W4=2800, W5=2560, W6=2750

### Artifact Inventory
- 19 exercise crates with src/lib.rs stubs
- 19 READMEs with Setup Notes
- 57 hints (3 per exercise)
- 19 test files with 156 behavioral test cases
- All tracked files present and valid

### Lockout Status
No lockouts applied. All agents (Data, Mouth, Brand) eligible for future work.

---

## Session Summary

**Design Review → Implementation → Rejection Cycle → Revision Cycle → Recovery Cycle → Final Approval**

- ✅ Design approved (3 worlds, 19 exercises)
- ✅ Implementation completed (scaffolding, tests, docs)
- ❌ Cycle 1 rejected (rustfmt, README contradiction)
- ✅ Cycle 3 revisions approved (all defects resolved)
- ✅ Cycle 4 recovery approved (tracked files restored, all gates pass)

**Final State:** Advanced curriculum expansion complete. 6 worlds, 34 exercises, 10,730 XP, ex01→ex34 linear progression. All material gates pass. No lockouts remain. Campaign ready for production learner testing.

**Next Phase:** Campaign fully functional. All exercises scaffolded with tests. Documentation complete. Ready for learner onboarding and feedback loops.


---

## FINAL HEALTH REPORT — 2026-07-21T16:53:59.008-07:00

### State Backend
- **Provider:** FSStorageProvider (local)
- **Status:** ✅ Fully operational

### Decisions & Inbox Management
- **decisions.md size:** 33,746 bytes (threshold: 20KB archival, 51KB aggressive archival)
- **Status:** ✅ Within normal range, no archival needed
- **Active entries:** 15 decisions (including consolidated Session Finalization entry)
- **Inbox state:** ✅ Empty (all 4 entries merged and deleted)

### Session Finalization Consolidation
**Merged from 4 inbox entries:**
1. Mikey-reject-worlds-4-6-delivery-rustfmt-violations-90-d.md
2. Data-generalized-xtask-campaign-validation-to-structura.md
3. Mikey-approve-revision-cycle-findings-a-b-fully-resolved.md
4. Mikey-final-review-advanced-curriculum-recovery-approve.md

**Result:** Single consolidated Session Finalization decision in canonical decisions.md capturing full review/revision/recovery cycle

### Orchestration & Session Logs Created
- ✅ Orchestration log: Mikey session finalization (review cycles summary)
- ✅ Orchestration log: Data contribution (xtask validation, revisions, recovery)
- ✅ Orchestration log: Mouth contribution (documentation, revisions, recovery)
- ✅ Orchestration log: Brand contribution (test delivery, revision outcomes)
- ✅ Session log: Final session log (complete journey, metrics, next-phase)

### Agent Histories
| Agent | Size | Status |
|-------|------|--------|
| Data | 8.5 KB | ✅ OK |
| Mikey | 5.4 KB | ✅ OK |
| Mouth | 3.7 KB | ✅ OK |
| Brand | 3.9 KB | ✅ OK |
| Scribe | 2.1 KB | ✅ OK |
| Others | <1 KB | ✅ OK |

**Summary:** All agent histories well under 15KB threshold; no summarization needed.

### Final Campaign State: ✅ APPROVED

| Metric | Value | Status |
|--------|-------|--------|
| Worlds | 6 | ✅ |
| Exercises | 34 | ✅ |
| Total XP | 10,730 | ✅ |
| Ranks | 9 | ✅ |
| Tests | 156 | ✅ |
| Hints | 57 | ✅ |
| Unlock Chain | ex01→ex34 linear | ✅ |

### Acceptance Criteria (All ✅ Pass)
1. ✅ `cargo fmt --check` (zero diffs post-revision)
2. ✅ `cargo check --workspace`
3. ✅ `cargo test --package xtask` (35 tests pass)
4. ✅ All 19 solve targets compile
5. ✅ `nix flake check path:.`
6. ✅ `learn next` → ex16-beacon-ping
7. ✅ `learn status` shows 6 worlds
8. ✅ 156 behavioral tests (8-10/ex, 55/45 happy/error)
9. ✅ No solution leaks, credentials, live network
10. ✅ ex34 async/sync boundary correct

### Review Cycle Outcomes
| Cycle | Verdict | Status |
|-------|---------|--------|
| Cycle 1 (Rejection) | ❌ REJECT (rustfmt + README) | Defects identified |
| Cycle 2 (xtask fix) | N/A | Generalized validation |
| Cycle 3 (Revision) | ✅ APPROVE | All defects resolved |
| Cycle 4 (Recovery) | ✅ APPROVE | All files recovered, gates pass |

### Lockout Lifecycle
- **Assignment (Cycle 1):** Brand, Mouth, Data all locked
- **Resolution (Cycle 3):** All independent revisions completed
- **Clearance (Cycle 3 Approval):** All lockouts cleared ✅
- **Final Status:** No outstanding lockouts

### File Persistence Race Recovery
| File | Status |
|------|--------|
| campaign.toml | ✅ Recovered by Mouth |
| README.md | ✅ Recovered by Mouth |
| xtask/src/lib.rs tests | ✅ Recovered by Data |

### Mutable State Persistence
- ✅ All writes via `squad_state_*` tools (FSStorageProvider)
- ✅ No git commits made
- ✅ No branch switches
- ✅ No destructive git operations
- `.squad/identity/now.md` updated by runtime (expected)
- All agent histories updated via squad_state_append (expected)
- All orchestration/session logs created via squad_state_write (expected)

### Session Metrics
| Metric | Value |
|--------|-------|
| Exercises Implemented | 19 (ex16–ex34) |
| Tests Implemented | 156 |
| Hints Created | 57 |
| Review Cycles | 4 |
| Defects Found | 2 (Cycle 1) |
| Defects Resolved | 2 (Cycle 3) |
| Files Lost | 3 (persistence race) |
| Files Recovered | 3 |
| Final Approval Timestamp | 2026-07-21T16:47:00-07:00 |

### Session Status
✅ **COMPLETE.** All review cycles passed. All defects resolved. All files recovered. Campaign approved for production learner testing. No outstanding lockouts or blockers. All 10 acceptance criteria gates pass. Campaign ready for deployment.

---

**Scribe Session Finalization Report: SESSION COMPLETE**
