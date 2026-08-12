---
name: Unit Test Request
about: Add focused regression coverage to an algorithm visualizer
title: 'test: Add coverage for '
labels: 'good first issue, help wanted, testing'
assignees: ''
---

## First Timers Welcome

This issue is designed to guide an early open-source contribution. Comment on
the issue before starting so it can be assigned to you.

## Goal

Add regression coverage for **[Problem Name] (LeetCode #[Problem ID])**. The
test should verify a meaningful result or state transition, not merely that the
visualizer produces a non-empty timeline.

## Suggested Workflow

1. Launch AlgoBuddy with `cargo run` and inspect the visualizer's default flow.
2. Identify the behavior named in this issue and one useful edge case.
3. Put the test in the closest appropriate location:
   - next to the generator in `src/algorithms/` for trace snapshots;
   - `src/app/tests.rs` for application-level outcomes;
   - `src/engine/tests.rs` for dispatch, input, and registry behavior; or
   - `src/engine/catalog_tests.rs` for catalog-wide invariants.
4. Assert the expected final result and any intermediate state that protects
   against the regression.
5. Run:
   ```text
   cargo fmt --all -- --check
   cargo clippy --all-targets -- -D warnings
   cargo test --all
   ```

## Acceptance Criteria

- [ ] The test covers the behavior requested by the issue.
- [ ] Assertions verify concrete values or transitions.
- [ ] Formatting, Clippy, and the complete test suite pass.
- [ ] The pull request targets `dev` and links this issue.

## Helpful Links

- [Contributing Guide](https://github.com/Rowrow620/AlgoBuddy/blob/dev/CONTRIBUTING.md)
- [README](https://github.com/Rowrow620/AlgoBuddy/blob/dev/README.md)
