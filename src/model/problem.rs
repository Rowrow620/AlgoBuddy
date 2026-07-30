use super::taxonomy::*;

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
    pub rationale: &'static str,
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

// ── Problem Enum (34 Problems) ──

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
    BalancedTree,
    SameTree,
    Subtree,
    ClimbingStairs,
    MinCostStairs,
    KthLargestStream,
    LastStone,
    MeetingRooms,
    HappyNumber,
    PlusOne,
    SingleNumber,
    CountBits,
    CountingBits,
    ReverseBits,
    MissingNumber,
    TwoSumII,
    ThreeSum,
    ContainerWater,
    TrappingRain,
    MinStack,
    EvalRPN,
    LongestSubstring,
    Search2DMatrix,
    HouseRobber,
    GenerateParentheses,
    DailyTemperatures,
    CarFleet,
    LargestRectangle,
    CharacterReplacement,
    PermutationInString,
    MinWindowSubstring,
    SlidingWindowMax,
    SearchRotatedArray,
    FindMinRotated,
    TimeKeyValueStore,
    FindMedianSortedArrays,
    KokoEatingBananas,
    ImplementTrie,
    WordDictionary,
    WordSearchII,
    Subsets,
    Permutations,
    KClosestPoints,
    TaskScheduler,
    FindMedianDataStream,
    CombinationSum,
    SubsetsII,
    CombinationSumII,
    WordSearch,
    NQueens,
    KthLargestArray,
    DesignTwitter,
    PalindromePartitioning,
    LetterCombinations,
    HouseRobberII,
    LongestPalindromicSubstring,
    PalindromicSubstrings,
    DecodeWays,
    CoinChange,
    MaxProductSubarray,
    WordBreak,
    LongestIncreasingSubsequence,
    PartitionEqualSubsetSum,
    Number1Bits,
    SumTwoIntegers,
    ReverseInteger,
    RotateImage,
    SpiralMatrix,
    SetMatrixZeroes,
    PowXN,
    MultiplyStrings,
    DetectSquares,
    MaximumSubarray,
    JumpGame,
    JumpGameII,
    GasStation,
    HandOfStraights,
    MergeTriplets,
    PartitionLabels,
    ValidParenthesisString,
    InsertInterval,
    MergeIntervals,
    NonOverlappingIntervals,
    MeetingRoomsII,
    MinIntervalQuery,
    NumberIslands,
    MaxAreaIsland,
    CloneGraph,
    WallsAndGates,
    RottingOranges,
    PacificAtlantic,
    SurroundedRegions,
    CourseSchedule,
    CourseScheduleII,
    GraphValidTree,
    ConnectedComponents,
    RedundantConnection,
    WordLadder,
    UniquePaths,
    LongestCommonSubsequence,
    BestTimeStockCooldown,
    CoinChangeII,
    TargetSum,
    InterleavingString,
    LongestIncreasingPath,
    DistinctSubsequences,
    EditDistance,
    BurstBalloons,
    RegularExpressionMatching,
    ReconstructItinerary,
    MinCostConnectPoints,
    NetworkDelayTime,
    SwimInRisingWater,
    AlienDictionary,
    CheapestFlights,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AuditStatus {
    Audited,
    Unaudited,
}

impl Problem {
    pub fn audit_status(&self) -> AuditStatus {
        match self {
            Problem::ContainsDuplicate | Problem::TwoSum | Problem::ValidAnagram => {
                AuditStatus::Audited
            }
            _ => AuditStatus::Unaudited,
        }
    }

    pub fn is_audited(&self) -> bool {
        self.audit_status() == AuditStatus::Audited
    }

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
            Problem::BalancedTree,
            Problem::SameTree,
            Problem::Subtree,
            Problem::ImplementTrie,
            Problem::WordDictionary,
            Problem::WordSearchII,
            Problem::Subsets,
            Problem::Permutations,
            Problem::KClosestPoints,
            Problem::TaskScheduler,
            Problem::FindMedianDataStream,
            Problem::CombinationSum,
            Problem::SubsetsII,
            Problem::CombinationSumII,
            Problem::WordSearch,
            Problem::NQueens,
            Problem::KthLargestArray,
            Problem::DesignTwitter,
            Problem::PalindromePartitioning,
            Problem::LetterCombinations,
            Problem::ClimbingStairs,
            Problem::MinCostStairs,
            Problem::HouseRobber,
            Problem::HouseRobberII,
            Problem::LongestPalindromicSubstring,
            Problem::PalindromicSubstrings,
            Problem::DecodeWays,
            Problem::CoinChange,
            Problem::MaxProductSubarray,
            Problem::WordBreak,
            Problem::LongestIncreasingSubsequence,
            Problem::PartitionEqualSubsetSum,
            Problem::KthLargestStream,
            Problem::LastStone,
            Problem::MeetingRooms,
            Problem::HappyNumber,
            Problem::PlusOne,
            Problem::SingleNumber,
            Problem::CountBits,
            Problem::CountingBits,
            Problem::ReverseBits,
            Problem::MissingNumber,
            Problem::Number1Bits,
            Problem::SumTwoIntegers,
            Problem::ReverseInteger,
            Problem::RotateImage,
            Problem::SpiralMatrix,
            Problem::SetMatrixZeroes,
            Problem::PowXN,
            Problem::MultiplyStrings,
            Problem::DetectSquares,
            Problem::MaximumSubarray,
            Problem::JumpGame,
            Problem::JumpGameII,
            Problem::GasStation,
            Problem::HandOfStraights,
            Problem::MergeTriplets,
            Problem::PartitionLabels,
            Problem::ValidParenthesisString,
            Problem::InsertInterval,
            Problem::MergeIntervals,
            Problem::NonOverlappingIntervals,
            Problem::MeetingRoomsII,
            Problem::MinIntervalQuery,
            Problem::NumberIslands,
            Problem::MaxAreaIsland,
            Problem::CloneGraph,
            Problem::WallsAndGates,
            Problem::RottingOranges,
            Problem::PacificAtlantic,
            Problem::SurroundedRegions,
            Problem::CourseSchedule,
            Problem::CourseScheduleII,
            Problem::GraphValidTree,
            Problem::ConnectedComponents,
            Problem::RedundantConnection,
            Problem::WordLadder,
            Problem::UniquePaths,
            Problem::LongestCommonSubsequence,
            Problem::BestTimeStockCooldown,
            Problem::CoinChangeII,
            Problem::TargetSum,
            Problem::InterleavingString,
            Problem::LongestIncreasingPath,
            Problem::DistinctSubsequences,
            Problem::EditDistance,
            Problem::BurstBalloons,
            Problem::RegularExpressionMatching,
            Problem::ReconstructItinerary,
            Problem::MinCostConnectPoints,
            Problem::NetworkDelayTime,
            Problem::SwimInRisingWater,
            Problem::AlienDictionary,
            Problem::CheapestFlights,
            Problem::TwoSumII,
            Problem::ThreeSum,
            Problem::ContainerWater,
            Problem::TrappingRain,
            Problem::MinStack,
            Problem::EvalRPN,
            Problem::LongestSubstring,
            Problem::Search2DMatrix,
            Problem::HouseRobber,
            Problem::GenerateParentheses,
            Problem::DailyTemperatures,
            Problem::CarFleet,
            Problem::LargestRectangle,
            Problem::CharacterReplacement,
            Problem::PermutationInString,
            Problem::MinWindowSubstring,
            Problem::SlidingWindowMax,
            Problem::SearchRotatedArray,
            Problem::FindMinRotated,
            Problem::TimeKeyValueStore,
            Problem::FindMedianSortedArrays,
            Problem::KokoEatingBananas,
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

