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

---

## Core Features

- **Topic Navigation & Search**: Filter problems by topic category, difficulty level (Easy, Medium, Hard), or direct keyword search.
- **Visual Memory State Renderers**:
  - Array and Dual-Pointer Renderers (converging pointers, character comparison indicators).
  - Vertical Stack Renderers (push/pop operations, bracket matching verification).
  - Sliding Window Stock Renderers (buy/sell day pointer cards, running profit metrics).
  - Binary Search Renderers (sorted array bounds, midpoint marker, target match indicators).
  - Linked List Pointer Renderers (pointer chain, next pointer flipping, reversed list construction).
  - Hash Map & Frequency Grid Renderers (key-value mapping, character count matrices).
  - Prefix & Suffix Array Product Renderers.
  - String Protocol Renderers (`length#string` encoding/decoding traces).
- **Synchronized Source Trace**: Python solution implementation with active line highlighting tied to visual state transitions.

---

## Supported Problems

| Problem ID | Problem Name | Category | Supported Approaches |
| :--- | :--- | :--- | :--- |
| **#1** | Two Sum | Arrays & Hashing | Hash Map O(N), Brute Force O(N²) |
| **#242** | Valid Anagram | Arrays & Hashing | Frequency Counter Array O(N), Sort Strings O(N log N) |
| **#347** | Top K Frequent Elements | Arrays & Hashing | Bucket Sort O(N), Min-Heap O(N log k), Sorting Pairs O(N log N) |
| **#238** | Product of Array Except Self | Arrays & Hashing | Prefix & Suffix Pass O(N) |
| **#271** | Encode and Decode Strings | Arrays & Hashing | Length Prefix (# Protocol) O(N) |
| **#125** | Valid Palindrome | Two Pointers | Two Pointers In-Place O(N), Reverse Filtered String O(N) |
| **#121** | Best Time to Buy and Sell Stock | Sliding Window | Two Pointers / Sliding Window O(N) |
| **#20** | Valid Parentheses | Stack | Stack Matching O(N) |
| **#704** | Binary Search | Binary Search | Binary Search Iterative O(log N) |
| **#206** | Reverse Linked List | Linked List | Iterative Pointers (prev, curr, nxt) O(N) |

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
        ├── two_sum.rs                  # Two Sum step generators
        ├── valid_anagram.rs            # Valid Anagram step generators
        ├── bucket_sort.rs              # Bucket Sort step generator
        ├── min_heap.rs                 # Min-Heap step generator
        ├── sorting.rs                  # Sorting step generator
        ├── product_except_self.rs      # Product Except Self step generator
        ├── encode_decode.rs            # Encode/Decode Strings step generator
        ├── valid_palindrome.rs         # Valid Palindrome step generators
        ├── best_time_stock.rs          # Best Time to Buy/Sell Stock step generator
        ├── valid_parentheses.rs        # Valid Parentheses step generator
        ├── binary_search.rs            # Binary Search step generator
        └── reverse_linked_list.rs      # Reverse Linked List step generator
```

---

## License

Distributed under the MIT License. See [LICENSE](LICENSE) for details.
