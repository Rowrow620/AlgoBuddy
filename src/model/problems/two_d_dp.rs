use crate::model::problem::{ApproachMeta, Example, Problem, ProblemDetails};
use crate::model::taxonomy::{Category, Difficulty};

pub fn get_details(problem: Problem) -> Option<ProblemDetails> {
    match problem {
        Problem::UniquePaths => Some(ProblemDetails {
                id: 62, title: "Unique Paths", difficulty: Difficulty::Medium, category: Category::TwoDDp,
                statement: "There is a robot on an m x n grid. The robot is initially located at the top-left corner (grid[0][0]) and tries to move to the bottom-right corner (grid[m-1][n-1]). Return the number of possible unique paths.",
                examples: &[Example { input: "m = 3, n = 7", output: "28", explanation: "Total 28 unique grid paths from top-left to bottom-right." }],
                constraints: &["1 <= m, n <= 100"], leetcode_url: "https://leetcode.com/problems/unique-paths/",
                approaches: &[ApproachMeta { id: 0, name: "2D Grid Dynamic Programming", time_complexity: "O(M * N)", space_complexity: "O(M * N)", rationale: "dp[r][c] = dp[r+1][c] + dp[r][c+1] accumulates path counts bottom-up in O(M*N) time.", description: "2D DP grid accumulation." }],
            }),
        Problem::LongestCommonSubsequence => Some(ProblemDetails {
                id: 1143, title: "Longest Common Subsequence", difficulty: Difficulty::Medium, category: Category::TwoDDp,
                statement: "Given two strings text1 and text2, return the length of their longest common subsequence.",
                examples: &[Example { input: "text1 = \"abcde\", text2 = \"ace\"", output: "3", explanation: "The longest common subsequence is \"ace\" of length 3." }],
                constraints: &["1 <= text1.length, text2.length <= 1000"], leetcode_url: "https://leetcode.com/problems/longest-common-subsequence/",
                approaches: &[ApproachMeta { id: 0, name: "2D Matrix DP Matching", time_complexity: "O(M * N)", space_complexity: "O(M * N)", rationale: "Matching chars adds 1 + dp[i+1][j+1], while mismatches take max(dp[i+1][j], dp[i][j+1]).", description: "2D DP char comparison table." }],
            }),
        Problem::BestTimeStockCooldown => Some(ProblemDetails {
                id: 309, title: "Best Time to Buy and Sell Stock with Cooldown", difficulty: Difficulty::Medium, category: Category::TwoDDp,
                statement: "Find the maximum profit you can achieve with stock transactions given that after you sell stock, you cannot buy stock on the next day (cooldown 1 day).",
                examples: &[Example { input: "prices = [1,2,3,0,2]", output: "3", explanation: "Transactions: [buy, sell, cooldown, buy, sell]." }],
                constraints: &["1 <= prices.length <= 5000"], leetcode_url: "https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/",
                approaches: &[ApproachMeta { id: 0, name: "State Machine DP / Memoization", time_complexity: "O(N)", space_complexity: "O(N)", rationale: "Caching (day, buying_state) prevents redundant recursive branch evaluations in linear O(N) time.", description: "Buying vs selling state memoization." }],
            }),
        Problem::CoinChangeII => Some(ProblemDetails {
                id: 518, title: "Coin Change II", difficulty: Difficulty::Medium, category: Category::TwoDDp,
                statement: "Return the number of combinations that make up an amount using given coins of different denominations.",
                examples: &[Example { input: "amount = 5, coins = [1,2,5]", output: "4", explanation: "5=5, 5=2+2+1, 5=2+1+1+1, 5=1+1+1+1+1." }],
                constraints: &["1 <= amount <= 5000", "1 <= coins.length <= 300"], leetcode_url: "https://leetcode.com/problems/coin-change-ii/",
                approaches: &[ApproachMeta { id: 0, name: "Unbounded Knapsack 2D DP Table", time_complexity: "O(N * Amount)", space_complexity: "O(Amount)", rationale: "Processing coins iteratively avoids duplicate permutations, yielding total distinct combinations.", description: "Bottom-up combinations table." }],
            }),
        Problem::TargetSum => Some(ProblemDetails {
                id: 494, title: "Target Sum", difficulty: Difficulty::Medium, category: Category::TwoDDp,
                statement: "Build an expression out of nums by adding '+' or '-' before each integer so that the expression evaluates to target. Return the number of different expressions.",
                examples: &[Example { input: "nums = [1,1,1,1,1], target = 3", output: "5", explanation: "5 ways to assign signs to sum to 3." }],
                constraints: &["1 <= nums.length <= 20"], leetcode_url: "https://leetcode.com/problems/target-sum/",
                approaches: &[ApproachMeta { id: 0, name: "Subset Sum 2D DP Memoization", time_complexity: "O(N * TotalSum)", space_complexity: "O(N * TotalSum)", rationale: "Caching (index, current_sum) avoids 2^N branch recalculations.", description: "Subproblem sum memoization." }],
            }),
        Problem::InterleavingString => Some(ProblemDetails {
                id: 97, title: "Interleaving String", difficulty: Difficulty::Medium, category: Category::TwoDDp,
                statement: "Given strings s1, s2, and s3, find whether s3 is formed by an interleaving of s1 and s2.",
                examples: &[Example { input: "s1 = \"aabcc\", s2 = \"dbbca\", s3 = \"aadbbcbcac\"", output: "true", explanation: "s3 contains interleaved chars from s1 and s2." }],
                constraints: &["0 <= s1.length, s2.length <= 100"], leetcode_url: "https://leetcode.com/problems/interleaving-string/",
                approaches: &[ApproachMeta { id: 0, name: "2D Grid DP Reachability Matrix", time_complexity: "O(M * N)", space_complexity: "O(M * N)", rationale: "dp[i][j] is true if s1[i] == s3[i+j] and dp[i+1][j] is true, or s2[j] == s3[i+j] and dp[i][j+1] is true.", description: "2D interleaving boolean grid." }],
            }),
        Problem::LongestIncreasingPath => Some(ProblemDetails {
                id: 329, title: "Longest Increasing Path in a Matrix", difficulty: Difficulty::Hard, category: Category::TwoDDp,
                statement: "Given an m x n integers matrix, return the length of the longest increasing path in matrix.",
                examples: &[Example { input: "matrix = [[9,9,4],[6,6,8],[2,1,1]]", output: "4", explanation: "Longest increasing path is [1, 2, 6, 9]." }],
                constraints: &["1 <= m, n <= 200"], leetcode_url: "https://leetcode.com/problems/longest-increasing-path-in-a-matrix/",
                approaches: &[ApproachMeta { id: 0, name: "DFS + Memoization Matrix DP", time_complexity: "O(M * N)", space_complexity: "O(M * N)", rationale: "Caching longest path lengths at cell (r, c) ensures each cell's maximum increasing streak is computed once.", description: "Memoized 2D grid DFS." }],
            }),
        Problem::DistinctSubsequences => Some(ProblemDetails {
                id: 115, title: "Distinct Subsequences", difficulty: Difficulty::Hard, category: Category::TwoDDp,
                statement: "Given two strings s and t, return the number of distinct subsequences of s which equals t.",
                examples: &[Example { input: "s = \"rabbbit\", t = \"rabbit\"", output: "3", explanation: "3 ways to select characters in s to form t." }],
                constraints: &["1 <= s.length, t.length <= 1000"], leetcode_url: "https://leetcode.com/problems/distinct-subsequences/",
                approaches: &[ApproachMeta { id: 0, name: "2D Matching Count DP Table", time_complexity: "O(M * N)", space_complexity: "O(M * N)", rationale: "If s[i] == t[j], dp[i][j] = dp[i+1][j+1] + dp[i+1][j], else dp[i+1][j].", description: "Bottom-up string matching table." }],
            }),
        Problem::EditDistance => Some(ProblemDetails {
                id: 72, title: "Edit Distance", difficulty: Difficulty::Medium, category: Category::TwoDDp,
                statement: "Given two strings word1 and word2, return the minimum number of operations (insert, delete, replace) required to convert word1 to word2.",
                examples: &[Example { input: "word1 = \"horse\", word2 = \"ros\"", output: "3", explanation: "horse -> rorse (replace) -> rose (remove r) -> ros (remove e)." }],
                constraints: &["0 <= word1.length, word2.length <= 500"], leetcode_url: "https://leetcode.com/problems/edit-distance/",
                approaches: &[ApproachMeta { id: 0, name: "Levenshtein 2D Distance Matrix", time_complexity: "O(M * N)", space_complexity: "O(M * N)", rationale: "dp[i][j] = 1 + min(insert, delete, replace) computes minimum edit steps systematically.", description: "2D edit operations matrix." }],
            }),
        Problem::BurstBalloons => Some(ProblemDetails {
                id: 312, title: "Burst Balloons", difficulty: Difficulty::Hard, category: Category::TwoDDp,
                statement: "You are given n balloons, indexed from 0 to n - 1. Each balloon is painted with a number on it represented by array nums. Return the maximum coins you can collect by bursting the balloons wisely.",
                examples: &[Example { input: "nums = [3,1,5,8]", output: "167", explanation: "Optimal order: burst 1, then 5, then 3, then 8." }],
                constraints: &["1 <= n <= 300"], leetcode_url: "https://leetcode.com/problems/burst-balloons/",
                approaches: &[ApproachMeta { id: 0, name: "Interval DP / Range Subproblems", time_complexity: "O(N^3)", space_complexity: "O(N^2)", rationale: "Choosing the LAST balloon to burst in subarray [l, r] decouples subproblems cleanly.", description: "Interval subproblem expansion table." }],
            }),
        Problem::RegularExpressionMatching => Some(ProblemDetails {
                id: 10, title: "Regular Expression Matching", difficulty: Difficulty::Hard, category: Category::TwoDDp,
                statement: "Given an input string s and a pattern p, implement regular expression matching with support for '.' and '*'.",
                examples: &[Example { input: "s = \"aa\", p = \"a*\"", output: "true", explanation: "'*' means zero or more of the preceding element 'a'." }],
                constraints: &["1 <= s.length <= 20", "1 <= p.length <= 20"], leetcode_url: "https://leetcode.com/problems/regular-expression-matching/",
                approaches: &[ApproachMeta { id: 0, name: "2D Regex Matching Matrix DP", time_complexity: "O(M * N)", space_complexity: "O(M * N)", rationale: "Handling '.' wildcard matching and '*' repetition zero-or-more branches in a 2D boolean grid.", description: "2D state transition table." }],
            }),
        _ => None,
    }
}

pub fn get_code_lines(problem: Problem, approach_id: usize) -> Option<Vec<(usize, &'static str)>> {
    match (problem, approach_id) {
        (Problem::UniquePaths, _) => Some(unique_paths_code_lines()),
        (Problem::LongestCommonSubsequence, _) => Some(lcs_code_lines()),
        (Problem::BestTimeStockCooldown, _) => Some(stock_cooldown_code_lines()),
        (Problem::CoinChangeII, _) => Some(coin_change_ii_code_lines()),
        (Problem::TargetSum, _) => Some(target_sum_code_lines()),
        (Problem::InterleavingString, _) => Some(interleaving_string_code_lines()),
        (Problem::LongestIncreasingPath, _) => Some(lip_code_lines()),
        (Problem::DistinctSubsequences, _) => Some(distinct_subsequences_code_lines()),
        (Problem::EditDistance, _) => Some(edit_distance_code_lines()),
        (Problem::BurstBalloons, _) => Some(burst_balloons_code_lines()),
        (Problem::RegularExpressionMatching, _) => Some(regex_matching_code_lines()),
        _ => None,
    }
}

// ── Helper Code Line Generators ──

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