    pub fn formula(&self) -> Option<&'static str> {
        match self {
            Problem::ContainsDuplicate => Some("if n ∈ seen ➔ duplicate found"),
            Problem::TwoSum => Some("diff = target - num  ➔  map[diff]"),
            Problem::ValidAnagram => Some("count_S[char] == count_T[char]"),
            Problem::GroupAnagrams => Some("key = tuple(count_26_chars)"),
            Problem::TopKFrequent => Some("freq[num]++ ➔ bucket[cnt].append(num)"),
            Problem::ProductExceptSelf => Some("out[i] = prefix[i-1] × suffix[i+1]"),
            Problem::EncodeDecode => Some("encoded = str.len() + '#' + str"),
            Problem::ValidSudoku => Some("val ∉ row[r] ∩ col[c] ∩ box[r//3, c//3]"),
            Problem::LongestConsecutive => Some("if (num - 1) ∉ set ➔ streak start"),
            Problem::ValidPalindrome => Some("s[left].lower() == s[right].lower()"),
            Problem::BestTimeStock => Some("profit = max(0, prices[r] - prices[l])"),
            Problem::ValidParentheses => Some("open ➔ push(c), close ➔ pop() == match(c)"),
            Problem::BinarySearch => Some("mid = left + (right - left) // 2"),
            Problem::ReverseLinkedList => Some("curr.next = prev; prev = curr; curr = nxt"),
            Problem::MergeTwoLists => Some("tail.next = min(l1.val, l2.val)"),
            Problem::LinkedListCycle => Some("slow = slow.next; fast = fast.next.next"),
            Problem::InvertTree => Some("swap(root.left, root.right)"),
            Problem::MaxDepthTree => Some("depth = 1 + max(depth(L), depth(R))"),
            Problem::DiameterTree => Some("diameter = max(d, height(L) + height(R))"),
            Problem::BalancedTree => Some("abs(height(L) - height(R)) ≤ 1"),
            Problem::SameTree => Some("p.val == q.val ∧ same(p.L, q.L) ∧ same(p.R, q.R)"),
            Problem::Subtree => {
                Some("isSameTree(root, subRoot) ∨ isSubtree(root.L) ∨ isSubtree(root.R)")
            }
            Problem::ClimbingStairs => Some("dp[i] = dp[i-1] + dp[i-2]"),
            Problem::MinCostStairs => Some("dp[i] = cost[i] + min(dp[i+1], dp[i+2])"),
            Problem::KthLargestStream => Some("heapq.heappush(val); if len > k ➔ pop()"),
            Problem::LastStone => Some("stone_1 - stone_2 ➔ max_heap"),
            Problem::MeetingRooms => Some("interval[i].start < interval[i-1].end"),
            Problem::HappyNumber => Some("n = ∑(digit²)  ➔  detect cycle in HashSet"),
            Problem::PlusOne => Some("digits[i] = (digits[i] + 1) % 10"),
            Problem::SingleNumber => Some("a ⊕ a = 0  ➔  res ^= n"),
            Problem::CountBits | Problem::Number1Bits => {
                Some("n = n & (n - 1)  ➔  clears lowest set bit")
            }
            Problem::CountingBits => Some("dp[i] = 1 + dp[i - offset]"),
            Problem::ReverseBits => Some("bit = (n >> i) & 1  ➔  res |= bit << (31 - i)"),
            Problem::MissingNumber => Some("∑(0..n) - ∑(nums)  ➔  res ^= i ⊕ nums[i]"),
            Problem::TwoSumII => Some("sum = nums[l] + nums[r]  ➔  adjust l, r"),
            Problem::ThreeSum => Some("a + nums[l] + nums[r] == 0"),
            Problem::ContainerWater => Some("area = (r - l) × min(h[l], h[r])"),
            Problem::TrappingRain => Some("water[i] = min(max_L, max_R) - height[i]"),
            Problem::MinStack => Some("min_stack.push(min(val, min_stack[-1]))"),
            Problem::EvalRPN => Some("b = pop(), a = pop() ➔ push(a op b)"),
            Problem::LongestSubstring => Some("while s[r] ∈ set ➔ set.remove(s[l]); l++"),
            Problem::Search2DMatrix => Some("val = matrix[m // COLS][m % COLS]"),
            Problem::HouseRobber => Some("rob[i] = max(rob[i-1], rob[i-2] + nums[i])"),
            Problem::GenerateParentheses => Some("open < n ➔ '(', closed < open ➔ ')'"),
            Problem::DailyTemperatures => Some("while t > stack[-1].val ➔ pop() & dist = i - idx"),
            Problem::CarFleet => Some("time = (target - p) / s; if t ≤ prev ➔ fleet merge"),
            Problem::LargestRectangle => Some("area = height × (i - start_index)"),
            Problem::CharacterReplacement => Some("window_len - max_freq ≤ k"),
            Problem::PermutationInString => Some("s1_count == s2_window_count"),
            Problem::MinWindowSubstring => Some("have == need ➔ shrink left window"),
            Problem::SlidingWindowMax => Some("deque monotonic decreasing indices"),
            Problem::SearchRotatedArray => Some("if nums[l] ≤ nums[m] ➔ left half sorted"),
            Problem::FindMinRotated => Some("if nums[m] > nums[r] ➔ min in right half"),
            Problem::TimeKeyValueStore => Some("binary search timestamp in key values"),
            Problem::FindMedianSortedArrays => Some("Aleft ≤ Bright ∧ Bleft ≤ Aright"),
            Problem::KokoEatingBananas => Some("hours <= h ➔ r = k - 1 else l = k + 1"),
            Problem::RotateImage => Some("transpose(matrix) ➔ reverse_rows(matrix)"),
            Problem::SpiralMatrix => Some("traverse(right, down, left, up) ➔ shrink bounds"),
            Problem::SetMatrixZeroes => Some("if cell == 0 ➔ mark row_flag[r] & col_flag[c]"),
            Problem::PowXN => Some("if N is odd ➔ res *= x; x *= x; N //= 2"),
            Problem::MultiplyStrings => Some("pos[i + j + 1] += d1 × d2 ➔ handle carry"),
            Problem::DetectSquares => Some("count += freq[p1] × freq[p2] × freq[p3]"),
            Problem::MaximumSubarray => Some("cur_sum = max(n, cur_sum + n)"),
            Problem::JumpGame => Some("max_reach = max(max_reach, i + nums[i])"),
            Problem::JumpGameII => Some("farthest = max(farthest, i + nums[i])"),
            Problem::GasStation => Some("total_tank += gas[i] - cost[i]"),
            Problem::CourseSchedule => Some("topological sort ➔ in_degree == 0"),
            Problem::UniquePaths => Some("dp[r][c] = dp[r+1][c] + dp[r][c+1]"),
            Problem::LongestCommonSubsequence => Some("if s1[i] == s2[j] ➔ 1 + dp[i+1][j+1]"),
            Problem::CoinChange => Some("dp[a] = min(dp[a], 1 + dp[a - coin])"),
            Problem::LongestIncreasingSubsequence => {
                Some("if nums[j] < nums[i] ➔ dp[i] = max(1 + dp[j])")
            }
            _ => None,
        }
    }

    pub fn details(&self) -> ProblemDetails {
        match self {
            Problem::ContainsDuplicate => ProblemDetails {
                id: 217, title: "Contains Duplicate", difficulty: Difficulty::Easy, category: Category::ArraysAndHashing,
                statement: "Given an integer array nums, return true if any value appears at least twice in the array.",
                examples: &[Example { input: "nums = [1, 2, 3, 1]", output: "true", explanation: "Digit 1 appears twice." }],
                constraints: &["1 <= nums.length <= 10^5"], leetcode_url: "https://leetcode.com/problems/contains-duplicate/",
                approaches: &[ApproachMeta { id: 0, name: "Hash Set Lookup", time_complexity: "O(N)", space_complexity: "O(N)", rationale: "Checking presence in a Hash Set takes O(1) average time per element, avoiding the O(N^2) nested loop comparison.", description: "Insert into set, return true on collision." }],
            },
            Problem::TwoSum => ProblemDetails {
                id: 1, title: "Two Sum", difficulty: Difficulty::Easy, category: Category::ArraysAndHashing,
                statement: "Given an array of integers nums and a target, return indices of the two numbers that add up to target.",
                examples: &[Example { input: "nums = [2, 7, 11, 15], target = 9", output: "[0, 1]", explanation: "nums[0] + nums[1] == 9" }],
                constraints: &["2 <= nums.length <= 10^4"], leetcode_url: "https://leetcode.com/problems/two-sum/",
                approaches: &[ApproachMeta { id: 0, name: "Hash Map (One Pass)", time_complexity: "O(N)", space_complexity: "O(N)", rationale: "Storing seen numbers in a Hash Map allows complement lookup in O(1) time instead of brute-force O(N^2) pair checking.", description: "Use hash map complement lookup." }],
            },
            Problem::ValidAnagram => ProblemDetails {
                id: 242, title: "Valid Anagram", difficulty: Difficulty::Easy, category: Category::ArraysAndHashing,
                statement: "Given two strings s and t, return true if t is an anagram of s.",
                examples: &[Example { input: "s = \"anagram\", t = \"nagaram\"", output: "true", explanation: "Frequencies match." }],
                constraints: &["1 <= s.length <= 5*10^4"], leetcode_url: "https://leetcode.com/problems/valid-anagram/",
                approaches: &[ApproachMeta { id: 0, name: "Hash Map Frequency Counter", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Using two Hash Maps to count character frequencies processes both strings in a single O(N) pass. Storing at most 26 letter keys requires O(1) auxiliary space.", description: "Count char frequencies using hash maps." }],
            },
            Problem::GroupAnagrams => ProblemDetails {
                id: 49, title: "Group Anagrams", difficulty: Difficulty::Medium, category: Category::ArraysAndHashing,
                statement: "Given an array of strings strs, group the anagrams together.",
                examples: &[Example { input: "strs = [\"eat\",\"tea\",\"tan\",\"ate\",\"nat\",\"bat\"]", output: "[[\"bat\"],[\"nat\",\"tan\"],[\"ate\",\"eat\",\"tea\"]]", explanation: "Anagrams grouped by key." }],
                constraints: &["1 <= strs.length <= 10^4"], leetcode_url: "https://leetcode.com/problems/group-anagrams/",
                approaches: &[ApproachMeta { id: 0, name: "Char Frequency Tuple Map", time_complexity: "O(N * K)", space_complexity: "O(N * K)", rationale: "Using character frequency tuples as Hash Map keys groups anagrams in O(N * K) time without sorting individual strings.", description: "Tuple key map." }],
            },
            Problem::TopKFrequent => ProblemDetails {
                id: 347, title: "Top K Frequent Elements", difficulty: Difficulty::Medium, category: Category::ArraysAndHashing,
                statement: "Given an integer array nums and integer k, return the k most frequent elements.",
                examples: &[Example { input: "nums = [1,1,1,2,2,3], k = 2", output: "[1, 2]", explanation: "1 appears 3x, 2 appears 2x." }],
                constraints: &["1 <= nums.length <= 10^5"], leetcode_url: "https://leetcode.com/problems/top-k-frequent-elements/",
                approaches: &[ApproachMeta { id: 0, name: "Bucket Sort", time_complexity: "O(N)", space_complexity: "O(N)", rationale: "Bucket sorting by frequency index allows linear O(N) extraction of top K elements, outperforming O(N log N) heap/sorting methods.", description: "Frequency buckets." }],
            },
            Problem::ProductExceptSelf => ProblemDetails {
                id: 238, title: "Product of Array Except Self", difficulty: Difficulty::Medium, category: Category::ArraysAndHashing,
                statement: "Return an array output where output[i] is the product of all elements except nums[i].",
                examples: &[Example { input: "nums = [1, 2, 4, 6]", output: "[48, 24, 12, 8]", explanation: "Prefix/suffix passes." }],
                constraints: &["2 <= nums.length <= 1000"], leetcode_url: "https://leetcode.com/problems/product-of-array-except-self/",
                approaches: &[ApproachMeta { id: 0, name: "Prefix & Suffix Pass", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Computing left prefix products and right suffix products in two O(N) passes avoids division while keeping extra space to O(1).", description: "Running prefix & suffix." }],
            },
            Problem::EncodeDecode => ProblemDetails {
                id: 271, title: "Encode and Decode Strings", difficulty: Difficulty::Medium, category: Category::ArraysAndHashing,
                statement: "Design an algorithm to encode a list of strings to a string and decode it back.",
                examples: &[Example { input: "strs = [\"Hello\",\"World\"]", output: "[\"Hello\",\"World\"]", explanation: "Encoded into 5#Hello5#World." }],
                constraints: &["0 <= strs.length < 100"], leetcode_url: "https://leetcode.com/problems/encode-and-decode-strings/",
                approaches: &[ApproachMeta { id: 0, name: "Length Prefix (# Protocol)", time_complexity: "O(N)", space_complexity: "O(N)", rationale: "Prepending character length and a delimiter (#) guarantees unambiguous parsing regardless of special characters in strings.", description: "Len#str encoding." }],
            },
            Problem::ValidSudoku => ProblemDetails {
                id: 36, title: "Valid Sudoku", difficulty: Difficulty::Medium, category: Category::ArraysAndHashing,
                statement: "Determine if a 9x9 Sudoku board is valid (rows, cols, 3x3 boxes).",
                examples: &[Example { input: "board = [[1, 2, ...]]", output: "true", explanation: "No duplicates." }],
                constraints: &["board.length == 9"], leetcode_url: "https://leetcode.com/problems/valid-sudoku/",
                approaches: &[ApproachMeta { id: 0, name: "HashSet Validation", time_complexity: "O(1)", space_complexity: "O(1)", rationale: "A single 9x9 grid scan verifies row, column, and 3x3 box constraints in deterministic O(1) constant time.", description: "Scan rows, cols, 3x3 boxes." }],
            },
            Problem::LongestConsecutive => ProblemDetails {
                id: 128, title: "Longest Consecutive Sequence", difficulty: Difficulty::Medium, category: Category::ArraysAndHashing,
                statement: "Return the length of the longest consecutive elements sequence.",
                examples: &[Example { input: "nums = [2, 20, 4, 10, 3, 4, 5]", output: "4", explanation: "Sequence [2, 3, 4, 5]." }],
                constraints: &["0 <= nums.length <= 1000"], leetcode_url: "https://leetcode.com/problems/longest-consecutive-sequence/",
                approaches: &[ApproachMeta { id: 0, name: "HashSet Sequence Start Expansion", time_complexity: "O(N)", space_complexity: "O(N)", rationale: "Only expanding streaks from sequence start numbers (where n-1 is not in set) guarantees each number is visited at most twice (O(N)).", description: "Expand from streak starts." }],
            },
            Problem::ValidPalindrome => ProblemDetails {
                id: 125, title: "Valid Palindrome", difficulty: Difficulty::Easy, category: Category::TwoPointers,
                statement: "Given a string s, return true if it is a palindrome.",
                examples: &[Example { input: "s = \"Was it a car or a cat I saw?\"", output: "true", explanation: "Alphanumeric filter palindrome." }],
                constraints: &["1 <= s.length <= 1000"], leetcode_url: "https://leetcode.com/problems/valid-palindrome/",
                approaches: &[ApproachMeta { id: 0, name: "Two Pointers In-Place", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Two pointers converging from both ends check symmetry in O(N) time without allocating extra string storage (O(1) space).", description: "Left and right pointers." }],
            },
            Problem::BestTimeStock => ProblemDetails {
                id: 121, title: "Best Time to Buy and Sell Stock", difficulty: Difficulty::Easy, category: Category::SlidingWindow,
                statement: "Choose a single day to buy and a future day to sell to maximize profit.",
                examples: &[Example { input: "prices = [10, 1, 5, 6, 7, 1]", output: "6", explanation: "Buy at 1, sell at 7." }],
                constraints: &["1 <= prices.length <= 100"], leetcode_url: "https://leetcode.com/problems/best-time-to-buy-and-sell-stock/",
                approaches: &[ApproachMeta { id: 0, name: "Two Pointers / Sliding Window", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Tracking the minimum buy price while scanning linear prices computes max profit in one O(N) pass with O(1) space.", description: "Min buy pointer, running max profit." }],
            },
            Problem::ValidParentheses => ProblemDetails {
                id: 20, title: "Valid Parentheses", difficulty: Difficulty::Easy, category: Category::Stack,
                statement: "Return true if the input bracket string is valid.",
                examples: &[Example { input: "s = \"([{}])\"", output: "true", explanation: "Matched brackets." }],
                constraints: &["1 <= s.length <= 1000"], leetcode_url: "https://leetcode.com/problems/valid-parentheses/",
                approaches: &[ApproachMeta { id: 0, name: "Stack Matching", time_complexity: "O(N)", space_complexity: "O(N)", rationale: "A LIFO stack matches closing brackets with the most recently opened bracket in O(N) time and O(N) memory.", description: "Push open, pop matching close." }],
            },
            Problem::BinarySearch => ProblemDetails {
                id: 704, title: "Binary Search", difficulty: Difficulty::Easy, category: Category::BinarySearch,
                statement: "Given sorted array nums and target, return index of target or -1.",
                examples: &[Example { input: "nums = [-1, 0, 2, 4, 6, 8], target = 4", output: "3", explanation: "Found at index 3." }],
                constraints: &["1 <= nums.length <= 10000"], leetcode_url: "https://leetcode.com/problems/binary-search/",
                approaches: &[ApproachMeta { id: 0, name: "Binary Search Iterative", time_complexity: "O(log N)", space_complexity: "O(1)", rationale: "Halving the search space at each midpoint step guarantees logarithmic O(log N) runtime on sorted arrays.", description: "Midpoint bounds." }],
            },
            Problem::ReverseLinkedList => ProblemDetails {
                id: 206, title: "Reverse Linked List", difficulty: Difficulty::Easy, category: Category::LinkedList,
                statement: "Reverse a singly linked list.",
                examples: &[Example { input: "head = [0, 1, 2, 3]", output: "[3, 2, 1, 0]", explanation: "Next pointers flipped." }],
                constraints: &["0 <= length <= 1000"], leetcode_url: "https://leetcode.com/problems/reverse-linked-list/",
                approaches: &[ApproachMeta { id: 0, name: "Iterative Pointers (prev, curr)", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Reversing link pointers iteratively requires only 3 pointer variables (prev, curr, nxt), achieving O(N) time and O(1) space.", description: "Flip next pointers." }],
            },
            Problem::MergeTwoLists => ProblemDetails {
                id: 21, title: "Merge Two Sorted Linked Lists", difficulty: Difficulty::Easy, category: Category::LinkedList,
                statement: "Merge two sorted linked lists into one sorted list.",
                examples: &[Example { input: "list1 = [1, 2, 4], list2 = [1, 3, 5]", output: "[1, 1, 2, 3, 4, 5]", explanation: "Merged in order." }],
                constraints: &["0 <= list1.length <= 100"], leetcode_url: "https://leetcode.com/problems/merge-two-sorted-lists/",
                approaches: &[ApproachMeta { id: 0, name: "Two Pointers Merge", time_complexity: "O(N + M)", space_complexity: "O(1)", rationale: "Splicing existing list nodes together using two pointers merges sorted lists in O(N + M) time with zero extra allocations.", description: "Tail node attachments." }],
            },
            Problem::LinkedListCycle => ProblemDetails {
                id: 141, title: "Linked List Cycle Detection", difficulty: Difficulty::Easy, category: Category::LinkedList,
                statement: "Return true if there is a cycle in the linked list.",
                examples: &[Example { input: "head = [1, 2, 3, 4], index = 1", output: "true", explanation: "Tail connects to index 1." }],
                constraints: &["0 <= length <= 1000"], leetcode_url: "https://leetcode.com/problems/linked-list-cycle/",
                approaches: &[ApproachMeta { id: 0, name: "Floyd's Tortoise & Hare", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Floyd's fast pointer moves at 2x speed; if a cycle exists, the distance between slow and fast decreases by 1 each step (O(N) catch-up).", description: "Slow and fast pointers." }],
            },
            Problem::InvertTree => ProblemDetails {
                id: 226, title: "Invert Binary Tree", difficulty: Difficulty::Easy, category: Category::Trees,
                statement: "Invert a binary tree (swap left and right subtrees for every node).",
                examples: &[Example { input: "root = [1, 2, 3, 4, 5, 6, 7]", output: "[1, 3, 2, 7, 6, 5, 4]", explanation: "Subtrees swapped." }],
                constraints: &["0 <= nodes <= 100"], leetcode_url: "https://leetcode.com/problems/invert-binary-tree/",
                approaches: &[ApproachMeta { id: 0, name: "Recursive DFS", time_complexity: "O(N)", space_complexity: "O(H)", rationale: "Swapping left and right child pointers recursively visits all N tree nodes in O(N) time.", description: "Post-order swap." }],
            },
            Problem::MaxDepthTree => ProblemDetails {
                id: 104, title: "Maximum Depth of Binary Tree", difficulty: Difficulty::Easy, category: Category::Trees,
                statement: "Return the maximum depth of a binary tree.",
                examples: &[Example { input: "root = [1, 2, 3, null, null, 4]", output: "3", explanation: "Longest path is 3 nodes." }],
                constraints: &["0 <= nodes <= 100"], leetcode_url: "https://leetcode.com/problems/maximum-depth-of-binary-tree/",
                approaches: &[ApproachMeta { id: 0, name: "Recursive DFS", time_complexity: "O(N)", space_complexity: "O(H)", rationale: "Depth-first search computes subtree heights recursively as 1 + max(left, right) in O(N) time.", description: "1 + max(left, right)." }],
            },
            Problem::DiameterTree => ProblemDetails {
                id: 543, title: "Diameter of Binary Tree", difficulty: Difficulty::Easy, category: Category::Trees,
                statement: "Return length of longest path between any two nodes.",
                examples: &[Example { input: "root = [1, null, 2, 3, 4, 5]", output: "3", explanation: "Longest path has 3 edges." }],
                constraints: &["1 <= nodes <= 100"], leetcode_url: "https://leetcode.com/problems/diameter-of-binary-tree/",
                approaches: &[ApproachMeta { id: 0, name: "Post-order Depth DFS", time_complexity: "O(N)", space_complexity: "O(H)", rationale: "Calculating longest left + right depth path at each node during DFS finds the global diameter in O(N) time.", description: "Left height + right height." }],
            },
            Problem::BalancedTree => ProblemDetails {
                id: 110, title: "Balanced Binary Tree", difficulty: Difficulty::Easy, category: Category::Trees,
                statement: "Determine if a binary tree is height-balanced (|height(left) - height(right)| <= 1).",
                examples: &[Example { input: "root = [3, 9, 20, null, null, 15, 7]", output: "true", explanation: "Balanced heights." }],
                constraints: &["0 <= nodes <= 5000"], leetcode_url: "https://leetcode.com/problems/balanced-binary-tree/",
                approaches: &[ApproachMeta { id: 0, name: "Bottom-Up Height DFS", time_complexity: "O(N)", space_complexity: "O(H)", rationale: "Bottom-up DFS returns -1 immediately upon detecting an unbalanced subtree, pruning unnecessary calculations in O(N) time.", description: "Check height difference at each node." }],
            },
            Problem::SameTree => ProblemDetails {
                id: 100, title: "Same Tree", difficulty: Difficulty::Easy, category: Category::Trees,
                statement: "Given roots of two binary trees p and q, return true if they are structural and value identical.",
                examples: &[Example { input: "p = [1, 2, 3], q = [1, 2, 3]", output: "true", explanation: "Trees match." }],
                constraints: &["0 <= nodes <= 100"], leetcode_url: "https://leetcode.com/problems/same-tree/",
                approaches: &[ApproachMeta { id: 0, name: "Recursive DFS Comparison", time_complexity: "O(N)", space_complexity: "O(H)", rationale: "Recursive DFS verifies value match and structural equality across both trees simultaneously in O(N) time.", description: "Check p.val == q.val and recurse." }],
            },
            Problem::Subtree => ProblemDetails {
                id: 572, title: "Subtree of Another Tree", difficulty: Difficulty::Easy, category: Category::Trees,
                statement: "Return true if there is a subtree of root with the same structure and node values as subRoot.",
                examples: &[Example { input: "root = [3, 4, 5, 1, 2], subRoot = [4, 1, 2]", output: "true", explanation: "Subtree matches." }],
                constraints: &["0 <= nodes <= 2000"], leetcode_url: "https://leetcode.com/problems/subtree-of-another-tree/",
                approaches: &[ApproachMeta { id: 0, name: "Recursive Tree Matching", time_complexity: "O(N * M)", space_complexity: "O(H)", rationale: "Comparing subtree matches recursively at each root node checks structural identity in O(N * M) time.", description: "Compare root node with subRoot recursively." }],
            },
            Problem::ImplementTrie => ProblemDetails {
                id: 208, title: "Implement Trie (Prefix Tree)", difficulty: Difficulty::Medium, category: Category::Tries,
                statement: "A trie (prefix tree) is a tree data structure used to efficiently store and retrieve keys in a dataset of strings.",
                examples: &[Example { input: "insert(\"apple\"), search(\"apple\"), startsWith(\"app\")", output: "[null, true, true]", explanation: "Word and prefix found." }],
                constraints: &["1 <= word.length <= 2000"], leetcode_url: "https://leetcode.com/problems/implement-trie-prefix-tree/",
                approaches: &[ApproachMeta { id: 0, name: "TrieNode Hash/Array", time_complexity: "O(N)", space_complexity: "O(N * 26)", rationale: "Navigating child nodes by character code provides O(L) lookup independent of the total number of stored words.", description: "N-ary tree with character map and is_end flag." }],
            },
            Problem::WordDictionary => ProblemDetails {
                id: 211, title: "Design Add and Search Words Data Structure", difficulty: Difficulty::Medium, category: Category::Tries,
                statement: "Design a data structure that supports adding new words and searching if a string matches any previously added string (supporting '.' wildcards).",
                examples: &[Example { input: "addWord(\"bad\"), search(\".ad\")", output: "[null, true]", explanation: "'.' matches 'b'." }],
                constraints: &["1 <= word.length <= 25"], leetcode_url: "https://leetcode.com/problems/design-add-and-search-words-data-structure/",
                approaches: &[ApproachMeta { id: 0, name: "Trie DFS Wildcard Match", time_complexity: "O(N * 26^M)", space_complexity: "O(N)", rationale: "Trie DFS branches across 26 child nodes only when encountering wildcard ('.'), efficiently searching word patterns.", description: "DFS traversal branching on wildcard '.'." }],
            },
            Problem::WordSearchII => ProblemDetails {
                id: 212, title: "Word Search II", difficulty: Difficulty::Hard, category: Category::Tries,
                statement: "Given an m x n board of characters and a list of strings words, return all words on the board.",
                examples: &[Example { input: "board = [[\"o\",\"a\",\"a\",\"n\"],[\"e\",\"t\",\"a\",\"e\"]], words = [\"oath\",\"pea\",\"eat\",\"rain\"]", output: "[\"oath\",\"eat\"]", explanation: "Words found on grid." }],
                constraints: &["1 <= words.length <= 3 * 10^4"], leetcode_url: "https://leetcode.com/problems/word-search-ii/",
                approaches: &[ApproachMeta { id: 0, name: "Trie Grid Backtracking DFS", time_complexity: "O(M * N * 4^L)", space_complexity: "O(W * L)", rationale: "Building a Trie from dictionary words allows early pruning of grid DFS paths that do not form valid prefixes.", description: "Prune grid DFS using dictionary Trie." }],
            },
            Problem::ClimbingStairs => ProblemDetails {
                id: 70, title: "Climbing Stairs", difficulty: Difficulty::Easy, category: Category::OneDDp,
                statement: "It takes n steps to reach top. Each time you can climb 1 or 2 steps. How many distinct ways?",
                examples: &[Example { input: "n = 3", output: "3", explanation: "1+1+1, 1+2, 2+1." }],
                constraints: &["1 <= n <= 45"], leetcode_url: "https://leetcode.com/problems/climbing-stairs/",
                approaches: &[ApproachMeta { id: 0, name: "Dynamic Programming (Fibonacci)", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Ways to step n equal Fibonacci(n); maintaining 2 variables (dp[i-1], dp[i-2]) solves the problem in O(N) time and O(1) space.", description: "dp[i] = dp[i-1] + dp[i-2]." }],
            },
            Problem::MinCostStairs => ProblemDetails {
                id: 746, title: "Min Cost Climbing Stairs", difficulty: Difficulty::Easy, category: Category::OneDDp,
                statement: "Return minimum cost to reach top of floor by taking 1 or 2 steps.",
                examples: &[Example { input: "cost = [10, 15, 20]", output: "15", explanation: "Start at index 1, pay 15." }],
                constraints: &["2 <= cost.length <= 1000"], leetcode_url: "https://leetcode.com/problems/min-cost-climbing-stairs/",
                approaches: &[ApproachMeta { id: 0, name: "Bottom-Up DP", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Subproblem optimal transition dp[i] = cost[i] + min(dp[i-1], dp[i-2]) computes minimum cost in a single O(N) DP pass.", description: "dp[i] = min(dp[i-1]+cost[i-1], dp[i-2]+cost[i-2])." }],
            },
            Problem::KthLargestStream => ProblemDetails {
                id: 703, title: "Kth Largest Element in a Stream", difficulty: Difficulty::Easy, category: Category::HeapPriorityQueue,
                statement: "Design a class to find the k-th largest element in a stream.",
                examples: &[Example { input: "k = 3, nums = [4, 5, 8, 2], val = 3", output: "4", explanation: "Min-heap of size k=3." }],
                constraints: &["1 <= k <= 10^4"], leetcode_url: "https://leetcode.com/problems/kth-largest-element-in-a-stream/",
                approaches: &[ApproachMeta { id: 0, name: "Min-Heap of Size k", time_complexity: "O(N log k)", space_complexity: "O(k)", rationale: "A min-heap of size k keeps the k largest elements at all times; the top element is always the k-th largest in O(log k) per add.", description: "Maintain min-heap of size k." }],
            },
            Problem::LastStone => ProblemDetails {
                id: 1046, title: "Last Stone Weight", difficulty: Difficulty::Easy, category: Category::HeapPriorityQueue,
                statement: "Smash two heaviest stones y and x until at most 1 stone remains.",
                examples: &[Example { input: "stones = [2, 7, 4, 1, 8, 1]", output: "1", explanation: "Smash 8 and 7, remaining 1." }],
                constraints: &["1 <= stones.length <= 30"], leetcode_url: "https://leetcode.com/problems/last-stone-weight/",
                approaches: &[ApproachMeta { id: 0, name: "Max-Heap Simulation", time_complexity: "O(N log N)", space_complexity: "O(N)", rationale: "A max-heap always provides the two heaviest stones in O(log N) time per smash iteration.", description: "Repeatedly smash top 2." }],
            },
            Problem::MeetingRooms => ProblemDetails {
                id: 252, title: "Meeting Rooms", difficulty: Difficulty::Easy, category: Category::Intervals,
                statement: "Given an array of meeting time intervals, determine if a person could attend all meetings.",
                examples: &[Example { input: "intervals = [[0,30],[5,10],[15,20]]", output: "false", explanation: "[0,30] and [5,10] overlap." }],
                constraints: &["0 <= intervals.length <= 10^4"], leetcode_url: "https://leetcode.com/problems/meeting-rooms/",
                approaches: &[ApproachMeta { id: 0, name: "Sort Intervals by Start Time", time_complexity: "O(N log N)", space_complexity: "O(1)", rationale: "Sorting interval start times in O(N log N) allows checking adjacent meeting overlaps in a single O(N) pass.", description: "Check adjacent overlap." }],
            },
            Problem::HappyNumber => ProblemDetails {
                id: 202, title: "Happy Number", difficulty: Difficulty::Easy, category: Category::MathAndGeometry,
                statement: "Determine if a number n is happy (sum of square of digits reaches 1).",
                examples: &[Example { input: "n = 19", output: "true", explanation: "1^2+9^2=82 -> 68 -> 100 -> 1." }],
                constraints: &["1 <= n <= 2^31 - 1"], leetcode_url: "https://leetcode.com/problems/happy-number/",
                approaches: &[ApproachMeta { id: 0, name: "HashSet Cycle Detection", time_complexity: "O(log N)", space_complexity: "O(log N)", rationale: "A HashSet tracks previously seen digit sum results to detect infinite cycles in logarithmic time.", description: "Track seen square sums." }],
            },
            Problem::PlusOne => ProblemDetails {
                id: 66, title: "Plus One", difficulty: Difficulty::Easy, category: Category::MathAndGeometry,
                statement: "Increment the large integer represented as a digit array by one.",
                examples: &[Example { input: "digits = [1, 2, 3]", output: "[1, 2, 4]", explanation: "123 + 1 = 124." }],
                constraints: &["1 <= digits.length <= 100"], leetcode_url: "https://leetcode.com/problems/plus-one/",
                approaches: &[ApproachMeta { id: 0, name: "Right-to-Left Carry Pass", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Iterating backwards handles digit carry in O(N) time, adding a new leading 1 only if all digits were 9.", description: "Add 1 from right, carry overflow." }],
            },
            Problem::SingleNumber => ProblemDetails {
                id: 136, title: "Single Number", difficulty: Difficulty::Easy, category: Category::BitManipulation,
                statement: "Given a non-empty array of integers where every element appears twice except for one, find it.",
                examples: &[Example { input: "nums = [4, 1, 2, 1, 2]", output: "4", explanation: "4 is non-duplicate." }],
                constraints: &["1 <= nums.length <= 3*10^4"], leetcode_url: "https://leetcode.com/problems/single-number/",
                approaches: &[ApproachMeta { id: 0, name: "Bitwise XOR", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Bitwise XOR properties (a ^ a = 0 and a ^ 0 = a) cancel out paired numbers, isolating the single number in O(N) time and O(1) space.", description: "a ^ a = 0 cancels duplicates." }],
            },
            Problem::CountBits => ProblemDetails {
                id: 191, title: "Number of 1 Bits", difficulty: Difficulty::Easy, category: Category::BitManipulation,
                statement: "Return the number of set bits (1s) in a 32-bit unsigned integer.",
                examples: &[Example { input: "n = 11 (0000...1011)", output: "3", explanation: "3 set bits." }],
                constraints: &["1 <= n <= 2^31 - 1"], leetcode_url: "https://leetcode.com/problems/number-of-1-bits/",
                approaches: &[ApproachMeta { id: 0, name: "Brian Kernighan's Algorithm", time_complexity: "O(1)", space_complexity: "O(1)", rationale: "Kernighan's operation n &= (n - 1) clears the lowest set bit, counting set bits in O(set_bits) operations.", description: "n &= n - 1 clears lowest 1 bit." }],
            },
            Problem::CountingBits => ProblemDetails {
                id: 338, title: "Counting Bits", difficulty: Difficulty::Easy, category: Category::BitManipulation,
                statement: "Given n, return an array ans of length n + 1 where ans[i] is the number of 1's in binary representation of i.",
                examples: &[Example { input: "n = 5", output: "[0,1,1,2,1,2]", explanation: "Bits for 0..5." }],
                constraints: &["0 <= n <= 10^5"], leetcode_url: "https://leetcode.com/problems/counting-bits/",
                approaches: &[ApproachMeta { id: 0, name: "Dynamic Programming (Bit Shift / Offset)", time_complexity: "O(N)", space_complexity: "O(N)", rationale: "Using DP transition bits[i] = bits[i >> 1] + (i & 1) computes bit counts for 0..N in linear O(N) time.", description: "dp[i] = 1 + dp[i - offset]." }],
            },
            Problem::ReverseBits => ProblemDetails {
                id: 190, title: "Reverse Bits", difficulty: Difficulty::Easy, category: Category::BitManipulation,
                statement: "Reverse bits of a given 32-bit unsigned integer.",
                examples: &[Example { input: "n = 43261596 (00000010100101000001111010011100)", output: "964176192", explanation: "Reversed bits." }],
                constraints: &["32-bit integer"], leetcode_url: "https://leetcode.com/problems/reverse-bits/",
                approaches: &[ApproachMeta { id: 0, name: "Bitwise Shift & Or", time_complexity: "O(1)", space_complexity: "O(1)", rationale: "Looping 32 bits and shifting the target bit to position (31 - i) reverses bit order in deterministic O(1) time.", description: "Shift bit i to 31 - i." }],
            },
            Problem::MissingNumber => ProblemDetails {
                id: 268, title: "Missing Number", difficulty: Difficulty::Easy, category: Category::BitManipulation,
                statement: "Given an array containing n distinct numbers in range [0, n], return the missing number.",
                examples: &[Example { input: "nums = [3, 0, 1]", output: "2", explanation: "Range [0..3], 2 is missing." }],
                constraints: &["1 <= n <= 10^4"], leetcode_url: "https://leetcode.com/problems/missing-number/",
                approaches: &[ApproachMeta { id: 0, name: "Gauss Sum Formula", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Gauss sum formula N*(N+1)/2 gives expected total; subtracting actual array sum finds missing number in O(N) time and O(1) space.", description: "expected_sum - actual_sum." }],
            },
            Problem::TwoSumII => ProblemDetails {
                id: 167, title: "Two Sum II - Input Array Is Sorted", difficulty: Difficulty::Medium, category: Category::TwoPointers,
                statement: "Given a 1-indexed array of integers that is already sorted in non-decreasing order, find two numbers such that they add up to a specific target number.",
                examples: &[Example { input: "numbers = [2, 7, 11, 15], target = 9", output: "[1, 2]", explanation: "numbers[1] + numbers[2] = 9." }],
                constraints: &["2 <= numbers.length <= 3*10^4", "numbers is sorted in non-decreasing order"], leetcode_url: "https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/",
                approaches: &[ApproachMeta { id: 0, name: "Two Pointers (Sorted)", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Because the array is sorted, moving left pointer right increases sum and right pointer left decreases sum in O(N) time and O(1) space.", description: "Left/right pointers converge on target sum." }],
            },
            Problem::ThreeSum => ProblemDetails {
                id: 15, title: "3Sum", difficulty: Difficulty::Medium, category: Category::TwoPointers,
                statement: "Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.",
                examples: &[Example { input: "nums = [-1, 0, 1, 2, -1, -4]", output: "[[-1, -1, 2], [-1, 0, 1]]", explanation: "Two unique triplets sum to 0." }],
                constraints: &["3 <= nums.length <= 3000"], leetcode_url: "https://leetcode.com/problems/3sum/",
                approaches: &[ApproachMeta { id: 0, name: "Sort + Two Pointers", time_complexity: "O(N^2)", space_complexity: "O(1)", rationale: "Sorting the array and using two pointers for each fixed anchor avoids duplicate triplets in O(N^2) time and O(1) auxiliary space.", description: "Fix anchor, two pointers for remaining pair." }],
            },
            Problem::ContainerWater => ProblemDetails {
                id: 11, title: "Container With Most Water", difficulty: Difficulty::Medium, category: Category::TwoPointers,
                statement: "Given n non-negative integers representing n vertical lines, find two lines that together with the x-axis form a container that holds the most water.",
                examples: &[Example { input: "height = [1, 8, 6, 2, 5, 4, 8, 3, 7]", output: "49", explanation: "Lines at index 1 and 8 form container of area 49." }],
                constraints: &["n == height.length", "2 <= n <= 10^5"], leetcode_url: "https://leetcode.com/problems/container-with-most-water/",
                approaches: &[ApproachMeta { id: 0, name: "Two Pointers Greedy", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Moving the pointer with shorter height inward is the only way to potentially find a larger area, achieving O(N) time.", description: "Move the shorter line inward to maximize area." }],
            },
            Problem::TrappingRain => ProblemDetails {
                id: 42, title: "Trapping Rain Water", difficulty: Difficulty::Hard, category: Category::TwoPointers,
                statement: "Given n non-negative integers representing an elevation map where the width of each bar is 1, compute how much water it can trap after raining.",
                examples: &[Example { input: "height = [0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]", output: "6", explanation: "6 units of rain water are trapped." }],
                constraints: &["n == height.length", "0 <= n <= 2*10^4"], leetcode_url: "https://leetcode.com/problems/trapping-rain-water/",
                approaches: &[ApproachMeta { id: 0, name: "Two Pointers (leftMax / rightMax)", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Maintaining leftMax and rightMax bounds computes trapped water per column in a single O(N) pass with O(1) space.", description: "Track max heights from both sides." }],
            },
            Problem::MinStack => ProblemDetails {
                id: 155, title: "Min Stack", difficulty: Difficulty::Medium, category: Category::Stack,
                statement: "Design a stack that supports push, pop, top, and retrieving the minimum element in constant time O(1).",
                examples: &[Example { input: "push(-2), push(0), push(-3), getMin(), pop(), top(), getMin()", output: "getMin() = -3, top() = 0, getMin() = -2", explanation: "Min stack tracks minimum element at every push level." }],
                constraints: &["-2^31 <= val <= 2^31 - 1", "Methods pop, top and getMin will always be called on non-empty stacks"], leetcode_url: "https://leetcode.com/problems/min-stack/",
                approaches: &[ApproachMeta { id: 0, name: "Two Stacks (Value + MinStack)", time_complexity: "O(1)", space_complexity: "O(N)", rationale: "Storing running minimums alongside stack values guarantees O(1) constant time retrieval for min operations.", description: "Maintain parallel stack tracking minimums." }],
            },
            Problem::EvalRPN => ProblemDetails {
                id: 150, title: "Evaluate Reverse Polish Notation", difficulty: Difficulty::Medium, category: Category::Stack,
                statement: "Evaluate the value of an arithmetic expression in Reverse Polish Notation (postfix). Valid operators are +, -, *, and /.",
                examples: &[Example { input: "tokens = [\"2\", \"1\", \"+\", \"3\", \"*\"]", output: "9", explanation: "((2 + 1) * 3) = 9." }],
                constraints: &["1 <= tokens.length <= 10^4", "tokens[i] is an operator or integer in range [-200, 200]"], leetcode_url: "https://leetcode.com/problems/evaluate-reverse-polish-notation/",
                approaches: &[ApproachMeta { id: 0, name: "Stack Operand Evaluation", time_complexity: "O(N)", space_complexity: "O(N)", rationale: "Postfix evaluation using a LIFO stack processes operands and applies operators in linear O(N) time.", description: "Push numbers, pop 2 numbers on operator." }],
            },
            Problem::LongestSubstring => ProblemDetails {
                id: 3, title: "Longest Substring Without Repeating Characters", difficulty: Difficulty::Medium, category: Category::SlidingWindow,
                statement: "Given a string s, find the length of the longest substring without repeating characters.",
                examples: &[Example { input: "s = \"abcabcbb\"", output: "3", explanation: "The answer is \"abc\", with length 3." }],
                constraints: &["0 <= s.length <= 5*10^4"], leetcode_url: "https://leetcode.com/problems/longest-substring-without-repeating-characters/",
                approaches: &[ApproachMeta { id: 0, name: "Sliding Window Set", time_complexity: "O(N)", space_complexity: "O(N)", rationale: "A sliding window HashSet expands right and shrinks left on duplicates, processing each character at most twice (O(N)).", description: "Expand right, shrink left on duplicate." }],
            },
            Problem::Search2DMatrix => ProblemDetails {
                id: 74, title: "Search a 2D Matrix", difficulty: Difficulty::Medium, category: Category::BinarySearch,
                statement: "Write an efficient algorithm that searches for a target value in an m x n integer matrix with sorted rows and columns.",
                examples: &[Example { input: "matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3", output: "true", explanation: "3 exists in row 0, col 1." }],
                constraints: &["m == matrix.length", "n == matrix[i].length", "1 <= m, n <= 100"], leetcode_url: "https://leetcode.com/problems/search-a-2d-matrix/",
                approaches: &[ApproachMeta { id: 0, name: "Binary Search Virtual 1D Array", time_complexity: "O(log(M*N))", space_complexity: "O(1)", rationale: "Treating the M x N matrix as a virtual 1D sorted array allows binary search in O(log(M*N)) time and O(1) space.", description: "Map mid index to matrix[mid / cols][mid % cols]." }],
            },
            Problem::HouseRobber => ProblemDetails {
                id: 198, title: "House Robber", difficulty: Difficulty::Medium, category: Category::OneDDp,
                statement: "Given an integer array nums representing the amount of money of each house, return the maximum amount of money you can rob tonight without robbing adjacent houses.",
                examples: &[Example { input: "nums = [1, 2, 3, 1]", output: "4", explanation: "Rob house 1 (money = 1) and rob house 3 (money = 3). Total = 1 + 3 = 4." }],
                constraints: &["1 <= nums.length <= 100"], leetcode_url: "https://leetcode.com/problems/house-robber/",
                approaches: &[ApproachMeta { id: 0, name: "Bottom-Up Dynamic Programming", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "DP state transition max(rob_prev_prev + num, rob_prev) computes max loot in O(N) time and O(1) space.", description: "dp[i] = max(dp[i-1], dp[i-2] + nums[i])." }],
            },
            Problem::GenerateParentheses => ProblemDetails {
                id: 22, title: "Generate Parentheses", difficulty: Difficulty::Medium, category: Category::Stack,
                statement: "Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.",
                examples: &[Example { input: "n = 3", output: "[\"((()))\",\"(()())\",\"(())()\",\"()(())\",\"()()()\"]", explanation: "All 5 valid combinations." }],
                constraints: &["1 <= n <= 8"], leetcode_url: "https://leetcode.com/problems/generate-parentheses/",
                approaches: &[ApproachMeta { id: 0, name: "Backtracking / Stack", time_complexity: "O(4^N / sqrt(N))", space_complexity: "O(N)", rationale: "Backtracking only branches when open_count < N or close_count < open_count, generating only valid combinations.", description: "Recursively build string adhering to open < n and close < open." }],
            },
            Problem::DailyTemperatures => ProblemDetails {
                id: 739, title: "Daily Temperatures", difficulty: Difficulty::Medium, category: Category::Stack,
                statement: "Given an array of integers temperatures represents the daily temperatures, return an array answer such that answer[i] is the number of days you have to wait after the i-th day to get a warmer temperature.",
                examples: &[Example { input: "temperatures = [73, 74, 75, 71, 69, 72, 76, 73]", output: "[1, 1, 4, 2, 1, 1, 0, 0]", explanation: "Monotonic stack tracking waiting days." }],
                constraints: &["1 <= temperatures.length <= 10^5", "30 <= temperatures[i] <= 100"], leetcode_url: "https://leetcode.com/problems/daily-temperatures/",
                approaches: &[ApproachMeta { id: 0, name: "Monotonic Decreasing Stack", time_complexity: "O(N)", space_complexity: "O(N)", rationale: "A monotonic decreasing stack stores indices waiting for warmer days, resolving each index in O(N) time.", description: "Maintain stack of indices in decreasing temperature order." }],
            },
            Problem::CarFleet => ProblemDetails {
                id: 853, title: "Car Fleet", difficulty: Difficulty::Medium, category: Category::Stack,
                statement: "There are n cars at given miles away from the starting mile 0, traveling to a destination target. Return the number of car fleets that will arrive at the destination.",
                examples: &[Example { input: "target = 12, position = [10,8,0,5,3], speed = [2,4,1,1,3]", output: "3", explanation: "Cars starting at 10 and 8 merge into 1 fleet, 5 and 3 merge into 1 fleet, 0 drives alone." }],
                constraints: &["n == position.length == speed.length", "1 <= n <= 10^5"], leetcode_url: "https://leetcode.com/problems/car-fleet/",
                approaches: &[ApproachMeta { id: 0, name: "Monotonic Time Stack (Position Order)", time_complexity: "O(N log N)", space_complexity: "O(N)", rationale: "Sorting cars by start position descending computes arrival times; if a car behind arrives earlier, it joins the fleet (O(N log N)).", description: "Sort by position desc, pop if car behind catches up." }],
            },
            Problem::LargestRectangle => ProblemDetails {
                id: 84, title: "Largest Rectangle in Histogram", difficulty: Difficulty::Hard, category: Category::Stack,
                statement: "Given an array of integers heights representing the histogram's bar height where the width of each bar is 1, return the area of the largest rectangle in the histogram.",
                examples: &[Example { input: "heights = [2, 1, 5, 6, 2, 3]", output: "10", explanation: "The largest rectangle has area = 10 units (heights 5 and 6)." }],
                constraints: &["1 <= heights.length <= 10^5", "0 <= heights[i] <= 10^4"], leetcode_url: "https://leetcode.com/problems/largest-rectangle-in-histogram/",
                approaches: &[ApproachMeta { id: 0, name: "Monotonic Increasing Stack", time_complexity: "O(N)", space_complexity: "O(N)", rationale: "A monotonic increasing stack of heights computes maximum rectangular area bounds upon popping in O(N) time.", description: "Maintain (index, height) pair stack, compute max area on pop." }],
            },
            Problem::CharacterReplacement => ProblemDetails {
                id: 424, title: "Longest Repeating Character Replacement", difficulty: Difficulty::Medium, category: Category::SlidingWindow,
                statement: "You are given a string s and an integer k. You can choose any character of the string and change it to any other uppercase English character at most k times. Return the length of the longest substring containing the same letter.",
                examples: &[Example { input: "s = \"ABAB\", k = 2", output: "4", explanation: "Replace the two 'A's with 'B's or vice versa." }],
                constraints: &["1 <= s.length <= 10^5", "0 <= k <= s.length"], leetcode_url: "https://leetcode.com/problems/longest-repeating-character-replacement/",
                approaches: &[ApproachMeta { id: 0, name: "Sliding Window Frequency Map", time_complexity: "O(N)", space_complexity: "O(26)", rationale: "Maintaining max character frequency in sliding window shrinks left bound when (window_len - max_freq) > k in O(N) time.", description: "Maintain max frequency, shrink left when (window_len - max_freq) > k." }],
            },
            Problem::PermutationInString => ProblemDetails {
                id: 567, title: "Permutation in String", difficulty: Difficulty::Medium, category: Category::SlidingWindow,
                statement: "Given two strings s1 and s2, return true if s2 contains a permutation of s1, or false otherwise.",
                examples: &[Example { input: "s1 = \"ab\", s2 = \"eidbaooo\"", output: "true", explanation: "s2 contains one permutation of s1 (\"ba\")." }],
                constraints: &["1 <= s1.length, s2.length <= 10^4"], leetcode_url: "https://leetcode.com/problems/permutation-in-string/",
                approaches: &[ApproachMeta { id: 0, name: "Fixed Size Sliding Window Matches Count", time_complexity: "O(N)", space_complexity: "O(26)", rationale: "A fixed-size sliding window of length len(s1) matches character count frequencies in O(N) time and O(1) space.", description: "Slide window of size len(s1), track matching char counts." }],
            },
            Problem::MinWindowSubstring => ProblemDetails {
                id: 76, title: "Minimum Window Substring", difficulty: Difficulty::Hard, category: Category::SlidingWindow,
                statement: "Given two strings s and t of lengths m and n respectively, return the minimum window substring of s such that every character in t (including duplicates) is included in the window.",
                examples: &[Example { input: "s = \"ADOBECODEBANC\", t = \"ABC\"", output: "\"BANC\"", explanation: "The minimum window substring \"BANC\" includes 'A', 'B', and 'C' from string t." }],
                constraints: &["m == s.length", "n == t.length", "1 <= m, n <= 10^5"], leetcode_url: "https://leetcode.com/problems/minimum-window-substring/",
                approaches: &[ApproachMeta { id: 0, name: "Dynamic Sliding Window Have/Need Map", time_complexity: "O(N)", space_complexity: "O(M+N)", rationale: "A dynamic sliding window expands right to satisfy target character counts and shrinks left to find minimum length in O(N) time.", description: "Expand right until valid, then shrink left to minimize window." }],
            },
            Problem::SlidingWindowMax => ProblemDetails {
                id: 239, title: "Sliding Window Maximum", difficulty: Difficulty::Hard, category: Category::SlidingWindow,
                statement: "You are given an array of integers nums, there is a sliding window of size k which is moving from the very left of the array to the very right. Return the max sliding window.",
                examples: &[Example { input: "nums = [1,3,-1,-3,5,3,6,7], k = 3", output: "[3,3,5,5,6,7]", explanation: "Monotonic deque tracks sliding window maximum in O(N)." }],
                constraints: &["1 <= nums.length <= 10^5", "1 <= k <= nums.length"], leetcode_url: "https://leetcode.com/problems/sliding-window-maximum/",
                approaches: &[ApproachMeta { id: 0, name: "Monotonic Decreasing Deque", time_complexity: "O(N)", space_complexity: "O(k)", rationale: "A monotonic decreasing deque stores indices of potential max values, removing expired indices in O(N) time.", description: "Maintain deque of indices with strictly decreasing values." }],
            },
            Problem::SearchRotatedArray => ProblemDetails {
                id: 33, title: "Search in Rotated Sorted Array", difficulty: Difficulty::Medium, category: Category::BinarySearch,
                statement: "Given the array nums after the possible rotation and an integer target, return the index of target if it is in nums, or -1 if it is not in nums.",
                examples: &[Example { input: "nums = [4,5,6,7,0,1,2], target = 0", output: "4", explanation: "Binary search comparing mid with boundary values." }],
                constraints: &["1 <= nums.length <= 5000", "-10^4 <= nums[i] <= 10^4"], leetcode_url: "https://leetcode.com/problems/search-in-rotated-sorted-array/",
                approaches: &[ApproachMeta { id: 0, name: "Rotated Binary Search", time_complexity: "O(log N)", space_complexity: "O(1)", rationale: "Identifying which half (left or right of mid) is sorted allows halving the search space in O(log N) time.", description: "Determine which half is sorted, check if target lies in that range." }],
            },
            Problem::FindMinRotated => ProblemDetails {
                id: 153, title: "Find Minimum in Rotated Sorted Array", difficulty: Difficulty::Medium, category: Category::BinarySearch,
                statement: "Suppose an array of length n sorted in ascending order is rotated between 1 and n times. Given the sorted rotated array nums of unique elements, return the minimum element of this array.",
                examples: &[Example { input: "nums = [3,4,5,1,2]", output: "1", explanation: "The original array was [1,2,3,4,5] rotated 3 times." }],
                constraints: &["1 <= n <= 5000", "-5000 <= nums[i] <= 5000"], leetcode_url: "https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/",
                approaches: &[ApproachMeta { id: 0, name: "Binary Search Right Boundary Comparison", time_complexity: "O(log N)", space_complexity: "O(1)", rationale: "Comparing nums[mid] to nums[right] determines whether the rotation pivot is in the left or right half in O(log N) time.", description: "Compare nums[mid] with nums[right] to halve search space." }],
            },
            Problem::TimeKeyValueStore => ProblemDetails {
                id: 981, title: "Time Based Key-Value Store", difficulty: Difficulty::Medium, category: Category::BinarySearch,
                statement: "Design a time-based key-value data structure that can store multiple values for the same key at different time stamps and retrieve the key's value at a certain timestamp.",
                examples: &[Example { input: "set(\"foo\", \"bar\", 1), get(\"foo\", 1), get(\"foo\", 3)", output: "\"bar\", \"bar\"", explanation: "Binary search list of (timestamp, value) pairs." }],
                constraints: &["1 <= key.length, value.length <= 100", "1 <= timestamp <= 10^7"], leetcode_url: "https://leetcode.com/problems/time-based-key-value-store/",
                approaches: &[ApproachMeta { id: 0, name: "HashMap + Binary Search Timestamps", time_complexity: "O(log N) get", space_complexity: "O(N)", rationale: "HashMap maps key to a vector of (timestamp, value) pairs; binary search finds upper bound timestamp in O(log N) time.", description: "HashMap maps key to sorted list of (time, val), binary search for upper bound." }],
            },
            Problem::FindMedianSortedArrays => ProblemDetails {
                id: 4, title: "Median of Two Sorted Arrays", difficulty: Difficulty::Hard, category: Category::BinarySearch,
                statement: "Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.",
                examples: &[Example { input: "nums1 = [1,3], nums2 = [2,4]", output: "2.5", explanation: "merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5." }],
                constraints: &["nums1.length == m", "nums2.length == n", "0 <= m, n <= 1000"], leetcode_url: "https://leetcode.com/problems/median-of-two-sorted-arrays/",
                approaches: &[ApproachMeta { id: 0, name: "Binary Search Partition on Smaller Array", time_complexity: "O(log(min(M, N)))", space_complexity: "O(1)", rationale: "Binary searching partition index on the smaller array balances left and right halves in O(log(min(M, N))) time.", description: "Binary search partition index i in A such that A[i-1] <= B[j] and B[j-1] <= A[i]." }],
            },
            Problem::KokoEatingBananas => ProblemDetails {
                id: 875, title: "Koko Eating Bananas", difficulty: Difficulty::Medium, category: Category::BinarySearch,
                statement: "Koko loves to eat bananas. There are n piles of bananas, the i-th pile has piles[i] bananas. The guards have gone and will come back in h hours. Koko can decide her banana-eating speed of k (bananas-per-hour). Each hour, she chooses some pile of bananas and eats k bananas from that pile. Return the minimum integer k such that she can eat all the bananas within h hours.",
                examples: &[Example { input: "piles = [3, 6, 7, 11], h = 8", output: "4", explanation: "At speed k = 4, total hours = 1 + 2 + 2 + 3 = 8 <= 8." }],
                constraints: &["1 <= piles.length <= 10^4", "piles.length <= h <= 10^9", "1 <= piles[i] <= 10^9"], leetcode_url: "https://leetcode.com/problems/koko-eating-bananas/",
                approaches: &[ApproachMeta { id: 0, name: "Binary Search on Eating Speed k", time_complexity: "O(N log(max(piles)))", space_complexity: "O(1)", rationale: "Binary search on speed k in range [1, max(piles)]. Calculate total hours needed for each candidate mid speed; if total_hours <= h, speed k works so we record it and search smaller speeds (r = mid - 1); otherwise we need a faster speed (l = mid + 1).", description: "Binary search eating speed k in range [1, max(piles)]." }],
            },
            Problem::Subsets => ProblemDetails {
                id: 78, title: "Subsets", difficulty: Difficulty::Medium, category: Category::Backtracking,
                statement: "Given an integer array nums of unique elements, return all possible subsets (the power set). The solution set must not contain duplicate subsets. Return the solution in any order.",
                examples: &[Example { input: "nums = [1,2,3]", output: "[[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]", explanation: "Generate all 2^N combinations using binary choice decision tree." }],
                constraints: &["1 <= nums.length <= 10", "-10 <= nums[i] <= 10"], leetcode_url: "https://leetcode.com/problems/subsets/",
                approaches: &[ApproachMeta { id: 0, name: "Cascading Backtracking Decision Tree", time_complexity: "O(N * 2^N)", space_complexity: "O(N)", rationale: "At each element, make a binary choice to include or exclude, producing 2^N subsets with O(N) recursion stack space.", description: "Recurse choosing to include or exclude each element." }],
            },
            Problem::Permutations => ProblemDetails {
                id: 46, title: "Permutations", difficulty: Difficulty::Medium, category: Category::Backtracking,
                statement: "Given an array nums of distinct integers, return all the possible permutations. You can return the answer in any order.",
                examples: &[Example { input: "nums = [1,2,3]", output: "[[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]", explanation: "Explore all N! ordering branches." }],
                constraints: &["1 <= nums.length <= 6", "-10 <= nums[i] <= 10"], leetcode_url: "https://leetcode.com/problems/permutations/",
                approaches: &[ApproachMeta { id: 0, name: "Backtracking with Used Flag Array", time_complexity: "O(N * N!)", space_complexity: "O(N)", rationale: "Exploring all N! permutations with a boolean used array takes O(N * N!) time and O(N) stack space.", description: "Track used elements and construct all distinct position orderings." }],
            },
            Problem::KClosestPoints => ProblemDetails {
                id: 973, title: "K Closest Points to Origin", difficulty: Difficulty::Medium, category: Category::HeapPriorityQueue,
                statement: "Given an array of points where points[i] = [xi, yi] and an integer k, return the k closest points to the origin (0, 0).",
                examples: &[Example { input: "points = [[1,3],[-2,2]], k = 1", output: "[[-2,2]]", explanation: "Distance of [1,3] is 10, distance of [-2,2] is 8. [-2,2] is closer." }],
                constraints: &["1 <= k <= points.length <= 10^4", "-10^4 <= xi, yi <= 10^4"], leetcode_url: "https://leetcode.com/problems/k-closest-points-to-origin/",
                approaches: &[ApproachMeta { id: 0, name: "Max-Heap of Size K", time_complexity: "O(N log K)", space_complexity: "O(K)", rationale: "Maintaining a max-heap of size K stores the smallest K distances seen so far in O(N log K) time.", description: "Push distances into max-heap of size K." }],
            },
            Problem::TaskScheduler => ProblemDetails {
                id: 621, title: "Task Scheduler", difficulty: Difficulty::Medium, category: Category::HeapPriorityQueue,
                statement: "Given a characters array tasks, representing the tasks a CPU needs to do, where each letter represents a different task. Tasks could be done in any order. Each task is done in one unit of time. For each unit of time, the CPU could have done a task or be idle. However, there is a non-negative integer n that represents the cooldown period between two same tasks.",
                examples: &[Example { input: "tasks = [\"A\",\"A\",\"A\",\"B\",\"B\",\"B\"], n = 2", output: "8", explanation: "A -> B -> idle -> A -> B -> idle -> A -> B." }],
                constraints: &["1 <= tasks.length <= 10^4", "0 <= n <= 100"], leetcode_url: "https://leetcode.com/problems/task-scheduler/",
                approaches: &[ApproachMeta { id: 0, name: "Max-Heap Frequency Priority Queue", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Always scheduling the highest frequency task via a Max-Heap minimizes CPU idle cooling cycles in O(N) time.", description: "Use Max-Heap of task counts to greedily schedule most frequent tasks." }],
            },
            Problem::FindMedianDataStream => ProblemDetails {
                id: 295, title: "Find Median from Data Stream", difficulty: Difficulty::Hard, category: Category::HeapPriorityQueue,
                statement: "The median is the middle value in an ordered integer list. If the size of the list is even, there is no middle value, and the median is the mean of the two middle values. Implement MedianFinder class.",
                examples: &[Example { input: "addNum(1), addNum(2), findMedian(), addNum(3), findMedian()", output: "1.5, 2.0", explanation: "Maintain small max-heap and large min-heap." }],
                constraints: &["-10^5 <= num <= 10^5", "At most 5 * 10^4 calls will be made to addNum and findMedian"], leetcode_url: "https://leetcode.com/problems/find-median-from-data-stream/",
                approaches: &[ApproachMeta { id: 0, name: "Two Heaps (Small Max-Heap & Large Min-Heap)", time_complexity: "O(log N) add, O(1) find", space_complexity: "O(N)", rationale: "Balancing two heaps keeps the middle elements accessible at the roots in O(1) time.", description: "Balance small max-heap and large min-heap." }],
            },
            Problem::CombinationSum => ProblemDetails {
                id: 39, title: "Combination Sum", difficulty: Difficulty::Medium, category: Category::Backtracking,
                statement: "Given an array of distinct integers candidates and a target integer target, return a list of all unique combinations of candidates where the chosen numbers sum to target.",
                examples: &[Example { input: "candidates = [2,3,6,7], target = 7", output: "[[2,2,3],[7]]", explanation: "The same number may be chosen an unlimited number of times." }],
                constraints: &["1 <= candidates.length <= 30", "2 <= target <= 40"], leetcode_url: "https://leetcode.com/problems/combination-sum/",
                approaches: &[ApproachMeta { id: 0, name: "Backtracking Search Tree", time_complexity: "O(2^T)", space_complexity: "O(T)", rationale: "Branching on including the current candidate multiple times or moving to the next candidate explores valid sum paths.", description: "Recurse exploring combinations with replacement." }],
            },
            Problem::SubsetsII => ProblemDetails {
                id: 90, title: "Subsets II", difficulty: Difficulty::Medium, category: Category::Backtracking,
                statement: "Given an integer array nums that may contain duplicates, return all possible subsets (the power set). The solution set must not contain duplicate subsets.",
                examples: &[Example { input: "nums = [1,2,2]", output: "[[],[1],[1,2],[1,2,2],[2],[2,2]]", explanation: "Skip duplicate elements at the same decision level." }],
                constraints: &["1 <= nums.length <= 10", "-10 <= nums[i] <= 10"], leetcode_url: "https://leetcode.com/problems/subsets-ii/",
                approaches: &[ApproachMeta { id: 0, name: "Sorted Backtracking with Duplicate Pruning", time_complexity: "O(N * 2^N)", space_complexity: "O(N)", rationale: "Sorting nums and skipping adjacent duplicates at the same tree depth prevents duplicate subset branches.", description: "Sort nums and skip duplicate adjacent elements during recursion." }],
            },
            Problem::CombinationSumII => ProblemDetails {
                id: 40, title: "Combination Sum II", difficulty: Difficulty::Medium, category: Category::Backtracking,
                statement: "Given a collection of candidate numbers (candidates) and a target number (target), find all unique combinations in candidates where the candidate numbers sum to target. Each number may only be used once.",
                examples: &[Example { input: "candidates = [10,1,2,7,6,1,5], target = 8", output: "[[1,1,6],[1,2,5],[1,7],[2,6]]", explanation: "Use each element once and skip duplicate branch starts." }],
                constraints: &["1 <= candidates.length <= 100", "1 <= target <= 30"], leetcode_url: "https://leetcode.com/problems/combination-sum-ii/",
                approaches: &[ApproachMeta { id: 0, name: "Backtracking with Pruning & Sorting", time_complexity: "O(2^N)", space_complexity: "O(N)", rationale: "Sorting candidates allows pruning search branches when sum exceeds target and avoiding duplicate combinations.", description: "Sort candidates and skip duplicate branch choices." }],
            },
            Problem::WordSearch => ProblemDetails {
                id: 79, title: "Word Search", difficulty: Difficulty::Medium, category: Category::Backtracking,
                statement: "Given an m x n grid of characters board and a string word, return true if word exists in the grid.",
                examples: &[Example { input: "board = [[\"A\",\"B\",\"C\",\"E\"],[\"S\",\"F\",\"C\",\"S\"],[\"A\",\"D\",\"E\",\"E\"]], word = \"ABCCED\"", output: "true", explanation: "Search adjacent cells grid DFS." }],
                constraints: &["m == board.length", "n == board[i].length", "1 <= word.length <= 15"], leetcode_url: "https://leetcode.com/problems/word-search/",
                approaches: &[ApproachMeta { id: 0, name: "2D Grid Backtracking DFS", time_complexity: "O(N * 4^L)", space_complexity: "O(L)", rationale: "Exploring 4-directional adjacent cells with in-place cell marking backtracks on dead ends.", description: "Grid DFS checking word character match step-by-step." }],
            },
            Problem::NQueens => ProblemDetails {
                id: 51, title: "N-Queens", difficulty: Difficulty::Hard, category: Category::Backtracking,
                statement: "The n-queens puzzle is the problem of placing n queens on an n x n chessboard such that no two queens attack each other.",
                examples: &[Example { input: "n = 4", output: "[\".Q..\",\"...Q\",\"Q...\",\"..Q.\"]", explanation: "Place N non-attacking queens on NxN board." }],
                constraints: &["1 <= n <= 9"], leetcode_url: "https://leetcode.com/problems/n-queens/",
                approaches: &[ApproachMeta { id: 0, name: "Row-by-Row Backtracking with Column & Diagonal Sets", time_complexity: "O(N!)", space_complexity: "O(N)", rationale: "Tracking occupied columns, positive diagonals (r + c), and negative diagonals (r - c) validates queen placements in O(1) per row.", description: "Row-by-row recursion with hashset conflict checks." }],
            },
            Problem::KthLargestArray => ProblemDetails {
                id: 215, title: "Kth Largest Element in an Array", difficulty: Difficulty::Medium, category: Category::HeapPriorityQueue,
                statement: "Given an integer array nums and an integer k, return the kth largest element in the array.",
                examples: &[Example { input: "nums = [3,2,1,5,6,4], k = 2", output: "5", explanation: "Sorted in descending order: 6, 5, 4, 3, 2, 1. The 2nd largest is 5." }],
                constraints: &["1 <= k <= nums.length <= 10^5", "-10^4 <= nums[i] <= 10^4"], leetcode_url: "https://leetcode.com/problems/kth-largest-element-in-an-array/",
                approaches: &[ApproachMeta { id: 0, name: "Min-Heap of Size K / QuickSelect", time_complexity: "O(N log K)", space_complexity: "O(K)", rationale: "Maintaining a Min-Heap of size K leaves the Kth largest element at the root.", description: "Push into Min-Heap of size K." }],
            },
            Problem::DesignTwitter => ProblemDetails {
                id: 355, title: "Design Twitter", difficulty: Difficulty::Medium, category: Category::HeapPriorityQueue,
                statement: "Design a simplified version of Twitter where users can post tweets, follow/unfollow another user, and see the 10 most recent tweets in their news feed.",
                examples: &[Example { input: "postTweet(1, 5), getNewsFeed(1), follow(1, 2), postTweet(2, 6), getNewsFeed(1)", output: "[5], [6, 5]", explanation: "News feed retrieves top 10 recent tweets across followed users using a Max-Heap." }],
                constraints: &["1 <= userId, followerId, followeeId <= 500", "0 <= tweetId <= 10^4"], leetcode_url: "https://leetcode.com/problems/design-twitter/",
                approaches: &[ApproachMeta { id: 0, name: "Max-Heap Feed Merging", time_complexity: "O(K log N)", space_complexity: "O(N)", rationale: "Merging most recent tweets across followed users via Max-Heap returns news feed in O(K log N) time.", description: "Max-Heap merge of followed users' tweet lists." }],
            },
            Problem::PalindromePartitioning => ProblemDetails {
                id: 131, title: "Palindrome Partitioning", difficulty: Difficulty::Medium, category: Category::Backtracking,
                statement: "Given a string s, partition s such that every substring of the partition is a palindrome. Return all possible palindrome partitioning of s.",
                examples: &[Example { input: "s = \"aab\"", output: "[[\"a\",\"a\",\"b\"],[\"aa\",\"b\"]]", explanation: "Explore all valid palindromic prefix cuts." }],
                constraints: &["1 <= s.length <= 16"], leetcode_url: "https://leetcode.com/problems/palindrome-partitioning/",
                approaches: &[ApproachMeta { id: 0, name: "Backtracking Palindrome Check", time_complexity: "O(N * 2^N)", space_complexity: "O(N)", rationale: "Recurse on valid palindromic prefix cuts to partition the string.", description: "Backtrack exploring palindromic prefix slices." }],
            },
            Problem::LetterCombinations => ProblemDetails {
                id: 17, title: "Letter Combinations of a Phone Number", difficulty: Difficulty::Medium, category: Category::Backtracking,
                statement: "Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent.",
                examples: &[Example { input: "digits = \"23\"", output: "[\"ad\",\"ae\",\"af\",\"bd\",\"be\",\"bf\",\"cd\",\"ce\",\"cf\"]", explanation: "Mapping digits 2='abc', 3='def'." }],
                constraints: &["0 <= digits.length <= 4"], leetcode_url: "https://leetcode.com/problems/letter-combinations-of-a-phone-number/",
                approaches: &[ApproachMeta { id: 0, name: "Backtracking Phone Mapping", time_complexity: "O(4^N)", space_complexity: "O(N)", rationale: "Branch on mapping characters for each digit in the input string.", description: "Recurse building string combinations from digit phone keymaps." }],
            },
            Problem::HouseRobberII => ProblemDetails {
                id: 213, title: "House Robber II", difficulty: Difficulty::Medium, category: Category::OneDDp,
                statement: "All houses at this place are arranged in a circle. That means the first house is the neighbor of the last one. Return the maximum amount of money you can rob tonight without alerting the police.",
                examples: &[Example { input: "nums = [2,3,2]", output: "3", explanation: "You cannot rob house 1 (money = 2) and then rob house 3 (money = 2), because they are adjacent houses." }],
                constraints: &["1 <= nums.length <= 100", "0 <= nums[i] <= 1000"], leetcode_url: "https://leetcode.com/problems/house-robber-ii/",
                approaches: &[ApproachMeta { id: 0, name: "Two 1D DP Subproblems (First vs Last House)", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Running House Robber I on nums[1..N] and nums[0..N-1] handles the circular constraint in O(N) time.", description: "Max of robbing sub-arrays nums[1..] and nums[..N-1]." }],
            },
            Problem::LongestPalindromicSubstring => ProblemDetails {
                id: 5, title: "Longest Palindromic Substring", difficulty: Difficulty::Medium, category: Category::OneDDp,
                statement: "Given a string s, return the longest palindromic substring in s.",
                examples: &[Example { input: "s = \"babad\"", output: "\"bab\"", explanation: "\"aba\" is also a valid answer." }],
                constraints: &["1 <= s.length <= 1000"], leetcode_url: "https://leetcode.com/problems/longest-palindromic-substring/",
                approaches: &[ApproachMeta { id: 0, name: "Expand Around Center", time_complexity: "O(N^2)", space_complexity: "O(1)", rationale: "Expanding outward from each character center checks odd and even length palindromes in O(N^2) time and O(1) space.", description: "Expand outward from each index as center." }],
            },
            Problem::PalindromicSubstrings => ProblemDetails {
                id: 647, title: "Palindromic Substrings", difficulty: Difficulty::Medium, category: Category::OneDDp,
                statement: "Given a string s, return the number of palindromic substrings in it.",
                examples: &[Example { input: "s = \"aaa\"", output: "6", explanation: "Six palindromic substrings: \"a\", \"a\", \"a\", \"aa\", \"aa\", \"aaa\"." }],
                constraints: &["1 <= s.length <= 1000"], leetcode_url: "https://leetcode.com/problems/palindromic-substrings/",
                approaches: &[ApproachMeta { id: 0, name: "Expand Around Center Counting", time_complexity: "O(N^2)", space_complexity: "O(1)", rationale: "Count all valid expansions from each center in O(N^2) time.", description: "Increment counter for each valid center expansion." }],
            },
            Problem::DecodeWays => ProblemDetails {
                id: 91, title: "Decode Ways", difficulty: Difficulty::Medium, category: Category::OneDDp,
                statement: "A message containing letters from A-Z can be encoded into numbers using 'A' -> '1' to 'Z' -> '26'. Given a string s containing only digits, return the number of ways to decode it.",
                examples: &[Example { input: "s = \"226\"", output: "3", explanation: "\"226\" could be decoded as \"BZ\" (2 26), \"VF\" (22 6), or \"BBF\" (2 2 6)." }],
                constraints: &["1 <= s.length <= 100"], leetcode_url: "https://leetcode.com/problems/decode-ways/",
                approaches: &[ApproachMeta { id: 0, name: "1D DP (Single vs Double Digit)", time_complexity: "O(N)", space_complexity: "O(N)", rationale: "dp[i] = dp[i+1] (if s[i] != '0') + dp[i+2] (if s[i..i+2] <= 26) calculates total decoding combinations in O(N) time.", description: "Build DP array from right to left checking single and double digit valid codes." }],
            },
            Problem::CoinChange => ProblemDetails {
                id: 322, title: "Coin Change", difficulty: Difficulty::Medium, category: Category::OneDDp,
                statement: "You are given an integer array coins representing coins of different denominations and an integer amount representing a total amount of money. Return the fewest number of coins that you need to make up that amount.",
                examples: &[Example { input: "coins = [1,2,5], amount = 11", output: "3", explanation: "11 = 5 + 5 + 1" }],
                constraints: &["1 <= coins.length <= 12", "0 <= amount <= 10^4"], leetcode_url: "https://leetcode.com/problems/coin-change/",
                approaches: &[ApproachMeta { id: 0, name: "Bottom-Up 1D DP Table", time_complexity: "O(N * amount)", space_complexity: "O(amount)", rationale: "dp[a] = min(dp[a], 1 + dp[a - c]) builds minimum coins needed for all values 1..amount.", description: "Fill dp array of size amount + 1 with min coin transitions." }],
            },
            Problem::MaxProductSubarray => ProblemDetails {
                id: 152, title: "Maximum Product Subarray", difficulty: Difficulty::Medium, category: Category::OneDDp,
                statement: "Given an integer array nums, find a subarray that has the largest product, and return the product.",
                examples: &[Example { input: "nums = [2,3,-2,4]", output: "6", explanation: "[2,3] has the largest product 6." }],
                constraints: &["1 <= nums.length <= 2 * 10^4", "-10 <= nums[i] <= 10"], leetcode_url: "https://leetcode.com/problems/maximum-product-subarray/",
                approaches: &[ApproachMeta { id: 0, name: "Min/Max Dynamic State Tracking", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Tracking current min and max products handles negative number sign flips in O(N) time.", description: "Track curMin and curMax while scanning nums." }],
            },
            Problem::WordBreak => ProblemDetails {
                id: 139, title: "Word Break", difficulty: Difficulty::Medium, category: Category::OneDDp,
                statement: "Given a string s and a dictionary of strings wordDict, return true if s can be segmented into a space-separated sequence of one or more dictionary words.",
                examples: &[Example { input: "s = \"leetcode\", wordDict = [\"leet\",\"code\"]", output: "true", explanation: "Return true because \"leetcode\" can be segmented as \"leet code\"." }],
                constraints: &["1 <= s.length <= 300", "1 <= wordDict.length <= 1000"], leetcode_url: "https://leetcode.com/problems/word-break/",
                approaches: &[ApproachMeta { id: 0, name: "1D DP Suffix Matching", time_complexity: "O(N * M * K)", space_complexity: "O(N)", rationale: "dp[i] = true if s[i..i+w.len()] == w and dp[i+w.len()] == true verifies valid segmentation.", description: "Fill boolean dp array from right to left for dictionary words." }],
            },
            Problem::LongestIncreasingSubsequence => ProblemDetails {
                id: 300, title: "Longest Increasing Subsequence", difficulty: Difficulty::Medium, category: Category::OneDDp,
                statement: "Given an integer array nums, return the length of the longest strictly increasing subsequence.",
                examples: &[Example { input: "nums = [10,9,2,5,3,7,101,18]", output: "4", explanation: "The longest increasing subsequence is [2,3,7,101], length 4." }],
                constraints: &["1 <= nums.length <= 2500", "-10^4 <= nums[i] <= 10^4"], leetcode_url: "https://leetcode.com/problems/longest-increasing-subsequence/",
                approaches: &[ApproachMeta { id: 0, name: "1D DP / Patient Sorting Binary Search", time_complexity: "O(N^2) or O(N log N)", space_complexity: "O(N)", rationale: "dp[i] = max(1, 1 + dp[j]) for j < i where nums[j] < nums[i] computes LIS in O(N^2) or O(N log N) time.", description: "Fill dp array storing longest subsequence ending at index i." }],
            },
            Problem::PartitionEqualSubsetSum => ProblemDetails {
                id: 416, title: "Partition Equal Subset Sum", difficulty: Difficulty::Medium, category: Category::OneDDp,
                statement: "Given an integer array nums, return true if you can partition the array into two subsets such that the sum of the elements in both subsets is equal.",
                examples: &[Example { input: "nums = [1,5,11,5]", output: "true", explanation: "The array can be partitioned as [1, 5, 5] and [11]." }],
                constraints: &["1 <= nums.length <= 200", "1 <= nums[i] <= 100"], leetcode_url: "https://leetcode.com/problems/partition-equal-subset-sum/",
                approaches: &[ApproachMeta { id: 0, name: "0/1 Knapsack DP Set", time_complexity: "O(N * sum)", space_complexity: "O(sum)", rationale: "Target sum is sum(nums) / 2; DP set stores reachable subset sums up to target.", description: "Iterate nums building reachable subset sums set." }],
            },
            Problem::Number1Bits => ProblemDetails {
                id: 191, title: "Number of 1 Bits", difficulty: Difficulty::Easy, category: Category::BitManipulation,
                statement: "Write a function that takes the binary representation of a positive integer and returns the number of set bits it has (also known as Hamming weight).",
                examples: &[Example { input: "n = 11 (binary 00000000000000000000000000001011)", output: "3", explanation: "Total 3 set bits." }],
                constraints: &["1 <= n <= 2^31 - 1"], leetcode_url: "https://leetcode.com/problems/number-of-1-bits/",
                approaches: &[ApproachMeta { id: 0, name: "Bitwise AND n & (n - 1) Clearing", time_complexity: "O(1)", space_complexity: "O(1)", rationale: "Repeatedly clearing the lowest set bit using n &= (n - 1) counts 1-bits in constant O(1) time.", description: "Loop while n != 0 executing n &= (n - 1)." }],
            },
            Problem::SumTwoIntegers => ProblemDetails {
                id: 371, title: "Sum of Two Integers", difficulty: Difficulty::Medium, category: Category::BitManipulation,
                statement: "Given two integers a and b, return the sum of the two integers without using the operators + and -.",
                examples: &[Example { input: "a = 1, b = 2", output: "3", explanation: "Bitwise XOR sum and AND carry bit shifts." }],
                constraints: &["-1000 <= a, b <= 1000"], leetcode_url: "https://leetcode.com/problems/sum-of-two-integers/",
                approaches: &[ApproachMeta { id: 0, name: "Bitwise XOR and Shifted Carry", time_complexity: "O(1)", space_complexity: "O(1)", rationale: "(a ^ b) computes sum without carry; (a & b) << 1 computes carry bits until carry is 0.", description: "Bitwise XOR and left-shift AND carry loop." }],
            },
            Problem::ReverseInteger => ProblemDetails {
                id: 7, title: "Reverse Integer", difficulty: Difficulty::Medium, category: Category::BitManipulation,
                statement: "Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes the value to go outside the signed 32-bit integer range [-2^31, 2^31 - 1], then return 0.",
                examples: &[Example { input: "x = 123", output: "321", explanation: "Reverse digits of 123 to get 321." }],
                constraints: &["-2^31 <= x <= 2^31 - 1"], leetcode_url: "https://leetcode.com/problems/reverse-integer/",
                approaches: &[ApproachMeta { id: 0, name: "Modulo & 32-Bit Overflow Boundary Check", time_complexity: "O(log10 X)", space_complexity: "O(1)", rationale: "Extracting digits via x % 10 and checking 32-bit INT_MAX boundaries before multiplying.", description: "Extract digits with modulo 10 and check overflow." }],
            },
            Problem::RotateImage => ProblemDetails {
                id: 48, title: "Rotate Image", difficulty: Difficulty::Medium, category: Category::MathAndGeometry,
                statement: "You are given an n x n 2D matrix representing an image, rotate the image by 90 degrees (clockwise) in-place.",
                examples: &[Example { input: "matrix = [[1,2,3],[4,5,6],[7,8,9]]", output: "[[7,4,1],[8,5,2],[9,6,3]]", explanation: "Rotate 90 degrees clockwise." }],
                constraints: &["n == matrix.length == matrix[i].length", "1 <= n <= 20"], leetcode_url: "https://leetcode.com/problems/rotate-image/",
                approaches: &[ApproachMeta { id: 0, name: "Matrix Transpose + Reverse Rows", time_complexity: "O(N^2)", space_complexity: "O(1)", rationale: "Transposing the matrix in-place and then reversing each row rotates the image 90 degrees clockwise in O(N^2) time.", description: "Transpose matrix across diagonal and reverse each row." }],
            },
            Problem::SpiralMatrix => ProblemDetails {
                id: 54, title: "Spiral Matrix", difficulty: Difficulty::Medium, category: Category::MathAndGeometry,
                statement: "Given an m x n matrix, return all elements of the matrix in spiral order.",
                examples: &[Example { input: "matrix = [[1,2,3],[4,5,6],[7,8,9]]", output: "[1,2,3,6,9,8,7,4,5]", explanation: "Traverse clockwise inward spiral." }],
                constraints: &["m == matrix.length", "n == matrix[i].length", "1 <= m, n <= 10"], leetcode_url: "https://leetcode.com/problems/spiral-matrix/",
                approaches: &[ApproachMeta { id: 0, name: "4-Boundary Shrinking Traversal", time_complexity: "O(M * N)", space_complexity: "O(1)", rationale: "Maintaining top, bottom, left, and right boundaries and traversing edges inward collects elements in spiral order.", description: "Shrink boundaries top/bottom/left/right while traversing outer edges." }],
            },
            Problem::SetMatrixZeroes => ProblemDetails {
                id: 73, title: "Set Matrix Zeroes", difficulty: Difficulty::Medium, category: Category::MathAndGeometry,
                statement: "Given an m x n integer matrix matrix, if an element is 0, set its entire row and column to 0's. You must do it in-place.",
                examples: &[Example { input: "matrix = [[1,1,1],[1,0,1],[1,1,1]]", output: "[[1,0,1],[0,0,0],[1,0,1]]", explanation: "Set row 1 and column 1 to all zeroes." }],
                constraints: &["m == matrix.length", "n == matrix[0].length", "1 <= m, n <= 200"], leetcode_url: "https://leetcode.com/problems/set-matrix-zeroes/",
                approaches: &[ApproachMeta { id: 0, name: "First Row/Column State Flags", time_complexity: "O(M * N)", space_complexity: "O(1)", rationale: "Using the matrix's first row and first column to store zero flags achieves in-place O(1) extra space complexity.", description: "Mark first row and col as zero flags." }],
            },
            Problem::PowXN => ProblemDetails {
                id: 50, title: "Pow(x, n)", difficulty: Difficulty::Medium, category: Category::MathAndGeometry,
                statement: "Implement pow(x, n), which calculates x raised to the power n (i.e., x^n).",
                examples: &[Example { input: "x = 2.00000, n = 10", output: "1024.00000", explanation: "2^10 = 1024." }],
                constraints: &["-100.0 < x < 100.0", "-2^31 <= n <= 2^31 - 1"], leetcode_url: "https://leetcode.com/problems/powx-n/",
                approaches: &[ApproachMeta { id: 0, name: "Binary Exponentiation (Divide & Conquer)", time_complexity: "O(log N)", space_complexity: "O(log N)", rationale: "Dividing n by 2 recursively computes x^n in logarithmic O(log N) time.", description: "Divide and conquer squaring x when n is even." }],
            },
            Problem::MultiplyStrings => ProblemDetails {
                id: 43, title: "Multiply Strings", difficulty: Difficulty::Medium, category: Category::MathAndGeometry,
                statement: "Given two non-negative integers num1 and num2 represented as strings, return the product of num1 and num2, also represented as a string.",
                examples: &[Example { input: "num1 = \"2\", num2 = \"3\"", output: "\"6\"", explanation: "2 * 3 = 6." }],
                constraints: &["1 <= num1.length, num2.length <= 200"], leetcode_url: "https://leetcode.com/problems/multiply-strings/",
                approaches: &[ApproachMeta { id: 0, name: "Positional Grade-School Digit Array", time_complexity: "O(N * M)", space_complexity: "O(N + M)", rationale: "Product of digit at num1[i] and num2[j] places result at res[i + j + 1].", description: "Digit-by-digit multiplication with carry array." }],
            },
            Problem::DetectSquares => ProblemDetails {
                id: 2013, title: "Detect Squares", difficulty: Difficulty::Medium, category: Category::MathAndGeometry,
                statement: "Design a data structure that accepts a stream of 2D points and counts the number of ways to form axis-aligned squares with a given query point.",
                examples: &[Example { input: "add([3, 10]), add([11, 2]), add([3, 2]), count([11, 10])", output: "1", explanation: "Query point [11, 10] forms 1 square of side length 8." }],
                constraints: &["0 <= x, y <= 1000"], leetcode_url: "https://leetcode.com/problems/detect-squares/",
                approaches: &[ApproachMeta { id: 0, name: "Point Frequency Map & Diagonal Search", time_complexity: "O(N) count", space_complexity: "O(N)", rationale: "Searching points with matching dx == dy diagonal distance counts valid 4-corner squares in O(N) time.", description: "Store point frequencies in hash map and count diagonal square matches." }],
            },
            Problem::MaximumSubarray => ProblemDetails {
                id: 53, title: "Maximum Subarray", difficulty: Difficulty::Medium, category: Category::Greedy,
                statement: "Given an integer array nums, find the subarray with the largest sum, and return its sum.",
                examples: &[Example { input: "nums = [-2,1,-3,4,-1,2,1,-5,4]", output: "6", explanation: "The subarray [4,-1,2,1] has the largest sum 6." }],
                constraints: &["1 <= nums.length <= 10^5"], leetcode_url: "https://leetcode.com/problems/maximum-subarray/",
                approaches: &[ApproachMeta { id: 0, name: "Kadane's Algorithm", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Tracking curSum and resetting to 0 when negative computes maximum subarray sum in linear time.", description: "Track curSum resetting at negative values." }],
            },
            Problem::JumpGame => ProblemDetails {
                id: 55, title: "Jump Game", difficulty: Difficulty::Medium, category: Category::Greedy,
                statement: "You are given an integer array nums. You are initially positioned at the array's first index, and each element in the array represents your maximum jump length at that position. Return true if you can reach the last index.",
                examples: &[Example { input: "nums = [2,3,1,1,4]", output: "true", explanation: "Jump 1 step from index 0 to 1, then 3 steps to the last index." }],
                constraints: &["1 <= nums.length <= 10^4"], leetcode_url: "https://leetcode.com/problems/jump-game/",
                approaches: &[ApproachMeta { id: 0, name: "Greedy Backwards Goal Shift", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Moving target goal backward from last index to 0 verifies reachability in O(N) time.", description: "Shift target goal backward if index + nums[i] >= goal." }],
            },
            Problem::JumpGameII => ProblemDetails {
                id: 45, title: "Jump Game II", difficulty: Difficulty::Medium, category: Category::Greedy,
                statement: "Return the minimum number of jumps to reach nums[n - 1].",
                examples: &[Example { input: "nums = [2,3,1,1,4]", output: "2", explanation: "The minimum number of jumps to reach the last index is 2." }],
                constraints: &["1 <= nums.length <= 10^4"], leetcode_url: "https://leetcode.com/problems/jump-game-ii/",
                approaches: &[ApproachMeta { id: 0, name: "BFS Level Window Greed", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Tracking current jump window [l, r] and farthest reachable index computes min jumps in O(N) time.", description: "Track current level window and farthest reach." }],
            },
            Problem::GasStation => ProblemDetails {
                id: 134, title: "Gas Station", difficulty: Difficulty::Medium, category: Category::Greedy,
                statement: "There are n gas stations along a circular route. Return the starting gas station's index if you can travel around the circuit once in the clockwise direction, otherwise return -1.",
                examples: &[Example { input: "gas = [1,2,3,4,5], cost = [3,4,5,1,2]", output: "3", explanation: "Start at station 3 (index 3) and fill up with 4 unit of gas." }],
                constraints: &["n == gas.length == cost.length"], leetcode_url: "https://leetcode.com/problems/gas-station/",
                approaches: &[ApproachMeta { id: 0, name: "Total Balance & Reset Start Index", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "If sum(gas) >= sum(cost), a solution is guaranteed; resetting start index when total tank drops below 0 finds it in O(N).", description: "Reset start index when accumulated tank < 0." }],
            },
            Problem::HandOfStraights => ProblemDetails {
                id: 846, title: "Hand of Straights", difficulty: Difficulty::Medium, category: Category::Greedy,
                statement: "Alice has a card hand given as an integer array. Rearrange the cards into groups so that each group is of size groupSize, and consists of groupSize consecutive cards.",
                examples: &[Example { input: "hand = [1,2,3,6,2,3,4,7,8], groupSize = 3", output: "true", explanation: "Hand can be rearranged as [1,2,3],[2,3,4],[6,7,8]." }],
                constraints: &["1 <= hand.length <= 10^4"], leetcode_url: "https://leetcode.com/problems/hand-of-straights/",
                approaches: &[ApproachMeta { id: 0, name: "Frequency Min-Heap / Sorted Map Greed", time_complexity: "O(N log N)", space_complexity: "O(N)", rationale: "Starting from the smallest available card and forming consecutive groups of groupSize.", description: "Greedily form groups from smallest card available." }],
            },
            Problem::MergeTriplets => ProblemDetails {
                id: 1899, title: "Merge Triplets to Form Target Triplet", difficulty: Difficulty::Medium, category: Category::Greedy,
                statement: "Given a 2D integer array triplets and an integer array target, return true if it is possible to obtain target by merging triplets.",
                examples: &[Example { input: "triplets = [[2,5,3],[2,3,4],[1,2,5],[5,2,3]], target = [5,5,5]", output: "true", explanation: "Merge triplets to form [5,5,5]." }],
                constraints: &["1 <= triplets.length <= 10^5"], leetcode_url: "https://leetcode.com/problems/merge-triplets-to-form-target-triplet/",
                approaches: &[ApproachMeta { id: 0, name: "Filter Oversized Triplets & Match Target Values", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Ignoring triplets with any value > target[i] and checking if remaining triplets cover all target values.", description: "Filter out invalid triplets and check target match coverage." }],
            },
            Problem::PartitionLabels => ProblemDetails {
                id: 763, title: "Partition Labels", difficulty: Difficulty::Medium, category: Category::Greedy,
                statement: "You are given a string s. We want to partition the string into as many parts as possible so that each letter appears in at most one part.",
                examples: &[Example { input: "s = \"ababcbacadefegdehijhklij\"", output: "[9,7,8]", explanation: "Partitions are \"ababcbaca\", \"defegde\", \"hijhklij\"." }],
                constraints: &["1 <= s.length <= 500"], leetcode_url: "https://leetcode.com/problems/partition-labels/",
                approaches: &[ApproachMeta { id: 0, name: "Last Index Hash Map & Window End Tracking", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Tracking the last occurrences of each character and expanding partition boundary until end == current index.", description: "Expand partition end boundary to max last-occurrence index." }],
            },
            Problem::ValidParenthesisString => ProblemDetails {
                id: 678, title: "Valid Parenthesis String", difficulty: Difficulty::Medium, category: Category::Greedy,
                statement: "Given a string s containing only '(', ')' and '*', return true if s is valid.",
                examples: &[Example { input: "s = \"(*)\"", output: "true", explanation: "'*' can act as closing parenthesis." }],
                constraints: &["1 <= s.length <= 100"], leetcode_url: "https://leetcode.com/problems/valid-parenthesis-string/",
                approaches: &[ApproachMeta { id: 0, name: "Min/Max Open Count Range Greed", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Tracking minOpen and maxOpen count range handles flexible '*' wildcard matches in O(N) time.", description: "Maintain [minOpen, maxOpen] count range." }],
            },
            Problem::InsertInterval => ProblemDetails {
                id: 57, title: "Insert Interval", difficulty: Difficulty::Medium, category: Category::Intervals,
                statement: "You are given an array of non-overlapping intervals intervals where intervals[i] = [starti, endi] sorted in ascending order by starti. Insert newInterval into intervals such that intervals is still sorted.",
                examples: &[Example { input: "intervals = [[1,3],[6,9]], newInterval = [2,5]", output: "[[1,5],[6,9]]", explanation: "Merge newInterval [2,5] with [1,3] into [1,5]." }],
                constraints: &["0 <= intervals.length <= 10^4"], leetcode_url: "https://leetcode.com/problems/insert-interval/",
                approaches: &[ApproachMeta { id: 0, name: "Three-Phase Linear Scan", time_complexity: "O(N)", space_complexity: "O(N)", rationale: "Collecting left non-overlapping intervals, merging overlapping intervals with newInterval, and appending right non-overlapping intervals.", description: "3-phase scan: left, merge overlapping, right." }],
            },
            Problem::MergeIntervals => ProblemDetails {
                id: 56, title: "Merge Intervals", difficulty: Difficulty::Medium, category: Category::Intervals,
                statement: "Given an array of intervals where intervals[i] = [starti, endi], merge all overlapping intervals.",
                examples: &[Example { input: "intervals = [[1,3],[2,6],[8,10],[15,18]]", output: "[[1,6],[8,10],[15,18]]", explanation: "[1,3] and [2,6] overlap into [1,6]." }],
                constraints: &["1 <= intervals.length <= 10^4"], leetcode_url: "https://leetcode.com/problems/merge-intervals/",
                approaches: &[ApproachMeta { id: 0, name: "Sort by Start & Merge Adjacent", time_complexity: "O(N log N)", space_complexity: "O(N)", rationale: "Sorting by start time allows merging overlapping intervals in a single linear pass.", description: "Sort intervals by start time and merge adjacent overlaps." }],
            },
            Problem::NonOverlappingIntervals => ProblemDetails {
                id: 435, title: "Non-overlapping Intervals", difficulty: Difficulty::Medium, category: Category::Intervals,
                statement: "Given an array of intervals intervals where intervals[i] = [starti, endi], return the minimum number of intervals you need to remove to make the rest of the intervals non-overlapping.",
                examples: &[Example { input: "intervals = [[1,2],[2,3],[3,4],[1,3]]", output: "1", explanation: "[1,3] can be removed and the rest of the intervals are non-overlapping." }],
                constraints: &["1 <= intervals.length <= 10^5"], leetcode_url: "https://leetcode.com/problems/non-overlapping-intervals/",
                approaches: &[ApproachMeta { id: 0, name: "Greedy Earliest End Time Selection", time_complexity: "O(N log N)", space_complexity: "O(1)", rationale: "Sorting by end time and keeping interval with smaller end time minimizes overlaps.", description: "Sort intervals by start time; remove interval with larger end time when overlapping." }],
            },
            Problem::MeetingRoomsII => ProblemDetails {
                id: 253, title: "Meeting Rooms II", difficulty: Difficulty::Medium, category: Category::Intervals,
                statement: "Given an array of meeting time intervals intervals where intervals[i] = [starti, endi], return the minimum number of conference rooms required.",
                examples: &[Example { input: "intervals = [[0,30],[5,10],[15,20]]", output: "2", explanation: "Room 1: [0,30]; Room 2: [5,10], [15,20]." }],
                constraints: &["1 <= intervals.length <= 10^4"], leetcode_url: "https://leetcode.com/problems/meeting-rooms-ii/",
                approaches: &[ApproachMeta { id: 0, name: "Two Pointers / Min-Heap Active Meeting Count", time_complexity: "O(N log N)", space_complexity: "O(N)", rationale: "Sorting start and end times separately and using two pointers tracks simultaneous active meetings.", description: "Two pointers on sorted start and end time arrays." }],
            },
            Problem::MinIntervalQuery => ProblemDetails {
                id: 1851, title: "Minimum Interval to Include Each Query", difficulty: Difficulty::Hard, category: Category::Intervals,
                statement: "Given 2D integer array intervals and queries array, return smallest interval length containing each query.",
                examples: &[Example { input: "intervals = [[1,4],[2,4],[3,6],[4,4]], queries = [2,3,4,5]", output: "[3,3,1,4]", explanation: "Query 2: smallest interval is [2,4] length 3." }],
                constraints: &["1 <= intervals.length, queries.length <= 10^5"], leetcode_url: "https://leetcode.com/problems/minimum-interval-to-include-each-query/",
                approaches: &[ApproachMeta { id: 0, name: "Offline Query Sorting & Priority Queue", time_complexity: "O(N log N + Q log Q)", space_complexity: "O(N + Q)", rationale: "Sorting queries and pushing valid intervals into min-heap ordered by length.", description: "Process sorted queries with min-heap of active interval lengths." }],
            },
            Problem::NumberIslands => ProblemDetails {
                id: 200, title: "Number of Islands", difficulty: Difficulty::Medium, category: Category::Graphs,
                statement: "Given an m x n 2D binary grid grid which represents a map of '1's (land) and '0's (water), return the number of islands.",
                examples: &[Example { input: "grid = [[\"1\",\"1\",\"1\",\"1\",\"0\"],[\"1\",\"1\",\"0\",\"1\",\"0\"],[\"1\",\"1\",\"0\",\"0\",\"0\"],[\"0\",\"0\",\"0\",\"0\",\"0\"]]", output: "1", explanation: "1 connected land mass." }],
                constraints: &["m == grid.length", "n == grid[i].length", "1 <= m, n <= 300"], leetcode_url: "https://leetcode.com/problems/number-of-islands/",
                approaches: &[ApproachMeta { id: 0, name: "BFS / DFS Grid Traversal", time_complexity: "O(M * N)", space_complexity: "O(M * N)", rationale: "Traversing connected land components using BFS/DFS counts unique islands in O(M * N) time.", description: "DFS/BFS flood fill set land cells to '0'." }],
            },
            Problem::MaxAreaIsland => ProblemDetails {
                id: 695, title: "Max Area of Island", difficulty: Difficulty::Medium, category: Category::Graphs,
                statement: "You are given an m x n binary matrix grid. An island is a group of 1's (representing land) connected 4-directionally. Return the maximum area of an island in grid.",
                examples: &[Example { input: "grid = [[0,0,1,0,0],[0,0,0,0,0],[0,1,1,1,0],[0,0,0,0,0]]", output: "3", explanation: "Max area island has 3 connected land cells." }],
                constraints: &["m == grid.length", "n == grid[i].length"], leetcode_url: "https://leetcode.com/problems/max-area-of-island/",
                approaches: &[ApproachMeta { id: 0, name: "DFS Connected Component Area Sum", time_complexity: "O(M * N)", space_complexity: "O(M * N)", rationale: "Accumulating connected land cell counts during DFS exploration tracks max island area.", description: "Sum 1 + dfs(up) + dfs(down) + dfs(left) + dfs(right)." }],
            },
            Problem::CloneGraph => ProblemDetails {
                id: 133, title: "Clone Graph", difficulty: Difficulty::Medium, category: Category::Graphs,
                statement: "Given a reference of a node in a connected undirected graph. Return a deep copy (clone) of the graph.",
                examples: &[Example { input: "adjList = [[2,4],[1,3],[2,4],[1,3]]", output: "[[2,4],[1,3],[2,4],[1,3]]", explanation: "Deep copy connected node graph structure." }],
                constraints: &["The number of nodes in the graph is in the range [0, 100]."], leetcode_url: "https://leetcode.com/problems/clone-graph/",
                approaches: &[ApproachMeta { id: 0, name: "DFS / BFS Hash Map Node Mapping", time_complexity: "O(V + E)", space_complexity: "O(V)", rationale: "Using a hash map to map old nodes to new cloned nodes prevents infinite recursion and handles cycles.", description: "Map old_node -> new_node and recursively clone neighbors." }],
            },
            Problem::WallsAndGates => ProblemDetails {
                id: 286, title: "Walls and Gates", difficulty: Difficulty::Medium, category: Category::Graphs,
                statement: "Fill each empty room (INF) with the distance to its nearest gate (0). If it is impossible to reach a gate, it should be filled with INF.",
                examples: &[Example { input: "rooms = [[2147483647,-1,0,2147483647],[2147483647,2147483647,2147483647,-1]]", output: "[[3,-1,0,1],[2,2,1,-1]]", explanation: "Fill empty rooms with shortest distance to gate 0." }],
                constraints: &["m == rooms.length", "n == rooms[i].length"], leetcode_url: "https://leetcode.com/problems/walls-and-gates/",
                approaches: &[ApproachMeta { id: 0, name: "Multi-Source BFS from Gates", time_complexity: "O(M * N)", space_complexity: "O(M * N)", rationale: "Enqueuing all gates simultaneously and expanding level-by-level computes shortest distance to gates in O(M * N) time.", description: "Multi-source BFS queue initialized with all gate (0) coordinates." }],
            },
            Problem::RottingOranges => ProblemDetails {
                id: 994, title: "Rotting Oranges", difficulty: Difficulty::Medium, category: Category::Graphs,
                statement: "Return the minimum number of minutes that must elapse until no cell has a fresh orange. If this is impossible, return -1.",
                examples: &[Example { input: "grid = [[2,1,1],[1,1,0],[0,1,1]]", output: "4", explanation: "Fresh oranges rot in 4 minutes." }],
                constraints: &["m == grid.length", "n == grid[i].length"], leetcode_url: "https://leetcode.com/problems/rotting-oranges/",
                approaches: &[ApproachMeta { id: 0, name: "Multi-Source BFS Minute Level Tracking", time_complexity: "O(M * N)", space_complexity: "O(M * N)", rationale: "Multi-source BFS enqueuing rotten oranges (2) tracks minutes until all fresh oranges (1) turn rotten.", description: "Level-by-level BFS from all rotten orange positions." }],
            },
            Problem::PacificAtlantic => ProblemDetails {
                id: 417, title: "Pacific Atlantic Water Flow", difficulty: Difficulty::Medium, category: Category::Graphs,
                statement: "Find the list of grid coordinates where water can flow to both the Pacific and Atlantic oceans.",
                examples: &[Example { input: "heights = [[1,2,2,3,5],[3,2,3,4,4],[2,4,5,3,1],[6,7,1,4,5],[5,1,1,2,4]]", output: "[[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[4,0]]", explanation: "Coordinates where water flows outward to both oceans." }],
                constraints: &["m == heights.length", "n == heights[i].length"], leetcode_url: "https://leetcode.com/problems/pacific-atlantic-water-flow/",
                approaches: &[ApproachMeta { id: 0, name: "Reverse Ocean Boundary DFS", time_complexity: "O(M * N)", space_complexity: "O(M * N)", rationale: "Starting DFS inward from Pacific and Atlantic ocean edges finds reachable cells; intersection is the answer.", description: "Reverse DFS from ocean borders uphill." }],
            },
            Problem::SurroundedRegions => ProblemDetails {
                id: 130, title: "Surrounded Regions", difficulty: Difficulty::Medium, category: Category::Graphs,
                statement: "Given an m x n matrix board containing 'X' and 'O', capture all regions that are 4-directionally surrounded by 'X'.",
                examples: &[Example { input: "board = [[\"X\",\"X\",\"X\",\"X\"],[\"X\",\"O\",\"O\",\"X\"],[\"X\",\"X\",\"O\",\"X\"],[\"X\",\"O\",\"X\",\"X\"]]", output: "[[\"X\",\"X\",\"X\",\"X\"],[\"X\",\"X\",\"X\",\"X\"],[\"X\",\"X\",\"X\",\"X\"],[\"X\",\"O\",\"X\",\"X\"]]", explanation: "Capture non-border surrounded 'O' regions." }],
                constraints: &["m == board.length", "n == board[i].length"], leetcode_url: "https://leetcode.com/problems/surrounded-regions/",
                approaches: &[ApproachMeta { id: 0, name: "Unsurrounded Border DFS Capture", time_complexity: "O(M * N)", space_complexity: "O(M * N)", rationale: "Marking border-connected 'O's as temporary 'T' via DFS leaves remaining 'O's surrounded to flip to 'X'.", description: "Mark border 'O's as 'T', flip remaining 'O' to 'X', then 'T' back to 'O'." }],
            },
            Problem::CourseSchedule => ProblemDetails {
                id: 207, title: "Course Schedule", difficulty: Difficulty::Medium, category: Category::Graphs,
                statement: "There are a total of numCourses courses you have to take, labeled from 0 to numCourses - 1. Return true if you can finish all courses.",
                examples: &[Example { input: "numCourses = 2, prerequisites = [[1,0]]", output: "true", explanation: "To take course 1 you should have finished course 0. So it is possible." }],
                constraints: &["1 <= numCourses <= 2000"], leetcode_url: "https://leetcode.com/problems/course-schedule/",
                approaches: &[ApproachMeta { id: 0, name: "Kahn's Topological Sort / DFS Cycle Detection", time_complexity: "O(V + E)", space_complexity: "O(V + E)", rationale: "Detecting directed cycles in the prerequisite graph verifies if a valid course order exists.", description: "Detect directed graph cycles using DFS visit states or indegrees." }],
            },
            Problem::CourseScheduleII => ProblemDetails {
                id: 210, title: "Course Schedule II", difficulty: Difficulty::Medium, category: Category::Graphs,
                statement: "Return the ordering of courses you should take to finish all courses. If it is impossible to finish all courses, return an empty array.",
                examples: &[Example { input: "numCourses = 2, prerequisites = [[1,0]]", output: "[0,1]", explanation: "Course 0 then course 1." }],
                constraints: &["1 <= numCourses <= 2000"], leetcode_url: "https://leetcode.com/problems/course-schedule-ii/",
                approaches: &[ApproachMeta { id: 0, name: "Topological Sort Order", time_complexity: "O(V + E)", space_complexity: "O(V + E)", rationale: "Kahn's BFS queue or DFS post-order reversal yields valid course completion sequence.", description: "Append course nodes to topological order queue." }],
            },
            Problem::GraphValidTree => ProblemDetails {
                id: 261, title: "Graph Valid Tree", difficulty: Difficulty::Medium, category: Category::Graphs,
                statement: "Given n nodes labeled from 0 to n - 1 and a list of undirected edges, write a function to check whether these edges make up a valid tree.",
                examples: &[Example { input: "n = 5, edges = [[0,1],[0,2],[0,3],[1,4]]", output: "true", explanation: "Graph is fully connected and has no cycles." }],
                constraints: &["1 <= n <= 2000"], leetcode_url: "https://leetcode.com/problems/graph-valid-tree/",
                approaches: &[ApproachMeta { id: 0, name: "Union-Find / DFS Cycle & Component Check", time_complexity: "O(V + E)", space_complexity: "O(V)", rationale: "A graph is a valid tree if E == V - 1 and all nodes are connected in a single component without cycles.", description: "Verify edges == n - 1 and single connected component." }],
            },
            Problem::ConnectedComponents => ProblemDetails {
                id: 323, title: "Number of Connected Components in an Undirected Graph", difficulty: Difficulty::Medium, category: Category::Graphs,
                statement: "Given n nodes and an array of undirected edges, return the number of connected components in the graph.",
                examples: &[Example { input: "n = 5, edges = [[0,1],[1,2],[3,4]]", output: "2", explanation: "Components are {0,1,2} and {3,4}." }],
                constraints: &["1 <= n <= 2000"], leetcode_url: "https://leetcode.com/problems/number-of-connected-components-in-an-undirected-graph/",
                approaches: &[ApproachMeta { id: 0, name: "Union-Find Disjoint Set", time_complexity: "O(V + E * alpha(V))", space_complexity: "O(V)", rationale: "Union-Find decrements component count upon uniting distinct node sets.", description: "Initialize V components and decrement on union(u, v)." }],
            },
            Problem::RedundantConnection => ProblemDetails {
                id: 684, title: "Redundant Connection", difficulty: Difficulty::Medium, category: Category::Graphs,
                statement: "Return an edge that can be removed so that the resulting graph is a tree of n nodes.",
                examples: &[Example { input: "edges = [[1,2],[1,3],[2,3]]", output: "[2,3]", explanation: "[2,3] creates a cycle in the graph." }],
                constraints: &["n == edges.length", "3 <= n <= 1000"], leetcode_url: "https://leetcode.com/problems/redundant-connection/",
                approaches: &[ApproachMeta { id: 0, name: "Union-Find Cycle Edge Identification", time_complexity: "O(N * alpha(N))", space_complexity: "O(N)", rationale: "The first edge connecting two already-united nodes forms the redundant cycle edge.", description: "Return edge where find(u) == find(v)." }],
            },
            Problem::WordLadder => ProblemDetails {
                id: 127, title: "Word Ladder", difficulty: Difficulty::Hard, category: Category::Graphs,
                statement: "Given two words, beginWord and endWord, and a dictionary wordList, return the number of words in the shortest transformation sequence from beginWord to endWord, or 0 if no such sequence exists.",
                examples: &[Example { input: "beginWord = \"hit\", endWord = \"cog\", wordList = [\"hot\",\"dot\",\"dog\",\"lot\",\"log\",\"cog\"]", output: "5", explanation: "Shortest transformation: hit -> hot -> dot -> dog -> cog (5 words)." }],
                constraints: &["1 <= beginWord.length <= 10", "1 <= wordList.length <= 5000"], leetcode_url: "https://leetcode.com/problems/word-ladder/",
                approaches: &[ApproachMeta { id: 0, name: "BFS Shortest Path Transformation Graph", time_complexity: "O(N * M^2)", space_complexity: "O(N * M^2)", rationale: "Building wildcard pattern adjacency buckets and performing BFS guarantees shortest transformation path.", description: "BFS on single-character pattern buckets." }],
            },
            Problem::UniquePaths => ProblemDetails {
                id: 62, title: "Unique Paths", difficulty: Difficulty::Medium, category: Category::TwoDDp,
                statement: "There is a robot on an m x n grid. The robot is initially located at the top-left corner (grid[0][0]) and tries to move to the bottom-right corner (grid[m-1][n-1]). Return the number of possible unique paths.",
                examples: &[Example { input: "m = 3, n = 7", output: "28", explanation: "Total 28 unique grid paths from top-left to bottom-right." }],
                constraints: &["1 <= m, n <= 100"], leetcode_url: "https://leetcode.com/problems/unique-paths/",
                approaches: &[ApproachMeta { id: 0, name: "2D Grid Dynamic Programming", time_complexity: "O(M * N)", space_complexity: "O(M * N)", rationale: "dp[r][c] = dp[r+1][c] + dp[r][c+1] accumulates path counts bottom-up in O(M*N) time.", description: "2D DP grid accumulation." }],
            },
            Problem::LongestCommonSubsequence => ProblemDetails {
                id: 1143, title: "Longest Common Subsequence", difficulty: Difficulty::Medium, category: Category::TwoDDp,
                statement: "Given two strings text1 and text2, return the length of their longest common subsequence.",
                examples: &[Example { input: "text1 = \"abcde\", text2 = \"ace\"", output: "3", explanation: "The longest common subsequence is \"ace\" of length 3." }],
                constraints: &["1 <= text1.length, text2.length <= 1000"], leetcode_url: "https://leetcode.com/problems/longest-common-subsequence/",
                approaches: &[ApproachMeta { id: 0, name: "2D Matrix DP Matching", time_complexity: "O(M * N)", space_complexity: "O(M * N)", rationale: "Matching chars adds 1 + dp[i+1][j+1], while mismatches take max(dp[i+1][j], dp[i][j+1]).", description: "2D DP char comparison table." }],
            },
            Problem::BestTimeStockCooldown => ProblemDetails {
                id: 309, title: "Best Time to Buy and Sell Stock with Cooldown", difficulty: Difficulty::Medium, category: Category::TwoDDp,
                statement: "Find the maximum profit you can achieve with stock transactions given that after you sell stock, you cannot buy stock on the next day (cooldown 1 day).",
                examples: &[Example { input: "prices = [1,2,3,0,2]", output: "3", explanation: "Transactions: [buy, sell, cooldown, buy, sell]." }],
                constraints: &["1 <= prices.length <= 5000"], leetcode_url: "https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/",
                approaches: &[ApproachMeta { id: 0, name: "State Machine DP / Memoization", time_complexity: "O(N)", space_complexity: "O(N)", rationale: "Caching (day, buying_state) prevents redundant recursive branch evaluations in linear O(N) time.", description: "Buying vs selling state memoization." }],
            },
            Problem::CoinChangeII => ProblemDetails {
                id: 518, title: "Coin Change II", difficulty: Difficulty::Medium, category: Category::TwoDDp,
                statement: "Return the number of combinations that make up an amount using given coins of different denominations.",
                examples: &[Example { input: "amount = 5, coins = [1,2,5]", output: "4", explanation: "5=5, 5=2+2+1, 5=2+1+1+1, 5=1+1+1+1+1." }],
                constraints: &["1 <= amount <= 5000", "1 <= coins.length <= 300"], leetcode_url: "https://leetcode.com/problems/coin-change-ii/",
                approaches: &[ApproachMeta { id: 0, name: "Unbounded Knapsack 2D DP Table", time_complexity: "O(N * Amount)", space_complexity: "O(Amount)", rationale: "Processing coins iteratively avoids duplicate permutations, yielding total distinct combinations.", description: "Bottom-up combinations table." }],
            },
            Problem::TargetSum => ProblemDetails {
                id: 494, title: "Target Sum", difficulty: Difficulty::Medium, category: Category::TwoDDp,
                statement: "Build an expression out of nums by adding '+' or '-' before each integer so that the expression evaluates to target. Return the number of different expressions.",
                examples: &[Example { input: "nums = [1,1,1,1,1], target = 3", output: "5", explanation: "5 ways to assign signs to sum to 3." }],
                constraints: &["1 <= nums.length <= 20"], leetcode_url: "https://leetcode.com/problems/target-sum/",
                approaches: &[ApproachMeta { id: 0, name: "Subset Sum 2D DP Memoization", time_complexity: "O(N * TotalSum)", space_complexity: "O(N * TotalSum)", rationale: "Caching (index, current_sum) avoids 2^N branch recalculations.", description: "Subproblem sum memoization." }],
            },
            Problem::InterleavingString => ProblemDetails {
                id: 97, title: "Interleaving String", difficulty: Difficulty::Medium, category: Category::TwoDDp,
                statement: "Given strings s1, s2, and s3, find whether s3 is formed by an interleaving of s1 and s2.",
                examples: &[Example { input: "s1 = \"aabcc\", s2 = \"dbbca\", s3 = \"aadbbcbcac\"", output: "true", explanation: "s3 contains interleaved chars from s1 and s2." }],
                constraints: &["0 <= s1.length, s2.length <= 100"], leetcode_url: "https://leetcode.com/problems/interleaving-string/",
                approaches: &[ApproachMeta { id: 0, name: "2D Grid DP Reachability Matrix", time_complexity: "O(M * N)", space_complexity: "O(M * N)", rationale: "dp[i][j] is true if s1[i] == s3[i+j] and dp[i+1][j] is true, or s2[j] == s3[i+j] and dp[i][j+1] is true.", description: "2D interleaving boolean grid." }],
            },
            Problem::LongestIncreasingPath => ProblemDetails {
                id: 329, title: "Longest Increasing Path in a Matrix", difficulty: Difficulty::Hard, category: Category::TwoDDp,
                statement: "Given an m x n integers matrix, return the length of the longest increasing path in matrix.",
                examples: &[Example { input: "matrix = [[9,9,4],[6,6,8],[2,1,1]]", output: "4", explanation: "Longest increasing path is [1, 2, 6, 9]." }],
                constraints: &["1 <= m, n <= 200"], leetcode_url: "https://leetcode.com/problems/longest-increasing-path-in-a-matrix/",
                approaches: &[ApproachMeta { id: 0, name: "DFS + Memoization Matrix DP", time_complexity: "O(M * N)", space_complexity: "O(M * N)", rationale: "Caching longest path lengths at cell (r, c) ensures each cell's maximum increasing streak is computed once.", description: "Memoized 2D grid DFS." }],
            },
            Problem::DistinctSubsequences => ProblemDetails {
                id: 115, title: "Distinct Subsequences", difficulty: Difficulty::Hard, category: Category::TwoDDp,
                statement: "Given two strings s and t, return the number of distinct subsequences of s which equals t.",
                examples: &[Example { input: "s = \"rabbbit\", t = \"rabbit\"", output: "3", explanation: "3 ways to select characters in s to form t." }],
                constraints: &["1 <= s.length, t.length <= 1000"], leetcode_url: "https://leetcode.com/problems/distinct-subsequences/",
                approaches: &[ApproachMeta { id: 0, name: "2D Matching Count DP Table", time_complexity: "O(M * N)", space_complexity: "O(M * N)", rationale: "If s[i] == t[j], dp[i][j] = dp[i+1][j+1] + dp[i+1][j], else dp[i+1][j].", description: "Bottom-up string matching table." }],
            },
            Problem::EditDistance => ProblemDetails {
                id: 72, title: "Edit Distance", difficulty: Difficulty::Medium, category: Category::TwoDDp,
                statement: "Given two strings word1 and word2, return the minimum number of operations (insert, delete, replace) required to convert word1 to word2.",
                examples: &[Example { input: "word1 = \"horse\", word2 = \"ros\"", output: "3", explanation: "horse -> rorse (replace) -> rose (remove r) -> ros (remove e)." }],
                constraints: &["0 <= word1.length, word2.length <= 500"], leetcode_url: "https://leetcode.com/problems/edit-distance/",
                approaches: &[ApproachMeta { id: 0, name: "Levenshtein 2D Distance Matrix", time_complexity: "O(M * N)", space_complexity: "O(M * N)", rationale: "dp[i][j] = 1 + min(insert, delete, replace) computes minimum edit steps systematically.", description: "2D edit operations matrix." }],
            },
            Problem::BurstBalloons => ProblemDetails {
                id: 312, title: "Burst Balloons", difficulty: Difficulty::Hard, category: Category::TwoDDp,
                statement: "You are given n balloons, indexed from 0 to n - 1. Each balloon is painted with a number on it represented by array nums. Return the maximum coins you can collect by bursting the balloons wisely.",
                examples: &[Example { input: "nums = [3,1,5,8]", output: "167", explanation: "Optimal order: burst 1, then 5, then 3, then 8." }],
                constraints: &["1 <= n <= 300"], leetcode_url: "https://leetcode.com/problems/burst-balloons/",
                approaches: &[ApproachMeta { id: 0, name: "Interval DP / Range Subproblems", time_complexity: "O(N^3)", space_complexity: "O(N^2)", rationale: "Choosing the LAST balloon to burst in subarray [l, r] decouples subproblems cleanly.", description: "Interval subproblem expansion table." }],
            },
            Problem::RegularExpressionMatching => ProblemDetails {
                id: 10, title: "Regular Expression Matching", difficulty: Difficulty::Hard, category: Category::TwoDDp,
                statement: "Given an input string s and a pattern p, implement regular expression matching with support for '.' and '*'.",
                examples: &[Example { input: "s = \"aa\", p = \"a*\"", output: "true", explanation: "'*' means zero or more of the preceding element 'a'." }],
                constraints: &["1 <= s.length <= 20", "1 <= p.length <= 20"], leetcode_url: "https://leetcode.com/problems/regular-expression-matching/",
                approaches: &[ApproachMeta { id: 0, name: "2D Regex Matching Matrix DP", time_complexity: "O(M * N)", space_complexity: "O(M * N)", rationale: "Handling '.' wildcard matching and '*' repetition zero-or-more branches in a 2D boolean grid.", description: "2D state transition table." }],
            },
            Problem::ReconstructItinerary => ProblemDetails {
                id: 332, title: "Reconstruct Itinerary", difficulty: Difficulty::Hard, category: Category::AdvancedGraphs,
                statement: "Reconstruct the itinerary in order from a list of airline tickets. All tickets belong to a man who departs from JFK.",
                examples: &[Example { input: "tickets = [[\"MUC\",\"LHR\"],[\"JFK\",\"MUC\"],[\"SFO\",\"SJC\"],[\"LHR\",\"SFO\"]]", output: "[\"JFK\",\"MUC\",\"LHR\",\"SFO\",\"SJC\"]", explanation: "Valid flight itinerary traversing all tickets." }],
                constraints: &["1 <= tickets.length <= 300"], leetcode_url: "https://leetcode.com/problems/reconstruct-itinerary/",
                approaches: &[ApproachMeta { id: 0, name: "Hierholzer's Eulerian Path Algorithm", time_complexity: "O(E log E)", space_complexity: "O(E)", rationale: "Greedily exploring smallest lexical airport destinations and post-order appending yields Eulerian path.", description: "Eulerian path post-order traversal." }],
            },
            Problem::MinCostConnectPoints => ProblemDetails {
                id: 1584, title: "Min Cost to Connect All Points", difficulty: Difficulty::Medium, category: Category::AdvancedGraphs,
                statement: "Return the minimum cost to make all points connected using Manhattan distance between points.",
                examples: &[Example { input: "points = [[0,0],[2,2],[3,10],[5,2],[7,0]]", output: "20", explanation: "Minimum Spanning Tree cost = 20." }],
                constraints: &["1 <= points.length <= 1000"], leetcode_url: "https://leetcode.com/problems/min-cost-to-connect-all-points/",
                approaches: &[ApproachMeta { id: 0, name: "Prim's Minimum Spanning Tree (MST)", time_complexity: "O(N^2)", space_complexity: "O(N)", rationale: "Growing an MST greedily by picking the minimum Manhattan distance edge to unvisited points guarantees minimal connection cost.", description: "Greedy MST edge selection." }],
            },
            Problem::NetworkDelayTime => ProblemDetails {
                id: 743, title: "Network Delay Time", difficulty: Difficulty::Medium, category: Category::AdvancedGraphs,
                statement: "You are given a network of n nodes labeled from 1 to n and times[i] = (ui, vi, wi). Return the minimum time it takes for all n nodes to receive a signal sent from node k.",
                examples: &[Example { input: "times = [[2,1,1],[2,3,1],[3,4,1]], n = 4, k = 2", output: "2", explanation: "Signal reaches node 4 at time t=2." }],
                constraints: &["1 <= k <= n <= 100"], leetcode_url: "https://leetcode.com/problems/network-delay-time/",
                approaches: &[ApproachMeta { id: 0, name: "Dijkstra's Shortest Path Algorithm", time_complexity: "O(E log V)", space_complexity: "O(V + E)", rationale: "Priority queue min-heap relaxation finds single-source shortest path arrival times to all nodes.", description: "Min-heap Dijkstra propagation." }],
            },
            Problem::SwimInRisingWater => ProblemDetails {
                id: 778, title: "Swim in Rising Water", difficulty: Difficulty::Hard, category: Category::AdvancedGraphs,
                statement: "You are given an n x n integer matrix grid where each cell represents the elevation at that point. Return the least time until you can reach the bottom right cell (n-1, n-1) from (0,0).",
                examples: &[Example { input: "grid = [[0,2],[1,3]]", output: "3", explanation: "At time 3, water level rises to 3 and top-left connects to bottom-right." }],
                constraints: &["n == grid.length"], leetcode_url: "https://leetcode.com/problems/swim-in-rising-water/",
                approaches: &[ApproachMeta { id: 0, name: "Dijkstra / Min-Heap Grid Expansion", time_complexity: "O(N^2 log N)", space_complexity: "O(N^2)", rationale: "Expanding paths by minimum required max-elevation bottleneck via Priority Queue reaches destination in least time.", description: "Bottleneck path Min-Heap." }],
            },
            Problem::AlienDictionary => ProblemDetails {
                id: 269, title: "Alien Dictionary", difficulty: Difficulty::Hard, category: Category::AdvancedGraphs,
                statement: "There is a new alien language that uses the English alphabet. Given a list of words from the alien dictionary, derive the order of letters in this language.",
                examples: &[Example { input: "words = [\"wrt\",\"wrf\",\"er\",\"ett\",\"rftt\"]", output: "\"wertf\"", explanation: "Alien character precedence order is wertf." }],
                constraints: &["1 <= words.length <= 100"], leetcode_url: "https://leetcode.com/problems/alien-dictionary/",
                approaches: &[ApproachMeta { id: 0, name: "Topological Sort / Post-Order DFS", time_complexity: "O(C)", space_complexity: "O(1)", rationale: "Constructing character precedence edges from adjacent word differences and detecting directed cycles yields valid alien alphabet.", description: "Directed character DAG topological sort." }],
            },
            Problem::CheapestFlights => ProblemDetails {
                id: 787, title: "Cheapest Flights Within K Stops", difficulty: Difficulty::Medium, category: Category::AdvancedGraphs,
                statement: "Return the cheapest price from src to dst with at most k stops. If there is no such route, return -1.",
                examples: &[Example { input: "n = 4, flights = [[0,1,100],[1,2,100],[2,0,100],[1,3,600],[2,3,200]], src = 0, dst = 3, k = 1", output: "700", explanation: "Cheapest route 0 -> 1 -> 3 cost 700 with 1 stop." }],
                constraints: &["1 <= n <= 100", "0 <= k < n"], leetcode_url: "https://leetcode.com/problems/cheapest-flights-within-k-stops/",
                approaches: &[ApproachMeta { id: 0, name: "Bellman-Ford Algorithm (K Iterations)", time_complexity: "O(K * E)", space_complexity: "O(V)", rationale: "Relaxing edge costs exactly K+1 times guarantees finding shortest path with at most K stops.", description: "K-step edge cost relaxation." }],
            },
        }
    }
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
            (4, "        for i in range(len(nums) - 1):"),
            (5, "            if nums[i] == nums[i + 1]:"),
            (6, "                return True"),
            (7, "        return False"),
        ],
        (Problem::ContainsDuplicate, _) => vec![
            (1, "class Solution:"),
            (2, "    def containsDuplicate(self, nums: List[int]) -> bool:"),
            (3, "        for i in range(len(nums)):"),
            (4, "            for j in range(i + 1, len(nums)):"),
            (5, "                if nums[i] == nums[j]: return True"),
            (6, "        return False"),
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
            (2, "    def twoSum(self, numbers: List[int], target: int) -> List[int]:"),
            (3, "        l, r = 0, len(numbers) - 1"),
            (4, "        while l < r:"),
            (5, "            curSum = numbers[l] + numbers[r]"),
            (6, "            if curSum == target: return [l + 1, r + 1]"),
            (7, "            elif curSum < target: l += 1"),
            (8, "            else: r -= 1"),
            (9, "        return []"),
        ],
        (Problem::TwoSum, _) => vec![
            (1, "class Solution:"),
            (2, "    def twoSum(self, nums: List[int], target: int) -> List[int]:"),
            (3, "        for i in range(len(nums)):"),
            (4, "            for j in range(i + 1, len(nums)):"),
            (5, "                if nums[i] + nums[j] == target: return [i, j]"),
            (6, "        return []"),
        ],
        (Problem::ValidAnagram, 0) => vec![
            (1, "class Solution:"),
            (2, "    def isAnagram(self, s: str, t: str) -> bool:"),
            (3, "        if len(s) != len(t): return False"),
            (4, "        countS, countT = {}, {}"),
            (5, "        for i in range(len(s)):"),
            (6, "            countS[s[i]] = 1 + countS.get(s[i], 0)"),
            (7, "            countT[t[i]] = 1 + countT.get(t[i], 0)"),
            (8, "        return countS == countT"),
        ],
        (Problem::ValidAnagram, _) => vec![
            (1, "class Solution:"),
            (2, "    def isAnagram(self, s: str, t: str) -> bool:"),
            (3, "        if len(s) != len(t): return False"),
            (4, "        return sorted(s) == sorted(t)"),
        ],
        (Problem::GroupAnagrams, 0) => vec![
            (1, "class Solution:"),
            (2, "    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:"),
            (3, "        res = defaultdict(list)"),
            (4, "        for s in strs:"),
            (5, "            count = [0] * 26"),
            (6, "            for c in s: count[ord(c) - ord('a')] += 1"),
            (7, "            res[tuple(count)].append(s)"),
            (8, "        return list(res.values())"),
        ],
        (Problem::TopKFrequent, 0) => crate::model::topk_code_lines(),
        (Problem::TopKFrequent, 1) => vec![
            (1, "class Solution:"),
            (2, "    def topKFrequent(self, nums: List[int], k: int) -> List[int]:"),
            (3, "        count = Counter(nums)"),
            (4, "        heap = []"),
            (5, "        for val, freq in count.items():"),
            (6, "            heapq.heappush(heap, (freq, val))"),
            (7, "            if len(heap) > k: heapq.heappop(heap)"),
            (8, "        return [val for freq, val in heap]"),
        ],
        (Problem::TopKFrequent, _) => vec![
            (1, "class Solution:"),
            (2, "    def topKFrequent(self, nums: List[int], k: int) -> List[int]:"),
            (3, "        count = Counter(nums)"),
            (4, "        sorted_items = sorted(count.items(), key=lambda x: x[1], reverse=True)"),
            (5, "        return [val for val, freq in sorted_items[:k]]"),
        ],
        (Problem::ProductExceptSelf, _) => crate::model::product_code_lines(),
        (Problem::EncodeDecode, _) => crate::model::encode_decode_code_lines(),
        (Problem::ValidSudoku, _) => vec![
            (1, "class Solution:"),
            (2, "    def isValidSudoku(self, board: List[List[str]]) -> bool:"),
            (3, "        rows, cols, squares = defaultdict(set), defaultdict(set), defaultdict(set)"),
            (4, "        for r in range(9):"),
            (5, "            for c in range(9):"),
            (6, "                if board[r][c] == '.': continue"),
            (7, "                val = board[r][c]"),
            (8, "                if val in rows[r] or val in cols[c] or val in squares[(r//3, c//3)]: return False"),
            (9, "                rows[r].add(val); cols[c].add(val); squares[(r//3, c//3)].add(val)"),
            (10, "        return True"),
        ],
        (Problem::LongestConsecutive, _) => vec![
            (1, "class Solution:"),
            (2, "    def longestConsecutive(self, nums: List[int]) -> int:"),
            (3, "        numSet = set(nums); longest = 0"),
            (4, "        for n in numSet:"),
            (5, "            if (n - 1) not in numSet:"),
            (6, "                length = 1"),
            (7, "                while (n + length) in numSet: length += 1"),
            (8, "                longest = max(longest, length)"),
            (9, "        return longest"),
        ],
        (Problem::ValidPalindrome, 0) => vec![
            (1, "class Solution:"),
            (2, "    def isPalindrome(self, s: str) -> bool:"),
            (3, "        l, r = 0, len(s) - 1"),
            (4, "        while l < r:"),
            (5, "            while l < r and not s[l].isalnum(): l += 1"),
            (6, "            while r > l and not s[r].isalnum(): r -= 1"),
            (7, "            if s[l].lower() != s[r].lower(): return False"),
            (8, "            l, r = l + 1, r - 1"),
            (9, "        return True"),
        ],
        (Problem::BestTimeStock, _) => vec![
            (1, "class Solution:"),
            (2, "    def maxProfit(self, prices: List[int]) -> int:"),
            (3, "        l, r = 0, 1; maxP = 0"),
            (4, "        while r < len(prices):"),
            (5, "            if prices[l] < prices[r]:"),
            (6, "                maxP = max(maxP, prices[r] - prices[l])"),
            (7, "            else: l = r"),
            (8, "            r += 1"),
            (9, "        return maxP"),
        ],
        (Problem::ValidParentheses, _) => vec![
            (1, "class Solution:"),
            (2, "    def isValid(self, s: str) -> bool:"),
            (3, "        stack = []"),
            (4, "        closeToOpen = {\")\": \"(\", \"]\": \"[\", \"}\": \"{\"}"),
            (5, "        for c in s:"),
            (6, "            if c in closeToOpen:"),
            (7, "                if stack and stack[-1] == closeToOpen[c]: stack.pop()"),
            (8, "                else: return False"),
            (9, "            else: stack.append(c)"),
            (10, "        return True if not stack else False"),
        ],
        (Problem::BinarySearch, _) => vec![
            (1, "class Solution:"),
            (2, "    def search(self, nums: List[int], target: int) -> int:"),
            (3, "        l, r = 0, len(nums) - 1"),
            (4, "        while l <= r:"),
            (5, "            m = l + ((r - l) // 2)"),
            (6, "            if nums[m] > target: r = m - 1"),
            (7, "            elif nums[m] < target: l = m + 1"),
            (8, "            else: return m"),
            (9, "        return -1"),
        ],
        (Problem::ReverseLinkedList, _) => vec![
            (1, "class Solution:"),
            (2, "    def reverseList(self, head: Optional[ListNode]) -> Optional[ListNode]:"),
            (3, "        prev, curr = None, head"),
            (4, "        while curr:"),
            (5, "            nxt = curr.next; curr.next = prev; prev = curr; curr = nxt"),
            (6, "        return prev"),
        ],
        (Problem::MergeTwoLists, _) => vec![
            (1, "class Solution:"),
            (2, "    def mergeTwoLists(self, list1: Optional[ListNode], list2: Optional[ListNode]) -> Optional[ListNode]:"),
            (3, "        dummy = tail = ListNode()"),
            (4, "        while list1 and list2:"),
            (5, "            if list1.val < list2.val: tail.next = list1; list1 = list1.next"),
            (6, "            else: tail.next = list2; list2 = list2.next"),
            (7, "            tail = tail.next"),
            (8, "        tail.next = list1 if list1 else list2"),
            (9, "        return dummy.next"),
        ],
        (Problem::LinkedListCycle, _) => vec![
            (1, "class Solution:"),
            (2, "    def hasCycle(self, head: Optional[ListNode]) -> bool:"),
            (3, "        slow, fast = head, head"),
            (4, "        while fast and fast.next:"),
            (5, "            slow = slow.next; fast = fast.next.next"),
            (6, "            if slow == fast: return True"),
            (7, "        return False"),
        ],
        (Problem::InvertTree, _) => vec![
            (1, "class Solution:"),
            (2, "    def invertTree(self, root: Optional[TreeNode]) -> Optional[TreeNode]:"),
            (3, "        if not root: return None"),
            (4, "        tmp = root.left; root.left = root.right; root.right = tmp"),
            (5, "        self.invertTree(root.left); self.invertTree(root.right)"),
            (6, "        return root"),
        ],
        (Problem::MaxDepthTree, _) => vec![
            (1, "class Solution:"),
            (2, "    def maxDepth(self, root: Optional[TreeNode]) -> int:"),
            (3, "        if not root: return 0"),
            (4, "        return 1 + max(self.maxDepth(root.left), self.maxDepth(root.right))"),
        ],
        (Problem::DiameterTree, _) => vec![
            (1, "class Solution:"),
            (2, "    def diameterOfBinaryTree(self, root: Optional[TreeNode]) -> int:"),
            (3, "        res = 0"),
            (4, "        def dfs(curr):"),
            (5, "            nonlocal res"),
            (6, "            if not curr: return 0"),
            (7, "            left, right = dfs(curr.left), dfs(curr.right)"),
            (8, "            res = max(res, left + right)"),
            (9, "            return 1 + max(left, right)"),
            (10, "        dfs(root); return res"),
        ],
        (Problem::BalancedTree, _) => vec![
            (1, "class Solution:"),
            (2, "    def isBalanced(self, root: Optional[TreeNode]) -> bool:"),
            (3, "        def dfs(root):"),
            (4, "            if not root: return [True, 0]"),
            (5, "            left, right = dfs(root.left), dfs(root.right)"),
            (6, "            balanced = left[0] and right[0] and abs(left[1] - right[1]) <= 1"),
            (7, "            return [balanced, 1 + max(left[1], right[1])]"),
            (8, "        return dfs(root)[0]"),
        ],
        (Problem::SameTree, _) => vec![
            (1, "class Solution:"),
            (2, "    def isSameTree(self, p: Optional[TreeNode], q: Optional[TreeNode]) -> bool:"),
            (3, "        if not p and not q: return True"),
            (4, "        if not p or not q or p.val != q.val: return False"),
            (5, "        return self.isSameTree(p.left, q.left) and self.isSameTree(p.right, q.right)"),
        ],
        (Problem::Subtree, _) => vec![
            (1, "class Solution:"),
            (2, "    def isSubtree(self, root: Optional[TreeNode], subRoot: Optional[TreeNode]) -> bool:"),
            (3, "        if not subRoot: return True"),
            (4, "        if not root: return False"),
            (5, "        if self.sameTree(root, subRoot): return True"),
            (6, "        return self.isSubtree(root.left, subRoot) or self.isSubtree(root.right, subRoot)"),
        ],
        (Problem::ClimbingStairs, _) => vec![
            (1, "class Solution:"),
            (2, "    def climbStairs(self, n: int) -> int:"),
            (3, "        one, two = 1, 1"),
            (4, "        for i in range(n - 1):"),
            (5, "            temp = one; one = one + two; two = temp"),
            (6, "        return one"),
        ],
        (Problem::MinCostStairs, _) => vec![
            (1, "class Solution:"),
            (2, "    def minCostClimbingStairs(self, cost: List[int]) -> int:"),
            (3, "        cost.append(0)"),
            (4, "        for i in range(len(cost) - 3, -1, -1):"),
            (5, "            cost[i] += min(cost[i + 1], cost[i + 2])"),
            (6, "        return min(cost[0], cost[1])"),
        ],
        (Problem::KthLargestStream, _) => vec![
            (1, "class KthLargest:"),
            (2, "    def __init__(self, k: int, nums: List[int]):"),
            (3, "        self.minHeap, self.k = nums, k"),
            (4, "        heapq.heapify(self.minHeap)"),
            (5, "        while len(self.minHeap) > k: heapq.heappop(self.minHeap)"),
            (6, "    def add(self, val: int) -> int:"),
            (7, "        heapq.heappush(self.minHeap, val)"),
            (8, "        if len(self.minHeap) > self.k: heapq.heappop(self.minHeap)"),
            (9, "        return self.minHeap[0]"),
        ],
        (Problem::LastStone, _) => vec![
            (1, "class Solution:"),
            (2, "    def lastStoneWeight(self, stones: List[int]) -> int:"),
            (3, "        stones = [-s for s in stones]"),
            (4, "        heapq.heapify(stones)"),
            (5, "        while len(stones) > 1:"),
            (6, "            first = heapq.heappop(stones); second = heapq.heappop(stones)"),
            (7, "            if second > first: heapq.heappush(stones, first - second)"),
            (8, "        stones.append(0)"),
            (9, "        return abs(stones[0])"),
        ],
        (Problem::MeetingRooms, _) => vec![
            (1, "class Solution:"),
            (2, "    def canAttendMeetings(self, intervals: List[Interval]) -> bool:"),
            (3, "        intervals.sort(key=lambda i: i.start)"),
            (4, "        for i in range(1, len(intervals)):"),
            (5, "            if intervals[i].start < intervals[i - 1].end: return False"),
            (6, "        return True"),
        ],
        (Problem::HappyNumber, _) => vec![
            (1, "class Solution:"),
            (2, "    def isHappy(self, n: int) -> bool:"),
            (3, "        visit = set()"),
            (4, "        while n not in visit:"),
            (5, "            visit.add(n); n = self.sumOfSquares(n)"),
            (6, "            if n == 1: return True"),
            (7, "        return False"),
        ],
        (Problem::PlusOne, _) => vec![
            (1, "class Solution:"),
            (2, "    def plusOne(self, digits: List[int]) -> List[int]:"),
            (3, "        for i in range(len(digits) - 1, -1, -1):"),
            (4, "            if digits[i] < 9: digits[i] += 1; return digits"),
            (5, "            digits[i] = 0"),
            (6, "        return [1] + digits"),
        ],
        (Problem::SingleNumber, _) => vec![
            (1, "class Solution:"),
            (2, "    def singleNumber(self, nums: List[int]) -> int:"),
            (3, "        res = 0"),
            (4, "        for n in nums: res ^= n"),
            (5, "        return res"),
        ],
        (Problem::CountBits, _) => vec![
            (1, "class Solution:"),
            (2, "    def hammingWeight(self, n: int) -> int:"),
            (3, "        res = 0"),
            (4, "        while n: n &= (n - 1); res += 1"),
            (5, "        return res"),
        ],
        (Problem::CountingBits, _) => vec![
            (1, "class Solution:"),
            (2, "    def countBits(self, n: int) -> List[int]:"),
            (3, "        dp = [0] * (n + 1); offset = 1"),
            (4, "        for i in range(1, n + 1):"),
            (5, "            if offset * 2 == i: offset = i"),
            (6, "            dp[i] = 1 + dp[i - offset]"),
            (7, "        return dp"),
        ],
        (Problem::ReverseBits, _) => vec![
            (1, "class Solution:"),
            (2, "    def reverseBits(self, n: int) -> int:"),
            (3, "        res = 0"),
            (4, "        for i in range(32):"),
            (5, "            bit = (n >> i) & 1; res |= (bit << (31 - i))"),
            (6, "        return res"),
        ],
        (Problem::MissingNumber, _) => vec![
            (1, "class Solution:"),
            (2, "    def missingNumber(self, nums: List[int]) -> int:"),
            (3, "        res = len(nums)"),
            (4, "        for i in range(len(nums)): res += (i - nums[i])"),
            (5, "        return res"),
        ],
        (Problem::TwoSumII, _) => vec![
            (1, "class Solution:"),
            (2, "    def twoSum(self, numbers: List[int], target: int) -> List[int]:"),
            (3, "        l, r = 0, len(numbers) - 1"),
            (4, "        while l < r:"),
            (5, "            curSum = numbers[l] + numbers[r]"),
            (6, "            if curSum == target: return [l + 1, r + 1]"),
            (7, "            elif curSum < target: l += 1"),
            (8, "            else: r -= 1"),
            (9, "        return []"),
        ],
        (Problem::ThreeSum, _) => vec![
            (1, "class Solution:"),
            (2, "    def threeSum(self, nums: List[int]) -> List[List[int]]:"),
            (3, "        res = []; nums.sort()"),
            (4, "        for i, a in enumerate(nums):"),
            (5, "            if i > 0 and a == nums[i - 1]: continue"),
            (6, "            l, r = i + 1, len(nums) - 1"),
            (7, "            while l < r:"),
            (8, "                threeSum = a + nums[l] + nums[r]"),
            (9, "                if threeSum < 0: l += 1"),
            (10, "                elif threeSum > 0: r -= 1"),
            (11, "                else:"),
            (12, "                    res.append([a, nums[l], nums[r]]); l += 1"),
            (13, "                    while nums[l] == nums[l - 1] and l < r: l += 1"),
            (14, "        return res"),
        ],
        (Problem::ContainerWater, _) => vec![
            (1, "class Solution:"),
            (2, "    def maxArea(self, height: List[int]) -> int:"),
            (3, "        l, r = 0, len(height) - 1"),
            (4, "        res = 0"),
            (5, "        while l < r:"),
            (6, "            area = (r - l) * min(height[l], height[r])"),
            (7, "            res = max(res, area)"),
            (8, "            if height[l] < height[r]: l += 1"),
            (9, "            else: r -= 1"),
            (10, "        return res"),
        ],
        (Problem::TrappingRain, _) => vec![
            (1, "class Solution:"),
            (2, "    def trap(self, height: List[int]) -> int:"),
            (3, "        if not height: return 0"),
            (4, "        l, r = 0, len(height) - 1"),
            (5, "        leftMax, rightMax = height[l], height[r]"),
            (6, "        res = 0"),
            (7, "        while l < r:"),
            (8, "            if leftMax < rightMax:"),
            (9, "                l += 1; leftMax = max(leftMax, height[l]); res += leftMax - height[l]"),
            (10, "            else:"),
            (11, "                r -= 1; rightMax = max(rightMax, height[r]); res += rightMax - height[r]"),
            (12, "        return res"),
        ],
        (Problem::MinStack, _) => vec![
            (1, "class MinStack:"),
            (2, "    def __init__(self):"),
            (3, "        self.stack = []"),
            (4, "        self.minStack = []"),
            (5, "    def push(self, val: int) -> None:"),
            (6, "        self.stack.append(val)"),
            (7, "        val = min(val, self.minStack[-1] if self.minStack else val)"),
            (8, "        self.minStack.append(val)"),
            (9, "    def pop(self) -> None:"),
            (10, "        self.stack.pop(); self.minStack.pop()"),
            (11, "    def top(self) -> int: return self.stack[-1]"),
            (12, "    def getMin(self) -> int: return self.minStack[-1]"),
        ],
        (Problem::EvalRPN, _) => vec![
            (1, "class Solution:"),
            (2, "    def evalRPN(self, tokens: List[str]) -> int:"),
            (3, "        stack = []"),
            (4, "        for c in tokens:"),
            (5, "            if c == '+': stack.append(stack.pop() + stack.pop())"),
            (6, "            elif c == '-': a, b = stack.pop(), stack.pop(); stack.append(b - a)"),
            (7, "            elif c == '*': stack.append(stack.pop() * stack.pop())"),
            (8, "            elif c == '/': a, b = stack.pop(), stack.pop(); stack.append(int(b / a))"),
            (9, "            else: stack.append(int(c))"),
            (10, "        return stack[0]"),
        ],
        (Problem::LongestSubstring, _) => vec![
            (1, "class Solution:"),
            (2, "    def lengthOfLongestSubstring(self, s: str) -> int:"),
            (3, "        charSet = set(); l = 0; res = 0"),
            (4, "        for r in range(len(s)):"),
            (5, "            while s[r] in charSet:"),
            (6, "                charSet.remove(s[l]); l += 1"),
            (7, "            charSet.add(s[r])"),
            (8, "            res = max(res, r - l + 1)"),
            (9, "        return res"),
        ],
        (Problem::Search2DMatrix, _) => vec![
            (1, "class Solution:"),
            (2, "    def searchMatrix(self, matrix: List[List[int]], target: int) -> bool:"),
            (3, "        ROWS, COLS = len(matrix), len(matrix[0])"),
            (4, "        l, r = 0, ROWS * COLS - 1"),
            (5, "        while l <= r:"),
            (6, "            m = (l + r) // 2"),
            (7, "            row, col = m // COLS, m % COLS"),
            (8, "            if target > matrix[row][col]: l = m + 1"),
            (9, "            elif target < matrix[row][col]: r = m - 1"),
            (10, "            else: return True"),
            (11, "        return False"),
        ],
        (Problem::HouseRobber, _) => house_robber_code_lines(),
        (Problem::GenerateParentheses, _) => vec![
            (1, "class Solution:"),
            (2, "    def generateParenthesis(self, n: int) -> List[str]:"),
            (3, "        stack = []; res = []"),
            (4, "        def backtrack(openN, closedN):"),
            (5, "            if openN == closedN == n: res.append(\"\".join(stack)); return"),
            (6, "            if openN < n: stack.append(\"(\"); backtrack(openN + 1, closedN); stack.pop()"),
            (7, "            if closedN < openN: stack.append(\")\"); backtrack(openN, closedN + 1); stack.pop()"),
            (8, "        backtrack(0, 0); return res"),
        ],
        (Problem::DailyTemperatures, _) => vec![
            (1, "class Solution:"),
            (2, "    def dailyTemperatures(self, temperatures: List[int]) -> List[int]:"),
            (3, "        res = [0] * len(temperatures); stack = []"),
            (4, "        for i, t in enumerate(temperatures):"),
            (5, "            while stack and t > stack[-1][0]:"),
            (6, "                stackT, stackInd = stack.pop(); res[stackInd] = i - stackInd"),
            (7, "            stack.append((t, i))"),
            (8, "        return res"),
        ],
        (Problem::CarFleet, _) => vec![
            (1, "class Solution:"),
            (2, "    def carFleet(self, target: int, position: List[int], speed: List[int]) -> int:"),
            (3, "        pair = [(p, s) for p, s in zip(position, speed)]"),
            (4, "        pair.sort(reverse=True); stack = []"),
            (5, "        for p, s in pair:"),
            (6, "            stack.append((target - p) / s)"),
            (7, "            if len(stack) >= 2 and stack[-1] <= stack[-2]: stack.pop()"),
            (8, "        return len(stack)"),
        ],
        (Problem::LargestRectangle, _) => vec![
            (1, "class Solution:"),
            (2, "    def largestRectangleArea(self, heights: List[int]) -> int:"),
            (3, "        maxArea = 0; stack = []"),
            (4, "        for i, h in enumerate(heights):"),
            (5, "            start = i"),
            (6, "            while stack and stack[-1][1] > h:"),
            (7, "                index, height = stack.pop(); maxArea = max(maxArea, height * (i - index)); start = index"),
            (8, "            stack.append((start, h))"),
            (9, "        for i, h in stack: maxArea = max(maxArea, h * (len(heights) - i))"),
            (10, "        return maxArea"),
        ],
        (Problem::CharacterReplacement, _) => vec![
            (1, "class Solution:"),
            (2, "    def characterReplacement(self, s: str, k: int) -> int:"),
            (3, "        count = {}; res = 0; l = 0; maxf = 0"),
            (4, "        for r in range(len(s)):"),
            (5, "            count[s[r]] = 1 + count.get(s[r], 0); maxf = max(maxf, count[s[r]])"),
            (6, "            while (r - l + 1) - maxf > k:"),
            (7, "                count[s[l]] -= 1; l += 1"),
            (8, "            res = max(res, r - l + 1)"),
            (9, "        return res"),
        ],
        (Problem::PermutationInString, _) => vec![
            (1, "class Solution:"),
            (2, "    def checkInclusion(self, s1: str, s2: str) -> bool:"),
            (3, "        if len(s1) > len(s2): return False"),
            (4, "        s1Count, s2Count = [0] * 26, [0] * 26"),
            (5, "        for i in range(len(s1)): s1Count[ord(s1[i]) - 97] += 1; s2Count[ord(s2[i]) - 97] += 1"),
            (6, "        matches = sum(1 for i in range(26) if s1Count[i] == s2Count[i])"),
            (7, "        l = 0"),
            (8, "        for r in range(len(s1), len(s2)):"),
            (9, "            if matches == 26: return True"),
            (10, "            index = ord(s2[r]) - 97; s2Count[index] += 1"),
            (11, "            if s1Count[index] == s2Count[index]: matches += 1"),
            (12, "            elif s1Count[index] + 1 == s2Count[index]: matches -= 1"),
            (13, "            index = ord(s2[l]) - 97; s2Count[index] -= 1"),
            (14, "            if s1Count[index] == s2Count[index]: matches += 1"),
            (15, "            elif s1Count[index] == s2Count[index] + 1: matches -= 1"),
            (16, "            l += 1"),
            (17, "        return matches == 26"),
        ],
        (Problem::MinWindowSubstring, _) => vec![
            (1, "class Solution:"),
            (2, "    def minWindow(self, s: str, t: str) -> str:"),
            (3, "        if not t: return \"\""),
            (4, "        countT, window = {}, {}"),
            (5, "        for c in t: countT[c] = 1 + countT.get(c, 0)"),
            (6, "        have, need = 0, len(countT); res, resLen = [-1, -1], float(\"infinity\"); l = 0"),
            (7, "        for r in range(len(s)):"),
            (8, "            c = s[r]; window[c] = 1 + window.get(c, 0)"),
            (9, "            if c in countT and window[c] == countT[c]: have += 1"),
            (10, "            while have == need:"),
            (11, "                if (r - l + 1) < resLen: res = [l, r]; resLen = r - l + 1"),
            (12, "                window[s[l]] -= 1"),
            (13, "                if s[l] in countT and window[s[l]] < countT[s[l]]: have -= 1"),
            (14, "                l += 1"),
            (15, "        l, r = res; return s[l : r + 1] if resLen != float(\"infinity\") else \"\""),
        ],
        (Problem::SlidingWindowMax, _) => vec![
            (1, "class Solution:"),
            (2, "    def maxSlidingWindow(self, nums: List[int], k: int) -> List[int]:"),
            (3, "        output = []; q = collections.deque(); l = r = 0"),
            (4, "        while r < len(nums):"),
            (5, "            while q and nums[q[-1]] < nums[r]: q.pop()"),
            (6, "            q.append(r)"),
            (7, "            if l > q[0]: q.popleft()"),
            (8, "            if (r + 1) >= k: output.append(nums[q[0]]); l += 1"),
            (9, "            r += 1"),
            (10, "        return output"),
        ],
        (Problem::SearchRotatedArray, _) => vec![
            (1, "class Solution:"),
            (2, "    def search(self, nums: List[int], target: int) -> int:"),
            (3, "        l, r = 0, len(nums) - 1"),
            (4, "        while l <= r:"),
            (5, "            mid = (l + r) // 2"),
            (6, "            if target == nums[mid]: return mid"),
            (7, "            if nums[l] <= nums[mid]:"),
            (8, "                if nums[l] <= target < nums[mid]: r = mid - 1"),
            (9, "                else: l = mid + 1"),
            (10, "            else:"),
            (11, "                if nums[mid] < target <= nums[r]: l = mid + 1"),
            (12, "                else: r = mid - 1"),
            (13, "        return -1"),
        ],
        (Problem::FindMinRotated, _) => vec![
            (1, "class Solution:"),
            (2, "    def findMin(self, nums: List[int]) -> int:"),
            (3, "        l, r = 0, len(nums) - 1"),
            (4, "        while l < r:"),
            (5, "            mid = (l + r) // 2"),
            (6, "            if nums[mid] > nums[r]: l = mid + 1"),
            (7, "            else: r = mid"),
            (8, "        return nums[l]"),
        ],
        (Problem::TimeKeyValueStore, _) => vec![
            (1, "class TimeMap:"),
            (2, "    def __init__(self): self.store = {}"),
            (3, "    def set(self, key: str, value: str, timestamp: int) -> None:"),
            (4, "        if key not in self.store: self.store[key] = []"),
            (5, "        self.store[key].append([value, timestamp])"),
            (6, "    def get(self, key: str, timestamp: int) -> str:"),
            (7, "        res, values = \"\", self.store.get(key, [])"),
            (8, "        l, r = 0, len(values) - 1"),
            (9, "        while l <= r:"),
            (10, "            m = (l + r) // 2"),
            (11, "            if values[m][1] <= timestamp: res = values[m][0]; l = m + 1"),
            (12, "            else: r = m - 1"),
            (13, "        return res"),
        ],
        (Problem::FindMedianSortedArrays, _) => vec![
            (1, "class Solution:"),
            (2, "    def findMedianSortedArrays(self, nums1: List[int], nums2: List[int]) -> float:"),
            (3, "        A, B = nums1, nums2; total = len(A) + len(B); half = total // 2"),
            (4, "        if len(B) < len(A): A, B = B, A"),
            (5, "        l, r = 0, len(A) - 1"),
            (6, "        while True:"),
            (7, "            i = (l + r) // 2; j = half - i - 2"),
            (8, "            Aleft = A[i] if i >= 0 else float(\"-inf\")"),
            (9, "            Aright = A[i + 1] if (i + 1) < len(A) else float(\"inf\")"),
            (10, "            Bleft = B[j] if j >= 0 else float(\"-inf\")"),
            (11, "            Bright = B[j + 1] if (j + 1) < len(B) else float(\"inf\")"),
            (12, "            if Aleft <= Bright and Bleft <= Aright:"),
            (13, "                if total % 2: return min(Aright, Bright)"),
            (14, "                return (max(Aleft, Bleft) + min(Aright, Bright)) / 2"),
            (15, "            elif Aleft > Bright: r = i - 1"),
            (16, "            else: l = i + 1"),
        ],
        (Problem::KokoEatingBananas, _) => vec![
            (1, "class Solution:"),
            (2, "    def minEatingSpeed(self, piles: List[int], h: int) -> int:"),
            (3, "        l, r = 1, max(piles); res = r"),
            (4, "        while l <= r:"),
            (5, "            k = (l + r) // 2"),
            (6, "            hours = sum(math.ceil(p / k) for p in piles)"),
            (7, "            if hours <= h:"),
            (8, "                res = k"),
            (9, "                r = k - 1"),
            (10, "            else:"),
            (11, "                l = k + 1"),
            (12, "        return res"),
        ],
        (Problem::ImplementTrie, _) => implement_trie_code_lines(),
        (Problem::WordDictionary, _) => word_dictionary_code_lines(),
        (Problem::WordSearchII, _) => word_search_ii_code_lines(),
        (Problem::Subsets, _) => subsets_code_lines(),
        (Problem::Permutations, _) => permutations_code_lines(),
        (Problem::KClosestPoints, _) => k_closest_points_code_lines(),
        (Problem::TaskScheduler, _) => task_scheduler_code_lines(),
        (Problem::FindMedianDataStream, _) => find_median_code_lines(),
        (Problem::CombinationSum, _) => combination_sum_code_lines(),
        (Problem::SubsetsII, _) => subsets_ii_code_lines(),
        (Problem::CombinationSumII, _) => combination_sum_ii_code_lines(),
        (Problem::WordSearch, _) => word_search_code_lines(),
        (Problem::NQueens, _) => n_queens_code_lines(),
        (Problem::KthLargestArray, _) => kth_largest_array_code_lines(),
        (Problem::DesignTwitter, _) => design_twitter_code_lines(),
        (Problem::PalindromePartitioning, _) => palindrome_partitioning_code_lines(),
        (Problem::LetterCombinations, _) => letter_combinations_code_lines(),
        (Problem::HouseRobberII, _) => house_robber_ii_code_lines(),
        (Problem::LongestPalindromicSubstring, _) => longest_palindromic_substring_code_lines(),
        (Problem::PalindromicSubstrings, _) => palindromic_substrings_code_lines(),
        (Problem::DecodeWays, _) => decode_ways_code_lines(),
        (Problem::CoinChange, _) => coin_change_code_lines(),
        (Problem::MaxProductSubarray, _) => max_product_subarray_code_lines(),
        (Problem::WordBreak, _) => word_break_code_lines(),
        (Problem::LongestIncreasingSubsequence, _) => longest_increasing_subsequence_code_lines(),
        (Problem::PartitionEqualSubsetSum, _) => partition_equal_subset_sum_code_lines(),
        (Problem::Number1Bits, _) => number_1_bits_code_lines(),
        (Problem::SumTwoIntegers, _) => sum_two_integers_code_lines(),
        (Problem::ReverseInteger, _) => reverse_integer_code_lines(),
        (Problem::RotateImage, _) => rotate_image_code_lines(),
        (Problem::SpiralMatrix, _) => spiral_matrix_code_lines(),
        (Problem::SetMatrixZeroes, _) => set_matrix_zeroes_code_lines(),
        (Problem::PowXN, _) => pow_xn_code_lines(),
        (Problem::MultiplyStrings, _) => multiply_strings_code_lines(),
        (Problem::DetectSquares, _) => detect_squares_code_lines(),
        (Problem::MaximumSubarray, _) => maximum_subarray_code_lines(),
        (Problem::JumpGame, _) => jump_game_code_lines(),
        (Problem::JumpGameII, _) => jump_game_ii_code_lines(),
        (Problem::GasStation, _) => gas_station_code_lines(),
        (Problem::HandOfStraights, _) => hand_of_straights_code_lines(),
        (Problem::MergeTriplets, _) => merge_triplets_code_lines(),
        (Problem::PartitionLabels, _) => partition_labels_code_lines(),
        (Problem::ValidParenthesisString, _) => valid_parenthesis_string_code_lines(),
        (Problem::InsertInterval, _) => insert_interval_code_lines(),
        (Problem::MergeIntervals, _) => merge_intervals_code_lines(),
        (Problem::NonOverlappingIntervals, _) => non_overlapping_intervals_code_lines(),
        (Problem::MeetingRoomsII, _) => meeting_rooms_ii_code_lines(),
        (Problem::MinIntervalQuery, _) => min_interval_query_code_lines(),
        (Problem::NumberIslands, _) => number_islands_code_lines(),
        (Problem::MaxAreaIsland, _) => max_area_island_code_lines(),
        (Problem::CloneGraph, _) => clone_graph_code_lines(),
        (Problem::WallsAndGates, _) => walls_and_gates_code_lines(),
        (Problem::RottingOranges, _) => rotting_oranges_code_lines(),
        (Problem::PacificAtlantic, _) => pacific_atlantic_code_lines(),
        (Problem::SurroundedRegions, _) => surrounded_regions_code_lines(),
        (Problem::CourseSchedule, _) => course_schedule_code_lines(),
        (Problem::CourseScheduleII, _) => course_schedule_ii_code_lines(),
        (Problem::GraphValidTree, _) => graph_valid_tree_code_lines(),
        (Problem::ConnectedComponents, _) => connected_components_code_lines(),
        (Problem::RedundantConnection, _) => redundant_connection_code_lines(),
        (Problem::WordLadder, _) => word_ladder_code_lines(),
        (Problem::UniquePaths, _) => unique_paths_code_lines(),
        (Problem::LongestCommonSubsequence, _) => lcs_code_lines(),
        (Problem::BestTimeStockCooldown, _) => stock_cooldown_code_lines(),
        (Problem::CoinChangeII, _) => coin_change_ii_code_lines(),
        (Problem::TargetSum, _) => target_sum_code_lines(),
        (Problem::InterleavingString, _) => interleaving_string_code_lines(),
        (Problem::LongestIncreasingPath, _) => lip_code_lines(),
        (Problem::DistinctSubsequences, _) => distinct_subsequences_code_lines(),
        (Problem::EditDistance, _) => edit_distance_code_lines(),
        (Problem::BurstBalloons, _) => burst_balloons_code_lines(),
        (Problem::RegularExpressionMatching, _) => regex_matching_code_lines(),
        (Problem::ReconstructItinerary, _) => reconstruct_itinerary_code_lines(),
        (Problem::MinCostConnectPoints, _) => min_cost_points_code_lines(),
        (Problem::NetworkDelayTime, _) => network_delay_code_lines(),
        (Problem::SwimInRisingWater, _) => swim_rising_water_code_lines(),
        (Problem::AlienDictionary, _) => alien_dictionary_code_lines(),
        (Problem::CheapestFlights, _) => cheapest_flights_code_lines(),
        _ => vec![(1, "# Approach implementation trace")],
    }
}

pub fn reconstruct_itinerary_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def findItinerary(self, tickets: List[List[str]]) -> List[str]:",
        ),
        (3, "        adj = { src: [] for src, dst in tickets }"),
        (4, "        tickets.sort()"),
        (5, "        for src, dst in tickets: adj[src].append(dst)"),
        (6, "        res = []"),
        (7, "        def dfs(src):"),
        (8, "            if src in adj:"),
        (9, "                while adj[src]:"),
        (10, "                    next_dest = adj[src].pop(0)"),
        (11, "                    dfs(next_dest)"),
        (12, "            res.append(src)"),
        (13, "        dfs(\"JFK\")"),
        (14, "        return res[::-1]"),
    ]
}

pub fn min_cost_points_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def minCostConnectPoints(self, points: List[List[int]]) -> int:",
        ),
        (3, "        N = len(points)"),
        (4, "        adj = { i: [] for i in range(N) }"),
        (5, "        for i in range(N):"),
        (6, "            x1, y1 = points[i]"),
        (7, "            for j in range(i + 1, N):"),
        (
            8,
            "                x2, y2 = points[j]; dist = abs(x1 - x2) + abs(y1 - y2)",
        ),
        (
            9,
            "                adj[i].append([dist, j]); adj[j].append([dist, i])",
        ),
        (10, "        res = 0; visit = set(); minH = [[0, 0]]"),
        (11, "        while len(visit) < N:"),
        (12, "            cost, i = heapq.heappop(minH)"),
        (13, "            if i in visit: continue"),
        (14, "            res += cost; visit.add(i)"),
        (15, "            for neiCost, nei in adj[i]:"),
        (
            16,
            "                if nei not in visit: heapq.heappush(minH, [neiCost, nei])",
        ),
        (17, "        return res"),
    ]
}

pub fn network_delay_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def networkDelayTime(self, times: List[List[int]], n: int, k: int) -> int:",
        ),
        (3, "        edges = collections.defaultdict(list)"),
        (4, "        for u, v, w in times: edges[u].append((v, w))"),
        (5, "        minHeap = [(0, k)]; visit = set(); t = 0"),
        (6, "        while minHeap:"),
        (7, "            w1, n1 = heapq.heappop(minHeap)"),
        (8, "            if n1 in visit: continue"),
        (9, "            visit.add(n1); t = w1"),
        (10, "            for n2, w2 in edges[n1]:"),
        (
            11,
            "                if n2 not in visit: heapq.heappush(minHeap, (w1 + w2, n2))",
        ),
        (12, "        return t if len(visit) == n else -1"),
    ]
}

pub fn swim_rising_water_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def swimInWater(self, grid: List[List[int]]) -> int:",
        ),
        (
            3,
            "        N = len(grid); visit = set(); minH = [[grid[0][0], 0, 0]]",
        ),
        (4, "        directions = [[0, 1], [0, -1], [1, 0], [-1, 0]]"),
        (5, "        visit.add((0, 0))"),
        (6, "        while minH:"),
        (7, "            t, r, c = heapq.heappop(minH)"),
        (8, "            if r == N - 1 and c == N - 1: return t"),
        (9, "            for dr, dc in directions:"),
        (10, "                row, col = r + dr, c + dc"),
        (
            11,
            "                if 0 <= row < N and 0 <= col < N and (row, col) not in visit:",
        ),
        (12, "                    visit.add((row, col))"),
        (
            13,
            "                    heapq.heappush(minH, [max(t, grid[row][col]), row, col])",
        ),
    ]
}

pub fn alien_dictionary_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def alienOrder(self, words: List[str]) -> str:"),
        (3, "        adj = { c: set() for w in words for c in w }"),
        (4, "        for i in range(len(words) - 1):"),
        (
            5,
            "            w1, w2 = words[i], words[i + 1]; minLen = min(len(w1), len(w2))",
        ),
        (
            6,
            "            if len(w1) > len(w2) and w1[:minLen] == w2[:minLen]: return \"\"",
        ),
        (7, "            for j in range(minLen):"),
        (
            8,
            "                if w1[j] != w2[j]: adj[w1[j]].add(w2[j]); break",
        ),
        (9, "        visit = {}; res = []"),
        (10, "        def dfs(c):"),
        (11, "            if c in visit: return visit[c]"),
        (12, "            visit[c] = True"),
        (13, "            for nei in adj[c]:"),
        (14, "                if dfs(nei): return True"),
        (15, "            visit[c] = False; res.append(c)"),
        (16, "        for c in adj:"),
        (17, "            if dfs(c): return \"\""),
        (18, "        return \"\".join(res[::-1])"),
    ]
}

pub fn cheapest_flights_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def findCheapestPrice(self, n: int, flights: List[List[int]], src: int, dst: int, k: int) -> int:"),
        (3, "        prices = [float(\"inf\")] * n; prices[src] = 0"),
        (4, "        for i in range(k + 1):"),
        (5, "            tmpPrices = list(prices)"),
        (6, "            for s, d, p in flights:"),
        (7, "                if prices[s] == float(\"inf\"): continue"),
        (8, "                if prices[s] + p < tmpPrices[d]: tmpPrices[d] = prices[s] + p"),
        (9, "            prices = tmpPrices"),
        (10, "        return prices[dst] if prices[dst] != float(\"inf\") else -1"),
    ]
}

pub fn unique_paths_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def uniquePaths(self, m: int, n: int) -> int:"),
        (3, "        row = [1] * n"),
        (4, "        for i in range(m - 1):"),
        (5, "            newRow = [1] * n"),
        (6, "            for j in range(n - 2, -1, -1):"),
        (7, "                newRow[j] = newRow[j + 1] + row[j]"),
        (8, "            row = newRow"),
        (9, "        return row[0]"),
    ]
}

pub fn lcs_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def longestCommonSubsequence(self, text1: str, text2: str) -> int:",
        ),
        (
            3,
            "        dp = [[0 for j in range(len(text2) + 1)] for i in range(len(text1) + 1)]",
        ),
        (4, "        for i in range(len(text1) - 1, -1, -1):"),
        (5, "            for j in range(len(text2) - 1, -1, -1):"),
        (
            6,
            "                if text1[i] == text2[j]: dp[i][j] = 1 + dp[i + 1][j + 1]",
        ),
        (
            7,
            "                else: dp[i][j] = max(dp[i + 1][j], dp[i][j + 1])",
        ),
        (8, "        return dp[0][0]"),
    ]
}

pub fn stock_cooldown_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def maxProfit(self, prices: List[int]) -> int:"),
        (3, "        dp = {}"),
        (4, "        def dfs(i, buying):"),
        (5, "            if i >= len(prices): return 0"),
        (6, "            if (i, buying) in dp: return dp[(i, buying)]"),
        (7, "            cooldown = dfs(i + 1, buying)"),
        (8, "            if buying: buy = dfs(i + 1, False) - prices[i]; dp[(i, buying)] = max(buy, cooldown)"),
        (9, "            else: sell = dfs(i + 2, True) + prices[i]; dp[(i, buying)] = max(sell, cooldown)"),
        (10, "            return dp[(i, buying)]"),
        (11, "        return dfs(0, True)"),
    ]
}

pub fn coin_change_ii_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def change(self, amount: int, coins: List[int]) -> int:",
        ),
        (3, "        dp = [0] * (amount + 1); dp[0] = 1"),
        (4, "        for coin in coins:"),
        (
            5,
            "            for a in range(coin, amount + 1): dp[a] += dp[a - coin]",
        ),
        (6, "        return dp[amount]"),
    ]
}

pub fn target_sum_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def findTargetSumWays(self, nums: List[int], target: int) -> int:"),
        (3, "        dp = {}"),
        (4, "        def backtrack(i, total):"),
        (5, "            if i == len(nums): return 1 if total == target else 0"),
        (6, "            if (i, total) in dp: return dp[(i, total)]"),
        (7, "            dp[(i, total)] = backtrack(i + 1, total + nums[i]) + backtrack(i + 1, total - nums[i])"),
        (8, "            return dp[(i, total)]"),
        (9, "        return backtrack(0, 0)"),
    ]
}

pub fn interleaving_string_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def isInterleave(self, s1: str, s2: str, s3: str) -> bool:"),
        (3, "        if len(s1) + len(s2) != len(s3): return False"),
        (4, "        dp = [[False] * (len(s2) + 1) for _ in range(len(s1) + 1)]"),
        (5, "        dp[len(s1)][len(s2)] = True"),
        (6, "        for i in range(len(s1), -1, -1):"),
        (7, "            for j in range(len(s2), -1, -1):"),
        (8, "                if i < len(s1) and s1[i] == s3[i + j] and dp[i + 1][j]: dp[i][j] = True"),
        (9, "                if j < len(s2) and s2[j] == s3[i + j] and dp[i][j + 1]: dp[i][j] = True"),
        (10, "        return dp[0][0]"),
    ]
}

pub fn lip_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def longestIncreasingPath(self, matrix: List[List[int]]) -> int:"),
        (3, "        ROWS, COLS = len(matrix), len(matrix[0]); dp = {}"),
        (4, "        def dfs(r, c, prevVal):"),
        (5, "            if r < 0 or r == ROWS or c < 0 or c == COLS or matrix[r][c] <= prevVal: return 0"),
        (6, "            if (r, c) in dp: return dp[(r, c)]"),
        (7, "            res = 1"),
        (8, "            for dr, dc in [[1,0],[-1,0],[0,1],[0,-1]]: res = max(res, 1 + dfs(r + dr, c + dc, matrix[r][c]))"),
        (9, "            dp[(r, c)] = res; return res"),
        (10, "        for r in range(ROWS):"),
        (11, "            for c in range(COLS): dfs(r, c, -1)"),
        (12, "        return max(dp.values())"),
    ]
}

pub fn distinct_subsequences_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def numDistinct(self, s: str, t: str) -> int:"),
        (3, "        dp = {}"),
        (4, "        def dfs(i, j):"),
        (5, "            if j == len(t): return 1"),
        (6, "            if i == len(s): return 0"),
        (7, "            if (i, j) in dp: return dp[(i, j)]"),
        (
            8,
            "            if s[i] == t[j]: dp[(i, j)] = dfs(i + 1, j + 1) + dfs(i + 1, j)",
        ),
        (9, "            else: dp[(i, j)] = dfs(i + 1, j)"),
        (10, "            return dp[(i, j)]"),
        (11, "        return dfs(0, 0)"),
    ]
}

pub fn edit_distance_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def minDistance(self, word1: str, word2: str) -> int:"),
        (3, "        dp = [[float(\"inf\")] * (len(word2) + 1) for _ in range(len(word1) + 1)]"),
        (4, "        for j in range(len(word2) + 1): dp[len(word1)][j] = len(word2) - j"),
        (5, "        for i in range(len(word1) + 1): dp[i][len(word2)] = len(word1) - i"),
        (6, "        for i in range(len(word1) - 1, -1, -1):"),
        (7, "            for j in range(len(word2) - 1, -1, -1):"),
        (8, "                if word1[i] == word2[j]: dp[i][j] = dp[i + 1][j + 1]"),
        (9, "                else: dp[i][j] = 1 + min(dp[i + 1][j], dp[i][j + 1], dp[i + 1][j + 1])"),
        (10, "        return dp[0][0]"),
    ]
}

pub fn burst_balloons_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def maxCoins(self, nums: List[int]) -> int:"),
        (3, "        nums = [1] + nums + [1]; dp = {}"),
        (4, "        def dfs(l, r):"),
        (5, "            if l > r: return 0"),
        (6, "            if (l, r) in dp: return dp[(l, r)]"),
        (7, "            dp[(l, r)] = 0"),
        (8, "            for i in range(l, r + 1):"),
        (
            9,
            "                coins = nums[l - 1] * nums[i] * nums[r + 1]",
        ),
        (10, "                coins += dfs(l, i - 1) + dfs(i + 1, r)"),
        (11, "                dp[(l, r)] = max(dp[(l, r)], coins)"),
        (12, "            return dp[(l, r)]"),
        (13, "        return dfs(1, len(nums) - 2)"),
    ]
}

pub fn regex_matching_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def isMatch(self, s: str, p: str) -> bool:"),
        (3, "        cache = {}"),
        (4, "        def dfs(i, j):"),
        (5, "            if (i, j) in cache: return cache[(i, j)]"),
        (6, "            if i >= len(s) and j >= len(p): return True"),
        (7, "            if j >= len(p): return False"),
        (
            8,
            "            match = i < len(s) and (s[i] == p[j] or p[j] == \".\")",
        ),
        (9, "            if (j + 1) < len(p) and p[j + 1] == \"*\":"),
        (
            10,
            "                cache[(i, j)] = (dfs(i, j + 2) or (match and dfs(i + 1, j)))",
        ),
        (11, "                return cache[(i, j)]"),
        (
            12,
            "            if match: cache[(i, j)] = dfs(i + 1, j + 1); return cache[(i, j)]",
        ),
        (13, "            cache[(i, j)] = False; return False"),
        (14, "        return dfs(0, 0)"),
    ]
}

pub fn number_islands_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def numIslands(self, grid: List[List[str]]) -> int:"),
        (3, "        if not grid: return 0"),
        (4, "        rows, cols = len(grid), len(grid[0]); visited = set(); islands = 0"),
        (5, "        def bfs(r, c):"),
        (6, "            q = collections.deque([(r, c)]); visited.add((r, c))"),
        (7, "            while q:"),
        (8, "                row, col = q.popleft()"),
        (9, "                for dr, dc in [[1,0],[-1,0],[0,1],[0,-1]]:"),
        (10, "                    r, c = row + dr, col + dc"),
        (11, "                    if 0 <= r < rows and 0 <= c < cols and grid[r][c] == '1' and (r,c) not in visited:"),
        (12, "                        q.append((r, c)); visited.add((r, c))"),
        (13, "        for r in range(rows):"),
        (14, "            for c in range(cols):"),
        (15, "                if grid[r][c] == '1' and (r, c) not in visited: bfs(r, c); islands += 1"),
        (16, "        return islands"),
    ]
}

pub fn max_area_island_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def maxAreaOfIsland(self, grid: List[List[int]]) -> int:"),
        (3, "        ROWS, COLS = len(grid), len(grid[0]); visit = set()"),
        (4, "        def dfs(r, c):"),
        (5, "            if r < 0 or r == ROWS or c < 0 or c == COLS or grid[r][c] == 0 or (r, c) in visit: return 0"),
        (6, "            visit.add((r, c))"),
        (7, "            return 1 + dfs(r + 1, c) + dfs(r - 1, c) + dfs(r, c + 1) + dfs(r, c - 1)"),
        (8, "        area = 0"),
        (9, "        for r in range(ROWS):"),
        (10, "            for c in range(COLS): area = max(area, dfs(r, c))"),
        (11, "        return area"),
    ]
}

