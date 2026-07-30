---
name: Problem Audit & Promotion
about: Audit an existing problem visualizer and promote it to Public Release status
title: 'audit: "[Problem Name]" (#[LeetCode Number]) to Public Release'
labels: 'audit, good first issue, help wanted'
assignees: ''
---

## First Timers Only
This issue is reserved for developers who are new to AlgoBuddy or open-source contributing. We know that submitting a first pull request can feel intimidating. The goal of this issue is to guide you step-by-step through making your first contribution to the AlgoBuddy repository.

## Description of the Issue
The visualizer for **[Problem Name] (LeetCode #[LeetCode Number])** is currently in Developer Mode (`[EXP]`). We need to audit its step generator, verify active line highlighting, add an automated unit test, and promote it to Public Release status (`AuditStatus::Audited`).

## Implementation Steps
1. Launch AlgoBuddy in Developer Mode: `cargo run -- --dev`
2. Select **[Problem Name] (#[LeetCode Number])** and scrub through the timeline steps to verify:
   - Variable snapshots and visual state representations are accurate.
   - Source code line highlighting (`code_line`) matches the active execution step.
3. Open `src/app.rs` under `#[cfg(test)] mod tests` and add a unit test asserting step output for a sample test case.
4. Open `src/model/problem.rs` and update `audit_status(&self)` to include `Problem::[ProblemVariant]`:
   ```rust
   match self {
       Problem::ContainsDuplicate | Problem::TwoSum | Problem::ValidAnagram | Problem::[ProblemVariant] => {
           AuditStatus::Audited
       }
       _ => AuditStatus::Unaudited,
   }
   ```
5. Run `cargo test` to verify all tests pass.

## Acceptance Criteria
To merge a pull request for this issue:
- Code Formatting: `cargo fmt --all -- --check` completes without warnings.
- All Tests Pass: `cargo test` passes 100% of unit tests.
- Issue Solved: `[Problem Name]` visualizer appears in Public Release Mode without requiring `--dev`.
- Clean Git Branch: Changes are committed to a feature branch targeting `dev`.

## Step-by-Step Contribution Guide
1. Claim this issue: Comment below that you are working on this issue to receive assignment.
2. Fork and Branch: Create a new branch off `dev` named `audit-[problem-slug]`.
3. Make and Test Changes: Run `cargo fmt --all` and `cargo test` locally to verify build health.
4. Submit PR: Push your branch to GitHub and create a Pull Request targeting the `dev` branch.

## Additional Information
- [Contributing Guide](https://github.com/Rowrow620/AlgoBuddy/blob/dev/CONTRIBUTING.md)
- [README](https://github.com/Rowrow620/AlgoBuddy/blob/dev/README.md)
