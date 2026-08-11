use crate::model::problem::{ApproachMeta, Example, Problem, ProblemDetails};
use crate::model::taxonomy::{Category, Difficulty};

pub fn get_details(problem: Problem) -> Option<ProblemDetails> {
    match problem {
        Problem::ContainsDuplicate => Some(ProblemDetails {
            id: 217,
            title: "Contains Duplicate",
            difficulty: Difficulty::Easy,
            category: Category::ArraysAndHashing,
            statement: "Given an integer array nums, return true if any value appears at least twice in the array.",
            examples: &[Example {
                input: "nums = [1, 2, 3, 1]",
                output: "true",
                explanation: "Digit 1 appears twice.",
            }],
            constraints: &["1 <= nums.length <= 10^5"],
            leetcode_url: "https://leetcode.com/problems/contains-duplicate/",
            approaches: &[
                ApproachMeta {
                    id: 0,
                    name: "Hash Set Lookup",
                    time_complexity: "O(N)",
                    space_complexity: "O(N)",
                    rationale: "Checking a Hash Set takes O(1) average time per element and avoids comparing every pair.",
                    description: "Insert each value into a set and stop when a value is already present.",
                },
                ApproachMeta {
                    id: 1,
                    name: "Sort + Adjacent Scan",
                    time_complexity: "O(N log N)",
                    space_complexity: "O(N)",
                    rationale: "Sorting places duplicate values next to each other, after which one linear scan can detect a match.",
                    description: "Sort the values, then compare each adjacent pair.",
                },
            ],
        }),
        Problem::TwoSum => Some(ProblemDetails {
            id: 1,
            title: "Two Sum",
            difficulty: Difficulty::Easy,
            category: Category::ArraysAndHashing,
            statement: "Given an array of integers nums and a target, return indices of the two numbers that add up to target.",
            examples: &[Example {
                input: "nums = [2, 7, 11, 15], target = 9",
                output: "[0, 1]",
                explanation: "nums[0] + nums[1] == 9",
            }],
            constraints: &["2 <= nums.length <= 10^4"],
            leetcode_url: "https://leetcode.com/problems/two-sum/",
            approaches: &[
                ApproachMeta {
                    id: 0,
                    name: "Hash Map (One Pass)",
                    time_complexity: "O(N)",
                    space_complexity: "O(N)",
                    rationale: "A Hash Map finds each value's complement in O(1) average time instead of checking every pair.",
                    description: "Store each visited value and look up the complement in one pass.",
                },
                ApproachMeta {
                    id: 1,
                    name: "Brute Force Pair Scan",
                    time_complexity: "O(N^2)",
                    space_complexity: "O(1)",
                    rationale: "Trying every distinct pair is the simplest baseline and makes the quadratic cost visible.",
                    description: "Check every pair of indices until their values sum to the target.",
                },
            ],
        }),
        Problem::ValidAnagram => Some(ProblemDetails {
            id: 242,
            title: "Valid Anagram",
            difficulty: Difficulty::Easy,
            category: Category::ArraysAndHashing,
            statement: "Given two strings s and t, return true if t is an anagram of s.",
            examples: &[Example {
                input: "s = \"anagram\", t = \"nagaram\"",
                output: "true",
                explanation: "Frequencies match.",
            }],
            constraints: &["1 <= s.length <= 5*10^4", "s and t contain lowercase English letters"],
            leetcode_url: "https://leetcode.com/problems/valid-anagram/",
            approaches: &[
                ApproachMeta {
                    id: 0,
                    name: "Frequency Counters",
                    time_complexity: "O(N)",
                    space_complexity: "O(1)",
                    rationale: "Two fixed-size counters process both lowercase strings in one pass and use only 26 slots each.",
                    description: "Count each letter in both strings, then compare the counters.",
                },
                ApproachMeta {
                    id: 1,
                    name: "Sort + Compare",
                    time_complexity: "O(N log N)",
                    space_complexity: "O(N)",
                    rationale: "Anagrams contain the same characters, so their sorted character sequences must be identical.",
                    description: "Sort both strings and compare the resulting character sequences.",
                },
            ],
        }),
        Problem::GroupAnagrams => Some(ProblemDetails {
                id: 49, title: "Group Anagrams", difficulty: Difficulty::Medium, category: Category::ArraysAndHashing,
                statement: "Given an array of strings strs, group the anagrams together.",
                examples: &[Example { input: "strs = [\"eat\",\"tea\",\"tan\",\"ate\",\"nat\",\"bat\"]", output: "[[\"bat\"],[\"nat\",\"tan\"],[\"ate\",\"eat\",\"tea\"]]", explanation: "Anagrams grouped by key." }],
                constraints: &["1 <= strs.length <= 10^4"], leetcode_url: "https://leetcode.com/problems/group-anagrams/",
                approaches: &[ApproachMeta { id: 0, name: "Char Frequency Tuple Map", time_complexity: "O(N * K)", space_complexity: "O(N * K)", rationale: "Using character frequency tuples as Hash Map keys groups anagrams in O(N * K) time without sorting individual strings.", description: "Tuple key map." }],
            }),
        Problem::TopKFrequent => Some(ProblemDetails {
                id: 347, title: "Top K Frequent Elements", difficulty: Difficulty::Medium, category: Category::ArraysAndHashing,
                statement: "Given an integer array nums and integer k, return the k most frequent elements.",
                examples: &[Example { input: "nums = [1,1,1,2,2,3], k = 2", output: "[1, 2]", explanation: "1 appears 3x, 2 appears 2x." }],
                constraints: &["1 <= nums.length <= 10^5"], leetcode_url: "https://leetcode.com/problems/top-k-frequent-elements/",
                approaches: &[ApproachMeta { id: 0, name: "Bucket Sort", time_complexity: "O(N)", space_complexity: "O(N)", rationale: "Bucket sorting by frequency index allows linear O(N) extraction of top K elements, outperforming O(N log N) heap/sorting methods.", description: "Frequency buckets." }],
            }),
        Problem::ProductExceptSelf => Some(ProblemDetails {
                id: 238, title: "Product of Array Except Self", difficulty: Difficulty::Medium, category: Category::ArraysAndHashing,
                statement: "Return an array output where output[i] is the product of all elements except nums[i].",
                examples: &[Example { input: "nums = [1, 2, 4, 6]", output: "[48, 24, 12, 8]", explanation: "Prefix/suffix passes." }],
                constraints: &["2 <= nums.length <= 1000"], leetcode_url: "https://leetcode.com/problems/product-of-array-except-self/",
                approaches: &[ApproachMeta { id: 0, name: "Prefix & Suffix Pass", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Computing left prefix products and right suffix products in two O(N) passes avoids division while keeping extra space to O(1).", description: "Running prefix & suffix." }],
            }),
        Problem::EncodeDecode => Some(ProblemDetails {
                id: 271, title: "Encode and Decode Strings", difficulty: Difficulty::Medium, category: Category::ArraysAndHashing,
                statement: "Design an algorithm to encode a list of strings to a string and decode it back.",
                examples: &[Example { input: "strs = [\"Hello\",\"World\"]", output: "[\"Hello\",\"World\"]", explanation: "Encoded into 5#Hello5#World." }],
                constraints: &["0 <= strs.length < 100"], leetcode_url: "https://leetcode.com/problems/encode-and-decode-strings/",
                approaches: &[ApproachMeta { id: 0, name: "Length Prefix (# Protocol)", time_complexity: "O(N)", space_complexity: "O(N)", rationale: "Prepending character length and a delimiter (#) guarantees unambiguous parsing regardless of special characters in strings.", description: "Len#str encoding." }],
            }),
        Problem::ValidSudoku => Some(ProblemDetails {
                id: 36, title: "Valid Sudoku", difficulty: Difficulty::Medium, category: Category::ArraysAndHashing,
                statement: "Determine if a 9x9 Sudoku board is valid (rows, cols, 3x3 boxes).",
                examples: &[Example { input: "board = [[1, 2, ...]]", output: "true", explanation: "No duplicates." }],
                constraints: &["board.length == 9"], leetcode_url: "https://leetcode.com/problems/valid-sudoku/",
                approaches: &[ApproachMeta { id: 0, name: "HashSet Validation", time_complexity: "O(1)", space_complexity: "O(1)", rationale: "A single 9x9 grid scan verifies row, column, and 3x3 box constraints in deterministic O(1) constant time.", description: "Scan rows, cols, 3x3 boxes." }],
            }),
        Problem::LongestConsecutive => Some(ProblemDetails {
                id: 128, title: "Longest Consecutive Sequence", difficulty: Difficulty::Medium, category: Category::ArraysAndHashing,
                statement: "Return the length of the longest consecutive elements sequence.",
                examples: &[Example { input: "nums = [2, 20, 4, 10, 3, 4, 5]", output: "4", explanation: "Sequence [2, 3, 4, 5]." }],
                constraints: &["0 <= nums.length <= 1000"], leetcode_url: "https://leetcode.com/problems/longest-consecutive-sequence/",
                approaches: &[ApproachMeta { id: 0, name: "HashSet Sequence Start Expansion", time_complexity: "O(N)", space_complexity: "O(N)", rationale: "Only expanding streaks from sequence start numbers (where n-1 is not in set) guarantees each number is visited at most twice (O(N)).", description: "Expand from streak starts." }],
            }),
        _ => None,
    }
}

pub fn get_code_lines(problem: Problem, approach_id: usize) -> Option<Vec<(usize, &'static str)>> {
    match (problem, approach_id) {
        (Problem::ContainsDuplicate, 0) => Some(vec![
            (1, "class Solution:"),
            (2, "    def containsDuplicate(self, nums: List[int]) -> bool:"),
            (3, "        seen = set()"),
            (4, "        for n in nums:"),
            (5, "            if n in seen:"),
            (6, "                return True"),
            (7, "            seen.add(n)"),
            (8, "        return False"),
        ]),
        (Problem::ContainsDuplicate, 1) => Some(vec![
            (1, "class Solution:"),
            (2, "    def containsDuplicate(self, nums: List[int]) -> bool:"),
            (3, "        nums.sort()"),
            (4, "        for i in range(1, len(nums)):"),
            (5, "            if nums[i - 1] == nums[i]:"),
            (6, "                return True"),
            (7, "        return False"),
        ]),
        (Problem::TwoSum, 0) => Some(vec![
            (1, "class Solution:"),
            (2, "    def twoSum(self, nums: List[int], target: int) -> List[int]:"),
            (3, "        prevMap = {} # val -> index"),
            (4, "        for i, n in enumerate(nums):"),
            (5, "            diff = target - n"),
            (6, "            if diff in prevMap:"),
            (7, "                return [prevMap[diff], i]"),
            (8, "            prevMap[n] = i"),
            (9, "        return []"),
        ]),
        (Problem::TwoSum, 1) => Some(vec![
            (1, "class Solution:"),
            (2, "    def twoSum(self, nums: List[int], target: int) -> List[int]:"),
            (3, "        for i in range(len(nums)):"),
            (4, "            for j in range(i + 1, len(nums)):"),
            (5, "                if nums[i] + nums[j] == target:"),
            (6, "                    return [i, j]"),
            (7, "        return []"),
        ]),
        (Problem::ValidAnagram, 0) => Some(vec![
            (1, "class Solution:"),
            (2, "    def isAnagram(self, s: str, t: str) -> bool:"),
            (3, "        if len(s) != len(t): return False"),
            (4, "        count_s, count_t = [0] * 26, [0] * 26"),
            (5, "        for i in range(len(s)):"),
            (6, "            count_s[ord(s[i]) - ord('a')] += 1"),
            (7, "            count_t[ord(t[i]) - ord('a')] += 1"),
            (8, ""),
            (9, "        return count_s == count_t"),
        ]),
        (Problem::ValidAnagram, 1) => Some(vec![
            (1, "class Solution:"),
            (2, "    def isAnagram(self, s: str, t: str) -> bool:"),
            (3, "        if len(s) != len(t):"),
            (4, "            return False"),
            (5, "        sorted_s = sorted(s)"),
            (6, "        sorted_t = sorted(t)"),
            (7, "        return sorted_s == sorted_t"),
        ]),
        (Problem::GroupAnagrams, 0) => Some(vec![
            (1, "class Solution:"),
            (2, "    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:"),
            (3, "        res = defaultdict(list)"),
            (4, "        for s in strs:"),
            (5, "            count = [0] * 26"),
            (6, "            for c in s:"),
            (7, "                count[ord(c) - ord('a')] += 1"),
            (8, "            res[tuple(count)].append(s)"),
            (9, "        return list(res.values())"),
        ]),
        (Problem::TopKFrequent, 0) => Some(topk_code_lines()),
        (Problem::TopKFrequent, 1) => Some(vec![
            (1, "class Solution:"),
            (2, "    def topKFrequent(self, nums: List[int], k: int) -> List[int]:"),
            (3, "        count = Counter(nums)"),
            (4, "        heap = []"),
            (5, "        for val, freq in count.items():"),
            (6, "            heapq.heappush(heap, (freq, val))"),
            (7, "            if len(heap) > k: heapq.heappop(heap)"),
            (8, "        return [val for freq, val in heap]"),
        ]),
        (Problem::TopKFrequent, _) => Some(vec![
            (1, "class Solution:"),
            (2, "    def topKFrequent(self, nums: List[int], k: int) -> List[int]:"),
            (3, "        count = Counter(nums)"),
            (4, "        sorted_items = sorted(count.items(), key=lambda x: x[1], reverse=True)"),
            (5, "        return [val for val, freq in sorted_items[:k]]"),
        ]),
        (Problem::ProductExceptSelf, _) => Some(product_code_lines()),
        (Problem::EncodeDecode, _) => Some(encode_decode_code_lines()),
        (Problem::ValidSudoku, _) => Some(vec![
            (1, "class Solution:"),
            (2, "    def isValidSudoku(self, board: List[List[str]]) -> bool:"),
            (3, "        rows = defaultdict(set)"),
            (4, "        cols = defaultdict(set)"),
            (5, "        squares = defaultdict(set)"),
            (6, "        for r in range(9):"),
            (7, "            for c in range(9):"),
            (8, "                if board[r][c] == '.': continue"),
            (9, "                val = board[r][c]"),
            (10, "                if val in rows[r] or val in cols[c] or val in squares[(r//3, c//3)]:"),
            (11, "                    return False"),
            (12, "                rows[r].add(val); cols[c].add(val); squares[(r//3, c//3)].add(val)"),
            (13, "        return True"),
        ]),
        (Problem::LongestConsecutive, _) => Some(vec![
            (1, "class Solution:"),
            (2, "    def longestConsecutive(self, nums: List[int]) -> int:"),
            (3, "        numSet = set(nums); longest = 0"),
            (4, ""),
            (5, "        for n in numSet:"),
            (6, "            # Only start counting at the beginning of a sequence"),
            (7, "            if (n - 1) not in numSet:"),
            (8, "                length = 1"),
            (9, "                while (n + length) in numSet: length += 1"),
            (10, ""),
            (11, "                longest = max(longest, length)"),
            (12, "        return longest"),
        ]),
        _ => None,
    }
}

// ── Helper Code Line Generators ──

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn easy_problem_variants_have_matching_source_shapes() {
        let contains_duplicate = get_code_lines(Problem::ContainsDuplicate, 1)
            .expect("sorting source must be registered")
            .into_iter()
            .map(|(_, line)| line)
            .collect::<Vec<_>>()
            .join("\n");
        assert!(contains_duplicate.contains("nums.sort()"));
        assert!(contains_duplicate.contains("nums[i - 1] == nums[i]"));

        let two_sum = get_code_lines(Problem::TwoSum, 1)
            .expect("brute-force source must be registered")
            .into_iter()
            .map(|(_, line)| line)
            .collect::<Vec<_>>()
            .join("\n");
        assert!(two_sum.contains("for j in range(i + 1"));
        assert!(!two_sum.contains("l, r ="));

        let valid_anagram = get_code_lines(Problem::ValidAnagram, 1)
            .expect("sorting source must be registered")
            .into_iter()
            .map(|(_, line)| line)
            .collect::<Vec<_>>()
            .join("\n");
        assert!(valid_anagram.contains("sorted_s = sorted(s)"));
        assert!(valid_anagram.contains("sorted_t = sorted(t)"));
    }
}