pub fn clone_graph_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def cloneGraph(self, node: 'Node') -> 'Node':"),
        (3, "        oldToNew = {}"),
        (4, "        def dfs(node):"),
        (5, "            if not node: return None"),
        (6, "            if node in oldToNew: return oldToNew[node]"),
        (
            7,
            "            copy = Node(node.val); oldToNew[node] = copy",
        ),
        (
            8,
            "            for nei in node.neighbors: copy.neighbors.append(dfs(nei))",
        ),
        (9, "            return copy"),
        (10, "        return dfs(node)"),
    ]
}

pub fn walls_and_gates_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def wallsAndGates(self, rooms: List[List[int]]) -> None:"),
        (3, "        ROWS, COLS = len(rooms), len(rooms[0]); q = collections.deque()"),
        (4, "        for r in range(ROWS):"),
        (5, "            for c in range(COLS):"),
        (6, "                if rooms[r][c] == 0: q.append((r, c))"),
        (7, "        dist = 0"),
        (8, "        while q:"),
        (9, "            for i in range(len(q)):"),
        (10, "                r, c = q.popleft()"),
        (11, "                rooms[r][c] = dist"),
        (12, "                for dr, dc in [[1,0],[-1,0],[0,1],[0,-1]]:"),
        (13, "                    nr, nc = r + dr, c + dc"),
        (14, "                    if 0 <= nr < ROWS and 0 <= nc < COLS and rooms[nr][nc] == 2147483647:"),
        (15, "                        q.append((nr, nc))"),
        (16, "            dist += 1"),
    ]
}

