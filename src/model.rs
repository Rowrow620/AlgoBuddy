use std::collections::BTreeMap;

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
    TwoSum,
    ValidAnagram,
    TopKFrequent,
    ProductExceptSelf,
    EncodeDecode,
    ValidPalindrome,
    ValidParentheses,
}

impl Problem {
    pub fn all() -> &'static [Problem] {
        &[
            Problem::TwoSum,
            Problem::ValidAnagram,
            Problem::TopKFrequent,
            Problem::ProductExceptSelf,
            Problem::EncodeDecode,
            Problem::ValidPalindrome,
            Problem::ValidParentheses,
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
            Problem::TwoSum => ProblemDetails {
                id: 1,
                title: "Two Sum",
                difficulty: Difficulty::Easy,
                category: Category::ArraysAndHashing,
                statement: "Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.\n\nYou may assume that each input would have exactly one solution, and you may not use the same element twice.",
                examples: &[
                    Example {
                        input: "nums = [2, 7, 11, 15], target = 9",
                        output: "[0, 1]",
                        explanation: "Because nums[0] + nums[1] == 9, we return [0, 1].",
                    },
                    Example {
                        input: "nums = [3, 2, 4], target = 6",
                        output: "[1, 2]",
                        explanation: "Because nums[1] + nums[2] == 6, we return [1, 2].",
                    },
                ],
                constraints: &[
                    "2 <= nums.length <= 10^4",
                    "-10^9 <= nums[i] <= 10^9",
                    "-10^9 <= target <= 10^9",
                    "Only one valid answer exists.",
                ],
                leetcode_url: "https://leetcode.com/problems/two-sum/",
                approaches: &[
                    ApproachMeta {
                        id: 0,
                        name: "Hash Map (One Pass)",
                        time_complexity: "O(N)",
                        space_complexity: "O(N)",
                        description: "Use a hash map to look up target - num in O(1) time as we iterate through the array.",
                    },
                    ApproachMeta {
                        id: 1,
                        name: "Brute Force",
                        time_complexity: "O(N²)",
                        space_complexity: "O(1)",
                        description: "Check all pairs (i, j) using two nested loops to see if nums[i] + nums[j] == target.",
                    },
                ],
            },
            Problem::ValidAnagram => ProblemDetails {
                id: 242,
                title: "Valid Anagram",
                difficulty: Difficulty::Easy,
                category: Category::ArraysAndHashing,
                statement: "Given two strings s and t, return true if t is an anagram of s, and false otherwise.\n\nAn Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.",
                examples: &[
                    Example {
                        input: "s = \"anagram\", t = \"nagaram\"",
                        output: "true",
                        explanation: "Both strings contain 3 'a's, 1 'n', 1 'g', 1 'r', and 1 'm'.",
                    },
                    Example {
                        input: "s = \"rat\", t = \"car\"",
                        output: "false",
                        explanation: "'rat' has 't' while 'car' has 'c'.",
                    },
                ],
                constraints: &[
                    "1 <= s.length, t.length <= 5 * 10^4",
                    "s and t consist of lowercase English letters.",
                ],
                leetcode_url: "https://leetcode.com/problems/valid-anagram/",
                approaches: &[
                    ApproachMeta {
                        id: 0,
                        name: "Frequency Counter Array",
                        time_complexity: "O(N)",
                        space_complexity: "O(1)",
                        description: "Count character frequencies in fixed size 26 arrays and check for equality.",
                    },
                    ApproachMeta {
                        id: 1,
                        name: "Sort Strings",
                        time_complexity: "O(N log N)",
                        space_complexity: "O(N)",
                        description: "Sort characters of both strings and compare if the sorted strings are identical.",
                    },
                ],
            },
            Problem::TopKFrequent => ProblemDetails {
                id: 347,
                title: "Top K Frequent Elements",
                difficulty: Difficulty::Medium,
                category: Category::ArraysAndHashing,
                statement: "Given an integer array nums and an integer k, return the k most frequent elements. You may return the answer in any order.",
                examples: &[
                    Example {
                        input: "nums = [1,1,1,2,2,3], k = 2",
                        output: "[1, 2]",
                        explanation: "1 appears 3 times, 2 appears 2 times, 3 appears 1 time. The 2 most frequent are [1, 2].",
                    },
                    Example {
                        input: "nums = [1], k = 1",
                        output: "[1]",
                        explanation: "1 is the only element.",
                    },
                ],
                constraints: &[
                    "1 <= nums.length <= 10^5",
                    "-10^4 <= nums[i] <= 10^4",
                    "k is in the range [1, number of unique elements in the array].",
                    "It is guaranteed that the answer is unique.",
                ],
                leetcode_url: "https://leetcode.com/problems/top-k-frequent-elements/",
                approaches: &[
                    ApproachMeta {
                        id: 0,
                        name: "Bucket Sort",
                        time_complexity: "O(N)",
                        space_complexity: "O(N)",
                        description: "Use array indices (0..N) as frequency buckets to collect elements without sorting.",
                    },
                    ApproachMeta {
                        id: 1,
                        name: "Min-Heap",
                        time_complexity: "O(N log k)",
                        space_complexity: "O(N)",
                        description: "Maintain a min-heap of size k storing (frequency, num) pairs.",
                    },
                    ApproachMeta {
                        id: 2,
                        name: "Sorting Pairs",
                        time_complexity: "O(N log N)",
                        space_complexity: "O(N)",
                        description: "Count frequencies in a hash map, convert to a list of pairs, and sort by frequency.",
                    },
                ],
            },
            Problem::ProductExceptSelf => ProblemDetails {
                id: 238,
                title: "Product of Array Except Self",
                difficulty: Difficulty::Medium,
                category: Category::ArraysAndHashing,
                statement: "Given an integer array nums, return an array answer such that answer[i] is equal to the product of all the elements of nums except nums[i].\n\nThe product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer. You must write an algorithm that runs in O(N) time and without using the division operation.",
                examples: &[
                    Example {
                        input: "nums = [1, 2, 3, 4]",
                        output: "[24, 12, 8, 6]",
                        explanation: "answer[0] = 2*3*4=24, answer[1] = 1*3*4=12, answer[2] = 1*2*4=8, answer[3] = 1*2*3=6.",
                    },
                    Example {
                        input: "nums = [-1, 1, 0, -3, 3]",
                        output: "[0, 0, 9, 0, 0]",
                        explanation: "The element 0 zeroes out all other positions except when excluding itself.",
                    },
                ],
                constraints: &[
                    "2 <= nums.length <= 10^5",
                    "-30 <= nums[i] <= 30",
                    "Do not use the division operator.",
                ],
                leetcode_url: "https://leetcode.com/problems/product-of-array-except-self/",
                approaches: &[
                    ApproachMeta {
                        id: 0,
                        name: "Prefix & Suffix Pass",
                        time_complexity: "O(N)",
                        space_complexity: "O(1)",
                        description: "Compute prefix products in output array, then multiply by running suffix products in reverse.",
                    },
                ],
            },
            Problem::EncodeDecode => ProblemDetails {
                id: 271,
                title: "Encode and Decode Strings",
                difficulty: Difficulty::Medium,
                category: Category::ArraysAndHashing,
                statement: "Design an algorithm to encode a list of strings to a string. The encoded string is then sent over the network and decoded back to the original list of strings.",
                examples: &[
                    Example {
                        input: "strs = [\"Hello\", \"World\"]",
                        output: "[\"Hello\", \"World\"]",
                        explanation: "Encoded into \"5#Hello5#World\", then decoded back to original strings.",
                    },
                ],
                constraints: &[
                    "1 <= strs.length <= 200",
                    "0 <= strs[i].length <= 200",
                    "strs[i] consists of any possible UTF-8 characters.",
                ],
                leetcode_url: "https://leetcode.com/problems/encode-and-decode-strings/",
                approaches: &[
                    ApproachMeta {
                        id: 0,
                        name: "Length Prefix (# Protocol)",
                        time_complexity: "O(N)",
                        space_complexity: "O(N)",
                        description: "Prefix each string with its length and a '#' delimiter: len#string.",
                    },
                ],
            },
            Problem::ValidPalindrome => ProblemDetails {
                id: 125,
                title: "Valid Palindrome",
                difficulty: Difficulty::Easy,
                category: Category::TwoPointers,
                statement: "A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward.\n\nGiven a string s, return true if it is a palindrome, or false otherwise.",
                examples: &[
                    Example {
                        input: "s = \"A man, a plan, a canal: Panama\"",
                        output: "true",
                        explanation: "\"amanaplanacanalpanama\" is a palindrome.",
                    },
                    Example {
                        input: "s = \"race a car\"",
                        output: "false",
                        explanation: "\"raceacar\" is not a palindrome.",
                    },
                ],
                constraints: &[
                    "1 <= s.length <= 2 * 10^5",
                    "s consists only of printable ASCII characters.",
                ],
                leetcode_url: "https://leetcode.com/problems/valid-palindrome/",
                approaches: &[
                    ApproachMeta {
                        id: 0,
                        name: "Two Pointers (In-Place)",
                        time_complexity: "O(N)",
                        space_complexity: "O(1)",
                        description: "Use left and right pointers converging inward, skipping non-alphanumeric characters.",
                    },
                    ApproachMeta {
                        id: 1,
                        name: "Reverse Filtered String",
                        time_complexity: "O(N)",
                        space_complexity: "O(N)",
                        description: "Build a new filtered lowercase alphanumeric string and compare it to its reverse.",
                    },
                ],
            },
            Problem::ValidParentheses => ProblemDetails {
                id: 20,
                title: "Valid Parentheses",
                difficulty: Difficulty::Easy,
                category: Category::Stack,
                statement: "Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.\n\nAn input string is valid if open brackets are closed by the same type of brackets, and open brackets are closed in the correct order.",
                examples: &[
                    Example {
                        input: "s = \"()[]{}\"",
                        output: "true",
                        explanation: "Every open bracket is closed by matching type.",
                    },
                    Example {
                        input: "s = \"(]\"",
                        output: "false",
                        explanation: "Open '(' is closed by mismatched ']'.",
                    },
                ],
                constraints: &[
                    "1 <= s.length <= 10^4",
                    "s consists of parentheses only '()[]{}'.",
                ],
                leetcode_url: "https://leetcode.com/problems/valid-parentheses/",
                approaches: &[
                    ApproachMeta {
                        id: 0,
                        name: "Stack Matching",
                        time_complexity: "O(N)",
                        space_complexity: "O(N)",
                        description: "Push opening brackets to stack; pop and match when encountering closing brackets.",
                    },
                ],
            },
        }
    }
}

