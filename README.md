# AlgoBuddy

[![Live Web Demo](https://img.shields.io/badge/Live%20Web%20Demo-Try%20in%20Browser-brightgreen.svg?style=for-the-badge&logo=webassembly)](https://rowrow620.github.io/AlgoBuddy)

[![CI & Quality Gates](https://github.com/Rowrow620/AlgoBuddy/actions/workflows/ci.yml/badge.svg)](https://github.com/Rowrow620/AlgoBuddy/actions/workflows/ci.yml)
[![CodeQL](https://github.com/Rowrow620/AlgoBuddy/actions/workflows/codeql.yml/badge.svg)](https://github.com/Rowrow620/AlgoBuddy/actions/workflows/codeql.yml)
[![Rust](https://img.shields.io/badge/Rust-2021-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![GUI: eframe/egui](https://img.shields.io/badge/GUI-eframe%2Fegui-blueviolet)](https://github.com/emilk/egui)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux%20%7C%20WebAssembly-lightgrey.svg)]()

AlgoBuddy is a work-in-progress, cross-platform algorithm visualizer built in Rust using `eframe` and `egui`. It follows the NeetCode 150 learning roadmap across 18 algorithmic categories on Windows, macOS, Linux, and the web through WebAssembly.

> [!WARNING]
> The roadmap catalog is available, but visualizer coverage and quality are still being audited. Some algorithms, approaches, inputs, and rendered states may be incomplete or inaccurate. Please treat AlgoBuddy as an evolving learning tool and report visual issues you encounter.

<img width="305" height="300" alt="ste" src="https://github.com/user-attachments/assets/a978e0c4-a7fa-4d5c-b6d4-ca172c07f8ff" /><br>

<img width="532" height="608" alt="steps" src="https://github.com/user-attachments/assets/b0fd306f-5024-4aa8-b4ba-7e8a9d80b25d" />



---

## Application Overview

![Full-Screen NeetCode 150 Mastery Dashboard](https://github.com/user-attachments/assets/2a7ebd79-3c5d-440f-9828-e32b60d93f50)

### Key Capabilities

- **18 Roadmap Categories**: Navigation is structured around the 150-problem NeetCode roadmap across Arrays & Hashing, Two Pointers, Sliding Window, Stack, Binary Search, Linked List, Trees, Tries, Backtracking, Heap / Priority Queue, 1D Dynamic Programming, Bit Manipulation, Math & Geometry, Greedy, Intervals, Graphs, 2D Dynamic Programming, and Advanced Graphs.
- **Roadmap Catalog (Work in Progress)**: The catalog includes all 150 roadmap entries, while individual visualizers continue to receive correctness, edge-case, and presentation audits.
- **Deterministic State Engine**: Models algorithm execution as discrete state snapshots, enabling forward and backward timeline scrubbing, variable speed playback (0.25x - 4.00x), and synchronized source line highlighting.
- **Solution Comparisons**: Some problems provide multiple approaches with their own trace, source listing, complexity analysis, invariant, and visual state. Comparison coverage is still expanding.
- **Specialized Visual Renderers**: Typed visual states drive dedicated renderers for arrays, hash-based collections, linked lists, trees and tries, graphs, and heaps.
- **Theme & Accessibility System**: Includes built-in dark/light themes alongside Protan/Deuteran Red-Green colorblind safe palettes.

---

## Quick Start

### WebAssembly (WASM)

Try AlgoBuddy directly in your browser without installation:

- **Live Demo**: [https://rowrow620.github.io/AlgoBuddy](https://rowrow620.github.io/AlgoBuddy)

### Build from Source

Requirements: a [stable Rust toolchain](https://www.rust-lang.org/tools/install) with `rustfmt` and Clippy. The crate uses the Rust 2021 edition.

```text
# Clone repository
git clone https://github.com/Rowrow620/AlgoBuddy.git
cd AlgoBuddy

# Launch AlgoBuddy
cargo run

# Execute the complete automated test suite
cargo test --all
```

For local WebAssembly development, also install Trunk and the Rust WASM target,
then run `trunk serve`:

```text
rustup target add wasm32-unknown-unknown
cargo install trunk
trunk serve
```

---

## Architecture Overview

AlgoBuddy uses a deterministic snapshot model where generator functions in `src/algorithms/` execute synchronously upfront to produce a timeline vector of discrete state snapshots (`Vec<Step>`). Execution is routed through `src/engine.rs` and the focused dispatch and input modules under `src/engine/`. The interactive GUI renders the active snapshot through component and canvas modules under `src/ui/`.

For architecture details, model definitions, and contribution workflows, see [CONTRIBUTING.md](CONTRIBUTING.md).

---

## Open Source & Community

We welcome community contributions! Please review our community guidelines:

- **Contributing Guide**: [Contributing](CONTRIBUTING.md)
- **Changelog**: [Changelog](CHANGELOG.md)
- **Release Process**: [Releasing AlgoBuddy](RELEASING.md)
- **Code of Conduct**: [Code of Conduct](CONTRIBUTING.md#code-of-conduct)
- **Security Policy**: [**Security Policy**](CONTRIBUTING.md#security-policy)

---

## License

Distributed under the MIT License. See [LICENSE](LICENSE) for details.
