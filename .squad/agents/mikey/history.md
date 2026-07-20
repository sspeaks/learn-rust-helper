# Project Context

- **Owner:** sspeaks610
- **Project:** learn-rust
- **Mission:** A gamified campaign of increasingly complex, partially finished Rust programs whose function stubs the learner completes.
- **Stack:** Rust, Cargo
- **Created:** 2026-07-19T13:55:57.659-07:00

## Learnings

Initial setup complete. The campaign should increase in complexity while preserving a satisfying, playable progression.


📌 Campaign Complete (2026-07-19T13:55:57.659-07:00): Final verdict issued. Campaign ships. 15 exercises, 3 worlds, 45 hints, 148 tests, Nix-only toolchain. All design review decisions, contract reviews, lockout revisions, and Nix directive integrated. Team: Data, Mouth, Brand. Implementation complete and verified. — Mikey


---

### 2026-07-20T12:22:07.761-07:00: Design Review & Final Approval — Guided `learn` CLI (3-Cycle Arc)

**Ceremony:** Design Review → Cycle 1 Review → Cycle 2 Incident Recovery → Cycle 3 Re-Review

**Cycle 1 (Design Review):** Approved the `learn` CLI contract.
- 6 architectural decisions documented
- File ownership established (Data: implementation/Nix, Mouth: docs, Brand: tests)
- 17-test acceptance criteria defined

**Cycle 1 (Final Review):** Conditional rejection.
- Implementation functionally correct and fully met approved contract
- Gating failure: `cargo fmt --all -- --check` reported 6 rustfmt diffs in Brand's test files
- Decision: Brand locked out; Data assigned as independent revision owner

**Cycle 2 Incident:**
- Accidental `git checkout . && git clean -fd` erased all uncommitted work
- Violated no-destructive-git policy (implicit squad convention)
- No new policy created (requires user approval); incident recorded for institutional memory

**Cycle 2 Recovery:**
- Mouth replayed its unrejected docs
- Data restored implementation and independently recreated test artifacts from approved behavioral contract
- Data formatted all code

**Cycle 3 Re-Review (13:16:00-07:00):** Unconditional approval.
- ✅ `cargo fmt --all -- --check` — Pass
- ✅ `cargo test --package xtask` — 24 tests pass
- ✅ `nix flake check path:.` — Pass
- ✅ `nix build path:.` → `result/bin/learn` (1.7 MB)
- ✅ `nix run path:. -- next` — Outputs `ex01-format-scoreboard`
- ✅ `cargo xtask status` — Legacy alias works
- ✅ Dashboard, no-arg behavior, all commands functional
- ✅ Progress isolation: `.learn-rust/progress.toml` never created
- ✅ Exercise READMEs (all 15): Use `learn check`/`learn hint`, zero stale refs
- ✅ Root README: Nix-only, `learn`-first, Cargo demoted
- ✅ All gating criteria pass; no prior evidence reused; all checks re-executed on current tree

**Status:** ✅ **Guided CLI ships** — Cycle 3 unconditional approval.