// ── Visual State Variants per Problem Type ──

#[derive(Debug, Clone)]
pub enum VisualState {
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
        // Two Sum
        (Problem::TwoSum, 0) => vec![ // Hash Map One Pass
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
        (Problem::TwoSum, 1) => vec![ // Brute Force O(N^2)
            (1, "class Solution:"),
            (2, "    def twoSum(self, nums: List[int], target: int) -> List[int]:"),
            (3, "        n = len(nums)"),
            (4, "        for i in range(n):"),
            (5, "            for j in range(i + 1, n):"),
            (6, "                if nums[i] + nums[j] == target:"),
            (7, "                    return [i, j]"),
            (8, "        return []"),
        ],

        // Valid Anagram
        (Problem::ValidAnagram, 0) => vec![ // Counter Array O(N)
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
        (Problem::ValidAnagram, 1) => vec![ // Sort Strings O(N log N)
            (1, "class Solution:"),
            (2, "    def isAnagram(self, s: str, t: str) -> bool:"),
            (3, "        if len(s) != len(t):"),
            (4, "            return False"),
            (5, "        return sorted(s) == sorted(t)"),
        ],

        // Top K Frequent
        (Problem::TopKFrequent, 0) => crate::model::topk_code_lines(),
        (Problem::TopKFrequent, 1) => vec![ // Min-Heap
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
        (Problem::TopKFrequent, 2) => vec![ // Sorting
            (1, "class Solution:"),
            (2, "    def topKFrequent(self, nums, k):"),
            (3, "        count = Counter(nums)"),
            (4, "        arr = [(cnt, num) for num, cnt in count.items()]"),
            (5, "        arr.sort(reverse=True)"),
            (6, "        return [num for cnt, num in arr[:k]]"),
        ],

        // Product Except Self
        (Problem::ProductExceptSelf, _) => crate::model::product_code_lines(),

        // Encode / Decode
        (Problem::EncodeDecode, _) => crate::model::encode_decode_code_lines(),

        // Valid Palindrome
        (Problem::ValidPalindrome, 0) => vec![ // Two Pointers
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
        (Problem::ValidPalindrome, 1) => vec![ // Reverse Filtered String
            (1, "class Solution:"),
            (2, "    def isPalindrome(self, s: str) -> bool:"),
            (3, "        newStr = \"\""),
            (4, "        for c in s:"),
            (5, "            if c.isalnum():"),
            (6, "                newStr += c.lower()"),
            (7, "        return newStr == newStr[::-1]"),
        ],

        // Valid Parentheses
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
