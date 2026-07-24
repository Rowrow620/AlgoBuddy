use std::collections::{BTreeMap, BTreeSet};
use eframe::egui::Color32;

// ── Themes & Accessibility ──

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    DarkVSCode,
    DarkCyber,
    LightClean,
}

impl Theme {
    pub fn label(&self) -> &'static str {
        match self {
            Theme::DarkVSCode => "VS Code Dark",
            Theme::DarkCyber => "Cyber Navy (Dark)",
            Theme::LightClean => "Clean Light",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorblindMode {
    Off,
    RedGreenSafe,
    HighContrast,
}

impl ColorblindMode {
    pub fn label(&self) -> &'static str {
        match self {
            ColorblindMode::Off => "Off (Standard)",
            ColorblindMode::RedGreenSafe => "Protan / Deuteran (Blue-Orange)",
            ColorblindMode::HighContrast => "High Contrast B&W",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ThemePalette {
    pub bg_dark: Color32,
    pub sidebar_bg: Color32,
    pub step_box_bg: Color32,
    pub cell_bg: Color32,
    pub cell_border: Color32,
    pub text_primary: Color32,
    pub text_muted: Color32,
    pub text_dim: Color32,
    pub cyan: Color32,
    pub purple: Color32,
    pub emerald: Color32,
    pub emerald_text: Color32,
    pub amber: Color32,
    pub pink: Color32,
    pub red: Color32,
    pub code_active_bg: Color32,
}

impl Theme {
    pub fn palette(&self, cb: ColorblindMode) -> ThemePalette {
        let (base_emerald, base_emerald_text, base_red) = match cb {
            ColorblindMode::Off => (
                Color32::from_rgb(16, 185, 129),   // Emerald Green
                Color32::from_rgb(52, 211, 153),  // Emerald Text
                Color32::from_rgb(244, 63, 94),   // Ruby Red
            ),
            ColorblindMode::RedGreenSafe => (
                Color32::from_rgb(37, 99, 235),   // Cobalt Blue (Valid / Success)
                Color32::from_rgb(96, 165, 250),  // Light Cobalt Text
                Color32::from_rgb(234, 88, 12),   // Safety Orange (Error / Duplicate)
            ),
            ColorblindMode::HighContrast => (
                Color32::from_rgb(255, 255, 255), // High Contrast White
                Color32::from_rgb(255, 255, 255),
                Color32::from_rgb(255, 255, 0),   // Vivid Yellow
            ),
        };

        match self {
            Theme::DarkVSCode => ThemePalette {
                bg_dark: Color32::from_rgb(24, 24, 24),        // VS Code #181818
                sidebar_bg: Color32::from_rgb(30, 30, 30),     // VS Code #1e1e1e
                step_box_bg: Color32::from_rgb(37, 37, 38),    // VS Code #252526
                cell_bg: Color32::from_rgb(45, 45, 48),        // VS Code panel #2d2d30
                cell_border: Color32::from_rgb(60, 60, 60),
                text_primary: Color32::from_rgb(220, 220, 220),
                text_muted: Color32::from_rgb(160, 160, 160),
                text_dim: Color32::from_rgb(110, 110, 110),
                cyan: Color32::from_rgb(86, 156, 214),        // VS Code Keyword Blue
                purple: Color32::from_rgb(197, 134, 192),     // VS Code Pink/Purple
                emerald: base_emerald,
                emerald_text: base_emerald_text,
                amber: Color32::from_rgb(206, 145, 120),       // VS Code String Orange
                pink: Color32::from_rgb(220, 100, 170),
                red: base_red,
                code_active_bg: Color32::from_rgb(9, 71, 113),  // VS Code Selection Blue
            },
            Theme::DarkCyber => ThemePalette {
                bg_dark: Color32::from_rgb(11, 15, 25),
                sidebar_bg: Color32::from_rgb(15, 23, 42),
                step_box_bg: Color32::from_rgb(30, 41, 59),
                cell_bg: Color32::from_rgb(30, 41, 59),
                cell_border: Color32::from_rgb(51, 65, 85),
                text_primary: Color32::from_rgb(248, 250, 252),
                text_muted: Color32::from_rgb(156, 163, 175),
                text_dim: Color32::from_rgb(100, 116, 139),
                cyan: Color32::from_rgb(56, 189, 248),
                purple: Color32::from_rgb(168, 85, 247),
                emerald: base_emerald,
                emerald_text: base_emerald_text,
                amber: Color32::from_rgb(245, 158, 11),
                pink: Color32::from_rgb(236, 72, 153),
                red: base_red,
                code_active_bg: Color32::from_rgb(14, 116, 144),
            },
            Theme::LightClean => ThemePalette {
                bg_dark: Color32::from_rgb(248, 250, 252),      // Slate 50
                sidebar_bg: Color32::from_rgb(255, 255, 255),   // White panel
                step_box_bg: Color32::from_rgb(241, 245, 249),  // Slate 100
                cell_bg: Color32::from_rgb(241, 245, 249),
                cell_border: Color32::from_rgb(203, 213, 225),  // Slate 300
                text_primary: Color32::from_rgb(15, 23, 42),    // Slate 900
                text_muted: Color32::from_rgb(71, 85, 105),     // Slate 600
                text_dim: Color32::from_rgb(148, 163, 184),    // Slate 400
                cyan: Color32::from_rgb(2, 132, 199),          // Sky 600
                purple: Color32::from_rgb(147, 51, 234),       // Purple 600
                emerald: base_emerald,
                emerald_text: if cb == ColorblindMode::Off { Color32::from_rgb(5, 150, 105) } else { base_emerald_text },
                amber: Color32::from_rgb(217, 119, 6),         // Amber 600
                pink: Color32::from_rgb(219, 39, 119),         // Pink 600
                red: base_red,
                code_active_bg: Color32::from_rgb(186, 230, 253), // Sky 200
            },
        }
    }
}

// ── Difficulty Level ──

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    pub fn label(&self) -> &'static str {
        match self {
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard",
        }
    }
}

// ── Roadmap Categories (NeetCode 150 Hierarchy) ──

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    ArraysAndHashing,
    TwoPointers,
    Stack,
    BinarySearch,
    SlidingWindow,
    LinkedList,
    Trees,
    Tries,
    Backtracking,
    HeapPriorityQueue,
    Graphs,
    OneDDp,
    Intervals,
    Greedy,
    AdvancedGraphs,
    TwoDDp,
    BitManipulation,
    MathAndGeometry,
}

impl Category {
    pub fn name(&self) -> &'static str {
        match self {
            Category::ArraysAndHashing => "Arrays & Hashing",
            Category::TwoPointers => "Two Pointers",
            Category::Stack => "Stack",
            Category::BinarySearch => "Binary Search",
            Category::SlidingWindow => "Sliding Window",
            Category::LinkedList => "Linked List",
            Category::Trees => "Trees",
            Category::Tries => "Tries",
            Category::Backtracking => "Backtracking",
            Category::HeapPriorityQueue => "Heap / Priority Queue",
            Category::Graphs => "Graphs",
            Category::OneDDp => "1-D DP",
            Category::Intervals => "Intervals",
            Category::Greedy => "Greedy",
            Category::AdvancedGraphs => "Advanced Graphs",
            Category::TwoDDp => "2-D DP",
            Category::BitManipulation => "Bit Manipulation",
            Category::MathAndGeometry => "Math & Geometry",
        }
    }

