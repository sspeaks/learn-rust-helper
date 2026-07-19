# Project Context

- **Owner:** sspeaks610
- **Project:** learn-rust
- **Mission:** A gamified campaign of increasingly complex, partially finished Rust programs whose function stubs the learner completes.
- **Stack:** Rust, Cargo
- **Created:** 2026-07-19T13:55:57.659-07:00

## Learnings

Initial setup complete. Exercises should provide useful compiler and test feedback while leaving learner-owned function bodies unfinished.


📌 Team update (2026-07-19T13:55:57.659-07:00): Design Review finalized. Architecture: Cargo workspace with one crate per exercise (15 exercises, 3 worlds). Stub convention: `todo!()` everywhere. You own all Rust source, Cargo configs, xtask implementation (A1, A4). See decisions.md for full spec. — Mikey


📌 Campaign Complete (2026-07-19T13:55:57.659-07:00): Final verdict from Mikey: all contract revisions approved, all behavioral specs resolved, Nix integration validated. Campaign ships. Workspace passes cargo check/test/fmt; nix build succeeds; xtask verified. File ownership maintained: Rust source, Cargo configs, xtask, flake infrastructure, workspace glob precision. — Mikey
