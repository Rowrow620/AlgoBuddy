# AlgoBuddy

[![Live Web Demo](https://img.shields.io/badge/Live%20Web%20Demo-Try%20in%20Browser-brightgreen.svg?style=for-the-badge&logo=webassembly)](https://rowrow620.github.io/AlgoBuddy)

[![Rust](https://img.shields.io/badge/Rust-2021-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![GUI: eframe/egui](https://img.shields.io/badge/GUI-eframe%2Fegui-blueviolet)](https://github.com/emilk/egui)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux%20%7C%20WebAssembly-lightgrey.svg)]()

> **[Try AlgoBuddy Live in Your Browser (No Installation Required)](https://rowrow620.github.io/AlgoBuddy)**


AlgoBuddy is a high-performance cross-platform application built in Rust using `eframe` and `egui` that provides interactive, step-by-step algorithm visualizations formatted according to the NeetCode 150 learning roadmap. Available natively on Windows/macOS/Linux or live in your browser via WebAssembly (WASM).


---



## Key Features

- **NeetCode 150 Category Taxonomy**: Navigation structured into 18 algorithmic topic categories including Arrays & Hashing, Two Pointers, Sliding Window, Stack, Binary Search, Linked List, Trees, Tries, Backtracking, Heap / Priority Queue, 1D Dynamic Programming, Bit Manipulation, Math & Geometry, Greedy, Intervals, Graphs, 2D Dynamic Programming, and Advanced Graphs.

<img width="400" height="556" alt="{61B1612F-DA34-4086-B8D1-5DB5E34FF24E}" src="https://github.com/user-attachments/assets/d5232cf6-e385-4916-86a5-4fa41fc9b555" />

- **Progressive Problem Auditing**: Actively auditing problem logic and visual step renderers across the roadmap to ensure 100% mathematical and code-trace precision before public release.

<img width="401" height="219" alt="{1B0B035B-083F-40E3-9708-333A14AACC74}" src="https://github.com/user-attachments/assets/6d18aeed-d9fe-401c-8336-a92fe54854fa" />

- **Full-Screen NeetCode 150 Mastery Dashboard**: Interactive category progress breakdown with custom problem completion checkmarks, reset controls, and automatic cross-session state persistence.

<img width="1907" height="1018" alt="git_ex" src="https://github.com/user-attachments/assets/2a7ebd79-3c5d-440f-9828-e32b60d93f50" />

- **Interactive State Renderers**: Visualizes 2D DP memoization tables, graph vector topology canvas, grid flood fills, topological sorts, decision transitions, and step-by-step trace arrays.
<img width="455" height="262" alt="graph_example" src="https://github.com/user-attachments/assets/21e6540e-4fe5-4ba3-9913-7e1e002c7594" />

- **Theme & Colorblind Accessibility System**: Live switcher supporting **VS Code Midnight Dark**, **Cyber Navy**, **Clean Light**, and **Protan/Deuteran Red-Green Colorblind Safe** palettes.
<img width="380" height="371" alt="settings_colors" src="https://github.com/user-attachments/assets/959de73b-5e05-42ca-a7c2-004b6af991c8" />

- **Multi-Approach Evaluation Engine**: Compare multiple valid solutions per problem with live execution updates.

<img width="628" height="387" alt="multiple_ex_example" src="https://github.com/user-attachments/assets/267fe985-6f0b-446c-8f11-18ed140b5082" />
- **Deterministic State Engine**: Models algorithm steps as discrete state snapshots, enabling forward and backward timeline scrubbing, variable speed multipliers (0.25x - 4.00x), and synchronized source line highlighting.
<img width="610" height="66" alt="{85EEE829-7411-401E-B113-E7B097E4D2D0}" src="https://github.com/user-attachments/assets/cbe397fb-3b5d-478f-847e-09625ec63405" />

- **Integrated Problem Specifications**: View problem statements, examples with input/output cases, operational constraints, and direct links to official LeetCode problems within the application context.
<img width="587" height="754" alt="{AB95300A-3E54-4311-92FD-A1135F9804D3}" src="https://github.com/user-attachments/assets/c332be37-ec06-43c6-b683-0429ef9e45c4" />

---

## Core Features

- **Topic Navigation & Search**: Filter problems by topic category, difficulty level (Easy, Medium, Hard), or direct keyword search.

  ![NeetCode Roadmap Sidebar](assets/sidebar.png)

- **Visual Memory State Renderers**:
  - Interactive Array, 2D DP Memoization Table, Matrix Grid, 2D Vector Node Graph, Stack, Deque, HashSet, Trie Prefix Tree, Dual Heap Tree & Array, and Binary Tree renderers.
  - Two Pointers, Sliding Window, and Binary Search range & mid highlights.
- **Synchronized Source Trace & Live Scope Inspector**: Python solution implementation with active line highlighting tied to visual state transitions and live variable scope inspection.

  ![Code Trace and Live Scope Inspector Panel](assets/inspector.png)

---

## Public Release vs. Developer Mode

AlgoBuddy uses a strict **Audit Gating System** to ensure public users only see 100% verified, audited problem visualizers:

- **Public Release Mode** (Default): Shows fully audited and verified problems (currently **Contains Duplicate**, **Two Sum**, and **Valid Anagram**).
- **Developer / Testing Mode**: Unlocks all 134 problem visualizers across 18 categories for development, testing, and contribution.

  ![AlgoBuddy Settings & Accessibility Modal](assets/settings.png)

  ![Developer & Release Mode Toggle](assets/dev_mode.png)

---

## Installation & Execution

### Build from Source

Clone the repository and run via Cargo:

```powershell
git clone https://github.com/Rowrow620/AlgoBuddy.git
cd AlgoBuddy

# Launch Public Release Mode (Audited Problems)
cargo run

# Launch Developer / Testing Mode (All 134 Problems Unlocked)
cargo run -- --dev

# Run Automated Test Suite
cargo test
```

### WebAssembly (WASM) Deployment

To view Developer Mode in a browser deployment, append `?dev=true` to the URL (e.g., `https://rowrow620.github.io/AlgoBuddy/?dev=true`). You can also toggle Developer Mode anytime inside the application **Settings** modal.

---

## License

Distributed under the MIT License. See [LICENSE](LICENSE) for details.
