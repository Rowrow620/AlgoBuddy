---
name: Good First Issue
about: Create a beginner-friendly issue template for new open-source contributors
title: '[good first issue] '
labels: 'good first issue, help wanted'
assignees: ''
---

## First Timers Only
This issue is reserved for developers who are new to AlgoBuddy or open-source contributing. We know that submitting a first pull request can feel intimidating. The goal of this issue is to guide you step-by-step through making your first contribution to the AlgoBuddy repository.

## Description of the Issue
[Describe the current behavior and why this enhancement/fix is needed]

## Proposed Solution
[Describe the proposed solution and user experience]

## Implementation Steps
1. Open `[file path]` and locate `[function name]`.
2. [Step-by-step implementation guide with code snippets if applicable]
3. Test locally using `cargo run` or `cargo test`.

## Acceptance Criteria
To merge a pull request for this issue:
- Code Formatting: `cargo fmt --all -- --check` completes without warnings.
- All Tests Pass: `cargo test` passes 100% of unit tests.
- Issue Solved: The implementation fully addresses the requirements described above.
- Clean Git Branch: Changes are committed to a feature branch targeting `dev`.

## Step-by-Step Contribution Guide
1. Claim this issue: Comment below that you are working on this issue to receive assignment.
2. Fork and Branch: Create a new branch off `dev` named `issue-[id]-[feature-name]`.
3. Make and Test Changes: Run `cargo fmt --all` and `cargo test` locally to verify build health.
4. Submit PR: Push your branch to GitHub and create a Pull Request targeting the `dev` branch.

## Additional Information
- [Contributing Guide](https://github.com/Rowrow620/AlgoBuddy/blob/dev/CONTRIBUTING.md)
- [README](https://github.com/Rowrow620/AlgoBuddy/blob/dev/README.md)
