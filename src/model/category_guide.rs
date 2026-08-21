use crate::model::taxonomy::Category;

pub struct CategoryGuideData {
    #[allow(dead_code)]
    pub category: Category,
    pub summary: &'static str,
    pub how_it_works: &'static str,
    pub key_patterns: &'static [&'static str],
    pub complexity_table: &'static [(&'static str, &'static str, &'static str)],
    pub pro_tips: &'static [&'static str],
}

pub fn get_category_guide(cat: Category) -> CategoryGuideData {
    match cat {
        Category::ArraysAndHashing => CategoryGuideData {
            category: cat,
            summary: "Arrays store elements sequentially in contiguous memory. Hash Tables map keys to values using hash functions for instant O(1) average lookup.",
            how_it_works: "Arrays provide O(1) random access via index offset arithmetic. Hash Maps map keys to array buckets via hash(key) % capacity, handling collisions via chaining or open addressing.",
            key_patterns: &[
                "Frequency Counting: Store element frequencies in a HashMap/Array to check anagrams or top K items.",
                "Index Mapping: Store value -> index mapping for O(1) complement lookup (Two Sum).",
                "Prefix Sums: Precompute cumulative sums to answer subarray range sum queries in O(1) time.",
            ],
            complexity_table: &[
                ("Array Random Access", "O(1)", "O(1)"),
                ("Array Search (Unsorted)", "O(n)", "O(1)"),
                ("Hash Map Insert / Search", "O(1) avg, O(n) worst", "O(n)"),
                ("Hash Set Membership", "O(1) avg", "O(n)"),
            ],
            pro_tips: &[
                "When space allows, trading O(n) memory for a Hash Set or HashMap often drops time complexity from O(n^2) to O(n).",
                "Watch out for integer key hashing and bucket collision degradation in worst-case scenarios.",
            ],
        },
        Category::TwoPointers => CategoryGuideData {
            category: cat,
            summary: "Two Pointers uses two integer index markers iterating through a linear sequence toward each other or in parallel to reduce nested O(n^2) loops to O(n).",
            how_it_works: "By taking advantage of sequence ordering (such as a sorted array), pointers shrink the search space monotonically at each step based on current comparison results.",
            key_patterns: &[
                "Converging Pointers (Left & Right): Start at opposite ends and move inward based on sum/predicate (Valid Palindrome, Two Sum II, Container Water).",
                "Fast & Slow Pointers: Move at different speeds to detect cycles or find middle nodes.",
                "In-place Partitioning: Track boundary marker while iterating to swap elements in-place.",
            ],
            complexity_table: &[
                ("Two Pointers Convergence", "O(n)", "O(1)"),
                ("Sorted Two Sum Search", "O(n)", "O(1)"),
                ("3Sum (Sort + Two Pointers)", "O(n^2)", "O(1) or O(n)"),
            ],
            pro_tips: &[
                "Two Pointers on arrays almost always requires sorting the array first unless searching inward from boundaries.",
                "Ensure pointer bounds check (left < right) is maintained to avoid index out-of-bounds.",
            ],
        },
        Category::Stack => CategoryGuideData {
            category: cat,
            summary: "Stacks enforce Last-In, First-Out (LIFO) access. Monotonic Stacks maintain ordered elements to find next greater or smaller elements efficiently.",
            how_it_works: "Elements are pushed onto the top of the stack and popped from the top. Monotonic stacks pop elements that violate monotonicity before pushing the new element.",
            key_patterns: &[
                "Matching Parentheses / Nested Scope: Push opening tokens, pop and match when encountering closing tokens.",
                "Monotonic Stack: Maintain strictly increasing or decreasing elements to solve Next Greater Element in O(n).",
                "Expression Evaluation: Process postfix (RPN) or infix expressions using operator precedence.",
            ],
            complexity_table: &[
                ("Push / Pop / Top", "O(1)", "O(n)"),
                ("Monotonic Stack Pass", "O(n)", "O(n)"),
                ("Evaluate RPN", "O(n)", "O(n)"),
            ],
            pro_tips: &[
                "Each element is pushed and popped at most once in a Monotonic Stack pass, ensuring amortized O(1) work per item.",
            ],
        },
        Category::BinarySearch => CategoryGuideData {
            category: cat,
            summary: "Binary Search repeatedly divides a sorted search space in half, achieving logarithmic O(log n) time complexity.",
            how_it_works: "Calculate mid = left + (right - left) / 2. Compare target with array[mid] and discard half of the search range at every step.",
            key_patterns: &[
                "Standard Binary Search: Find exact target in a strictly sorted array.",
                "Rotated Sorted Array: Identify which half is sorted first, then decide target range.",
                "Binary Search on Answer: Search over a discrete range of feasible answers (e.g. Koko Eating Bananas).",
            ],
            complexity_table: &[
                ("Binary Search", "O(log n)", "O(1)"),
                ("Search Rotated Array", "O(log n)", "O(1)"),
                ("Search 2D Matrix", "O(log(m * n))", "O(1)"),
            ],
            pro_tips: &[
                "Use `left + (right - left) / 2` instead of `(left + right) / 2` to prevent integer overflow in large arrays.",
            ],
        },
        Category::SlidingWindow => CategoryGuideData {
            category: cat,
            summary: "Sliding Window maintains a dynamic contiguous subarray or substring boundary [L, R] to process sequential range queries in O(n).",
            how_it_works: "Expand right boundary R to include new elements; when window constraint is violated, shrink left boundary L until valid again.",
            key_patterns: &[
                "Fixed Window Size: Maintain a window of length K and slide it across the array.",
                "Variable Window (Max Length): Expand R, record max valid window size before shrinking L.",
                "Variable Window (Min Length): Expand R until target condition is met, then contract L to minimize window size (Min Window Substring).",
            ],
            complexity_table: &[
                ("Fixed Size Window Pass", "O(n)", "O(1)"),
                ("Variable Window Pass", "O(n)", "O(k)"),
                ("Sliding Window Maximum (Deque)", "O(n)", "O(k)"),
            ],
            pro_tips: &[
                "Even though there is a nested while loop inside the outer loop, both L and R pointers only move forward, giving total time O(n).",
            ],
        },
        Category::LinkedList => CategoryGuideData {
            category: cat,
            summary: "Linked Lists consist of nodes containing data and pointer references (next / prev) allocated non-contiguously in memory.",
            how_it_works: "Operations manipulate node pointers rather than shifting elements. Floyd's Tortoise and Hare uses fast and slow pointers to detect cycles in O(1) space.",
            key_patterns: &[
                "Dummy Head Node: Simplifies edge cases when inserting/deleting list heads.",
                "Two-Pointer Cycle Detection: Fast pointer moves 2 steps, slow moves 1 step.",
                "In-Place Reversal: Rebind node pointers (curr.next = prev) using iterative or recursive traversal.",
            ],
            complexity_table: &[
                ("Prepend / Append (with tail pointer)", "O(1)", "O(1)"),
                ("Search / Access by Index", "O(n)", "O(1)"),
                ("Reverse List", "O(n)", "O(1)"),
                ("Cycle Detection (Floyd)", "O(n)", "O(1)"),
            ],
            pro_tips: &[
                "Always store next_node = curr.next before modifying curr.next pointer during reversals to avoid losing the remaining list.",
            ],
        },
        Category::Trees => CategoryGuideData {
            category: cat,
            summary: "Trees are hierarchical non-linear graphs with a root node and subtrees. Binary Search Trees (BSTs) keep left subtree values smaller and right subtree values larger.",
            how_it_works: "Traversals explore nodes recursively or iteratively. Preorder (N-L-R), Inorder (L-N-R, sorted in BST), Postorder (L-R-N), and Level-Order (BFS queue).",
            key_patterns: &[
                "DFS Recursion / Postorder Gain: Return subtree metrics (height, path gain) up the tree (Diameter, Max Path Sum).",
                "BST Inorder Traversal: Produces elements in strictly sorted order.",
                "BFS Level-Order: Use double-ended queue to visit nodes layer by layer.",
            ],
            complexity_table: &[
                ("BST Search / Insert / Delete", "O(log n) avg, O(n) worst", "O(h)"),
                ("Full Tree Traversal (DFS / BFS)", "O(n)", "O(h) or O(w)"),
                ("LCA Lookup", "O(n)", "O(h)"),
            ],
            pro_tips: &[
                "Tree recursion space complexity depends on tree height `h`: O(log n) for balanced trees, O(n) for skewed linear trees.",
            ],
        },
        Category::Tries => CategoryGuideData {
            category: cat,
            summary: "Tries (Prefix Trees) store strings character-by-character in a tree structure where shared prefixes share root path nodes.",
            how_it_works: "Each node contains a fixed array / HashMap of child links (a-z) and an is_end_of_word boolean flag.",
            key_patterns: &[
                "Prefix Matching: Check if any word starts with prefix in O(k) time where k is string length.",
                "Word Dictionary Search: Support wildcard matching using DFS over Trie children.",
            ],
            complexity_table: &[
                ("Insert Word (length k)", "O(k)", "O(k * ALPHABET)"),
                ("Search Word / Prefix", "O(k)", "O(1)"),
            ],
            pro_tips: &[
                "Tries avoid re-comparing shared prefixes, making word grid searches (Word Search II) drastically faster than raw string array checks.",
            ],
        },
        Category::Backtracking => CategoryGuideData {
            category: cat,
            summary: "Backtracking builds solution candidates incrementally and prunes invalid choices early using Depth-First Search recursion.",
            how_it_works: "Make a choice, recurse down the decision tree, then undo (backtrack) the choice to explore alternate paths.",
            key_patterns: &[
                "Subsets & Combinations: Include/exclude current item or loop through remaining candidates.",
                "Permutations: Swap or track visited elements to generate distinct orderings.",
                "Grid / Constraint Search: Mark grid cell visited, explore 4-directional neighbors, unmark cell.",
            ],
            complexity_table: &[
                ("Subsets (2^n choices)", "O(n * 2^n)", "O(n)"),
                ("Permutations (n! choices)", "O(n * n!)", "O(n)"),
                ("N-Queens / Grid Backtrack", "O(N!)", "O(N)"),
            ],
            pro_tips: &[
                "Always prune invalid branches as early as possible before making recursive calls to keep execution times fast.",
            ],
        },
        Category::HeapPriorityQueue => CategoryGuideData {
            category: cat,
            summary: "Heaps (Priority Queues) maintain the minimum or maximum element at the root in O(1) access time with O(log n) pushes and pops.",
            how_it_works: "Binary heaps store elements in complete binary trees backed by flat arrays (parent at i/2, children at 2i, 2i+1).",
            key_patterns: &[
                "Top K Elements: Maintain a Min-Heap of size K to find K largest elements in O(n log k).",
                "Two Heaps (Median Finder): Max-Heap for lower half, Min-Heap for upper half.",
                "K-Way Merge: Push smallest element from K sorted lists into a Min-Heap.",
            ],
            complexity_table: &[
                ("Peek Min / Max", "O(1)", "O(1)"),
                ("Push / Pop Heap Element", "O(log n)", "O(n)"),
                ("Heapify Array", "O(n)", "O(n)"),
            ],
            pro_tips: &[
                "Building a heap from an unsorted array of size n with heapify takes O(n) time, NOT O(n log n).",
            ],
        },
        Category::Graphs => CategoryGuideData {
            category: cat,
            summary: "Graphs represent networks of vertices connected by edges. Traversal algorithms explore nodes via Depth-First Search (DFS) or Breadth-First Search (BFS).",
            how_it_works: "Track visited nodes using a Set or 2D array to prevent infinite loops in cyclic graphs. Use BFS for shortest path in unweighted graphs.",
            key_patterns: &[
                "Connected Components (Grid / Adjacency): Flood fill / DFS to count island clusters.",
                "Topological Sort (Kahn / DFS): Process DAG dependency ordering (Course Schedule).",
                "Cycle Detection: Track visiting/visited states or fast/slow parent markers.",
            ],
            complexity_table: &[
                ("BFS / DFS Traversal", "O(V + E)", "O(V)"),
                ("Topological Sort", "O(V + E)", "O(V)"),
                ("Matrix Island Search (M x N)", "O(M * N)", "O(M * N)"),
            ],
            pro_tips: &[
                "BFS guarantees shortest path in terms of edge count on unweighted graphs; DFS does not.",
            ],
        },
        Category::OneDDp => CategoryGuideData {
            category: cat,
            summary: "Dynamic Programming (DP) solves complex problems by breaking them into overlapping subproblems, storing optimal results in a table or state variable.",
            how_it_works: "Define state transition recurrence relation: dp[i] = f(dp[i-1], dp[i-2], ...). Compute bottom-up or top-down with memoization.",
            key_patterns: &[
                "Fibonacci / Staircase: State depends on previous 1 or 2 steps (dp[i] = dp[i-1] + dp[i-2]).",
                "Unbounded / 0-1 Knapsack Subproblems: Iterate over target sum / capacity storing min/max combinations.",
                "Space Optimization: If state only depends on last K steps, reduce DP array to K variables.",
            ],
            complexity_table: &[
                ("Climbing Stairs / House Robber", "O(n)", "O(1) optimized"),
                ("Coin Change (Amount A, N coins)", "O(N * A)", "O(A)"),
                ("Longest Increasing Subsequence", "O(n^2) DP, O(n log n) Binary Search", "O(n)"),
            ],
            pro_tips: &[
                "Identify overlapping subproblems and optimal substructure before writing DP array loops.",
            ],
        },
        Category::Intervals => CategoryGuideData {
            category: cat,
            summary: "Interval problems involve ranges [start, end]. Sorting intervals by start time reveals overlaps and simplified merging patterns.",
            how_it_works: "Sort intervals by start time. Compare current interval's start with previous interval's end to detect overlap.",
            key_patterns: &[
                "Merge Overlapping Intervals: If curr.start <= prev.end, merge by updating prev.end = max(prev.end, curr.end).",
                "Insert Interval: Add pre, merge overlapping, add post.",
                "Meeting Rooms / Minimum Platforms: Track concurrent overlapping intervals using Min-Heap or two sorted boundary arrays.",
            ],
            complexity_table: &[
                ("Sort + Interval Scan", "O(n log n)", "O(n) or O(1)"),
                ("Insert Interval", "O(n)", "O(n)"),
            ],
            pro_tips: &[
                "Sorting by start time guarantees that any potential overlap must occur between adjacent intervals in the sorted sequence.",
            ],
        },
        Category::Greedy => CategoryGuideData {
            category: cat,
            summary: "Greedy algorithms make the locally optimal choice at each step, proving that local choices lead to a global optimal solution.",
            how_it_works: "Iterate through elements making irreversible locally optimal choices without backtracking or full DP tables.",
            key_patterns: &[
                "Maximum Subarray (Kadane's Algorithm): Reset running sum to current element when running sum drops below zero.",
                "Jump Game: Maintain furthest reachable index boundary at each position.",
                "Gas Station: Reset starting station whenever net fuel sum drops below zero.",
            ],
            complexity_table: &[
                ("Kadane's Algorithm", "O(n)", "O(1)"),
                ("Jump Game Reachability", "O(n)", "O(1)"),
                ("Gas Station Scan", "O(n)", "O(1)"),
            ],
            pro_tips: &[
                "Greedy choices must be mathematically provable; if local choices can lead to a dead end, use DP or Backtracking instead.",
            ],
        },
        Category::AdvancedGraphs => CategoryGuideData {
            category: cat,
            summary: "Advanced Graphs deal with weighted edges, minimum spanning trees, network flows, and multi-source shortest paths.",
            how_it_works: "Dijkstra uses Min-Heap for non-negative weighted shortest path. Prim / Kruskal use Union-Find / Priority Queue for MST.",
            key_patterns: &[
                "Dijkstra's Shortest Path: Min-Heap priority queue to extract distance-minimal vertex.",
                "Prim / Kruskal MST: Build minimum weight edge tree connecting all vertices.",
                "Eulerian Path Traversal: Hierholzer's algorithm to visit every edge exactly once.",
            ],
            complexity_table: &[
                ("Dijkstra's Shortest Path", "O((V + E) log V)", "O(V)"),
                ("Prim's MST Algorithm", "O(E log V)", "O(V)"),
                ("Kruskal's MST (Union-Find)", "O(E log E)", "O(V)"),
            ],
            pro_tips: &[
                "Dijkstra fails on negative edge weights (use Bellman-Ford for negative weights).",
            ],
        },
        Category::TwoDDp => CategoryGuideData {
            category: cat,
            summary: "2D Dynamic Programming computes states over two dimensions (e.g. string lengths, grid coordinates, or choice constraints).",
            how_it_works: "Maintain a 2D table `dp[i][j]` representing optimal values for subproblems `(i, j)`. Compute values row-by-row or column-by-column.",
            key_patterns: &[
                "Grid Path Problems (Unique Paths): dp[r][c] = dp[r-1][c] + dp[r][c-1].",
                "String Matching (LCS / Edit Distance): Compare s1[i] and s2[j] to decide match vs insert/delete/replace transitions.",
                "0-1 Knapsack (Item i, Capacity w): Include vs exclude current item.",
            ],
            complexity_table: &[
                ("Unique Paths Grid (M x N)", "O(M * N)", "O(N) optimized"),
                ("Longest Common Subsequence", "O(M * N)", "O(M * N)"),
                ("Edit Distance (Levenshtein)", "O(M * N)", "O(M * N)"),
            ],
            pro_tips: &[
                "2D DP space can often be compressed from O(M * N) down to O(N) by keeping only the previous row in memory.",
            ],
        },
        Category::BitManipulation => CategoryGuideData {
            category: cat,
            summary: "Bit Manipulation performs operations directly on binary representations of integers using bitwise operators (&, |, ^, ~, <<, >>).",
            how_it_works: "Utilize bit properties: x ^ x = 0, x ^ 0 = x, and n & (n - 1) clears the lowest set bit.",
            key_patterns: &[
                "Single Number (XOR): XORing all numbers cancels duplicates, leaving the unique element.",
                "Counting Bits (Kernighan): n & (n - 1) removes rightmost 1-bit in O(set_bits) operations.",
                "Bitmask Subset State: Use integer bitmasks (1 << i) to represent subset choices.",
            ],
            complexity_table: &[
                ("Single Number (XOR)", "O(n)", "O(1)"),
                ("Bit Counting (n & (n - 1))", "O(number of 1s)", "O(1)"),
                ("Reverse Bits (32-bit)", "O(1)", "O(1)"),
            ],
            pro_tips: &[
                "Bitwise XOR is commutative and associative, making sequence order irrelevant for cancellation.",
            ],
        },
        Category::MathAndGeometry => CategoryGuideData {
            category: cat,
            summary: "Math & Geometry algorithms solve numeric, matrix, spatial, and modular arithmetic problems using algebraic and geometric properties.",
            how_it_works: "Leverage mathematical invariants (e.g. matrix transposition + row reversing = 90 degree clockwise rotation).",
            key_patterns: &[
                "Matrix In-Place Rotation: Transpose matrix then reverse each row to rotate 90 degrees clockwise.",
                "Spiral Matrix Traversal: Maintain top, bottom, left, right boundary pointers.",
                "Happy Number (Floyd Cycle): Sum of squared digits mapped to fast/slow cycle detection.",
            ],
            complexity_table: &[
                ("Rotate Image Matrix (N x N)", "O(N^2)", "O(1)"),
                ("Spiral Matrix Bounds Pass", "O(M * N)", "O(1)"),
                ("Pow(x, n) Binary Exponentiation", "O(log n)", "O(1)"),
            ],
            pro_tips: &[
                "Always check for overflow when performing matrix bounds or integer power calculations.",
            ],
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_every_category_has_valid_guide() {
        for &cat in Category::all() {
            let guide = get_category_guide(cat);
            assert_eq!(guide.category, cat);
            assert!(!guide.summary.is_empty());
            assert!(!guide.how_it_works.is_empty());
            assert!(!guide.key_patterns.is_empty());
            assert!(!guide.complexity_table.is_empty());
            assert!(!guide.pro_tips.is_empty());
        }
    }
}
