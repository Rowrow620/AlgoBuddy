# AlgoBuddy

[![Rust](https://img.shields.io/badge/Rust-2021-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![GUI: eframe/egui](https://img.shields.io/badge/GUI-eframe%2Fegui-blueviolet)](https://github.com/emilk/egui)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg)]()

AlgoBuddy is a native desktop application built in Rust using `eframe` and `egui` that provides interactive, step-by-step algorithm visualizations formatted according to the NeetCode 150 learning roadmap.

---

## Architectural Highlights

- **NeetCode 150 Category Taxonomy**: Navigation structured into 18 algorithmic topic categories including Arrays & Hashing, Two Pointers, Sliding Window, Stack, Binary Search, Linked List, Trees, Tries, Backtracking, Heap / Priority Queue, Graphs, Dynamic Programming, Bit Manipulation, and Math & Geometry.
- **Completion Milestones**:
  - **Arrays & Hashing Category (9/9)**: 100% Complete
  - **All Easy Problems Across NeetCode 150 (27/27)**: 100% Complete
- **Theme & Colorblind Accessibility System**: Live switcher supporting **VS Code Midnight Dark**, **Cyber Navy**, **Clean Light**, and **Protan/Deuteran Red-Green Colorblind Safe** palettes.
- **Multi-Approach Evaluation Engine**: Compare multiple valid solutions per problem with live execution updates.
- **Deterministic State Engine**: Models algorithm steps as discrete state snapshots, enabling forward and backward timeline scrubbing, variable auto-stepping delay (100ms - 1500ms), and synchronized source line highlighting.
- **Integrated Problem Specifications**: View problem statements, examples with input/output cases, operational constraints, and direct links to official LeetCode problems within the application context.

---

## Core Features

- **Topic Navigation & Search**: Filter problems by topic category, difficulty level (Easy, Medium, Hard), or direct keyword search.
- **Visual Memory State Renderers**:
  - Group Anagrams Signature Bucketed Map (tuple/sorted key transformers, category cards).
  - Contains Duplicate HashSet Scan & Duplicate Collision Renderer.
  - Longest Consecutive Sequence Set & Streak Expansion Visualizer.
  - 9x9 Sudoku Board Validation Grid (3x3 sub-box boundaries, row/col highlights).
  - Binary Tree Node Graph Renderers (level-order hierarchy, balance checks, depth and diameter metrics).
  - Single and Dual Linked List Renderers (pointer chain, next pointer flipping, slow/fast cycle detection, sorted list merging).
  - Bitwise Manipulation Renderers (XOR cancellation, Brian Kernighan set bit clear, bit shift reverse).
  - Dynamic Programming Stairs Renderers (Fibonacci stepping, min cost pass).
- **Synchronized Source Trace**: Python solution implementation with active line highlighting tied to visual state transitions.

---

## Supported Problems (34 Problems Total)

| Problem ID | Problem Name | Category | Difficulty |
| :--- | :--- | :--- | :--- |
| **#217** | Contains Duplicate | Arrays & Hashing | Easy (100%) |
| **#1** | Two Sum | Arrays & Hashing | Easy (100%) |
| **#242** | Valid Anagram | Arrays & Hashing | Easy (100%) |
| **#49** | Group Anagrams | Arrays & Hashing | Medium |
| **#347** | Top K Frequent Elements | Arrays & Hashing | Medium |
| **#238** | Product of Array Except Self | Arrays & Hashing | Medium |
| **#271** | Encode and Decode Strings | Arrays & Hashing | Medium |
| **#36** | Valid Sudoku | Arrays & Hashing | Medium |
| **#128** | Longest Consecutive Sequence | Arrays & Hashing | Medium |
| **#125** | Valid Palindrome | Two Pointers | Easy (100%) |
| **#121** | Best Time to Buy and Sell Stock | Sliding Window | Easy (100%) |
| **#20** | Valid Parentheses | Stack | Easy (100%) |
| **#704** | Binary Search | Binary Search | Easy (100%) |
| **#206** | Reverse Linked List | Linked List | Easy (100%) |
| **#21** | Merge Two Sorted Linked Lists | Linked List | Easy (100%) |
| **#141** | Linked List Cycle Detection | Linked List | Easy (100%) |
| **#226** | Invert Binary Tree | Trees | Easy (100%) |
| **#104** | Maximum Depth of Binary Tree | Trees | Easy (100%) |
| **#543** | Diameter of Binary Tree | Trees | Easy (100%) |
| **#110** | Balanced Binary Tree | Trees | Easy (100%) |
| **#100** | Same Tree | Trees | Easy (100%) |
| **#572** | Subtree of Another Tree | Trees | Easy (100%) |
| **#70** | Climbing Stairs | 1-D DP | Easy (100%) |
| **#746** | Min Cost Climbing Stairs | 1-D DP | Easy (100%) |
| **#703** | Kth Largest Element in a Stream | Heap / Priority Queue | Easy (100%) |
| **#1046** | Last Stone Weight | Heap / Priority Queue | Easy (100%) |
| **#252** | Meeting Rooms | Intervals | Easy (100%) |
| **#202** | Happy Number | Math & Geometry | Easy (100%) |
| **#66** | Plus One | Math & Geometry | Easy (100%) |
| **#136** | Single Number | Bit Manipulation | Easy (100%) |
| **#191** | Number of 1 Bits | Bit Manipulation | Easy (100%) |
| **#338** | Counting Bits | Bit Manipulation | Easy (100%) |
| **#190** | Reverse Bits | Bit Manipulation | Easy (100%) |
| **#268** | Missing Number | Bit Manipulation | Easy (100%) |

---

## Installation & Execution

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
