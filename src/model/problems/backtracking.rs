use crate::model::problem::{ApproachMeta, Example, Problem, ProblemDetails};
use crate::model::taxonomy::{Category, Difficulty};

pub fn get_details(problem: Problem) -> Option<ProblemDetails> {
    match problem {
        Problem::Subsets => Some(ProblemDetails {
                id: 78, title: "Subsets", difficulty: Difficulty::Medium, category: Category::Backtracking,
                statement: "Given an integer array nums of unique elements, return all possible subsets (the power set). The solution set must not contain duplicate subsets. Return the solution in any order.",
                examples: &[Example { input: "nums = [1,2,3]", output: "[[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]", explanation: "Generate all 2^N combinations using binary choice decision tree." }],
                constraints: &["1 <= nums.length <= 10", "-10 <= nums[i] <= 10"], leetcode_url: "https://leetcode.com/problems/subsets/",
                approaches: &[ApproachMeta { id: 0, name: "Cascading Backtracking Decision Tree", time_complexity: "O(N * 2^N)", space_complexity: "O(N)", rationale: "At each element, make a binary choice to include or exclude, producing 2^N subsets with O(N) recursion stack space.", description: "Recurse choosing to include or exclude each element." }],
            }),
        Problem::Permutations => Some(ProblemDetails {
                id: 46, title: "Permutations", difficulty: Difficulty::Medium, category: Category::Backtracking,
                statement: "Given an array nums of distinct integers, return all the possible permutations. You can return the answer in any order.",
                examples: &[Example { input: "nums = [1,2,3]", output: "[[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]", explanation: "Explore all N! ordering branches." }],
                constraints: &["1 <= nums.length <= 6", "-10 <= nums[i] <= 10"], leetcode_url: "https://leetcode.com/problems/permutations/",
                approaches: &[ApproachMeta { id: 0, name: "Backtracking with Used Flag Array", time_complexity: "O(N * N!)", space_complexity: "O(N)", rationale: "Exploring all N! permutations with a boolean used array takes O(N * N!) time and O(N) stack space.", description: "Track used elements and construct all distinct position orderings." }],
            }),
        Problem::CombinationSum => Some(ProblemDetails {
                id: 39, title: "Combination Sum", difficulty: Difficulty::Medium, category: Category::Backtracking,
                statement: "Given an array of distinct integers candidates and a target integer target, return a list of all unique combinations of candidates where the chosen numbers sum to target.",
                examples: &[Example { input: "candidates = [2,3,6,7], target = 7", output: "[[2,2,3],[7]]", explanation: "The same number may be chosen an unlimited number of times." }],
                constraints: &["1 <= candidates.length <= 30", "2 <= target <= 40"], leetcode_url: "https://leetcode.com/problems/combination-sum/",
                approaches: &[ApproachMeta { id: 0, name: "Backtracking Search Tree", time_complexity: "O(2^T)", space_complexity: "O(T)", rationale: "Branching on including the current candidate multiple times or moving to the next candidate explores valid sum paths.", description: "Recurse exploring combinations with replacement." }],
            }),
        Problem::SubsetsII => Some(ProblemDetails {
                id: 90, title: "Subsets II", difficulty: Difficulty::Medium, category: Category::Backtracking,
                statement: "Given an integer array nums that may contain duplicates, return all possible subsets (the power set). The solution set must not contain duplicate subsets.",
                examples: &[Example { input: "nums = [1,2,2]", output: "[[],[1],[1,2],[1,2,2],[2],[2,2]]", explanation: "Skip duplicate elements at the same decision level." }],
                constraints: &["1 <= nums.length <= 10", "-10 <= nums[i] <= 10"], leetcode_url: "https://leetcode.com/problems/subsets-ii/",
                approaches: &[ApproachMeta { id: 0, name: "Sorted Backtracking with Duplicate Pruning", time_complexity: "O(N * 2^N)", space_complexity: "O(N)", rationale: "Sorting nums and skipping adjacent duplicates at the same tree depth prevents duplicate subset branches.", description: "Sort nums and skip duplicate adjacent elements during recursion." }],
            }),
        Problem::CombinationSumII => Some(ProblemDetails {
                id: 40, title: "Combination Sum II", difficulty: Difficulty::Medium, category: Category::Backtracking,
                statement: "Given a collection of candidate numbers (candidates) and a target number (target), find all unique combinations in candidates where the candidate numbers sum to target. Each number may only be used once.",
                examples: &[Example { input: "candidates = [10,1,2,7,6,1,5], target = 8", output: "[[1,1,6],[1,2,5],[1,7],[2,6]]", explanation: "Use each element once and skip duplicate branch starts." }],
                constraints: &["1 <= candidates.length <= 100", "1 <= target <= 30"], leetcode_url: "https://leetcode.com/problems/combination-sum-ii/",
                approaches: &[ApproachMeta { id: 0, name: "Backtracking with Pruning & Sorting", time_complexity: "O(2^N)", space_complexity: "O(N)", rationale: "Sorting candidates allows pruning search branches when sum exceeds target and avoiding duplicate combinations.", description: "Sort candidates and skip duplicate branch choices." }],
            }),
        Problem::WordSearch => Some(ProblemDetails {
                id: 79, title: "Word Search", difficulty: Difficulty::Medium, category: Category::Backtracking,
                statement: "Given an m x n grid of characters board and a string word, return true if word exists in the grid.",
                examples: &[Example { input: "board = [[\"A\",\"B\",\"C\",\"E\"],[\"S\",\"F\",\"C\",\"S\"],[\"A\",\"D\",\"E\",\"E\"]], word = \"ABCCED\"", output: "true", explanation: "Search adjacent cells grid DFS." }],
                constraints: &["m == board.length", "n == board[i].length", "1 <= word.length <= 15"], leetcode_url: "https://leetcode.com/problems/word-search/",
                approaches: &[ApproachMeta { id: 0, name: "2D Grid Backtracking DFS", time_complexity: "O(N * 4^L)", space_complexity: "O(L)", rationale: "Exploring 4-directional adjacent cells with in-place cell marking backtracks on dead ends.", description: "Grid DFS checking word character match step-by-step." }],
            }),
        Problem::NQueens => Some(ProblemDetails {
                id: 51, title: "N-Queens", difficulty: Difficulty::Hard, category: Category::Backtracking,
                statement: "The n-queens puzzle is the problem of placing n queens on an n x n chessboard such that no two queens attack each other.",
                examples: &[Example { input: "n = 4", output: "[\".Q..\",\"...Q\",\"Q...\",\"..Q.\"]", explanation: "Place N non-attacking queens on NxN board." }],
                constraints: &["1 <= n <= 9"], leetcode_url: "https://leetcode.com/problems/n-queens/",
                approaches: &[ApproachMeta { id: 0, name: "Row-by-Row Backtracking with Column & Diagonal Sets", time_complexity: "O(N!)", space_complexity: "O(N)", rationale: "Tracking occupied columns, positive diagonals (r + c), and negative diagonals (r - c) validates queen placements in O(1) per row.", description: "Row-by-row recursion with hashset conflict checks." }],
            }),
        Problem::PalindromePartitioning => Some(ProblemDetails {
                id: 131, title: "Palindrome Partitioning", difficulty: Difficulty::Medium, category: Category::Backtracking,
                statement: "Given a string s, partition s such that every substring of the partition is a palindrome. Return all possible palindrome partitioning of s.",
                examples: &[Example { input: "s = \"aab\"", output: "[[\"a\",\"a\",\"b\"],[\"aa\",\"b\"]]", explanation: "Explore all valid palindromic prefix cuts." }],
                constraints: &["1 <= s.length <= 16"], leetcode_url: "https://leetcode.com/problems/palindrome-partitioning/",
                approaches: &[ApproachMeta { id: 0, name: "Backtracking Palindrome Check", time_complexity: "O(N * 2^N)", space_complexity: "O(N)", rationale: "Recurse on valid palindromic prefix cuts to partition the string.", description: "Backtrack exploring palindromic prefix slices." }],
            }),
        Problem::LetterCombinations => Some(ProblemDetails {
                id: 17, title: "Letter Combinations of a Phone Number", difficulty: Difficulty::Medium, category: Category::Backtracking,
                statement: "Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent.",
                examples: &[Example { input: "digits = \"23\"", output: "[\"ad\",\"ae\",\"af\",\"bd\",\"be\",\"bf\",\"cd\",\"ce\",\"cf\"]", explanation: "Mapping digits 2='abc', 3='def'." }],
                constraints: &["0 <= digits.length <= 4"], leetcode_url: "https://leetcode.com/problems/letter-combinations-of-a-phone-number/",
                approaches: &[ApproachMeta { id: 0, name: "Backtracking Phone Mapping", time_complexity: "O(4^N)", space_complexity: "O(N)", rationale: "Branch on mapping characters for each digit in the input string.", description: "Recurse building string combinations from digit phone keymaps." }],
            }),
        _ => None,
    }
}

pub fn get_code_lines(problem: Problem, approach_id: usize) -> Option<Vec<(usize, &'static str)>> {
    match (problem, approach_id) {
        (Problem::Subsets, _) => Some(subsets_code_lines()),
        (Problem::Permutations, _) => Some(permutations_code_lines()),
        (Problem::CombinationSum, _) => Some(combination_sum_code_lines()),
        (Problem::SubsetsII, _) => Some(subsets_ii_code_lines()),
        (Problem::CombinationSumII, _) => Some(combination_sum_ii_code_lines()),
        (Problem::WordSearch, _) => Some(word_search_code_lines()),
        (Problem::NQueens, _) => Some(n_queens_code_lines()),
        (Problem::PalindromePartitioning, _) => Some(palindrome_partitioning_code_lines()),
        (Problem::LetterCombinations, _) => Some(letter_combinations_code_lines()),
        _ => None,
    }
}

// ── Helper Code Line Generators ──

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
