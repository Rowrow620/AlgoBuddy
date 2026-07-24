# AlgoBuddy

[![Rust](https://img.shields.io/badge/Rust-2021-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![GUI: eframe/egui](https://img.shields.io/badge/GUI-eframe%2Fegui-blueviolet)](https://github.com/emilk/egui)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg)]()

AlgoBuddy is a native desktop application built in Rust using `eframe` and `egui` that provides interactive, step-by-step algorithm visualizations formatted according to the NeetCode 150 learning roadmap.

---

## Architectural Highlights

- **NeetCode 150 Category Taxonomy**: Navigation structured into 18 algorithmic topic categories including Arrays & Hashing, Two Pointers, Sliding Window, Stack, Binary Search, Linked List, Trees, Tries, Backtracking, Heap / Priority Queue, Graphs, Dynamic Programming, Bit Manipulation, and Math & Geometry.
- **Multi-Approach Evaluation Engine**: Compare multiple valid solutions per problem (e.g., Hash Map vs. Brute Force or Bucket Sort vs. Min-Heap vs. Array Sorting) with live execution updates.
- **Deterministic State Engine**: Models algorithm steps as discrete state snapshots, enabling forward and backward timeline scrubbing, variable auto-stepping delay (100ms - 1500ms), and synchronized source line highlighting.
- **Integrated Problem Specifications**: View problem statements, examples with input/output cases, operational constraints, and direct links to official LeetCode problems within the application context.
- **Collapsible UI Panels**: Toggle left roadmap sidebar and right code/details panels to maximize canvas space.

---

## Core Features

- **Topic Navigation & Search**: Filter problems by topic category, difficulty level (Easy, Medium, Hard), or direct keyword search.
- **Visual Memory State Renderers**:
  - Group Anagrams Signature Bucketed Map (tuple/sorted key transformers, category cards).
  - Contains Duplicate HashSet Scan & Duplicate Collision Renderer.
  - Longest Consecutive Sequence Set & Streak Expansion Visualizer (HashSet tracking, streak chain cards, running max length metrics).
  - 9x9 Sudoku Board Validation Grid (3x3 sub-box boundaries, row/col highlights, red duplicate alert detection).
  - Binary Tree Node Graph Renderers (level-order hierarchy, active node highlight, depth and diameter metrics).
  - Single and Dual Linked List Renderers (pointer chain, next pointer flipping, slow/fast cycle detection, sorted list merging).
  - Sliding Window Stock Renderers (buy/sell day pointer cards, running profit metrics).
  - Binary Search Renderers (sorted array bounds, midpoint marker, target match indicators).
  - Array and Dual-Pointer Renderers (converging pointers, character comparison indicators).
  - Vertical Stack Renderers (push/pop operations, bracket matching verification).
  - Hash Map & Frequency Grid Renderers (key-value mapping, character count matrices).
  - Prefix & Suffix Array Product Renderers.
  - String Protocol Renderers (`length#string` encoding/decoding traces).
- **Synchronized Source Trace**: Python solution implementation with active line highlighting tied to visual state transitions.

---

## Supported Problems (19 Problems across 7 Roadmap Categories)

> **Category Milestone**: 🎉 **Arrays & Hashing Category (9/9) is 100% COMPLETE!**

| Problem ID | Problem Name | Category | Supported Approaches |
| :--- | :--- | :--- | :--- |
| **#217** | Contains Duplicate | Arrays & Hashing (100%) | HashSet Lookup O(N), Sorting Array O(N log N) |
| **#1** | Two Sum | Arrays & Hashing (100%) | Hash Map O(N), Brute Force O(N²) |
| **#242** | Valid Anagram | Arrays & Hashing (100%) | Frequency Counter Array O(N), Sort Strings O(N log N) |
| **#49** | Group Anagrams | Arrays & Hashing (100%) | Char Frequency Tuple Map O(N * K), Sorted String Key O(N * K log K) |
| **#347** | Top K Frequent Elements | Arrays & Hashing (100%) | Bucket Sort O(N), Min-Heap O(N log k), Sorting Pairs O(N log N) |
| **#238** | Product of Array Except Self | Arrays & Hashing (100%) | Prefix & Suffix Pass O(N) |
| **#271** | Encode and Decode Strings | Arrays & Hashing (100%) | Length Prefix (# Protocol) O(N) |
| **#36** | Valid Sudoku | Arrays & Hashing (100%) | HashSet Validation (Rows, Cols, 3x3 Boxes) O(1) |
| **#128** | Longest Consecutive Sequence | Arrays & Hashing (100%) | HashSet Sequence Start Expansion O(N) |
| **#125** | Valid Palindrome | Two Pointers | Two Pointers In-Place O(N), Reverse Filtered String O(N) |
| **#121** | Best Time to Buy and Sell Stock | Sliding Window | Two Pointers / Sliding Window O(N) |
| **#20** | Valid Parentheses | Stack | Stack Matching O(N) |
| **#704** | Binary Search | Binary Search | Binary Search Iterative O(log N) |
| **#206** | Reverse Linked List | Linked List | Iterative Pointers (prev, curr, nxt) O(N) |
| **#21** | Merge Two Sorted Linked Lists | Linked List | Two Pointers Merge O(N + M) |
| **#141** | Linked List Cycle Detection | Linked List | Floyd's Tortoise & Hare Slow/Fast Pointers O(N) |
| **#226** | Invert Binary Tree | Trees | Recursive DFS Subtree Swap O(N) |
| **#104** | Maximum Depth of Binary Tree | Trees | Recursive DFS Height Calculation O(N) |
| **#543** | Diameter of Binary Tree | Trees | Post-order DFS Path Diameter O(N) |

---

## System Requirements

- Rust Toolchain (`rustc 1.97` or newer with Cargo)

---

## Installation & Execution

### Build from Source

Clone the repository and build using Cargo:

```powershell
git clone https://github.com/Rowrow620/AlgoBuddy.git
cd AlgoBuddy
cargo run
```

### Pre-compiled Binary

Run the compiled executable directly from the target directory:

```powershell
.\target\debug\algobuddy.exe
```

---

## Directory Structure

```text
AlgoBuddy/
├── Cargo.toml                          # Project manifest and dependencies
├── README.md                           # Documentation
├── LICENSE                             # MIT License
├── .gitignore                          # Version control exclusions
└── src/
    ├── main.rs                         # Native window entry point
    ├── app.rs                          # Application layout, navigation, and visual renderers
    ├── model.rs                        # Domain models, visual states, and problem specifications
    └── algorithms/                     # Step generator modules
        ├── mod.rs
        ├── contains_duplicate.rs       # Contains Duplicate step generator
        ├── two_sum.rs                  # Two Sum step generators
        ├── valid_anagram.rs            # Valid Anagram step generators
        ├── group_anagrams.rs           # Group Anagrams step generator
        ├── bucket_sort.rs              # Bucket Sort step generator
        ├── min_heap.rs                 # Min-Heap step generator
        ├── sorting.rs                  # Sorting step generator
        ├── product_except_self.rs      # Product Except Self step generator
        ├── encode_decode.rs            # Encode/Decode Strings step generator
        ├── valid_sudoku.rs             # Valid Sudoku step generator
        ├── longest_consecutive.rs      # Longest Consecutive Sequence step generator
        ├── valid_palindrome.rs         # Valid Palindrome step generators
        ├── best_time_stock.rs          # Best Time to Buy/Sell Stock step generator
        ├── valid_parentheses.rs        # Valid Parentheses step generator
        ├── binary_search.rs            # Binary Search step generator
        ├── reverse_linked_list.rs      # Reverse Linked List step generator
        ├── merge_two_lists.rs          # Merge Two Lists step generator
        ├── linked_list_cycle.rs        # Linked List Cycle step generator
        ├── invert_tree.rs              # Invert Binary Tree step generator
        ├── max_depth_tree.rs           # Max Depth Binary Tree step generator
        └── diameter_tree.rs            # Diameter of Binary Tree step generator
```

---

## License

Distributed under the MIT License. See [LICENSE](LICENSE) for details.
