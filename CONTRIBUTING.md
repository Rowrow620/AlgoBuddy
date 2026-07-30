# Contributing to AlgoBuddy

Thank you for your interest in contributing to AlgoBuddy! We welcome contributions ranging from bug fixes and documentation improvements to new algorithm visualizers and accessibility features.

## Table of Contents

- [Overview](#overview)
- [Quick Start](#quick-start)
- [Architecture & Engine](#architecture--engine)
- [Auditing & Promoting Problems](#auditing--promoting-problems)
- [Quality Standards & Testing](#quality-standards--testing)
- [Community Guidelines](#community-guidelines)

---

## Overview

AlgoBuddy is an open-source, interactive algorithm learning suite built in Rust using `eframe` and `egui`. It follows the NeetCode 150 learning roadmap across 18 topic categories.

Contributions generally fall into three categories:

1. **Algorithm Visualizers**: Auditing existing problem step generators or building visual state renderers.
2. **UI & Accessibility**: Improving theme contrast, layout math, keyboard navigation, or canvas rendering.
3. **Core Engine & Tools**: Optimizing step snapshot generation, WASM compilation, or test utilities.

---

## Quick Start

### Prerequisites

- [Rust 2021 Edition](https://www.rust-lang.org/) (installed via `rustup`)
- Git

### Development Setup

1. Fork and clone the repository:
   ```powershell
   git clone https://github.com/Rowrow620/AlgoBuddy.git
   cd AlgoBuddy
   ```

2. Run the application in **Developer Mode** (unlocks all 135 problem visualizers):
   ```powershell
   cargo run -- --dev
   ```

3. Run the automated test suite:
   ```powershell
   cargo test
   ```

4. Verify linter compliance:
   ```powershell
   cargo clippy --all-targets -- -D warnings
   ```

---

## Architecture & Engine

The AlgoBuddy codebase is structured into four primary component areas:

- `src/main.rs`: Application entry points for native execution (`eframe::run_native`) and WASM execution (`eframe::WebRunner`).
- `src/model/`: Problem definitions (`Problem`), category taxonomy (`Category`), difficulty levels (`Difficulty`), metadata specs (`ProblemDetails`), and visual state snapshots (`VisualState`).
- `src/app.rs`: Main GUI application state (`VisualizerApp`), navigation, playback controls, canvas renderers, and theme palettes.
- `src/algorithms/`: Step snapshot generator functions (`generate_*_steps`) for each algorithm.

### Deterministic State Engine

Algorithms in AlgoBuddy do not execute asynchronously during playback. Generator functions in `src/algorithms/` execute synchronously upfront and return a `Vec<Step>` snapshot vector. The GUI renders state snapshots based on the active timeline index (`current_step_idx`), enabling forward and backward timeline scrubbing.

### Audit Gating System

Problems in AlgoBuddy carry an audit status (`is_audited(&self) -> bool`):
- **Public Release Mode** (Default): Displays verified, fully audited problem visualizers.
- **Developer Mode** (`cargo run -- --dev`): Unlocks all 135 implemented problem visualizers, marking unaudited implementations with an `[EXP]` tag.

---

## Auditing & Promoting Problems

To audit an existing problem visualizer and promote it to Public Release status:

1. Launch the application in Developer Mode (`cargo run -- --dev`).
2. Verify that the algorithm step generator produces accurate state snapshots for standard and edge-case inputs.
3. Ensure active line highlighting (`code_line`) matches the associated source code snippet.
4. Add a unit test in `src/app.rs` under `#[cfg(test)] mod tests` asserting expected output values.
5. In `src/model/problem.rs`, update the `is_audited` match arm for the target problem to return `true`.
6. Run `cargo test` to verify build and test compliance.

---

## Quality Standards & Testing

Before submitting a Pull Request, ensure your changes adhere to these requirements:

- **Formatting**: Run `cargo fmt --all` to ensure standard Rust code formatting.
- **Clippy Clean**: Run `cargo clippy --all-targets -- -D warnings` to verify zero warnings.
- **Unit Tests**: Add unit tests for any new algorithm step generators or parser functions.
- **Commit Messages**: Write concise, descriptive commit messages (e.g., `feat: add visualizer for problem #X`, `fix: resolve bounds checking on timeline scrubber`).

---

## Community Guidelines

We want AlgoBuddy to be a welcoming project for all developers.

- **Code of Conduct**: Please review our [.github/CODE_OF_CONDUCT.md](.github/CODE_OF_CONDUCT.md).
- **Security Policy**: To report security vulnerabilities privately, refer to [.github/SECURITY.md](.github/SECURITY.md).
- **License**: By contributing, you agree that your contributions will be licensed under the project's [MIT License](LICENSE).