pub fn rotting_oranges_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def orangesRotting(self, grid: List[List[int]]) -> int:",
        ),
        (3, "        q = collections.deque(); time, fresh = 0, 0"),
        (4, "        ROWS, COLS = len(grid), len(grid[0])"),
        (5, "        for r in range(ROWS):"),
        (6, "            for c in range(COLS):"),
        (7, "                if grid[r][c] == 1: fresh += 1"),
        (8, "                if grid[r][c] == 2: q.append([r, c])"),
        (9, "        while q and fresh > 0:"),
        (10, "            for i in range(len(q)):"),
        (11, "                r, c = q.popleft()"),
        (
            12,
            "                for dr, dc in [[1,0],[-1,0],[0,1],[0,-1]]:",
        ),
        (13, "                    row, col = r + dr, c + dc"),
        (
            14,
            "                    if 0 <= row < ROWS and 0 <= col < COLS and grid[row][col] == 1:",
        ),
        (
            15,
            "                        grid[row][col] = 2; q.append([row, col]); fresh -= 1",
        ),
        (16, "            time += 1"),
        (17, "        return time if fresh == 0 else -1"),
    ]
}

pub fn pacific_atlantic_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def pacificAtlantic(self, heights: List[List[int]]) -> List[List[int]]:"),
        (3, "        ROWS, COLS = len(heights), len(heights[0]); pac, atl = set(), set()"),
        (4, "        def dfs(r, c, visit, prevHeight):"),
        (5, "            if ((r, c) in visit or r < 0 or c < 0 or r == ROWS or c == COLS or heights[r][c] < prevHeight): return"),
        (6, "            visit.add((r, c))"),
        (7, "            for dr, dc in [[1,0],[-1,0],[0,1],[0,-1]]: dfs(r + dr, c + dc, visit, heights[r][c])"),
        (8, "        for c in range(COLS): dfs(0, c, pac, heights[0][c]); dfs(ROWS - 1, c, atl, heights[ROWS - 1][c])"),
        (9, "        for r in range(ROWS): dfs(r, 0, pac, heights[r][0]); dfs(r, COLS - 1, atl, heights[r][COLS - 1])"),
        (10, "        return list(pac & atl)"),
    ]
}

