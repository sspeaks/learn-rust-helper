# Squad Decisions

## Active Decisions

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

## Design Review — learn-rust v1 Architecture

(See archived decision section below for v1 design reference; this is the canonical active decision record.)

**Core Deliverable:** 15-exercise gamified Rust learning campaign

**Architecture:** Cargo workspace (D1 updated with glob specificity), 3 worlds × 5 exercises, one crate per exercise. Learner stubs use `todo!()` (D2). Workspace compiles clean, tests fail on untouched stubs (D3). Per-exercise structure: `src/lib.rs` (stubs), `tests/solve.rs` (behavioral tests), `README.md` (quest prompt), `hints/` (3-tier progressive guides) (D4). Stub detection via panic message (D5). Test philosophy: behavioral black-box only, never enforce implementation shape (D6). v1 scope: 15 exercises (D7). Gamification: local XP + ranks + progressive hints, xtask runner (D8, D9). Unlock logic: advisory only (D10).

**File Ownership (Non-Overlapping)**
| Agent | Owns | Paths |
|-------|------|-------|
| Data | All Rust source, Cargo configs, xtask implementation, workspace glob precision | `Cargo.toml`, `xtask/**/*.rs`, `xtask/Cargo.toml`, `exercises/*/ex[0-9][0-9]-*/Cargo.toml`, `exercises/*/ex[0-9][0-9]-*/src/lib.rs`, `flake.nix`, `Cargo.lock`, `.envrc` |
| Mouth | All learner-facing content, campaign metadata, hints, onboarding README | `exercises/*/ex[0-9][0-9]-*/README.md`, `exercises/*/ex[0-9][0-9]-*/hints/*.md`, `campaign.toml`, `README.md` (root), `.gitignore` additions |
| Brand | All test code and test utilities (under original design review; locked out for ex08/ex15 revisions; Mikey-approved Data revisions now canonical) | `exercises/*/ex[0-9][0-9]-*/tests/solve.rs`, shared test helpers if any |

**Verification Criteria**
1. `cargo check --workspace` passes on fresh clone. ✅
2. `cargo test -p ex01-hello-fix` fails with recognizable "not started" message. ✅
3. Filling in a correct implementation makes `cargo test -p exNN` pass. ✅
4. Multiple valid implementations pass the same test suite. ✅
5. `cargo xtask status` displays progress correctly. ✅
6. Hints never reveal the complete solution. ✅
7. No exercise depends on another exercise's completion to compile. ✅
8. `nix flake check path:.` passes (added by Nix directive). ✅

---

## Implementation Status: ✅ COMPLETE — CAMPAIGN SHIPS

- **Data:** Workspace skeleton (A1) ✅ + xtask progress tracking (A4) ✅ + Nix integration ✅ + contract revisions ✅
- **Mouth:** Exercise READMEs (A2) ✅ + campaign.toml ✅ + 45 hints (3×15) ✅ + root README ✅ + Nix-only onboarding ✅
- **Brand:** 148 behavioral tests (A3) ✅ + test utilities ✅ + nix build validation ✅ (then locked out; Data revisions approved)
- **Mikey:** Design review ✅ + contract review ✅ + re-review of Data revisions ✅ + final verdict ✅

---

## Governance

- All meaningful changes require team consensus ✅
- Document architectural decisions here ✅
- Keep history focused on work, decisions focused on direction ✅
