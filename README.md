# AlgoBuddy 🦀

A high-performance native desktop application written in Rust using `eframe` / `egui` to interactively visualize LeetCode algorithm solutions step by step, organized by the **NeetCode 150 Roadmap**.

---

## 🌟 Key Features

* **NeetCode 150 Roadmap Navigation**:
  * Organized into all **18 topic categories** (Arrays & Hashing, Two Pointers, Stack, Sliding Window, Binary Search, Linked List, Trees, Tries, Backtracking, Heap, Graphs, 1D/2D DP, Bit Manipulation, Math & Geometry).
  * Real-time search bar to filter problems by name or number.
* **Difficulty Filter**:
  * Filter problems by **All**, **Easy** (Green), **Medium** (Amber), and **Hard** (Red).
* **Multi-Approach Algorithm Support**:
  * Compare multiple valid approaches per problem (e.g. **Hash Map $O(N)$** vs **Brute Force $O(N^2)$**, or **Bucket Sort $O(N)$** vs **Min-Heap $O(N \log k)$**).
  * Dynamically updates time & space complexity badges, code line highlighting, and animation cards.
* **Integrated Problem Statement Reference Panel**:
  * Full problem narrative, example input/output cards, constraints list, and direct **"Open on LeetCode.com ↗"** button.
* **Full Playback Engine**:
  * **Play / Pause** auto-stepping with delay slider (100ms - 1500ms).
  * **Step Previous / Step Next / Reset** buttons + interactive timeline scrubbing.

---

## 🚀 How to Run

### Option 1: Cargo Command (Recommended)
Open a terminal in the project directory and run:

```powershell
cargo run
```

### Option 2: Pre-compiled Executable
Directly run the compiled native binary:

```powershell
.\target\debug\algobuddy.exe
```

---

## 📂 Project Structure

```text
algobuddy/
├── Cargo.toml                          # Project manifest with eframe/egui & open dependencies
├── README.md                           # Project documentation
├── LICENSE                             # MIT License
├── .gitignore                          # Git exclude rules
└── src/
    ├── main.rs                         # Native desktop window launcher
    ├── app.rs                          # GUI layout, roadmap sidebar, canvas renderers
    ├── model.rs                        # Problem registry metadata, approach models, visual states
    └── algorithms/                     # Step Generators
        ├── mod.rs
        ├── two_sum.rs                  # #1 Two Sum (Hash Map O(N) vs Brute Force O(N^2))
        ├── valid_anagram.rs            # #242 Valid Anagram (Counter O(N) vs Sorting O(N log N))
        ├── bucket_sort.rs              # #347 Top K Frequent: Bucket Sort O(N)
        ├── min_heap.rs                 # #347 Top K Frequent: Min-Heap O(N log k)
        ├── sorting.rs                  # #347 Top K Frequent: Sorting O(N log N)
        ├── product_except_self.rs      # #238 Product of Array Except Self
        ├── encode_decode.rs            # #271 Encode and Decode Strings
        ├── valid_palindrome.rs         # #125 Valid Palindrome (Two Pointers O(1) vs Reverse O(N))
        └── valid_parentheses.rs        # #20 Valid Parentheses (Stack O(N))
```

---

## 🛠️ Built With

* **Rust** — Memory safety & performance
* **eframe / egui** — GPU-accelerated immediate-mode GUI framework
* **open** — Native web browser opening