pub fn surrounded_regions_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def solve(self, board: List[List[str]]) -> None:"),
        (3, "        ROWS, COLS = len(board), len(board[0])"),
        (4, "        def capture(r, c):"),
        (
            5,
            "            if r < 0 or c < 0 or r == ROWS or c == COLS or board[r][c] != 'O': return",
        ),
        (6, "            board[r][c] = 'T'"),
        (
            7,
            "            for dr, dc in [[1,0],[-1,0],[0,1],[0,-1]]: capture(r + dr, c + dc)",
        ),
        (
            8,
            "        for r in range(ROWS): capture(r, 0); capture(r, COLS - 1)",
        ),
        (
            9,
            "        for c in range(COLS): capture(0, c); capture(ROWS - 1, c)",
        ),
        (10, "        for r in range(ROWS):"),
        (11, "            for c in range(COLS):"),
        (
            12,
            "                if board[r][c] == 'O': board[r][c] = 'X'",
        ),
        (
            13,
            "                elif board[r][c] == 'T': board[r][c] = 'O'",
        ),
    ]
}

pub fn course_schedule_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def canFinish(self, numCourses: int, prerequisites: List[List[int]]) -> bool:",
        ),
        (3, "        preMap = { i: [] for i in range(numCourses) }"),
        (
            4,
            "        for crs, pre in prerequisites: preMap[crs].append(pre)",
        ),
        (5, "        visitSet = set()"),
        (6, "        def dfs(crs):"),
        (7, "            if crs in visitSet: return False"),
        (8, "            if preMap[crs] == []: return True"),
        (9, "            visitSet.add(crs)"),
        (10, "            for pre in preMap[crs]:"),
        (11, "                if not dfs(pre): return False"),
        (12, "            visitSet.remove(crs); preMap[crs] = []"),
        (13, "            return True"),
        (14, "        for crs in range(numCourses):"),
        (15, "            if not dfs(crs): return False"),
        (16, "        return True"),
    ]
}