    pub fn all() -> &'static [Category] {
        &[
            Category::ArraysAndHashing,
            Category::TwoPointers,
            Category::Stack,
            Category::BinarySearch,
            Category::SlidingWindow,
            Category::LinkedList,
            Category::Trees,
            Category::Tries,
            Category::Backtracking,
            Category::HeapPriorityQueue,
            Category::Graphs,
            Category::OneDDp,
            Category::Intervals,
            Category::Greedy,
            Category::AdvancedGraphs,
            Category::TwoDDp,
            Category::BitManipulation,
            Category::MathAndGeometry,
        ]
    }
}

// ── Problem Statement & Example Data Structures ──

pub struct Example {
    pub input: &'static str,
    pub output: &'static str,
    pub explanation: &'static str,
}

pub struct ApproachMeta {
    pub id: usize,
    pub name: &'static str,
    pub time_complexity: &'static str,
    pub space_complexity: &'static str,
    pub description: &'static str,
}

pub struct ProblemDetails {
    pub id: u32,
    pub title: &'static str,
    pub difficulty: Difficulty,
    pub category: Category,
    pub statement: &'static str,
    pub examples: &'static [Example],
    pub constraints: &'static [&'static str],
    pub leetcode_url: &'static str,
    pub approaches: &'static [ApproachMeta],
}

// ── Problem Enum ──

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Problem {
    ContainsDuplicate,
    TwoSum,
    ValidAnagram,
    GroupAnagrams,
    TopKFrequent,
    ProductExceptSelf,
    EncodeDecode,
    ValidSudoku,
    LongestConsecutive,
    ValidPalindrome,
    BestTimeStock,
    ValidParentheses,
    BinarySearch,
    ReverseLinkedList,
    MergeTwoLists,
    LinkedListCycle,
    InvertTree,
    MaxDepthTree,
    DiameterTree,
}

