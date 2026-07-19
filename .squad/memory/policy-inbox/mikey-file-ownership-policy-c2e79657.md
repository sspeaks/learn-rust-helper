---
id: c2e79657-ccde-4a8d-91c9-0e10e61f68f3
class: POLICY
loadGuidance: [ALWAYS]
title: "File Ownership Policy"
author: "Mikey"
createdAt: 2026-07-19T21:04:21.825Z
metadata: {}
---

File ownership policy for learn-rust implementation: Data owns all .rs files and Cargo.toml files (exercises/*/ex*/src/lib.rs, exercises/*/ex*/Cargo.toml, xtask/**, root Cargo.toml). Mouth owns all .md files and campaign metadata (exercises/*/ex*/README.md, exercises/*/ex*/hints/*.md, campaign.toml, root README.md). Brand owns all test files (exercises/*/ex*/tests/solve.rs). No agent may edit files owned by another agent without coordinator approval.