pub fn course_schedule_ii_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def findOrder(self, numCourses: int, prerequisites: List[List[int]]) -> List[int]:"),
        (3, "        prereq = { i: [] for i in range(numCourses) }"),
        (4, "        for crs, pre in prerequisites: prereq[crs].append(pre)"),
        (5, "        output = []; visit, cycle = set(), set()"),
        (6, "        def dfs(crs):"),
        (7, "            if crs in cycle: return False"),
        (8, "            if crs in visit: return True"),
        (9, "            cycle.add(crs)"),
        (10, "            for pre in prereq[crs]:"),
        (11, "                if not dfs(pre): return False"),
        (12, "            cycle.remove(crs); visit.add(crs); output.append(crs)"),
        (13, "            return True"),
        (14, "        for c in range(numCourses):"),
        (15, "            if not dfs(c): return []"),
        (16, "        return output"),
    ]
}

pub fn graph_valid_tree_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def validTree(self, n: int, edges: List[List[int]]) -> bool:",
        ),
        (3, "        if not n: return True"),
        (4, "        adj = { i: [] for i in range(n) }"),
        (
            5,
            "        for n1, n2 in edges: adj[n1].append(n2); adj[n2].append(n1)",
        ),
        (6, "        visit = set()"),
        (7, "        def dfs(i, prev):"),
        (8, "            if i in visit: return False"),
        (9, "            visit.add(i)"),
        (10, "            for j in adj[i]:"),
        (11, "                if j == prev: continue"),
        (12, "                if not dfs(j, i): return False"),
        (13, "            return True"),
        (14, "        return dfs(0, -1) and len(visit) == n"),
    ]
}

