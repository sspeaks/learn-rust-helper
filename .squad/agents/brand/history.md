# Project Context

- **Owner:** sspeaks610
- **Project:** learn-rust
- **Mission:** A gamified campaign of increasingly complex, partially finished Rust programs whose function stubs the learner completes.
- **Stack:** Rust, Cargo
- **Created:** 2026-07-19T13:55:57.659-07:00

## Learnings

Initial setup complete. Validation must distinguish untouched stubs from correct solutions while allowing multiple idiomatic implementations.


📌 Team update (2026-07-19T13:55:57.659-07:00): Design Review finalized. Test philosophy: behavioral black-box only; never enforce implementation shape. Your role: write tests/solve.rs for all 15 exercises with educational failure messages and stub-detection logic. A3 is your deliverable. See decisions.md for verification criteria. — Mikey


📌 Campaign Complete (2026-07-19T13:55:57.659-07:00): Reviewer lockout lifted. Final verdict from Mikey: Data's contract revisions (ex08 charge capping, ex15 pipe delimiter, ex15 0-based indexing, ex15 all-items top_targets) approved. 148 test cases validated under Nix build. All tests behavioral black-box compliant. Campaign ships. — Mikey


---

### 2026-07-20T12:22:07.761-07:00: Acceptance Tests — Guided `learn` CLI (Lockout & Supersession)

**Original Deliverables:**
- `xtask/tests/cli.rs` (17 behavioral tests)
- `xtask/tests/support/mod.rs` (fixture workspace, fake cargo runner)

**Cycle 1 Review:** Functional correctness approved; conditional rejection due to 6 rustfmt diffs in Brand-authored test files.
- **Rustfmt failures:** `cargo fmt --all -- --check` reported formatting issues in line length, argument formatting, method chain formatting.
- **Outcome:** Brand locked out. Data assigned as independent revision owner (per lockout map — Brand authored these files; revision goes to a different agent).

**Cycle 2 Incident:** Accidental `git checkout . && git clean -fd` erased all uncommitted work.

**Cycle 3 Recovery & Supersession:**
- Data independently recreated test artifacts from the approved behavioral contract
- Data applied formatting; `cargo fmt --all -- --check` passed clean
- Data's independent recreation supersedes Brand's original submission (still approved in spirit; test contract and behavior unchanged, only formatting refined)

**Cycle 3 Final Status:**
- ✅ 17 acceptance tests pass
- ✅ All gating criteria pass
- ✅ Tests approved as part of Data's Cycle 3 delivery

**Note:** Brand's original test design and behavioral contract were sound. The conditional rejection was purely a formatting gate, now resolved by Data's independent recreation and formatting.
