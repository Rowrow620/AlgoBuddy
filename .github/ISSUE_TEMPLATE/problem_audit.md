---
name: Problem Visualizer Audit
about: Verify and improve an existing problem visualizer's correctness and synchronization
title: 'audit: "[Problem Name]" (#[LeetCode Number])'
labels: 'audit, good first issue, help wanted'
assignees: ''
---

## First Timers Welcome

This issue is designed to be approachable for developers making an early
open-source contribution. Comment on the issue before starting so it can be
assigned to you.

## Goal

Audit **[Problem Name] (LeetCode #[LeetCode Number])** and make sure its
algorithm result, timeline, inspector, code highlighting, and canvas all tell
the same story.

## Suggested Workflow

1. Launch AlgoBuddy with `cargo run`.
2. Select the problem and inspect every timeline step for the default input.
3. Repeat with at least one representative edge case.
4. Verify that each step has:
   - the correct algorithm result and intermediate state;
   - a description matching the highlighted source line;
   - complete, in-bounds visual data; and
   - no disappearing, clipped, or misleading rendered state.
5. Add focused regression coverage in the closest appropriate test module:
   - next to the generator in `src/algorithms/` for trace behavior;
   - `src/app/tests.rs` for application-level behavior;
   - `src/engine/tests.rs` for dispatch and input behavior; or
   - `src/engine/catalog_tests.rs` for catalog-wide invariants.
6. Run the contributor quality gates:
   ```text
   cargo fmt --all -- --check
   cargo clippy --all-targets -- -D warnings
   cargo test --all
   ```

## Acceptance Criteria

- [ ] The default result and one edge-case result are correct.
- [ ] Timeline descriptions, source lines, inspector values, and canvas state
      remain synchronized.
- [ ] The visualizer remains usable with Play, Pause, Prev, Next, scrub, and
      Reset.
- [ ] Focused regression coverage is included where practical.
- [ ] Formatting, Clippy, and all tests pass.
- [ ] The pull request targets the `dev` branch and links this issue.

## Helpful Links

- [Contributing Guide](https://github.com/Rowrow620/AlgoBuddy/blob/dev/CONTRIBUTING.md)
- [README](https://github.com/Rowrow620/AlgoBuddy/blob/dev/README.md)