pub fn connected_components_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def countComponents(self, n: int, edges: List[List[int]]) -> int:",
        ),
        (3, "        par = [i for i in range(n)]; rank = [1] * n"),
        (4, "        def find(n1):"),
        (5, "            res = n1"),
        (
            6,
            "            while res != par[res]: par[res] = par[par[res]]; res = par[res]",
        ),
        (7, "            return res"),
        (8, "        def union(n1, n2):"),
        (9, "            p1, p2 = find(n1), find(n2)"),
        (10, "            if p1 == p2: return 0"),
        (
            11,
            "            if rank[p2] > rank[p1]: par[p1] = p2; rank[p2] += rank[p1]",
        ),
        (12, "            else: par[p2] = p1; rank[p1] += rank[p2]"),
        (13, "            return 1"),
        (14, "        res = n"),
        (15, "        for n1, n2 in edges: res -= union(n1, n2)"),
        (16, "        return res"),
    ]
}

pub fn redundant_connection_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def findRedundantConnection(self, edges: List[List[int]]) -> List[int]:",
        ),
        (
            3,
            "        par = [i for i in range(len(edges) + 1)]; rank = [1] * (len(edges) + 1)",
        ),
        (4, "        def find(n):"),
        (5, "            p = par[n]"),
        (
            6,
            "            while p != par[p]: par[p] = par[par[p]]; p = par[p]",
        ),
        (7, "            return p"),
        (8, "        def union(n1, n2):"),
        (9, "            p1, p2 = find(n1), find(n2)"),
        (10, "            if p1 == p2: return False"),
        (
            11,
            "            if rank[p1] > rank[p2]: par[p2] = p1; rank[p1] += rank[p2]",
        ),
        (12, "            else: par[p1] = p2; rank[p2] += rank[p1]"),
        (13, "            return True"),
        (14, "        for n1, n2 in edges:"),
        (15, "            if not union(n1, n2): return [n1, n2]"),
    ]
}

pub fn word_ladder_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def ladderLength(self, beginWord: str, endWord: str, wordList: List[str]) -> int:"),
        (3, "        if endWord not in wordList: return 0"),
        (4, "        nei = collections.defaultdict(list); wordList.append(beginWord)"),
        (5, "        for word in wordList:"),
        (6, "            for j in range(len(word)):"),
        (7, "                pattern = word[:j] + \"*\" + word[j+1:]"),
        (8, "                nei[pattern].append(word)"),
        (9, "        visit = set([beginWord]); q = collections.deque([beginWord]); res = 1"),
        (10, "        while q:"),
        (11, "            for i in range(len(q)):"),
        (12, "                word = q.popleft()"),
        (13, "                if word == endWord: return res"),
        (14, "                for j in range(len(word)):"),
        (15, "                    pattern = word[:j] + \"*\" + word[j+1:]"),
        (16, "                    for neiWord in nei[pattern]:"),
        (17, "                        if neiWord not in visit: visit.add(neiWord); q.append(neiWord)"),
        (18, "            res += 1"),
        (19, "        return 0"),
    ]
}

pub fn maximum_subarray_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def maxSubArray(self, nums: List[int]) -> int:"),
        (3, "        maxSub, curSum = nums[0], 0"),
        (4, "        for n in nums:"),
        (5, "            if curSum < 0: curSum = 0"),
        (6, "            curSum += n"),
        (7, "            maxSub = max(maxSub, curSum)"),
        (8, "        return maxSub"),
    ]
}

pub fn jump_game_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def canJump(self, nums: List[int]) -> bool:"),
        (3, "        goal = len(nums) - 1"),
        (4, "        for i in range(len(nums) - 2, -1, -1):"),
        (5, "            if i + nums[i] >= goal: goal = i"),
        (6, "        return goal == 0"),
    ]
}

pub fn jump_game_ii_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def jump(self, nums: List[int]) -> int:"),
        (3, "        res = 0; l = r = 0"),
        (4, "        while r < len(nums) - 1:"),
        (5, "            farthest = 0"),
        (
            6,
            "            for i in range(l, r + 1): farthest = max(farthest, i + nums[i])",
        ),
        (7, "            l = r + 1; r = farthest; res += 1"),
        (8, "        return res"),
    ]
}

pub fn gas_station_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def canCompleteCircuit(self, gas: List[int], cost: List[int]) -> int:",
        ),
        (3, "        if sum(gas) < sum(cost): return -1"),
        (4, "        total, start = 0, 0"),
        (5, "        for i in range(len(gas)):"),
        (6, "            total += (gas[i] - cost[i])"),
        (7, "            if total < 0: total = 0; start = i + 1"),
        (8, "        return start"),
    ]
}

pub fn hand_of_straights_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def isNStraightHand(self, hand: List[int], groupSize: int) -> bool:",
        ),
        (3, "        if len(hand) % groupSize: return False"),
        (
            4,
            "        count = Counter(hand); minH = list(count.keys()); heapq.heapify(minH)",
        ),
        (5, "        while minH:"),
        (6, "            first = minH[0]"),
        (7, "            for i in range(first, first + groupSize):"),
        (8, "                if i not in count: return False"),
        (9, "                count[i] -= 1"),
        (10, "                if count[i] == 0:"),
        (11, "                    if i != minH[0]: return False"),
        (12, "                    heapq.heappop(minH)"),
        (13, "        return True"),
    ]
}

pub fn merge_triplets_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def mergeTriplets(self, triplets: List[List[int]], target: List[int]) -> bool:",
        ),
        (3, "        good = set()"),
        (4, "        for t in triplets:"),
        (
            5,
            "            if t[0] > target[0] or t[1] > target[1] or t[2] > target[2]: continue",
        ),
        (6, "            for i, v in enumerate(t):"),
        (7, "                if v == target[i]: good.add(i)"),
        (8, "        return len(good) == 3"),
    ]
}

pub fn partition_labels_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def partitionLabels(self, s: str) -> List[int]:"),
        (3, "        lastIndex = { c: i for i, c in enumerate(s) }"),
        (4, "        res = []; size = end = 0"),
        (5, "        for i, c in enumerate(s):"),
        (6, "            size += 1; end = max(end, lastIndex[c])"),
        (7, "            if i == end: res.append(size); size = 0"),
        (8, "        return res"),
    ]
}

pub fn valid_parenthesis_string_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def checkValidString(self, s: str) -> bool:"),
        (3, "        leftMin = leftMax = 0"),
        (4, "        for c in s:"),
        (5, "            if c == \"(\": leftMin += 1; leftMax += 1"),
        (6, "            elif c == \")\": leftMin -= 1; leftMax -= 1"),
        (7, "            else: leftMin -= 1; leftMax += 1"),
        (8, "            if leftMax < 0: return False"),
        (9, "            if leftMin < 0: leftMin = 0"),
        (10, "        return leftMin == 0"),
    ]
}

