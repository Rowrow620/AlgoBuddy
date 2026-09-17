# Contributing to AlgoBuddy

Thank you for your interest in contributing to AlgoBuddy! We welcome contributions ranging from bug fixes and documentation improvements to new algorithm visualizers and accessibility features.

## Table of Contents

- [Overview](#overview)
- [Quick Start](#quick-start)
- [Architecture & Engine](#architecture--engine)
- [Unit Testing & Contributing Visualizers](#unit-testing--contributing-visualizers)
- [Maintainer Workflow](#maintainer-workflow)
- [Quality Standards & Testing](#quality-standards--testing)
- [Code of Conduct](#code-of-conduct)
- [Security Policy](#security-policy)

---

## Overview

AlgoBuddy is an open-source, interactive algorithm learning suite built in Rust using `eframe` and `egui`. It follows the NeetCode 150 learning roadmap across 18 topic categories.

Contributions generally fall into three categories:

1. **Algorithm Visualizers**: Improving existing problem step generators or building visual state renderers.
2. **UI & Accessibility**: Improving theme contrast, layout math, keyboard navigation, or canvas rendering.
3. **Core Engine & Tools**: Optimizing step snapshot generation, WASM compilation, or test utilities.

---

## Quick Start

### Prerequisites

- A [stable Rust toolchain](https://www.rust-lang.org/tools/install), including `rustfmt` and Clippy. The crate uses the Rust 2021 edition.
- Git
- For WebAssembly, browser-startup, or hosting changes: Node.js 20 or newer, npm, Trunk 0.21.5, and the `wasm32-unknown-unknown` Rust target

### Development Setup

1. Fork and clone the repository:
   ```text
   git clone https://github.com/Rowrow620/AlgoBuddy.git
   cd AlgoBuddy
   ```

2. Run the application:
   ```text
   cargo run
   ```

3. Run the automated test suite:
   ```text
   cargo test --all
   ```

4. Verify linter compliance:
   ```text
   cargo clippy --all-targets -- -D warnings
   ```

### WebAssembly Launch Test

Pull requests and pushes to `dev` or `main` run a browser launch test in CI. If
your change affects WebAssembly startup, `index.html`, GitHub Actions, or hosting,
run the same check locally:

```text
rustup target add wasm32-unknown-unknown
npm ci
npx playwright install chromium
trunk build --release --public-url ./
npm run stamp:wasm
npm run test:wasm-smoke
```

The smoke test serves the generated bundle at the same `/AlgoBuddy/` subpath
used by GitHub Pages. It verifies the JavaScript and WebAssembly responses,
WebAssembly MIME type, rendered-app readiness, and the bundle's Git revision.
`npm run stamp:wasm` reads the current commit from Git when `DEPLOY_SHA` is not
set, and the local smoke test reads that revision from the generated bundle.

---

## Architecture & Engine

The AlgoBuddy codebase is structured into six primary component areas:

- `src/main.rs`: Application entry points for native execution (`eframe::run_native`) and WASM execution (`eframe::WebRunner`).
- `src/model/`: Problem definitions (`Problem`), category taxonomy (`Category`), difficulty levels (`Difficulty`), metadata specs (`ProblemDetails`), and canonical visual state snapshots (`VisualState`). Problem details and code lines are modularized by category under `src/model/problems/`.
- `src/app.rs`: Main GUI application state (`VisualizerApp`). Manages dynamic playground input state via `input_strings` and `input_integers` `HashMap` state stores (`get_input_str`, `set_input_str`, `get_input_int`, `set_input_int`).
- `src/ui/`: UI submodules containing navigation panels, playback controls, canvas renderers, and theme palettes. Larger renderer families are grouped below `src/ui/canvas/`, such as the sequence, search, hashing, pointer, and product modules under `src/ui/canvas/arrays/`.
- `src/engine.rs` and `src/engine/`: Core deterministic execution routing, category dispatch modules, input parsing, focused engine tests, and catalog-wide validation.
- `src/algorithms/`: Step snapshot generator functions (`generate_*_steps`) for each algorithm. Larger families use focused submodules; for example, `src/algorithms/trees/` separates basic operations, BST problems, traversals, construction, path calculations, codecs, and tests.

### Deterministic State Engine & Visual Normalization

Algorithms in AlgoBuddy do not execute asynchronously during playback. Generator functions in `src/algorithms/` execute synchronously upfront and return a `Vec<Step>` snapshot vector. Typed `VisualState` variants are routed to specialized renderers so algorithm state, layout, and playback remain decoupled.

## Unit Testing & Contributing Visualizers

To contribute a unit test or visualizer improvement:

1. Create a descriptive feature branch off `dev`, such as `issue-[id]-[problem-slug]`.
2. Verify that the algorithm step generator produces accurate state snapshots for standard and edge-case inputs.
3. Ensure active line highlighting (`code_line`) matches the associated source code snippet.
4. Add focused coverage next to the generator in `src/algorithms/`, in `src/app/tests.rs` for application behavior, in `src/engine/tests.rs` for dispatch and input behavior, or in `src/engine/catalog_tests.rs` for catalog-wide invariants.
5. Run `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo test --all` locally.
6. Submit a Pull Request targeting the `dev` branch.

### Adding a Solution Variant

A problem can expose more than one solution through its `approaches` metadata. To add a variant:

1. Add an `ApproachMeta` entry with a unique ID in the problem's category file under `src/model/problems/`.
2. Route that ID to the matching step generator. Unknown IDs must not fall through to a different solution.
3. Add the exact source listing for that ID and keep every `Step::code_line` synchronized with it.
4. Make the canvas, invariant, and scope variables reflect the selected solution rather than assuming the primary approach.
5. Validate the displayed solution's input assumptions before building its trace. For unsupported custom input, return `TraceUnavailable` and explain the requirement beside the input controls.
6. If a trace clones large state or grows faster than linearly, add a visible visualization limit and return `TraceUnavailable` beyond it instead of silently truncating or reporting a false result.
7. Add focused tests showing that each solution returns a valid result, then run the catalog test to validate every advertised trace.

The header builds its solution controls from metadata. Selecting one preserves the current input while restarting playback at step 1 with the new trace.

---

## Maintainer Workflow

Maintainers follow the same review path as other contributors for normal development. Maintainer access is not a reason to bypass a branch, review, or required checks.

- Treat `dev` as the integration branch. Start features, fixes, refactors, and documentation work on a short-lived branch based on the latest `dev`, then open a Pull Request back into `dev`.
- Reserve direct commits to `dev` for coordinated release-candidate assembly or narrowly scoped fixes required to complete the release checks. Run the complete release check before pushing that candidate.
- Treat `main` as the production branch. Do not commit or push directly to `main`; promote releases only through a reviewed Pull Request from `dev`.
- Do not rewrite the shared history of `dev` or `main`.
- Wait for CI and CodeQL to pass before merging. CI now runs the native quality gates before its WebAssembly launch test. A `main` deployment uses that exact tested bundle and then launches the published site in a second browser check. Release Pull Requests must also satisfy the version, changelog, native-build, and WebAssembly-build requirements in [RELEASING.md](RELEASING.md).

---

## Quality Standards & Testing

Before submitting a Pull Request, ensure your changes adhere to these requirements:

- **Formatting**: Run `cargo fmt --all -- --check` to verify standard Rust formatting.
- **Clippy Clean**: Run `cargo clippy --all-targets -- -D warnings` to verify zero warnings.
- **Unit Tests**: Add unit tests for any new algorithm step generators or parser functions.
- **WebAssembly Startup**: For changes to the web entry point, loader, build workflow, or hosting behavior, run the WebAssembly launch test documented above.
- **Synchronized State**: Keep timeline descriptions, inspector values, canvas state, final results, and highlighted source lines consistent at every step.
- **Plain Text Interface**: Do not add decorative emoji to UI labels, documentation, code comments, contributor templates, or release notes. A functional text symbol may represent an established control state, such as `★`/`☆` for favorites, when the control also provides a clear tooltip.
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

The latest published release series is actively supported with security updates.

### Vulnerability Reporting

If you discover a security defect or vulnerability, please report it privately:

1. **Do Not File a Public Issue**: Please do not open public issues for security bugs.
2. **Email**: Send vulnerability details privately to fender620@gmail.com.
3. **Response**: We acknowledge security reports within 48 hours and release verified fixes in patch updates.
