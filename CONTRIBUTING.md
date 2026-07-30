# Contributing to AlgoBuddy

Thank you for your interest in contributing to AlgoBuddy. This document outlines the development workflow, project architecture, testing requirements, and submission process for pull requests.

---

## Development Setup

### Prerequisites
- Rust 1.75 or later (`rustup update stable`)
- Cargo (included with standard Rust installation)
- Trunk (for WebAssembly builds): `cargo install trunk`
- WASM target: `rustup target add wasm32-unknown-unknown`

### Local Execution

To run the native desktop application:
```bash
cargo run
```

To run the WebAssembly application locally in a browser:
```bash
trunk serve
```
Then navigate to `http://127.0.0.1:8080`.

---

## Running Tests

All algorithm step generators and parsing logic are validated using automated Rust unit tests. Run the full test suite with:

```bash
cargo test
```

Ensure all tests pass before submitting a pull request.

---

## Architecture Overview

AlgoBuddy is structured into four core areas:

1. `src/main.rs`: Application entry points for native execution (`eframe::run_native`) and WASM execution (`eframe::WebRunner`).
2. `src/model.rs`: Problem definitions (`Problem`), category taxonomy (`Category`), difficulty levels (`Difficulty`), metadata specs (`ProblemDetails`), and visual state snapshots (`VisualState`).
3. `src/app.rs`: Main GUI application state (`VisualizerApp`), UI view modes, navigation, playback controls, canvas renderers, and theme palettes.
4. `src/algorithms/`: Step snapshot generator functions (`generate_*_steps`) for each algorithm.

### Deterministic State Engine
Algorithms in AlgoBuddy do not execute asynchronously during playback. Instead, generator functions in `src/algorithms/` execute synchronously upfront and produce a `Vec<Step>` snapshot vector. The GUI renders state snapshots based on the active timeline index (`current_step_idx`), allowing forward and backward scrubbing.

### Release Mode and Audit Gating
Problems in AlgoBuddy carry an audit status (`AuditStatus::Audited` or `AuditStatus::Unaudited`).
- By default (Public Release Mode), the UI presents only audited problems.
- Developer Mode (toggleable in Settings) displays all implemented problems, flagging unaudited implementations with an `[EXP]` tag.

---

## How to Audit or Promote a Problem

To audit an existing problem visualizer and promote it to Public Release status:

1. Open the application in Developer Mode (`show_unaudited: true`).
2. Verify that the algorithm step generator produces accurate state snapshots for standard and edge-case inputs.
3. Ensure active line highlighting (`code_line`) matches the associated source code snippet.
4. Add a unit test in `src/app.rs` under `#[cfg(test)] mod tests` asserting expected output values.
5. In `src/model.rs`, update the `audit_status` match arm for the target problem to return `AuditStatus::Audited`.
6. Run `cargo test` to verify build and test compliance.

---

## Pull Request Guidelines

1. **Branch Naming**: Use descriptive branch names such as `feat/audit-two-sum` or `fix/canvas-render-bounds`.
2. **Code Formatting**: Format code using `cargo fmt` before committing.
3. **Clippy Compliance**: Ensure `cargo clippy --all-targets -- -D warnings` reports zero warnings.
4. **Test Coverage**: Include unit tests for any new algorithm step generators or parser utilities.
5. **Commit Messages**: Write concise commit messages following standard conventions (e.g., `feat: add visualizer for problem #X`, `fix: resolve bounds checking on timeline scrubber`).