impl Problem {
    pub fn all() -> &'static [Problem] {
        &[
            Problem::ContainsDuplicate,
            Problem::TwoSum,
            Problem::ValidAnagram,
            Problem::GroupAnagrams,
            Problem::TopKFrequent,
            Problem::ProductExceptSelf,
            Problem::EncodeDecode,
            Problem::ValidSudoku,
            Problem::LongestConsecutive,
            Problem::ValidPalindrome,
            Problem::BestTimeStock,
            Problem::ValidParentheses,
            Problem::BinarySearch,
            Problem::ReverseLinkedList,
            Problem::MergeTwoLists,
            Problem::LinkedListCycle,
            Problem::InvertTree,
            Problem::MaxDepthTree,
            Problem::DiameterTree,
        ]
    }

    pub fn id(&self) -> u32 {
        self.details().id
    }

    pub fn title(&self) -> &'static str {
        self.details().title
    }

    pub fn difficulty(&self) -> Difficulty {
        self.details().difficulty
    }

    pub fn category(&self) -> Category {
        self.details().category
    }

    pub fn details(&self) -> ProblemDetails {
        match self {
            Problem::ContainsDuplicate => ProblemDetails {
                id: 217,
                title: "Contains Duplicate",
                difficulty: Difficulty::Easy,
                category: Category::ArraysAndHashing,
                statement: "Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.",
                examples: &[
                    Example { input: "nums = [1, 2, 3, 1]", output: "true", explanation: "Digit 1 appears twice." },
                    Example { input: "nums = [1, 2, 3, 4]", output: "false", explanation: "All elements are distinct." },
                    Example { input: "nums = [1, 1, 1, 3, 3, 4, 3, 2, 4, 2]", output: "true", explanation: "Multiple duplicates exist." },
                ],
                constraints: &["1 <= nums.length <= 10^5", "-10^9 <= nums[i] <= 10^9"],
                leetcode_url: "https://leetcode.com/problems/contains-duplicate/",
                approaches: &[
                    ApproachMeta { id: 0, name: "Hash Set Lookup", time_complexity: "O(N)", space_complexity: "O(N)", description: "Insert into set, return true on collision." },
                    ApproachMeta { id: 1, name: "Sorting Array", time_complexity: "O(N log N)", space_complexity: "O(1)", description: "Sort array, check adjacent elements." },
                ],
            },
            Problem::TwoSum => ProblemDetails {
                id: 1,
                title: "Two Sum",
                difficulty: Difficulty::Easy,
                category: Category::ArraysAndHashing,
                statement: "Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.",
                examples: &[
                    Example { input: "nums = [2, 7, 11, 15], target = 9", output: "[0, 1]", explanation: "nums[0] + nums[1] == 9" },
                ],
                constraints: &["2 <= nums.length <= 10^4", "-10^9 <= nums[i] <= 10^9"],
                leetcode_url: "https://leetcode.com/problems/two-sum/",
                approaches: &[
                    ApproachMeta { id: 0, name: "Hash Map (One Pass)", time_complexity: "O(N)", space_complexity: "O(N)", description: "Use hash map complement lookup." },
                    ApproachMeta { id: 1, name: "Brute Force", time_complexity: "O(N²)", space_complexity: "O(1)", description: "Nested pairs loop." },
                ],
            },
            Problem::ValidAnagram => ProblemDetails {
                id: 242,
                title: "Valid Anagram",
                difficulty: Difficulty::Easy,
                category: Category::ArraysAndHashing,
                statement: "Given two strings s and t, return true if t is an anagram of s, and false otherwise.",
                examples: &[
                    Example { input: "s = \"anagram\", t = \"nagaram\"", output: "true", explanation: "Character frequencies match." },
                ],
                constraints: &["1 <= s.length, t.length <= 5 * 10^4"],
                leetcode_url: "https://leetcode.com/problems/valid-anagram/",
                approaches: &[
                    ApproachMeta { id: 0, name: "Frequency Counter Array", time_complexity: "O(N)", space_complexity: "O(1)", description: "Count char frequencies in 26 arrays." },
                    ApproachMeta { id: 1, name: "Sort Strings", time_complexity: "O(N log N)", space_complexity: "O(N)", description: "Compare sorted strings." },
                ],
            },
            Problem::GroupAnagrams => ProblemDetails {
                id: 49,
                title: "Group Anagrams",
                difficulty: Difficulty::Medium,
                category: Category::ArraysAndHashing,
                statement: "Given an array of strings strs, group the anagrams together. You can return the answer in any order. An Anagram is a word or phrase formed by rearranging the letters of a different word using all original letters exactly once.",
                examples: &[
                    Example {
                        input: "strs = [\"eat\", \"tea\", \"tan\", \"ate\", \"nat\", \"bat\"]",
                        output: "[[\"bat\"], [\"nat\", \"tan\"], [\"ate\", \"eat\", \"tea\"]]",
                        explanation: "Anagrams mapped to identical frequency keys or sorted signatures.",
                    },
                    Example { input: "strs = [\"\"]", output: "[[\"\"]]", explanation: "Single empty string." },
                    Example { input: "strs = [\"a\"]", output: "[[\"a\"]]", explanation: "Single character string." },
                ],
                constraints: &["1 <= strs.length <= 10^4", "0 <= strs[i].length <= 100", "strs[i] consists of lowercase English letters."],
                leetcode_url: "https://leetcode.com/problems/group-anagrams/",
                approaches: &[
                    ApproachMeta { id: 0, name: "Char Frequency Tuple Map", time_complexity: "O(N * K)", space_complexity: "O(N * K)", description: "Use 26-element character count tuple as HashMap key." },
                    ApproachMeta { id: 1, name: "Sorted String Key Map", time_complexity: "O(N * K log K)", space_complexity: "O(N * K)", description: "Use sorted string signature as HashMap key." },
                ],
            },
            Problem::TopKFrequent => ProblemDetails {
                id: 347,
                title: "Top K Frequent Elements",
                difficulty: Difficulty::Medium,
                category: Category::ArraysAndHashing,
                statement: "Given an integer array nums and an integer k, return the k most frequent elements.",
                examples: &[
                    Example { input: "nums = [1,1,1,2,2,3], k = 2", output: "[1, 2]", explanation: "1 appears 3 times, 2 appears 2 times." },
                ],
                constraints: &["1 <= nums.length <= 10^5"],
                leetcode_url: "https://leetcode.com/problems/top-k-frequent-elements/",
                approaches: &[
                    ApproachMeta { id: 0, name: "Bucket Sort", time_complexity: "O(N)", space_complexity: "O(N)", description: "Array indices as frequency buckets." },
                    ApproachMeta { id: 1, name: "Min-Heap", time_complexity: "O(N log k)", space_complexity: "O(N)", description: "Maintain min-heap of size k." },
                    ApproachMeta { id: 2, name: "Sorting Pairs", time_complexity: "O(N log N)", space_complexity: "O(N)", description: "Sort count pairs." },
                ],
            },
            Problem::ProductExceptSelf => ProblemDetails {
                id: 238,
                title: "Product of Array Except Self",
                difficulty: Difficulty::Medium,
                category: Category::ArraysAndHashing,
                statement: "Given an integer array nums, return an array output where output[i] is the product of all elements of nums except nums[i]. Must run in O(n) without division.",
                examples: &[
                    Example { input: "nums = [1, 2, 4, 6]", output: "[48, 24, 12, 8]", explanation: "output[0] = 2*4*6=48, output[1] = 1*4*6=24, output[2] = 1*2*6=12, output[3] = 1*2*4=8." },
                    Example { input: "nums = [-1, 0, 1, 2, 3]", output: "[0, -6, 0, 0, 0]", explanation: "Zero element zeroes out other indices." },
                ],
                constraints: &["2 <= nums.length <= 1000", "-20 <= nums[i] <= 20"],
                leetcode_url: "https://leetcode.com/problems/product-of-array-except-self/",
                approaches: &[
                    ApproachMeta { id: 0, name: "Prefix & Suffix Pass", time_complexity: "O(N)", space_complexity: "O(1)", description: "Prefix array and running suffix." },
                ],
            },
            Problem::EncodeDecode => ProblemDetails {
                id: 271,
                title: "Encode and Decode Strings",
                difficulty: Difficulty::Medium,
                category: Category::ArraysAndHashing,
                statement: "Design an algorithm to encode a list of strings to a string, then decode it back to the original list of strings.",
                examples: &[
                    Example { input: "strs = [\"Hello\", \"World\"]", output: "[\"Hello\", \"World\"]", explanation: "Encoded into 5#Hello5#World, then decoded back." },
                    Example { input: "strs = [\"\"]", output: "[\"\"]", explanation: "Encoded into 0#." },
                ],
                constraints: &["0 <= strs.length < 100", "0 <= strs[i].length < 200"],
                leetcode_url: "https://leetcode.com/problems/encode-and-decode-strings/",
                approaches: &[
                    ApproachMeta { id: 0, name: "Length Prefix (# Protocol)", time_complexity: "O(N)", space_complexity: "O(N)", description: "Prefix len#string." },
                ],
            },
            Problem::ValidSudoku => ProblemDetails {
                id: 36,
                title: "Valid Sudoku",
                difficulty: Difficulty::Medium,
                category: Category::ArraysAndHashing,
                statement: "You are given a 9 x 9 Sudoku board. A Sudoku board is valid if:\n1. Each row must contain digits 1-9 without duplicates.\n2. Each column must contain digits 1-9 without duplicates.\n3. Each 3 x 3 sub-box of the grid must contain digits 1-9 without duplicates.\n\nNote: A board does not need to be full or solvable to be valid.",
                examples: &[
                    Example {
                        input: "board = [[1, 2, ., ., 3, ., ., ., .], ...]",
                        output: "true",
                        explanation: "No duplicate digits in any row, column, or 3x3 sub-box.",
                    },
                    Example {
                        input: "board = [[1, 2, ., ., 3, ., ., ., .], [4, ., ., 5, ., ., ., ., .], [., 9, 1, ., ., ., ., ., 3], ...]",
                        output: "false",
                        explanation: "There are two 1's in the top-left 3x3 sub-box.",
                    },
                ],
                constraints: &[
                    "board.length == 9",
                    "board[i].length == 9",
                    "board[i][j] is a digit 1-9 or '.'",
                ],
                leetcode_url: "https://leetcode.com/problems/valid-sudoku/",
                approaches: &[
                    ApproachMeta {
                        id: 0,
                        name: "HashSet Validation (Rows, Cols, 3x3 Boxes)",
                        time_complexity: "O(1) [9x9=81 cells]",
                        space_complexity: "O(1) [81 items]",
                        description: "Scan every cell (r, c). Use 9 row sets, 9 column sets, and 9 box sets to detect duplicates instantly.",
                    },
                ],
            },
            Problem::LongestConsecutive => ProblemDetails {
                id: 128,
                title: "Longest Consecutive Sequence",
                difficulty: Difficulty::Medium,
                category: Category::ArraysAndHashing,
                statement: "Given an array of integers nums, return the length of the longest consecutive sequence of elements that can be formed. A consecutive sequence is a sequence in which each element is exactly 1 greater than the previous. Must run in O(n) time.",
                examples: &[
                    Example {
                        input: "nums = [2, 20, 4, 10, 3, 4, 5]",
                        output: "4",
                        explanation: "The longest consecutive sequence is [2, 3, 4, 5].",
                    },
                    Example {
                        input: "nums = [0, 3, 2, 5, 4, 6, 1, 1]",
                        output: "7",
                        explanation: "The longest consecutive sequence is [0, 1, 2, 3, 4, 5, 6].",
                    },
                ],
                constraints: &[
                    "0 <= nums.length <= 1000",
                    "-10^9 <= nums[i] <= 10^9",
                ],
                leetcode_url: "https://leetcode.com/problems/longest-consecutive-sequence/",
                approaches: &[
                    ApproachMeta {
                        id: 0,
                        name: "HashSet Sequence Start Expansion",
                        time_complexity: "O(N)",
                        space_complexity: "O(N)",
                        description: "Convert array to HashSet. Only start expanding a streak if (n - 1) is not in the set.",
                    },
                ],
            },
            Problem::ValidPalindrome => ProblemDetails {
                id: 125,
                title: "Valid Palindrome",
                difficulty: Difficulty::Easy,
                category: Category::TwoPointers,
                statement: "Given a string s, return true if it is a palindrome, otherwise return false.",
                examples: &[
                    Example { input: "s = \"Was it a car or a cat I saw?\"", output: "true", explanation: "After filtering: \"wasitacaroracatisaw\" is a palindrome." },
                    Example { input: "s = \"tab a cat\"", output: "false", explanation: "\"tabacat\" is not a palindrome." },
                ],
                constraints: &["1 <= s.length <= 1000"],
                leetcode_url: "https://leetcode.com/problems/valid-palindrome/",
                approaches: &[
                    ApproachMeta { id: 0, name: "Two Pointers (In-Place)", time_complexity: "O(N)", space_complexity: "O(1)", description: "Converging left and right pointers." },
                    ApproachMeta { id: 1, name: "Reverse Filtered String", time_complexity: "O(N)", space_complexity: "O(N)", description: "Compare filtered string to reverse." },
                ],
            },
            Problem::BestTimeStock => ProblemDetails {
                id: 121,
                title: "Best Time to Buy and Sell Stock",
                difficulty: Difficulty::Easy,
                category: Category::SlidingWindow,
                statement: "You are given an integer array prices where prices[i] is the price of NeetCoin on the i-th day. Choose a single day to buy one NeetCoin and a future day to sell it to maximize profit.",
                examples: &[
                    Example { input: "prices = [10, 1, 5, 6, 7, 1]", output: "6", explanation: "Buy day 1 (price=1), sell day 4 (price=7), profit = 7 - 1 = 6." },
                    Example { input: "prices = [10, 8, 7, 5, 2]", output: "0", explanation: "No profitable transactions, profit = 0." },
                ],
                constraints: &["1 <= prices.length <= 100", "0 <= prices[i] <= 100"],
                leetcode_url: "https://leetcode.com/problems/best-time-to-buy-and-sell-stock/",
                approaches: &[
                    ApproachMeta { id: 0, name: "Two Pointers / Sliding Window", time_complexity: "O(N)", space_complexity: "O(1)", description: "Left pointer tracks min price day, right pointer scans for sell days." },
                ],
            },
            Problem::ValidParentheses => ProblemDetails {
                id: 20,
                title: "Valid Parentheses",
                difficulty: Difficulty::Easy,
                category: Category::Stack,
                statement: "Given a string s consisting of '(', ')', '{', '}', '[' and ']', return true if input string is valid.",
                examples: &[
                    Example { input: "s = \"[]\"", output: "true", explanation: "Valid bracket match." },
                    Example { input: "s = \"([{}])\"", output: "true", explanation: "Nested brackets closed in order." },
                    Example { input: "s = \"[(])\"", output: "false", explanation: "Mismatched order." },
                ],
                constraints: &["1 <= s.length <= 1000"],
                leetcode_url: "https://leetcode.com/problems/valid-parentheses/",
                approaches: &[
                    ApproachMeta { id: 0, name: "Stack Matching", time_complexity: "O(N)", space_complexity: "O(N)", description: "Push opening brackets; pop and match closing brackets." },
                ],
            },
            Problem::BinarySearch => ProblemDetails {
                id: 704,
                title: "Binary Search",
                difficulty: Difficulty::Easy,
                category: Category::BinarySearch,
                statement: "Given a sorted array of distinct integers nums and a target integer, return index of target, or -1 if not found. Must run in O(log n) time.",
                examples: &[
                    Example { input: "nums = [-1, 0, 2, 4, 6, 8], target = 4", output: "3", explanation: "Target 4 exists at index 3." },
                    Example { input: "nums = [-1, 0, 2, 4, 6, 8], target = 3", output: "-1", explanation: "Target 3 does not exist." },
                ],
                constraints: &["1 <= nums.length <= 10000"],
                leetcode_url: "https://leetcode.com/problems/binary-search/",
                approaches: &[
                    ApproachMeta { id: 0, name: "Binary Search Iterative", time_complexity: "O(log N)", space_complexity: "O(1)", description: "Narrow search window using midpoint m." },
                ],
            },
            Problem::ReverseLinkedList => ProblemDetails {
                id: 206,
                title: "Reverse Linked List",
                difficulty: Difficulty::Easy,
                category: Category::LinkedList,
                statement: "Given the beginning of a singly linked list head, reverse the list, and return the new head.",
                examples: &[
                    Example { input: "head = [0, 1, 2, 3]", output: "[3, 2, 1, 0]", explanation: "Next pointers reversed." },
                    Example { input: "head = []", output: "[]", explanation: "Empty list." },
                ],
                constraints: &["0 <= length <= 1000"],
                leetcode_url: "https://leetcode.com/problems/reverse-linked-list/",
                approaches: &[
                    ApproachMeta { id: 0, name: "Iterative Pointers (prev, curr)", time_complexity: "O(N)", space_complexity: "O(1)", description: "Flip next pointers one by one." },
                ],
            },
            Problem::MergeTwoLists => ProblemDetails {
                id: 21,
                title: "Merge Two Sorted Linked Lists",
                difficulty: Difficulty::Easy,
                category: Category::LinkedList,
                statement: "You are given the heads of two sorted linked lists list1 and list2. Merge the two lists into one sorted linked list and return the head of the new sorted list.",
                examples: &[
                    Example { input: "list1 = [1, 2, 4], list2 = [1, 3, 5]", output: "[1, 1, 2, 3, 4, 5]", explanation: "Nodes merged in ascending order." },
                    Example { input: "list1 = [], list2 = [1, 2]", output: "[1, 2]", explanation: "Merging empty list with list2." },
                ],
                constraints: &["0 <= list1.length, list2.length <= 100", "-100 <= Node.val <= 100"],
                leetcode_url: "https://leetcode.com/problems/merge-two-sorted-lists/",
                approaches: &[
                    ApproachMeta { id: 0, name: "Two Pointers Merge (Dummy Node)", time_complexity: "O(N + M)", space_complexity: "O(1)", description: "Compare list1 and list2 heads, attach smaller node to tail." },
                ],
            },
            Problem::LinkedListCycle => ProblemDetails {
                id: 141,
                title: "Linked List Cycle Detection",
                difficulty: Difficulty::Easy,
                category: Category::LinkedList,
                statement: "Given the beginning of a linked list head, return true if there is a cycle in the linked list. Otherwise, return false.",
                examples: &[
                    Example { input: "head = [1, 2, 3, 4], index = 1", output: "true", explanation: "Tail node connects back to index 1 (cycle exists)." },
                    Example { input: "head = [1, 2], index = -1", output: "false", explanation: "Tail node points to null (no cycle)." },
                ],
                constraints: &["0 <= length <= 1000", "index is -1 or a valid node index"],
                leetcode_url: "https://leetcode.com/problems/linked-list-cycle/",
                approaches: &[
                    ApproachMeta { id: 0, name: "Floyd's Tortoise and Hare (Slow/Fast Pointers)", time_complexity: "O(N)", space_complexity: "O(1)", description: "Slow advances 1 step, fast advances 2 steps. If fast catches slow, cycle exists." },
                ],
            },
            Problem::InvertTree => ProblemDetails {
                id: 226,
                title: "Invert Binary Tree",
                difficulty: Difficulty::Easy,
                category: Category::Trees,
                statement: "You are given the root of a binary tree root. Invert the binary tree (swap left and right subtrees for every node) and return its root.",
                examples: &[
                    Example { input: "root = [1, 2, 3, 4, 5, 6, 7]", output: "[1, 3, 2, 7, 6, 5, 4]", explanation: "Left and right subtrees swapped at every level." },
                    Example { input: "root = [3, 2, 1]", output: "[3, 1, 2]", explanation: "Child nodes 2 and 1 swapped." },
                ],
                constraints: &["0 <= number of nodes <= 100"],
                leetcode_url: "https://leetcode.com/problems/invert-binary-tree/",
                approaches: &[
                    ApproachMeta { id: 0, name: "Recursive DFS (Post-order Swap)", time_complexity: "O(N)", space_complexity: "O(H)", description: "Recursively invert left and right subtrees, then swap root.left and root.right." },
                ],
            },
            Problem::MaxDepthTree => ProblemDetails {
                id: 104,
                title: "Maximum Depth of Binary Tree",
                difficulty: Difficulty::Easy,
                category: Category::Trees,
                statement: "Given the root of a binary tree, return its maximum depth. The depth is the number of nodes along the longest path from the root node down to the farthest leaf node.",
                examples: &[
                    Example { input: "root = [1, 2, 3, null, null, 4]", output: "3", explanation: "Longest path is 1 -> 3 -> 4 of length 3 nodes." },
                    Example { input: "root = []", output: "0", explanation: "Empty tree has depth 0." },
                ],
                constraints: &["0 <= number of nodes <= 100"],
                leetcode_url: "https://leetcode.com/problems/maximum-depth-of-binary-tree/",
                approaches: &[
                    ApproachMeta { id: 0, name: "Recursive DFS (1 + max(left, right))", time_complexity: "O(N)", space_complexity: "O(H)", description: "Return 1 + max(maxDepth(left), maxDepth(right))." },
                ],
            },
            Problem::DiameterTree => ProblemDetails {
                id: 543,
                title: "Diameter of Binary Tree",
                difficulty: Difficulty::Easy,
                category: Category::Trees,
                statement: "The diameter of a binary tree is defined as the length of the longest path between any two nodes in the tree. The path does not necessarily pass through the root.",
                examples: &[
                    Example { input: "root = [1, null, 2, 3, 4, 5]", output: "3", explanation: "Longest path is [5, 3, 2, 4] with 3 edges." },
                    Example { input: "root = [1, 2, 3]", output: "2", explanation: "Longest path is [2, 1, 3] with 2 edges." },
                ],
                constraints: &["1 <= number of nodes <= 100"],
                leetcode_url: "https://leetcode.com/problems/diameter-of-binary-tree/",
                approaches: &[
                    ApproachMeta { id: 0, name: "Post-order Depth DFS", time_complexity: "O(N)", space_complexity: "O(H)", description: "Compute left height + right height at each node, track max diameter." },
                ],
            },
        }
    }
}

