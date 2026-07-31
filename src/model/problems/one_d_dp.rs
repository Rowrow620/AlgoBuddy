use crate::model::problem::{ApproachMeta, Example, Problem, ProblemDetails};
use crate::model::taxonomy::{Category, Difficulty};

pub fn get_details(problem: Problem) -> Option<ProblemDetails> {
    match problem {
        Problem::ClimbingStairs => Some(ProblemDetails {
                id: 70, title: "Climbing Stairs", difficulty: Difficulty::Easy, category: Category::OneDDp,
                statement: "It takes n steps to reach top. Each time you can climb 1 or 2 steps. How many distinct ways?",
                examples: &[Example { input: "n = 3", output: "3", explanation: "1+1+1, 1+2, 2+1." }],
                constraints: &["1 <= n <= 45"], leetcode_url: "https://leetcode.com/problems/climbing-stairs/",
                approaches: &[ApproachMeta { id: 0, name: "Dynamic Programming (Fibonacci)", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Ways to step n equal Fibonacci(n); maintaining 2 variables (dp[i-1], dp[i-2]) solves the problem in O(N) time and O(1) space.", description: "dp[i] = dp[i-1] + dp[i-2]." }],
            }),
        Problem::MinCostStairs => Some(ProblemDetails {
                id: 746, title: "Min Cost Climbing Stairs", difficulty: Difficulty::Easy, category: Category::OneDDp,
                statement: "Return minimum cost to reach top of floor by taking 1 or 2 steps.",
                examples: &[Example { input: "cost = [10, 15, 20]", output: "15", explanation: "Start at index 1, pay 15." }],
                constraints: &["2 <= cost.length <= 1000"], leetcode_url: "https://leetcode.com/problems/min-cost-climbing-stairs/",
                approaches: &[ApproachMeta { id: 0, name: "Bottom-Up DP", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Subproblem optimal transition dp[i] = cost[i] + min(dp[i-1], dp[i-2]) computes minimum cost in a single O(N) DP pass.", description: "dp[i] = min(dp[i-1]+cost[i-1], dp[i-2]+cost[i-2])." }],
            }),
        Problem::HouseRobber => Some(ProblemDetails {
                id: 198, title: "House Robber", difficulty: Difficulty::Medium, category: Category::OneDDp,
                statement: "Given an integer array nums representing the amount of money of each house, return the maximum amount of money you can rob tonight without robbing adjacent houses.",
                examples: &[Example { input: "nums = [1, 2, 3, 1]", output: "4", explanation: "Rob house 1 (money = 1) and rob house 3 (money = 3). Total = 1 + 3 = 4." }],
                constraints: &["1 <= nums.length <= 100"], leetcode_url: "https://leetcode.com/problems/house-robber/",
                approaches: &[ApproachMeta { id: 0, name: "Bottom-Up Dynamic Programming", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "DP state transition max(rob_prev_prev + num, rob_prev) computes max loot in O(N) time and O(1) space.", description: "dp[i] = max(dp[i-1], dp[i-2] + nums[i])." }],
            }),
        Problem::HouseRobberII => Some(ProblemDetails {
                id: 213, title: "House Robber II", difficulty: Difficulty::Medium, category: Category::OneDDp,
                statement: "All houses at this place are arranged in a circle. That means the first house is the neighbor of the last one. Return the maximum amount of money you can rob tonight without alerting the police.",
                examples: &[Example { input: "nums = [2,3,2]", output: "3", explanation: "You cannot rob house 1 (money = 2) and then rob house 3 (money = 2), because they are adjacent houses." }],
                constraints: &["1 <= nums.length <= 100", "0 <= nums[i] <= 1000"], leetcode_url: "https://leetcode.com/problems/house-robber-ii/",
                approaches: &[ApproachMeta { id: 0, name: "Two 1D DP Subproblems (First vs Last House)", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Running House Robber I on nums[1..N] and nums[0..N-1] handles the circular constraint in O(N) time.", description: "Max of robbing sub-arrays nums[1..] and nums[..N-1]." }],
            }),
        Problem::LongestPalindromicSubstring => Some(ProblemDetails {
                id: 5, title: "Longest Palindromic Substring", difficulty: Difficulty::Medium, category: Category::OneDDp,
                statement: "Given a string s, return the longest palindromic substring in s.",
                examples: &[Example { input: "s = \"babad\"", output: "\"bab\"", explanation: "\"aba\" is also a valid answer." }],
                constraints: &["1 <= s.length <= 1000"], leetcode_url: "https://leetcode.com/problems/longest-palindromic-substring/",
                approaches: &[ApproachMeta { id: 0, name: "Expand Around Center", time_complexity: "O(N^2)", space_complexity: "O(1)", rationale: "Expanding outward from each character center checks odd and even length palindromes in O(N^2) time and O(1) space.", description: "Expand outward from each index as center." }],
            }),
        Problem::PalindromicSubstrings => Some(ProblemDetails {
                id: 647, title: "Palindromic Substrings", difficulty: Difficulty::Medium, category: Category::OneDDp,
                statement: "Given a string s, return the number of palindromic substrings in it.",
                examples: &[Example { input: "s = \"aaa\"", output: "6", explanation: "Six palindromic substrings: \"a\", \"a\", \"a\", \"aa\", \"aa\", \"aaa\"." }],
                constraints: &["1 <= s.length <= 1000"], leetcode_url: "https://leetcode.com/problems/palindromic-substrings/",
                approaches: &[ApproachMeta { id: 0, name: "Expand Around Center Counting", time_complexity: "O(N^2)", space_complexity: "O(1)", rationale: "Count all valid expansions from each center in O(N^2) time.", description: "Increment counter for each valid center expansion." }],
            }),
        Problem::DecodeWays => Some(ProblemDetails {
                id: 91, title: "Decode Ways", difficulty: Difficulty::Medium, category: Category::OneDDp,
                statement: "A message containing letters from A-Z can be encoded into numbers using 'A' -> '1' to 'Z' -> '26'. Given a string s containing only digits, return the number of ways to decode it.",
                examples: &[Example { input: "s = \"226\"", output: "3", explanation: "\"226\" could be decoded as \"BZ\" (2 26), \"VF\" (22 6), or \"BBF\" (2 2 6)." }],
                constraints: &["1 <= s.length <= 100"], leetcode_url: "https://leetcode.com/problems/decode-ways/",
                approaches: &[ApproachMeta { id: 0, name: "1D DP (Single vs Double Digit)", time_complexity: "O(N)", space_complexity: "O(N)", rationale: "dp[i] = dp[i+1] (if s[i] != '0') + dp[i+2] (if s[i..i+2] <= 26) calculates total decoding combinations in O(N) time.", description: "Build DP array from right to left checking single and double digit valid codes." }],
            }),
        Problem::CoinChange => Some(ProblemDetails {
                id: 322, title: "Coin Change", difficulty: Difficulty::Medium, category: Category::OneDDp,
                statement: "You are given an integer array coins representing coins of different denominations and an integer amount representing a total amount of money. Return the fewest number of coins that you need to make up that amount.",
                examples: &[Example { input: "coins = [1,2,5], amount = 11", output: "3", explanation: "11 = 5 + 5 + 1" }],
                constraints: &["1 <= coins.length <= 12", "0 <= amount <= 10^4"], leetcode_url: "https://leetcode.com/problems/coin-change/",
                approaches: &[ApproachMeta { id: 0, name: "Bottom-Up 1D DP Table", time_complexity: "O(N * amount)", space_complexity: "O(amount)", rationale: "dp[a] = min(dp[a], 1 + dp[a - c]) builds minimum coins needed for all values 1..amount.", description: "Fill dp array of size amount + 1 with min coin transitions." }],
            }),
        Problem::MaxProductSubarray => Some(ProblemDetails {
                id: 152, title: "Maximum Product Subarray", difficulty: Difficulty::Medium, category: Category::OneDDp,
                statement: "Given an integer array nums, find a subarray that has the largest product, and return the product.",
                examples: &[Example { input: "nums = [2,3,-2,4]", output: "6", explanation: "[2,3] has the largest product 6." }],
                constraints: &["1 <= nums.length <= 2 * 10^4", "-10 <= nums[i] <= 10"], leetcode_url: "https://leetcode.com/problems/maximum-product-subarray/",
                approaches: &[ApproachMeta { id: 0, name: "Min/Max Dynamic State Tracking", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Tracking current min and max products handles negative number sign flips in O(N) time.", description: "Track curMin and curMax while scanning nums." }],
            }),
        Problem::WordBreak => Some(ProblemDetails {
                id: 139, title: "Word Break", difficulty: Difficulty::Medium, category: Category::OneDDp,
                statement: "Given a string s and a dictionary of strings wordDict, return true if s can be segmented into a space-separated sequence of one or more dictionary words.",
                examples: &[Example { input: "s = \"leetcode\", wordDict = [\"leet\",\"code\"]", output: "true", explanation: "Return true because \"leetcode\" can be segmented as \"leet code\"." }],
                constraints: &["1 <= s.length <= 300", "1 <= wordDict.length <= 1000"], leetcode_url: "https://leetcode.com/problems/word-break/",
                approaches: &[ApproachMeta { id: 0, name: "1D DP Suffix Matching", time_complexity: "O(N * M * K)", space_complexity: "O(N)", rationale: "dp[i] = true if s[i..i+w.len()] == w and dp[i+w.len()] == true verifies valid segmentation.", description: "Fill boolean dp array from right to left for dictionary words." }],
            }),
        Problem::LongestIncreasingSubsequence => Some(ProblemDetails {
                id: 300, title: "Longest Increasing Subsequence", difficulty: Difficulty::Medium, category: Category::OneDDp,
                statement: "Given an integer array nums, return the length of the longest strictly increasing subsequence.",
                examples: &[Example { input: "nums = [10,9,2,5,3,7,101,18]", output: "4", explanation: "The longest increasing subsequence is [2,3,7,101], length 4." }],
                constraints: &["1 <= nums.length <= 2500", "-10^4 <= nums[i] <= 10^4"], leetcode_url: "https://leetcode.com/problems/longest-increasing-subsequence/",
                approaches: &[ApproachMeta { id: 0, name: "1D DP / Patient Sorting Binary Search", time_complexity: "O(N^2) or O(N log N)", space_complexity: "O(N)", rationale: "dp[i] = max(1, 1 + dp[j]) for j < i where nums[j] < nums[i] computes LIS in O(N^2) or O(N log N) time.", description: "Fill dp array storing longest subsequence ending at index i." }],
            }),
        Problem::PartitionEqualSubsetSum => Some(ProblemDetails {
                id: 416, title: "Partition Equal Subset Sum", difficulty: Difficulty::Medium, category: Category::OneDDp,
                statement: "Given an integer array nums, return true if you can partition the array into two subsets such that the sum of the elements in both subsets is equal.",
                examples: &[Example { input: "nums = [1,5,11,5]", output: "true", explanation: "The array can be partitioned as [1, 5, 5] and [11]." }],
                constraints: &["1 <= nums.length <= 200", "1 <= nums[i] <= 100"], leetcode_url: "https://leetcode.com/problems/partition-equal-subset-sum/",
                approaches: &[ApproachMeta { id: 0, name: "0/1 Knapsack DP Set", time_complexity: "O(N * sum)", space_complexity: "O(sum)", rationale: "Target sum is sum(nums) / 2; DP set stores reachable subset sums up to target.", description: "Iterate nums building reachable subset sums set." }],
            }),
        _ => None,
    }
}

pub fn get_code_lines(problem: Problem, approach_id: usize) -> Option<Vec<(usize, &'static str)>> {
    match (problem, approach_id) {
        (Problem::ClimbingStairs, _) => Some(vec![
            (1, "class Solution:"),
            (2, "    def climbStairs(self, n: int) -> int:"),
            (3, "        one, two = 1, 1"),
            (4, "        for i in range(n - 1):"),
            (5, "            temp = one; one = one + two; two = temp"),
            (6, "        return one"),
        ]),
        (Problem::MinCostStairs, _) => Some(vec![
            (1, "class Solution:"),
            (
                2,
                "    def minCostClimbingStairs(self, cost: List[int]) -> int:",
            ),
            (3, "        cost.append(0)"),
            (4, "        for i in range(len(cost) - 3, -1, -1):"),
            (5, "            cost[i] += min(cost[i + 1], cost[i + 2])"),
            (6, "        return min(cost[0], cost[1])"),
        ]),
        (Problem::HouseRobber, _) => Some(house_robber_code_lines()),
        (Problem::HouseRobberII, _) => Some(house_robber_ii_code_lines()),
        (Problem::LongestPalindromicSubstring, _) => {
            Some(longest_palindromic_substring_code_lines())
        }
        (Problem::PalindromicSubstrings, _) => Some(palindromic_substrings_code_lines()),
        (Problem::DecodeWays, _) => Some(decode_ways_code_lines()),
        (Problem::CoinChange, _) => Some(coin_change_code_lines()),
        (Problem::MaxProductSubarray, _) => Some(max_product_subarray_code_lines()),
        (Problem::WordBreak, _) => Some(word_break_code_lines()),
        (Problem::LongestIncreasingSubsequence, _) => {
            Some(longest_increasing_subsequence_code_lines())
        }
        (Problem::PartitionEqualSubsetSum, _) => Some(partition_equal_subset_sum_code_lines()),
        _ => None,
    }
}

// ── Helper Code Line Generators ──

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
