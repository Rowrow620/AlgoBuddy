---
name: Unit Test Request
about: Create a beginner-friendly issue template for adding unit test coverage to an algorithm visualizer
title: 'test: Add unit test for '
labels: 'good first issue, help wanted, testing'
assignees: ''
---

## First Timers Only
This issue is reserved for developers who are new to AlgoBuddy or open-source contributing. We know that submitting a first pull request can feel intimidating. The goal of this issue is to guide you step-by-step through writing a unit test for an algorithm visualizer in AlgoBuddy.

## Description of the Issue
The visualizer for **[Problem Name] (LeetCode #[Problem ID])** is live in Public Release, but needs an automated unit test in `src/app.rs` to verify that step generation produces the expected visual snapshots and state transitions.

## Implementation Steps
1. Launch AlgoBuddy locally: `cargo run`
2. Select **[Problem Name]** and scrub through the timeline steps to verify variable snapshots and visual state outputs.
3. Open `src/app.rs` under `#[cfg(test)] mod tests`.
4. Add a new test function `test_[problem_slug]_logic_correctness()` following existing test patterns (e.g. `test_two_sum_logic_correctness`):
   ```rust
   #[test]
   fn test_[problem_slug]_logic_correctness() {
       let mut app = VisualizerApp::default();
       app.current_problem = Problem::[ProblemVariant];
       // Set input fields...
       app.recompute_steps();

       let last_step = app.steps.last().expect("Steps should not be empty");
       // Assert visual state properties...
   }
   ```
5. Run `cargo test` locally to verify your test passes.

## Acceptance Criteria
- Code Formatting: `cargo fmt --all -- --check` completes without warnings.
- All Tests Pass: `cargo test` passes 100% of unit tests.
- Issue Solved: Automated unit test is added and passes cleanly.
- Clean Git Branch: Changes are committed to a feature branch targeting `dev`.

## Step-by-Step Contribution Guide
1. Claim this issue: Comment below that you are working on this issue to receive assignment.
2. Fork and Branch: Create a new branch off `dev` named `test-[problem-slug]`.
3. Make and Test Changes: Run `cargo fmt --all` and `cargo test` locally to verify build health.
4. Submit PR: Push your branch to GitHub and create a Pull Request targeting the `dev` branch.

## Additional Information
- [Contributing Guide](https://github.com/Rowrow620/AlgoBuddy/blob/dev/CONTRIBUTING.md)
- [README](https://github.com/Rowrow620/AlgoBuddy/blob/dev/README.md)