// ── Visual State Variants per Problem Type ──

#[derive(Debug, Clone)]
pub enum VisualState {
    ContainsDuplicate {
        nums: Vec<i32>,
        active_idx: Option<usize>,
        seen_set: BTreeSet<i32>,
        duplicate_val: Option<i32>,
        has_duplicate: Option<bool>,
    },
    GroupAnagrams {
        input_strs: Vec<String>,
        active_idx: Option<usize>,
        key_fmt: String,
        groups: BTreeMap<String, Vec<String>>,
    },
    TopK {
        nums: Vec<i32>,
        active_nums_idx: Option<usize>,
        count_map: BTreeMap<i32, usize>,
        buckets: Vec<Vec<i32>>,
        active_bucket_idx: Option<usize>,
        result: Vec<i32>,
    },
    EncodeDecode {
        input_strs: Vec<String>,
        encoded_so_far: String,
        decoded_so_far: Vec<String>,
        pointer: usize,
        active_str_idx: Option<usize>,
        phase: EncodeDecodePhase,
    },
    Product {
        nums: Vec<i32>,
        output: Vec<i64>,
        active_idx: Option<usize>,
        prefix_val: i64,
        suffix_val: i64,
        phase: ProductPhase,
    },
    ValidSudoku {
        board: [[char; 9]; 9],
        active_r: Option<usize>,
        active_c: Option<usize>,
        duplicate_pos: Option<(usize, usize)>,
        is_valid: Option<bool>,
    },
    LongestConsecutive {
        nums: Vec<i32>,
        num_set: BTreeSet<i32>,
        current_num: Option<i32>,
        current_seq: Vec<i32>,
        max_length: usize,
        is_seq_start: Option<bool>,
    },
    TwoSum {
        nums: Vec<i32>,
        target: i32,
        active_idx: Option<usize>,
        secondary_idx: Option<usize>,
        map: BTreeMap<i32, usize>,
        found_indices: Option<(usize, usize)>,
    },
    ValidAnagram {
        s: String,
        t: String,
        s_counts: [usize; 26],
        t_counts: [usize; 26],
        active_s_idx: Option<usize>,
        active_t_idx: Option<usize>,
        is_anagram: Option<bool>,
    },
    TwoPointers {
        chars: Vec<char>,
        left: usize,
        right: usize,
        is_valid: Option<bool>,
        skipped: bool,
    },
    Stack {
        chars: Vec<char>,
        active_idx: Option<usize>,
        stack: Vec<char>,
        is_valid: Option<bool>,
    },
    BestTimeStock {
        prices: Vec<i32>,
        left_buy: usize,
        right_sell: usize,
        current_profit: i32,
        max_profit: i32,
    },
    BinarySearch {
        nums: Vec<i32>,
        target: i32,
        left: usize,
        right: usize,
        mid: Option<usize>,
        found_idx: Option<usize>,
    },
    LinkedList {
        nodes: Vec<i32>,
        prev_idx: Option<usize>,
        curr_idx: Option<usize>,
        next_idx: Option<usize>,
        reversed_so_far: Vec<i32>,
    },
    MergeLinkedLists {
        list1: Vec<i32>,
        list2: Vec<i32>,
        p1_idx: Option<usize>,
        p2_idx: Option<usize>,
        merged_so_far: Vec<i32>,
    },
    LinkedListCycle {
        nodes: Vec<i32>,
        cycle_target_idx: Option<usize>,
        slow_idx: Option<usize>,
        fast_idx: Option<usize>,
        has_cycle: Option<bool>,
    },
    TreeVisual {
        tree_nodes: Vec<Option<i32>>,
        active_node_idx: Option<usize>,
        secondary_node_idx: Option<usize>,
        depth_val: Option<i32>,
        max_diameter: Option<i32>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EncodeDecodePhase {
    Init,
    Encoding,
    EncodingComplete,
    Decoding,
    Complete,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProductPhase {
    Init,
    PrefixPass,
    SuffixPass,
    Complete,
}

// ── Generic Execution Step ──

#[derive(Debug, Clone)]
pub struct Step {
    pub code_line: usize,
    pub description: String,
    pub visual: VisualState,
}

// ── Approach-Specific Code Lines Providers ──

pub fn approach_code_lines(problem: Problem, approach_id: usize) -> Vec<(usize, &'static str)> {
    match (problem, approach_id) {
        (Problem::ContainsDuplicate, 0) => vec![
            (1, "class Solution:"),
            (2, "    def containsDuplicate(self, nums: List[int]) -> bool:"),
            (3, "        seen = set()"),
            (4, "        for n in nums:"),
            (5, "            if n in seen:"),
            (6, "                return True"),
            (7, "            seen.add(n)"),
            (8, "        return False"),
        ],
        (Problem::ContainsDuplicate, 1) => vec![
            (1, "class Solution:"),
            (2, "    def containsDuplicate(self, nums: List[int]) -> bool:"),
            (3, "        nums.sort()"),
            (4, "        for i in range(1, len(nums)):"),
            (5, "            if nums[i] == nums[i - 1]:"),
            (6, "                return True"),
            (7, "        return False"),
        ],

        (Problem::TwoSum, 0) => vec![
            (1, "class Solution:"),
            (2, "    def twoSum(self, nums: List[int], target: int) -> List[int]:"),
            (3, "        prevMap = {} # val -> index"),
            (4, "        for i, n in enumerate(nums):"),
            (5, "            diff = target - n"),
            (6, "            if diff in prevMap:"),
            (7, "                return [prevMap[diff], i]"),
            (8, "            prevMap[n] = i"),
            (9, "        return []"),
        ],
        (Problem::TwoSum, 1) => vec![
            (1, "class Solution:"),
            (2, "    def twoSum(self, nums: List[int], target: int) -> List[int]:"),
            (3, "        n = len(nums)"),
            (4, "        for i in range(n):"),
            (5, "            for j in range(i + 1, n):"),
            (6, "                if nums[i] + nums[j] == target:"),
            (7, "                    return [i, j]"),
            (8, "        return []"),
        ],

        (Problem::ValidAnagram, 0) => vec![
            (1, "class Solution:"),
            (2, "    def isAnagram(self, s: str, t: str) -> bool:"),
            (3, "        if len(s) != len(t):"),
            (4, "            return False"),
            (5, "        countS, countT = {}, {}"),
            (6, "        for i in range(len(s)):"),
            (7, "            countS[s[i]] = 1 + countS.get(s[i], 0)"),
            (8, "            countT[t[i]] = 1 + countT.get(t[i], 0)"),
            (9, "        return countS == countT"),
        ],
        (Problem::ValidAnagram, 1) => vec![
            (1, "class Solution:"),
            (2, "    def isAnagram(self, s: str, t: str) -> bool:"),
            (3, "        if len(s) != len(t):"),
            (4, "            return False"),
            (5, "        return sorted(s) == sorted(t)"),
        ],

        (Problem::GroupAnagrams, 0) => vec![
            (1, "class Solution:"),
            (2, "    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:"),
            (3, "        res = defaultdict(list)"),
            (4, "        for s in strs:"),
            (5, "            count = [0] * 26"),
            (6, "            for c in s:"),
            (7, "                count[ord(c) - ord('a')] += 1"),
            (8, "            res[tuple(count)].append(s)"),
            (9, "        return list(res.values())"),
        ],
        (Problem::GroupAnagrams, 1) => vec![
            (1, "class Solution:"),
            (2, "    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:"),
            (3, "        res = defaultdict(list)"),
            (4, "        for s in strs:"),
            (5, "            key = \"\".join(sorted(s))"),
            (6, "            res[key].append(s)"),
            (7, "        return list(res.values())"),
        ],

        (Problem::TopKFrequent, 0) => crate::model::topk_code_lines(),
        (Problem::TopKFrequent, 1) => vec![
            (1, "class Solution:"),
            (2, "    def topKFrequent(self, nums, k):"),
            (3, "        count = Counter(nums)"),
            (4, "        heap = []"),
            (5, "        for num, freq in count.items():"),
            (6, "            heappush(heap, (freq, num))"),
            (7, "            if len(heap) > k:"),
            (8, "                heappop(heap)"),
            (9, "        return [num for freq, num in heap]"),
        ],
        (Problem::TopKFrequent, 2) => vec![
            (1, "class Solution:"),
            (2, "    def topKFrequent(self, nums, k):"),
            (3, "        count = Counter(nums)"),
            (4, "        arr = [(cnt, num) for num, cnt in count.items()]"),
            (5, "        arr.sort(reverse=True)"),
            (6, "        return [num for cnt, num in arr[:k]]"),
        ],

        (Problem::ProductExceptSelf, _) => crate::model::product_code_lines(),
        (Problem::EncodeDecode, _) => crate::model::encode_decode_code_lines(),

        (Problem::ValidSudoku, _) => vec![
            (1, "class Solution:"),
            (2, "    def isValidSudoku(self, board: List[List[str]]) -> bool:"),
            (3, "        cols = defaultdict(set)"),
            (4, "        rows = defaultdict(set)"),
            (5, "        squares = defaultdict(set) # key = (r//3, c//3)"),
            (6, "        for r in range(9):"),
            (7, "            for c in range(9):"),
            (8, "                if board[r][c] == \".\": continue"),
            (9, "                val = board[r][c]"),
            (10, "                if (val in rows[r] or val in cols[c] or val in squares[(r//3, c//3)]):"),
            (11, "                    return False"),
            (12, "                rows[r].add(val); cols[c].add(val); squares[(r//3, c//3)].add(val)"),
            (13, "        return True"),
        ],

        (Problem::LongestConsecutive, _) => vec![
            (1, "class Solution:"),
            (2, "    def longestConsecutive(self, nums: List[int]) -> int:"),
            (3, "        numSet = set(nums)"),
            (4, "        longest = 0"),
            (5, "        for n in numSet:"),
            (6, "            # check if it is the start of a sequence"),
            (7, "            if (n - 1) not in numSet:"),
            (8, "                length = 1"),
            (9, "                while (n + length) in numSet:"),
            (10, "                    length += 1"),
            (11, "                longest = max(longest, length)"),
            (12, "        return longest"),
        ],

        (Problem::ValidPalindrome, 0) => vec![
            (1, "class Solution:"),
            (2, "    def isPalindrome(self, s: str) -> bool:"),
            (3, "        l, r = 0, len(s) - 1"),
            (4, "        while l < r:"),
            (5, "            while l < r and not s[l].isalnum():"),
            (6, "                l += 1"),
            (7, "            while r > l and not s[r].isalnum():"),
            (8, "                r -= 1"),
            (9, "            if s[l].lower() != s[r].lower():"),
            (10, "                return False"),
            (11, "            l, r = l + 1, r - 1"),
            (12, "        return True"),
        ],
        (Problem::ValidPalindrome, 1) => vec![
            (1, "class Solution:"),
            (2, "    def isPalindrome(self, s: str) -> bool:"),
            (3, "        newStr = \"\""),
            (4, "        for c in s:"),
            (5, "            if c.isalnum():"),
            (6, "                newStr += c.lower()"),
            (7, "        return newStr == newStr[::-1]"),
        ],

        (Problem::BestTimeStock, _) => vec![
            (1, "class Solution:"),
            (2, "    def maxProfit(self, prices: List[int]) -> int:"),
            (3, "        l, r = 0, 1 # buy=l, sell=r"),
            (4, "        maxP = 0"),
            (5, "        while r < len(prices):"),
            (6, "            if prices[l] < prices[r]:"),
            (7, "                profit = prices[r] - prices[l]"),
            (8, "                maxP = max(maxP, profit)"),
            (9, "            else:"),
            (10, "                l = r"),
            (11, "            r += 1"),
            (12, "        return maxP"),
        ],

        (Problem::ValidParentheses, _) => vec![
            (1, "class Solution:"),
            (2, "    def isValid(self, s: str) -> bool:"),
            (3, "        stack = []"),
            (4, "        closeToOpen = {\")\": \"(\", \"]\": \"[\", \"}\": \"{\"}"),
            (5, "        for c in s:"),
            (6, "            if c in closeToOpen:"),
            (7, "                if stack and stack[-1] == closeToOpen[c]:"),
            (8, "                    stack.pop()"),
            (9, "                else:"),
            (10, "                    return False"),
            (11, "            else:"),
            (12, "                stack.append(c)"),
            (13, "        return True if not stack else False"),
        ],

        (Problem::BinarySearch, _) => vec![
            (1, "class Solution:"),
            (2, "    def search(self, nums: List[int], target: int) -> int:"),
            (3, "        l, r = 0, len(nums) - 1"),
            (4, "        while l <= r:"),
            (5, "            m = l + ((r - l) // 2)"),
            (6, "            if nums[m] > target:"),
            (7, "                r = m - 1"),
            (8, "            elif nums[m] < target:"),
            (9, "                l = m + 1"),
            (10, "            else:"),
            (11, "                return m"),
            (12, "        return -1"),
        ],

        (Problem::ReverseLinkedList, _) => vec![
            (1, "class Solution:"),
            (2, "    def reverseList(self, head: Optional[ListNode]) -> Optional[ListNode]:"),
            (3, "        prev, curr = None, head"),
            (4, "        while curr:"),
            (5, "            nxt = curr.next"),
            (6, "            curr.next = prev"),
            (7, "            prev = curr"),
            (8, "            curr = nxt"),
            (9, "        return prev"),
        ],

        (Problem::MergeTwoLists, _) => vec![
            (1, "class Solution:"),
            (2, "    def mergeTwoLists(self, list1: Optional[ListNode], list2: Optional[ListNode]) -> Optional[ListNode]:"),
            (3, "        dummy = ListNode()"),
            (4, "        tail = dummy"),
            (5, "        while list1 and list2:"),
            (6, "            if list1.val < list2.val:"),
            (7, "                tail.next = list1; list1 = list1.next"),
            (8, "            else:"),
            (9, "                tail.next = list2; list2 = list2.next"),
            (10, "            tail = tail.next"),
            (11, "        tail.next = list1 if list1 else list2"),
            (12, "        return dummy.next"),
        ],

        (Problem::LinkedListCycle, _) => vec![
            (1, "class Solution:"),
            (2, "    def hasCycle(self, head: Optional[ListNode]) -> bool:"),
            (3, "        slow, fast = head, head"),
            (4, "        while fast and fast.next:"),
            (5, "            slow = slow.next"),
            (6, "            fast = fast.next.next"),
            (7, "            if slow == fast:"),
            (8, "                return True"),
            (9, "        return False"),
        ],

        (Problem::InvertTree, _) => vec![
            (1, "class Solution:"),
            (2, "    def invertTree(self, root: Optional[TreeNode]) -> Optional[TreeNode]:"),
            (3, "        if not root:"),
            (4, "            return None"),
            (5, "        tmp = root.left"),
            (6, "        root.left = root.right"),
            (7, "        root.right = tmp"),
            (8, "        self.invertTree(root.left)"),
            (9, "        self.invertTree(root.right)"),
            (10, "        return root"),
        ],

        (Problem::MaxDepthTree, _) => vec![
            (1, "class Solution:"),
            (2, "    def maxDepth(self, root: Optional[TreeNode]) -> int:"),
            (3, "        if not root:"),
            (4, "            return 0"),
            (5, "        left_depth = self.maxDepth(root.left)"),
            (6, "        right_depth = self.maxDepth(root.right)"),
            (7, "        return 1 + max(left_depth, right_depth)"),
        ],

        (Problem::DiameterTree, _) => vec![
            (1, "class Solution:"),
            (2, "    def diameterOfBinaryTree(self, root: Optional[TreeNode]) -> int:"),
            (3, "        res = 0"),
            (4, "        def dfs(curr):"),
            (5, "            nonlocal res"),
            (6, "            if not curr: return 0"),
            (7, "            left = dfs(curr.left)"),
            (8, "            right = dfs(curr.right)"),
            (9, "            res = max(res, left + right)"),
            (10, "            return 1 + max(left, right)"),
            (11, "        dfs(root)"),
            (12, "        return res"),
        ],

        _ => vec![(1, "# Approach implementation trace")],
    }
}

pub fn topk_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def topKFrequent(self, nums, k):"),
        (3, "        count = {}"),
        (4, "        freq = [[] for i in range(len(nums) + 1)]"),
        (5, ""),
        (6, "        for n in nums:"),
        (7, "            count[n] = 1 + count.get(n, 0)"),
        (8, "        for n, c in count.items():"),
        (9, "            freq[c].append(n)"),
        (10, ""),
        (11, "        res = []"),
        (12, "        for i in range(len(freq) - 1, 0, -1):"),
        (13, "            for n in freq[i]:"),
        (14, "                res.append(n)"),
        (15, "                if len(res) == k:"),
        (16, "                    return res"),
    ]
}

pub fn encode_decode_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def encode(self, strs):"),
        (3, "        res = \"\""),
        (4, "        for s in strs:"),
        (5, "            res += str(len(s)) + \"#\" + s"),
        (6, "        return res"),
        (7, ""),
        (8, "    def decode(self, s):"),
        (9, "        res = []"),
        (10, "        i = 0"),
        (11, "        while i < len(s):"),
        (12, "            j = i"),
        (13, "            while s[j] != \"#\":"),
        (14, "                j += 1"),
        (15, "            length = int(s[i:j])"),
        (16, "            res.append(s[j+1 : j+1+length])"),
        (17, "            i = j + 1 + length"),
        (18, "        return res"),
    ]
}

pub fn product_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def productExceptSelf(self, nums):"),
        (3, "        n = len(nums)"),
        (4, "        output = [1] * n"),
        (5, ""),
        (6, "        prefix = 1"),
        (7, "        for i in range(n):"),
        (8, "            output[i] = prefix"),
        (9, "            prefix *= nums[i]"),
        (10, ""),
        (11, "        suffix = 1"),
        (12, "        for i in range(n - 1, -1, -1):"),
        (13, "            output[i] *= suffix"),
        (14, "            suffix *= nums[i]"),
        (15, ""),
        (16, "        return output"),
    ]
}
