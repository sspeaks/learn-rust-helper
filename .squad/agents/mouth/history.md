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

---

### 2026-07-21T16:40:10.072-07:00: Advanced Campaign Recovery

**Trigger:** Concurrent task merge reverted `campaign.toml` and `README.md` to Worlds 1–3 only. All 19 World 4–6 exercise READMEs and 57 hints survived intact. Prior reviewer lockouts cleared before this task.

**Files recovered:**
- `campaign.toml`: Added 4 new ranks (Vanguard ✦ 3500, Admiral ⬡ 5500, Architect ⬟ 7500, Sovereign ⬢ 10000). Changed ex15 `unlocks` from `[]` to `["ex16-beacon-ping"]`. Appended Worlds 4 (7 exercises, 2,800 XP), 5 (6 exercises, 2,560 XP), 6 (6 exercises, 2,750 XP) with full linear prerequisite/unlock chain ex01→ex34.
- `README.md`: Updated exercise count (15→34), total XP (2,620→10,730), corrected World 2/3 XP figures (850/1,170), added Worlds 4–6 tables with dependency notes, updated file structure, updated rank table with new entries, updated footer.

**Validation:** Python tomli parse confirmed 6 worlds, 34 exercises, 9 ranks, zero prereq/unlock issues, single linear chain ex01→ex34 terminal, total XP 10,730. `git status` confirmed both files modified (M README.md, M campaign.toml).

**Status:** ✅ Complete — no exercise README or hint was altered.


---

## 2026-07-21T16:53:59.008-07:00 | Session Finalization Contribution

**Role:** Game Designer

**Final Contributions:**
- All ex27/ex29 source revisions (rustfmt fixes)
- Recovery: Restored campaign.toml (6W/34E) after persistence race
- Recovery: Restored README.md Advanced section after persistence race
- Verified per-world XP totals (W1=600, W2=850, W3=1170, W4=2800, W5=2560, W6=2750)

**Lockout Status:** Cleared ✅

**Gates Owned:** All ✅ pass


📌 Team update (2026-07-22T11:53:34.452-07:00): Framing copy contract finalized and recorded in decisions.md. Issue #1 solution command execution order: Mouth→Data→Brand. All 5 player-facing surfaces specified (success display, incomplete guidance, missing artifact, dashboard discoverability, post-check message). Ready to write 15 solution.rs files. — Scribe

📌 Team update (2026-07-22T16:27:03Z): Design Review campaign completeness completed. You own action item: Fix ex19 learner docs/hints. Correct endpoint to `/relay/dispatch` (not `/relay`). — Mikey (facilitator)

✅ Campaign Complete (2026-07-22T16:56:56Z): Ex19 endpoint guidance successfully corrected across README, hint2, and hint3 (`{base_url}/relay/dispatch`). Approved by Brand. Campaign now 34/34. — Mikey (facilitator)

📌 Team update (2026-07-23T12:34:45.793-07:00): Learner-facing contract completeness — all exercise instructions must fully specify test behavior; learners must never need to read tests to discover requirements. — decided by Seth Speaks
