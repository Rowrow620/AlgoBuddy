# Contributing to AlgoBuddy

Thank you for your interest in contributing to AlgoBuddy! We welcome contributions ranging from bug fixes and documentation improvements to new algorithm visualizers and accessibility features.

## Table of Contents

- [Overview](#overview)
- [Quick Start](#quick-start)
- [Architecture & Engine](#architecture--engine)
- [Auditing & Promoting Problems](#auditing--promoting-problems)
- [Quality Standards & Testing](#quality-standards--testing)
- [Code of Conduct](#code-of-conduct)
- [Security Policy](#security-policy)

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

2. Run the application in **Developer Mode** (unlocks all 150 problem visualizers):
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
- **Developer Mode** (`cargo run -- --dev`): Unlocks all 150 implemented problem visualizers, marking unaudited implementations with an `[EXP]` tag.

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

## Code of Conduct

### Our Pledge

We as members, contributors, and leaders pledge to make participation in our community a harassment-free experience for everyone, regardless of age, body size, visible or invisible disability, ethnicity, sex characteristics, gender identity and expression, level of experience, education, socio-economic status, nationality, personal appearance, race, religion, or sexual identity and orientation.

### Standards & Enforcement

- **Acceptable Behavior**: Demonstrating empathy, being respectful of differing viewpoints, gracefully accepting feedback, and focusing on the community's best interest.
- **Unacceptable Behavior**: Sexualized language/imagery, trolling, derogatory comments, personal attacks, or public/private harassment.
- **Reporting**: Instances of unacceptable behavior may be reported privately to fender620@gmail.com.

---

## Security Policy

### Supported Versions

Current release series `0.5.x` is actively supported with security updates.

### Vulnerability Reporting

If you discover a security defect or vulnerability, please report it privately:

1. **Do Not File a Public Issue**: Please do not open public issues for security bugs.
2. **Email**: Send vulnerability details privately to fender620@gmail.com.
3. **Response**: We acknowledge security reports within 48 hours and release verified fixes in patch updates.