pub fn insert_interval_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def insert(self, intervals: List[List[int]], newInterval: List[int]) -> List[List[int]]:"),
        (3, "        res = []"),
        (4, "        for i in range(len(intervals)):"),
        (5, "            if newInterval[1] < intervals[i][0]: res.append(newInterval); return res + intervals[i:]"),
        (6, "            elif newInterval[0] > intervals[i][1]: res.append(intervals[i])"),
        (7, "            else: newInterval = [min(newInterval[0], intervals[i][0]), max(newInterval[1], intervals[i][1])]"),
        (8, "        res.append(newInterval)"),
        (9, "        return res"),
    ]
}

pub fn merge_intervals_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def merge(self, intervals: List[List[int]]) -> List[List[int]]:",
        ),
        (
            3,
            "        intervals.sort(key=lambda i: i[0]); output = [intervals[0]]",
        ),
        (4, "        for start, end in intervals[1:]:"),
        (5, "            lastEnd = output[-1][1]"),
        (
            6,
            "            if start <= lastEnd: output[-1][1] = max(lastEnd, end)",
        ),
        (7, "            else: output.append([start, end])"),
        (8, "        return output"),
    ]
}

pub fn non_overlapping_intervals_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def eraseOverlapIntervals(self, intervals: List[List[int]]) -> int:",
        ),
        (
            3,
            "        intervals.sort(key=lambda x: x[0]); res = 0; prevEnd = intervals[0][1]",
        ),
        (4, "        for start, end in intervals[1:]:"),
        (5, "            if start >= prevEnd: prevEnd = end"),
        (6, "            else: res += 1; prevEnd = min(end, prevEnd)"),
        (7, "        return res"),
    ]
}

pub fn meeting_rooms_ii_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def minMeetingRooms(self, intervals: List[List[int]]) -> int:",
        ),
        (3, "        start = sorted([i[0] for i in intervals])"),
        (4, "        end = sorted([i[1] for i in intervals])"),
        (5, "        res = count = s = e = 0"),
        (6, "        while s < len(intervals):"),
        (7, "            if start[s] < end[e]: s += 1; count += 1"),
        (8, "            else: e += 1; count -= 1"),
        (9, "            res = max(res, count)"),
        (10, "        return res"),
    ]
}

pub fn min_interval_query_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def minInterval(self, intervals: List[List[int]], queries: List[int]) -> List[int]:"),
        (3, "        intervals.sort(); minHeap = []; res = {}; i = 0"),
        (4, "        for q in sorted(queries):"),
        (5, "            while i < len(intervals) and intervals[i][0] <= q:"),
        (6, "                l, r = intervals[i]"),
        (7, "                heapq.heappush(minHeap, (r - l + 1, r)); i += 1"),
        (8, "            while minHeap and minHeap[0][1] < q: heapq.heappop(minHeap)"),
        (9, "            res[q] = minHeap[0][0] if minHeap else -1"),
        (10, "        return [res[q] for q in queries]"),
    ]
}

pub fn number_1_bits_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def hammingWeight(self, n: int) -> int:"),
        (3, "        res = 0"),
        (4, "        while n:"),
        (5, "            n &= (n - 1)"),
        (6, "            res += 1"),
        (7, "        return res"),
    ]
}

pub fn sum_two_integers_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def getSum(self, a: int, b: int) -> int:"),
        (3, "        mask = 0xFFFFFFFF"),
        (4, "        while (b & mask) > 0:"),
        (5, "            carry = (a & b) << 1"),
        (6, "            a = (a ^ b)"),
        (7, "            b = carry"),
        (8, "        return (a & mask) if b > 0 else a"),
    ]
}

pub fn reverse_integer_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def reverse(self, x: int) -> int:"),
        (3, "        MIN = -2147483648; MAX = 2147483647; res = 0"),
        (4, "        while x:"),
        (5, "            digit = int(math.fmod(x, 10))"),
        (6, "            x = int(x / 10)"),
        (
            7,
            "            if (res > MAX // 10 or (res == MAX // 10 and digit >= 7)): return 0",
        ),
        (
            8,
            "            if (res < MIN // 10 or (res == MIN // 10 and digit <= -8)): return 0",
        ),
        (9, "            res = (res * 10) + digit"),
        (10, "        return res"),
    ]
}

pub fn rotate_image_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def rotate(self, matrix: List[List[int]]) -> None:"),
        (3, "        l, r = 0, len(matrix) - 1"),
        (4, "        while l < r:"),
        (5, "            for i in range(r - l):"),
        (6, "                top, bottom = l, r"),
        (7, "                topLeft = matrix[top][l + i]"),
        (
            8,
            "                matrix[top][l + i] = matrix[bottom - i][l]",
        ),
        (
            9,
            "                matrix[bottom - i][l] = matrix[bottom][r - i]",
        ),
        (
            10,
            "                matrix[bottom][r - i] = matrix[top + i][r]",
        ),
        (11, "                matrix[top + i][r] = topLeft"),
        (12, "            r -= 1; l += 1"),
    ]
}

pub fn spiral_matrix_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def spiralOrder(self, matrix: List[List[int]]) -> List[int]:"),
        (3, "        res = []; left, right = 0, len(matrix[0]); top, bottom = 0, len(matrix)"),
        (4, "        while left < right and top < bottom:"),
        (5, "            for i in range(left, right): res.append(matrix[top][i])"),
        (6, "            top += 1"),
        (7, "            for i in range(top, bottom): res.append(matrix[i][right - 1])"),
        (8, "            right -= 1"),
        (9, "            if not (left < right and top < bottom): break"),
        (10, "            for i in range(right - 1, left - 1, -1): res.append(matrix[bottom - 1][i])"),
        (11, "            bottom -= 1"),
        (12, "            for i in range(bottom - 1, top - 1, -1): res.append(matrix[i][left])"),
        (13, "            left += 1"),
        (14, "        return res"),
    ]
}

pub fn set_matrix_zeroes_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def setZeroes(self, matrix: List[List[int]]) -> None:",
        ),
        (
            3,
            "        ROWS, COLS = len(matrix), len(matrix[0]); rowZero = False",
        ),
        (4, "        for r in range(ROWS):"),
        (5, "            for c in range(COLS):"),
        (6, "                if matrix[r][c] == 0:"),
        (7, "                    matrix[0][c] = 0"),
        (8, "                    if r > 0: matrix[r][0] = 0"),
        (9, "                    else: rowZero = True"),
        (10, "        for r in range(1, ROWS):"),
        (11, "            for c in range(1, COLS):"),
        (
            12,
            "                if matrix[0][c] == 0 or matrix[r][0] == 0: matrix[r][c] = 0",
        ),
        (13, "        if matrix[0][0] == 0:"),
        (14, "            for r in range(ROWS): matrix[r][0] = 0"),
        (15, "        if rowZero:"),
        (16, "            for c in range(COLS): matrix[0][c] = 0"),
    ]
}

pub fn pow_xn_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def myPow(self, x: float, n: int) -> float:"),
        (3, "        def helper(x, n):"),
        (4, "            if x == 0: return 0"),
        (5, "            if n == 0: return 1"),
        (6, "            res = helper(x * x, n // 2)"),
        (7, "            return x * res if n % 2 else res"),
        (8, "        res = helper(x, abs(n))"),
        (9, "        return res if n >= 0 else 1 / res"),
    ]
}

pub fn multiply_strings_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def multiply(self, num1: str, num2: str) -> str:"),
        (3, "        if \"0\" in [num1, num2]: return \"0\""),
        (4, "        res = [0] * (len(num1) + len(num2))"),
        (5, "        num1, num2 = num1[::-1], num2[::-1]"),
        (6, "        for i1 in range(len(num1)):"),
        (7, "            for i2 in range(len(num2)):"),
        (8, "                digit = int(num1[i1]) * int(num2[i2])"),
        (9, "                res[i1 + i2] += digit"),
        (10, "                res[i1 + i2 + 1] += res[i1 + i2] // 10"),
        (11, "                res[i1 + i2] %= 10"),
        (12, "        res, beg = res[::-1], 0"),
        (
            13,
            "        while beg < len(res) and res[beg] == 0: beg += 1",
        ),
        (14, "        res = map(str, res[beg:])"),
        (15, "        return \"\".join(res)"),
    ]
}

pub fn detect_squares_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class DetectSquares:"),
        (2, "    def __init__(self):"),
        (3, "        self.ptsCount = defaultdict(int); self.pts = []"),
        (4, "    def add(self, point: List[int]) -> None:"),
        (
            5,
            "        self.ptsCount[tuple(point)] += 1; self.pts.append(point)",
        ),
        (6, "    def count(self, point: List[int]) -> int:"),
        (7, "        res = 0; px, py = point"),
        (8, "        for x, y in self.pts:"),
        (
            9,
            "            if (abs(py - y) != abs(px - x)) or x == px or y == py: continue",
        ),
        (
            10,
            "            res += self.ptsCount[(x, py)] * self.ptsCount[(px, y)]",
        ),
        (11, "        return res"),
    ]
}

pub fn house_robber_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def rob(self, nums: List[int]) -> int:"),
        (3, "        rob1, rob2 = 0, 0"),
        (4, "        for n in nums:"),
        (5, "            temp = max(n + rob1, rob2)"),
        (6, "            rob1 = rob2"),
        (7, "            rob2 = temp"),
        (8, "        return rob2"),
    ]
}

pub fn house_robber_ii_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def rob(self, nums: List[int]) -> int:"),
        (
            3,
            "        return max(nums[0], self.helper(nums[1:]), self.helper(nums[:-1]))",
        ),
        (4, "    def helper(self, nums):"),
        (5, "        rob1, rob2 = 0, 0"),
        (6, "        for n in nums:"),
        (7, "            temp = max(n + rob1, rob2)"),
        (8, "            rob1 = rob2; rob2 = temp"),
        (9, "        return rob2"),
    ]
}

pub fn longest_palindromic_substring_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def longestPalindrome(self, s: str) -> str:"),
        (3, "        res = \"\"; resLen = 0"),
        (4, "        for i in range(len(s)):"),
        (5, "            l, r = i, i"),
        (
            6,
            "            while l >= 0 and r < len(s) and s[l] == s[r]:",
        ),
        (
            7,
            "                if (r - l + 1) > resLen: res = s[l:r+1]; resLen = r - l + 1",
        ),
        (8, "                l -= 1; r += 1"),
        (9, "            l, r = i, i + 1"),
        (
            10,
            "            while l >= 0 and r < len(s) and s[l] == s[r]:",
        ),
        (
            11,
            "                if (r - l + 1) > resLen: res = s[l:r+1]; resLen = r - l + 1",
        ),
        (12, "                l -= 1; r += 1"),
        (13, "        return res"),
    ]
}

pub fn palindromic_substrings_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def countSubstrings(self, s: str) -> int:"),
        (3, "        res = 0"),
        (4, "        for i in range(len(s)):"),
        (5, "            res += self.countPali(s, i, i)"),
        (6, "            res += self.countPali(s, i, i + 1)"),
        (7, "        return res"),
        (8, "    def countPali(self, s, l, r):"),
        (9, "        res = 0"),
        (10, "        while l >= 0 and r < len(s) and s[l] == s[r]:"),
        (11, "            res += 1; l -= 1; r += 1"),
        (12, "        return res"),
    ]
}

pub fn decode_ways_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def numDecodings(self, s: str) -> int:"),
        (3, "        dp = { len(s) : 1 }"),
        (4, "        for i in range(len(s) - 1, -1, -1):"),
        (5, "            if s[i] == \"0\": dp[i] = 0"),
        (6, "            else: dp[i] = dp[i + 1]"),
        (7, "            if (i + 1 < len(s) and (s[i] == \"1\" or (s[i] == \"2\" and s[i+1] in \"0123456\"))):"),
        (8, "                dp[i] += dp[i + 2]"),
        (9, "        return dp[0]"),
    ]
}

pub fn coin_change_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def coinChange(self, coins: List[int], amount: int) -> int:",
        ),
        (3, "        dp = [amount + 1] * (amount + 1)"),
        (4, "        dp[0] = 0"),
        (5, "        for a in range(1, amount + 1):"),
        (6, "            for c in coins:"),
        (
            7,
            "                if a - c >= 0: dp[a] = min(dp[a], 1 + dp[a - c])",
        ),
        (
            8,
            "        return dp[amount] if dp[amount] != amount + 1 else -1",
        ),
    ]
}

pub fn max_product_subarray_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def maxProduct(self, nums: List[int]) -> int:"),
        (3, "        res = max(nums); curMin, curMax = 1, 1"),
        (4, "        for n in nums:"),
        (5, "            if n == 0: curMin, curMax = 1, 1; continue"),
        (6, "            tmp = curMax * n"),
        (7, "            curMax = max(n * curMax, n * curMin, n)"),
        (8, "            curMin = min(tmp, n * curMin, n)"),
        (9, "            res = max(res, curMax)"),
        (10, "        return res"),
    ]
}

pub fn word_break_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def wordBreak(self, s: str, wordDict: List[str]) -> bool:",
        ),
        (3, "        dp = [False] * (len(s) + 1); dp[len(s)] = True"),
        (4, "        for i in range(len(s) - 1, -1, -1):"),
        (5, "            for w in wordDict:"),
        (
            6,
            "                if (i + len(w)) <= len(s) and s[i : i + len(w)] == w:",
        ),
        (7, "                    dp[i] = dp[i + len(w)]"),
        (8, "                if dp[i]: break"),
        (9, "        return dp[0]"),
    ]
}

pub fn longest_increasing_subsequence_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def lengthOfLIS(self, nums: List[int]) -> int:"),
        (3, "        LIS = [1] * len(nums)"),
        (4, "        for i in range(len(nums) - 1, -1, -1):"),
        (5, "            for j in range(i + 1, len(nums)):"),
        (
            6,
            "                if nums[i] < nums[j]: LIS[i] = max(LIS[i], 1 + LIS[j])",
        ),
        (7, "        return max(LIS)"),
    ]
}

pub fn partition_equal_subset_sum_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def canPartition(self, nums: List[int]) -> bool:"),
        (3, "        if sum(nums) % 2: return False"),
        (4, "        dp = set(); dp.add(0); target = sum(nums) // 2"),
        (5, "        for i in range(len(nums) - 1, -1, -1):"),
        (6, "            nextDP = set()"),
        (7, "            for t in dp:"),
        (8, "                if (t + nums[i]) == target: return True"),
        (9, "                nextDP.add(t + nums[i]); nextDP.add(t)"),
        (10, "            dp = nextDP"),
        (11, "        return True if target in dp else False"),
    ]
}

pub fn kth_largest_array_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def findKthLargest(self, nums: List[int], k: int) -> int:",
        ),
        (3, "        heap = nums[:k]"),
        (4, "        heapq.heapify(heap)"),
        (5, "        for num in nums[k:]:"),
        (6, "            if num > heap[0]:"),
        (7, "                heapq.heappushpop(heap, num)"),
        (8, "        return heap[0]"),
    ]
}

pub fn design_twitter_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Twitter:"),
        (2, "    def __init__(self):"),
        (3, "        self.count = 0; self.tweetMap = defaultdict(list); self.followMap = defaultdict(set)"),
        (4, "    def postTweet(self, userId: int, tweetId: int) -> None:"),
        (5, "        self.tweetMap[userId].append([self.count, tweetId]); self.count -= 1"),
        (6, "    def getNewsFeed(self, userId: int) -> List[int]:"),
        (7, "        res, minHeap = [], []"),
        (8, "        self.followMap[userId].add(userId)"),
        (9, "        for followeeId in self.followMap[userId]:"),
        (10, "            if followeeId in self.tweetMap:"),
        (11, "                index = len(self.tweetMap[followeeId]) - 1"),
        (12, "                count, tweetId = self.tweetMap[followeeId][index]"),
        (13, "                minHeap.append([count, tweetId, followeeId, index - 1])"),
        (14, "        heapq.heapify(minHeap)"),
        (15, "        while minHeap and len(res) < 10:"),
        (16, "            count, tweetId, followeeId, index = heapq.heappop(minHeap)"),
        (17, "            res.append(tweetId)"),
        (18, "            if index >= 0:"),
        (19, "                count, tweetId = self.tweetMap[followeeId][index]"),
        (20, "                heapq.heappush(minHeap, [count, tweetId, followeeId, index - 1])"),
        (21, "        return res"),
    ]
}

pub fn palindrome_partitioning_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def partition(self, s: str) -> List[List[str]]:"),
        (3, "        res, part = [], []"),
        (4, "        def dfs(i):"),
        (
            5,
            "            if i >= len(s): res.append(part.copy()); return",
        ),
        (6, "            for j in range(i, len(s)):"),
        (7, "                if self.isPali(s, i, j):"),
        (8, "                    part.append(s[i : j + 1])"),
        (9, "                    dfs(j + 1)"),
        (10, "                    part.pop()"),
        (11, "        dfs(0); return res"),
        (12, "    def isPali(self, s, l, r):"),
        (13, "        while l < r:"),
        (14, "            if s[l] != s[r]: return False"),
        (15, "            l, r = l + 1, r - 1"),
        (16, "        return True"),
    ]
}

pub fn letter_combinations_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def letterCombinations(self, digits: str) -> List[str]:"),
        (3, "        res = []"),
        (4, "        digitToChar = {\"2\":\"abc\",\"3\":\"def\",\"4\":\"ghi\",\"5\":\"jkl\",\"6\":\"mno\",\"7\":\"qprs\",\"8\":\"tuv\",\"9\":\"wxyz\"}"),
        (5, "        def backtrack(i, curStr):"),
        (6, "            if len(curStr) == len(digits): res.append(curStr); return"),
        (7, "            for c in digitToChar[digits[i]]:"),
        (8, "                backtrack(i + 1, curStr + c)"),
        (9, "        if digits: backtrack(0, \"\")"),
        (10, "        return res"),
    ]
}

pub fn find_median_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class MedianFinder:"),
        (2, "    def __init__(self):"),
        (3, "        self.small, self.large = [], []"),
        (4, "    def addNum(self, num: int) -> None:"),
        (5, "        heapq.heappush(self.small, -1 * num)"),
        (
            6,
            "        if self.small and self.large and (-1 * self.small[0]) > self.large[0]:",
        ),
        (7, "            val = -1 * heapq.heappop(self.small)"),
        (8, "            heapq.heappush(self.large, val)"),
        (9, "        if len(self.small) > len(self.large) + 1:"),
        (10, "            val = -1 * heapq.heappop(self.small)"),
        (11, "            heapq.heappush(self.large, val)"),
        (12, "        if len(self.large) > len(self.small) + 1:"),
        (13, "            val = heapq.heappop(self.large)"),
        (14, "            heapq.heappush(self.small, -1 * val)"),
        (15, "    def findMedian(self) -> float:"),
        (
            16,
            "        if len(self.small) > len(self.large): return -1 * self.small[0]",
        ),
        (
            17,
            "        if len(self.large) > len(self.small): return self.large[0]",
        ),
        (
            18,
            "        return (-1 * self.small[0] + self.large[0]) / 2.0",
        ),
    ]
}

pub fn combination_sum_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def combinationSum(self, candidates: List[int], target: int) -> List[List[int]]:",
        ),
        (3, "        res = []"),
        (4, "        def dfs(i, cur, total):"),
        (5, "            if total == target:"),
        (6, "                res.append(cur.copy())"),
        (7, "                return"),
        (8, "            if i >= len(candidates) or total > target:"),
        (9, "                return"),
        (10, "            cur.append(candidates[i])"),
        (11, "            dfs(i, cur, total + candidates[i])"),
        (12, "            cur.pop()"),
        (13, "            dfs(i + 1, cur, total)"),
        (14, "        dfs(0, [], 0)"),
        (15, "        return res"),
    ]
}

pub fn subsets_ii_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def subsetsWithDup(self, nums: List[int]) -> List[List[int]]:",
        ),
        (3, "        res = []"),
        (4, "        nums.sort()"),
        (5, "        def backtrack(i, subset):"),
        (6, "            if i == len(nums):"),
        (7, "                res.append(subset.copy())"),
        (8, "                return"),
        (9, "            subset.append(nums[i])"),
        (10, "            backtrack(i + 1, subset)"),
        (11, "            subset.pop()"),
        (
            12,
            "            while i + 1 < len(nums) and nums[i] == nums[i + 1]: i += 1",
        ),
        (13, "            backtrack(i + 1, subset)"),
        (14, "        backtrack(0, [])"),
        (15, "        return res"),
    ]
}

pub fn combination_sum_ii_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def combinationSum2(self, candidates: List[int], target: int) -> List[List[int]]:",
        ),
        (3, "        candidates.sort()"),
        (4, "        res = []"),
        (5, "        def backtrack(pos, cur, target):"),
        (
            6,
            "            if target == 0: res.append(cur.copy()); return",
        ),
        (7, "            if target <= 0: return"),
        (8, "            prev = -1"),
        (9, "            for i in range(pos, len(candidates)):"),
        (10, "                if candidates[i] == prev: continue"),
        (11, "                cur.append(candidates[i])"),
        (
            12,
            "                backtrack(i + 1, cur, target - candidates[i])",
        ),
        (13, "                cur.pop()"),
        (14, "                prev = candidates[i]"),
        (15, "        backtrack(0, [], target)"),
        (16, "        return res"),
    ]
}

pub fn word_search_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def exist(self, board: List[List[str]], word: str) -> bool:",
        ),
        (3, "        ROWS, COLS = len(board), len(board[0])"),
        (4, "        path = set()"),
        (5, "        def dfs(r, c, i):"),
        (6, "            if i == len(word): return True"),
        (
            7,
            "            if (r < 0 or c < 0 or r >= ROWS or c >= COLS or",
        ),
        (
            8,
            "                word[i] != board[r][c] or (r, c) in path): return False",
        ),
        (9, "            path.add((r, c))"),
        (
            10,
            "            res = (dfs(r + 1, c, i + 1) or dfs(r - 1, c, i + 1) or",
        ),
        (
            11,
            "                   dfs(r, c + 1, i + 1) or dfs(r, c - 1, i + 1))",
        ),
        (12, "            path.remove((r, c))"),
        (13, "            return res"),
        (14, "        for r in range(ROWS):"),
        (15, "            for c in range(COLS):"),
        (16, "                if dfs(r, c, 0): return True"),
        (17, "        return False"),
    ]
}

pub fn n_queens_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def solveNQueens(self, n: int) -> List[List[str]]:"),
        (3, "        col, posDiag, negDiag = set(), set(), set()"),
        (
            4,
            "        res = []; board = [[\".\"] * n for _ in range(n)]",
        ),
        (5, "        def backtrack(r):"),
        (6, "            if r == n:"),
        (
            7,
            "                copy = [\"\".join(row) for row in board]",
        ),
        (8, "                res.append(copy); return"),
        (9, "            for c in range(n):"),
        (
            10,
            "                if c in col or (r + c) in posDiag or (r - c) in negDiag: continue",
        ),
        (
            11,
            "                col.add(c); posDiag.add(r + c); negDiag.add(r - c)",
        ),
        (12, "                board[r][c] = \"Q\""),
        (13, "                backtrack(r + 1)"),
        (
            14,
            "                col.remove(c); posDiag.remove(r + c); negDiag.remove(r - c)",
        ),
        (15, "                board[r][c] = \".\""),
        (16, "        backtrack(0)"),
        (17, "        return res"),
    ]
}

pub fn subsets_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def subsets(self, nums: List[int]) -> List[List[int]]:",
        ),
        (3, "        res = []"),
        (4, "        subset = []"),
        (5, "        def dfs(i):"),
        (6, "            if i >= len(nums):"),
        (7, "                res.append(subset.copy())"),
        (8, "                return"),
        (9, "            subset.append(nums[i])"),
        (10, "            dfs(i + 1)"),
        (11, "            subset.pop()"),
        (12, "            dfs(i + 1)"),
        (13, "        dfs(0)"),
        (14, "        return res"),
    ]
}

pub fn permutations_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def permute(self, nums: List[int]) -> List[List[int]]:",
        ),
        (3, "        res = []"),
        (4, "        def backtrack(curr, used):"),
        (5, "            if len(curr) == len(nums):"),
        (6, "                res.append(curr.copy())"),
        (7, "                return"),
        (8, "            for i in range(len(nums)):"),
        (9, "                if not used[i]:"),
        (10, "                    used[i] = True"),
        (11, "                    curr.append(nums[i])"),
        (12, "                    backtrack(curr, used)"),
        (13, "                    curr.pop()"),
        (14, "                    used[i] = False"),
        (15, "        backtrack([], [False] * len(nums))"),
        (16, "        return res"),
    ]
}

pub fn k_closest_points_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def kClosest(self, points: List[List[int]], k: int) -> List[List[int]]:",
        ),
        (3, "        minHeap = []"),
        (4, "        for x, y in points:"),
        (5, "            dist = (x ** 2) + (y ** 2)"),
        (6, "            minHeap.append([dist, x, y])"),
        (7, "        heapq.heapify(minHeap)"),
        (8, "        res = []"),
        (9, "        for _ in range(k):"),
        (10, "            dist, x, y = heapq.heappop(minHeap)"),
        (11, "            res.append([x, y])"),
        (12, "        return res"),
    ]
}

pub fn task_scheduler_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def leastInterval(self, tasks: List[str], n: int) -> int:",
        ),
        (3, "        count = Counter(tasks)"),
        (4, "        maxHeap = [-cnt for cnt in count.values()]"),
        (5, "        heapq.heapify(maxHeap)"),
        (6, "        time = 0"),
        (7, "        q = deque()"),
        (8, "        while maxHeap or q:"),
        (9, "            time += 1"),
        (10, "            if maxHeap:"),
        (11, "                cnt = 1 + heapq.heappop(maxHeap)"),
        (12, "                if cnt: q.append([cnt, time + n])"),
        (13, "            if q and q[0][1] == time:"),
        (
            14,
            "                heapq.heappush(maxHeap, q.popleft()[0])",
        ),
        (15, "        return time"),
    ]
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

pub fn implement_trie_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class TrieNode:"),
        (2, "    def __init__(self):"),
        (3, "        self.children = {}"),
        (4, "        self.is_end = False"),
        (5, ""),
        (6, "class Trie:"),
        (7, "    def insert(self, word: str) -> None:"),
        (8, "        curr = self.root"),
        (9, "        for c in word:"),
        (10, "            if c not in curr.children:"),
        (11, "                curr.children[c] = TrieNode()"),
        (12, "            curr = curr.children[c]"),
        (13, "        curr.is_end = True"),
        (14, ""),
        (15, "    def search(self, word: str) -> bool:"),
        (16, "        curr = self.root"),
        (17, "        for c in word:"),
        (18, "            if c not in curr.children: return False"),
        (19, "            curr = curr.children[c]"),
        (20, "        return curr.is_end"),
    ]
}

pub fn word_dictionary_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class WordDictionary:"),
        (2, "    def addWord(self, word: str) -> None:"),
        (3, "        curr = self.root"),
        (4, "        for c in word:"),
        (
            5,
            "            if c not in curr.children: curr.children[c] = TrieNode()",
        ),
        (6, "            curr = curr.children[c]"),
        (7, "        curr.is_end = True"),
        (8, ""),
        (9, "    def search(self, word: str) -> bool:"),
        (10, "        def dfs(j, root):"),
        (11, "            curr = root"),
        (12, "            for i in range(j, len(word)):"),
        (13, "                c = word[i]"),
        (14, "                if c == '.':"),
        (
            15,
            "                    for child in curr.children.values():",
        ),
        (
            16,
            "                        if dfs(i + 1, child): return True",
        ),
        (17, "                    return False"),
        (
            18,
            "                if c not in curr.children: return False",
        ),
        (19, "                curr = curr.children[c]"),
        (20, "            return curr.is_end"),
        (21, "        return dfs(0, self.root)"),
    ]
}

pub fn word_search_ii_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def findWords(self, board, words):"),
        (3, "        root = TrieNode()"),
        (4, "        for w in words: root.addWord(w)"),
        (5, "        res, visited = set(), set()"),
        (6, ""),
        (7, "        def dfs(r, c, node, word):"),
        (
            8,
            "            if r < 0 or c < 0 or r >= ROWS or c >= COLS: return",
        ),
        (
            9,
            "            if (r, c) in visited or board[r][c] not in node.children: return",
        ),
        (10, "            visited.add((r, c))"),
        (11, "            node = node.children[board[r][c]]"),
        (12, "            word += board[r][c]"),
        (13, "            if node.is_end: res.add(word)"),
        (14, "            for dr, dc in [(-1,0),(1,0),(0,-1),(0,1)]:"),
        (15, "                dfs(r + dr, c + dc, node, word)"),
        (16, "            visited.remove((r, c))"),
        (17, ""),
        (18, "        return list(res)"),
    ]
}
