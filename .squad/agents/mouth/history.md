# Project Context

- **Owner:** sspeaks610
- **Project:** learn-rust
- **Mission:** A gamified campaign of increasingly complex, partially finished Rust programs whose function stubs the learner completes.
- **Stack:** Rust, Cargo
- **Created:** 2026-07-19T13:55:57.659-07:00

## Learnings

Initial setup complete. The experience should feel like a campaign, with visible progress and spoiler-light hints rather than a flat exercise list.


📌 Team update (2026-07-19T13:55:57.659-07:00): Design Review finalized. Architecture: 15 exercises across 3 worlds (Foundations, Ownership, Collections & Errors). You own learner-facing content: READMEs, hints (3 per exercise, progressive nudge-direction-structure), campaign.toml metadata, root README, and progress schema. A2 is your deliverable. See decisions.md for full spec. — Mikey


📌 Campaign Complete (2026-07-19T13:55:57.659-07:00): Final verdict from Mikey: all learner content approved, hints correct and non-spoiler, campaign.toml schema validated, Nix-only onboarding integrated into root README. Campaign ships. File ownership maintained: exercise READMEs, hints (45 total), campaign.toml, root README, .gitignore. — Mikey


---

### 2026-07-20T12:22:07.761-07:00: Documentation Rewrite — Guided `learn` CLI

**Deliverables:**
- `README.md` (root): Nix-only setup, `learn`-first workflow, Cargo demoted to Advanced section
- 15× exercise `README.md` files: Updated command references from `cargo xtask` to `learn check`/`learn hint`
- Removed all stale `cargo xtask` references from learner-facing content

**Cycle 1:** Docs approved and unrejected.

**Incident (Cycle 2):** Accidental `git checkout . && git clean -fd` erased all uncommitted work.
- **Recovery:** Replayed docs (no conflicts, no revisions needed).

**Cycle 3 Review:** Documentation approved as-is.
- Root README: Nix-only setup, `learn`-first, Cargo demoted
- Exercise READMEs: All use `learn check`/`learn hint`, zero `cargo xtask` references

**Status:** ✅ Approved (Cycle 3) — Learner docs complete.
