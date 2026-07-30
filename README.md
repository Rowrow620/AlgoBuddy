# AlgoBuddy

[![Live Web Demo](https://img.shields.io/badge/Live%20Web%20Demo-Try%20in%20Browser-brightgreen.svg?style=for-the-badge&logo=webassembly)](https://rowrow620.github.io/AlgoBuddy)

[![CI & Quality Gates](https://github.com/Rowrow620/AlgoBuddy/actions/workflows/ci.yml/badge.svg)](https://github.com/Rowrow620/AlgoBuddy/actions/workflows/ci.yml)
[![CodeQL](https://github.com/Rowrow620/AlgoBuddy/actions/workflows/codeql.yml/badge.svg)](https://github.com/Rowrow620/AlgoBuddy/actions/workflows/codeql.yml)
[![Rust](https://img.shields.io/badge/Rust-2021-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![GUI: eframe/egui](https://img.shields.io/badge/GUI-eframe%2Fegui-blueviolet)](https://github.com/emilk/egui)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux%20%7C%20WebAssembly-lightgrey.svg)]()

AlgoBuddy is a high-performance, cross-platform algorithm visualizer built in Rust using `eframe` and `egui`. It provides step-by-step interactive visualizations formatted according to the NeetCode 150 learning roadmap across 18 algorithmic categories. Available natively on Windows, macOS, and Linux, or live in your browser via WebAssembly.

---

## Application Overview

![Full-Screen NeetCode 150 Mastery Dashboard](https://github.com/user-attachments/assets/2a7ebd79-3c5d-440f-9828-e32b60d93f50)

### Key Capabilities

- **18 NeetCode Topic Categories**: Covers Arrays & Hashing, Two Pointers, Sliding Window, Stack, Binary Search, Linked List, Trees, Tries, Backtracking, Heap / Priority Queue, 1D Dynamic Programming, Bit Manipulation, Math & Geometry, Greedy, Intervals, Graphs, 2D Dynamic Programming, and Advanced Graphs.
- **Deterministic State Engine**: Models algorithm execution as discrete state snapshots, enabling forward and backward timeline scrubbing, variable speed playback (0.25x - 4.00x), and synchronized source line highlighting.
- **Interactive Visual Renderers**: Features custom 2D DP memoization tables, matrix grid flood-fills, graph vector topology canvases, dual heap trees, and array trace renderers.
- **Theme & Accessibility System**: Includes built-in dark/light themes alongside Protan/Deuteran Red-Green colorblind safe palettes.
- **Audit Gating System**: Public Release Mode presents audited problem visualizers, while Developer Mode unlocks all 135 implemented problem visualizers for testing and contribution.

---

## Quick Start

### WebAssembly (WASM)

Try AlgoBuddy directly in your browser without installation:

- **Public Release Demo**: [https://rowrow620.github.io/AlgoBuddy](https://rowrow620.github.io/AlgoBuddy)
- **Developer Demo (All 135 Problems)**: [https://rowrow620.github.io/AlgoBuddy/?dev=true](https://rowrow620.github.io/AlgoBuddy/?dev=true)

### Build from Source

Requirements: [Rust 2021 Edition](https://www.rust-lang.org/)

```powershell
# Clone repository
git clone https://github.com/Rowrow620/AlgoBuddy.git
cd AlgoBuddy

# Launch Public Release Mode
cargo run

# Launch Developer Mode (All 135 Problems Unlocked)
cargo run -- --dev

# Execute Automated Test Suite
cargo test
```

---

## Architecture Overview

AlgoBuddy uses a deterministic snapshot model where generator functions in `src/algorithms/` execute synchronously upfront to produce a timeline vector of discrete state snapshots (`Vec<Step>`). The GUI renders snapshots based on the active timeline scrubber index.

For detailed architecture diagrams, model definitions, and problem auditing workflows, see [CONTRIBUTING.md](CONTRIBUTING.md).

---

## Open Source & Community

We welcome community contributions! Please review our community guidelines:

- **Contributing Guide**: [CONTRIBUTING.md](CONTRIBUTING.md)
- **Changelog**: [CHANGELOG.md](CHANGELOG.md)
- **Code of Conduct**: [.github/CODE_OF_CONDUCT.md](.github/CODE_OF_CONDUCT.md)
- **Security Policy**: [.github/SECURITY.md](.github/SECURITY.md)

---

## License

Distributed under the MIT License. See [LICENSE](LICENSE) for details.
