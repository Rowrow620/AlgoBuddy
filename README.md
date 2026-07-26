# AlgoBuddy

[![Live Web Demo](https://img.shields.io/badge/Live%20Web%20Demo-Try%20in%20Browser-brightgreen.svg?style=for-the-badge&logo=webassembly)](https://rowrow620.github.io/AlgoBuddy)

[![Rust](https://img.shields.io/badge/Rust-2021-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![GUI: eframe/egui](https://img.shields.io/badge/GUI-eframe%2Fegui-blueviolet)](https://github.com/emilk/egui)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux%20%7C%20WebAssembly-lightgrey.svg)]()

> **[Try AlgoBuddy Live in Your Browser (No Installation Required)](https://rowrow620.github.io/AlgoBuddy)**


AlgoBuddy is a high-performance cross-platform application built in Rust using `eframe` and `egui` that provides interactive, step-by-step algorithm visualizations formatted according to the NeetCode 150 learning roadmap. Available natively on Windows/macOS/Linux or live in your browser via WebAssembly (WASM).


---



## Architectural Highlights

- **NeetCode 150 Category Taxonomy**: Navigation structured into 18 algorithmic topic categories including Arrays & Hashing, Two Pointers, Sliding Window, Stack, Binary Search, Linked List, Trees, Tries, Backtracking, Heap / Priority Queue, 1D Dynamic Programming, Bit Manipulation, Math & Geometry, Greedy, Intervals, Graphs, 2D Dynamic Programming, and Advanced Graphs.
- **Completion Milestones**:
  - **Progress**: 150 / 150 Problems Implemented (100.0% Complete Roadmap!)
  - **100% Completed Categories (18/18)**: Arrays & Hashing (9/9), Two Pointers (5/5), Stack (7/7), Sliding Window (6/6), Binary Search (7/7), Linked List (11/11), Trees (14/14), Tries (3/3), Heap / Priority Queue (7/7), Backtracking (9/9), 1D Dynamic Programming (12/12), Bit Manipulation (7/7), Math & Geometry (8/8), Greedy (8/8), Intervals (6/6), Graphs (13/13), 2D Dynamic Programming (11/11), Advanced Graphs (6/6)
  - **All Easy Problems (28/28)**: 100% Complete
- **Full-Screen NeetCode 150 Mastery Dashboard**: Interactive category progress breakdown with custom problem completion checkmarks, reset controls, and automatic cross-session state persistence.
- **Interactive State Renderers**: Visualizes 2D DP memoization tables, graph vector topology canvas, grid flood fills, topological sorts, decision transitions, and step-by-step trace arrays.
- **Theme & Colorblind Accessibility System**: Live switcher supporting **VS Code Midnight Dark**, **Cyber Navy**, **Clean Light**, and **Protan/Deuteran Red-Green Colorblind Safe** palettes.
- **Multi-Approach Evaluation Engine**: Compare multiple valid solutions per problem with live execution updates.
- **Deterministic State Engine**: Models algorithm steps as discrete state snapshots, enabling forward and backward timeline scrubbing, variable speed multipliers (0.25x - 4.00x), and synchronized source line highlighting.
- **Integrated Problem Specifications**: View problem statements, examples with input/output cases, operational constraints, and direct links to official LeetCode problems within the application context.

---

## Core Features

- **Topic Navigation & Search**: Filter problems by topic category, difficulty level (Easy, Medium, Hard), or direct keyword search.

  ![AlgoBuddy Demo](assets/demo.gif)

- **Visual Memory State Renderers**:
  - Interactive Array, 2D DP Memoization Table, Matrix Grid, 2D Vector Node Graph, Stack, Deque, HashSet, Trie Prefix Tree, Dual Heap Tree & Array, and Binary Tree renderers.
  - Two Pointers, Sliding Window, and Binary Search range & mid highlights.
- **Synchronized Source Trace**: Python solution implementation with active line highlighting tied to visual state transitions.

---

## Installation & Execution

### Download Binary

Download the pre-compiled executable `AlgoBuddy-v0.5.0-Beta.exe` from the latest GitHub release.

### Build from Source

Clone the repository and build using Cargo:

```powershell
git clone https://github.com/Rowrow620/AlgoBuddy.git
cd AlgoBuddy
cargo run
```

---

## License

Distributed under the MIT License. See [LICENSE](LICENSE) for details.
