use crate::model::problem::{ApproachMeta, Example, Problem, ProblemDetails};
use crate::model::taxonomy::{Category, Difficulty};

pub fn get_details(problem: Problem) -> Option<ProblemDetails> {
    match problem {
        Problem::MaximumSubarray => Some(ProblemDetails {
                id: 53, title: "Maximum Subarray", difficulty: Difficulty::Medium, category: Category::Greedy,
                statement: "Given an integer array nums, find the subarray with the largest sum, and return its sum.",
                examples: &[Example { input: "nums = [-2,1,-3,4,-1,2,1,-5,4]", output: "6", explanation: "The subarray [4,-1,2,1] has the largest sum 6." }],
                constraints: &["1 <= nums.length <= 10^5"], leetcode_url: "https://leetcode.com/problems/maximum-subarray/",
                approaches: &[ApproachMeta { id: 0, name: "Kadane's Algorithm", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Tracking curSum and resetting to 0 when negative computes maximum subarray sum in linear time.", description: "Track curSum resetting at negative values." }],
            }),
        Problem::JumpGame => Some(ProblemDetails {
                id: 55, title: "Jump Game", difficulty: Difficulty::Medium, category: Category::Greedy,
                statement: "You are given an integer array nums. You are initially positioned at the array's first index, and each element in the array represents your maximum jump length at that position. Return true if you can reach the last index.",
                examples: &[Example { input: "nums = [2,3,1,1,4]", output: "true", explanation: "Jump 1 step from index 0 to 1, then 3 steps to the last index." }],
                constraints: &["1 <= nums.length <= 10^4"], leetcode_url: "https://leetcode.com/problems/jump-game/",
                approaches: &[ApproachMeta { id: 0, name: "Greedy Backwards Goal Shift", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Moving target goal backward from last index to 0 verifies reachability in O(N) time.", description: "Shift target goal backward if index + nums[i] >= goal." }],
            }),
        Problem::JumpGameII => Some(ProblemDetails {
                id: 45, title: "Jump Game II", difficulty: Difficulty::Medium, category: Category::Greedy,
                statement: "Return the minimum number of jumps to reach nums[n - 1].",
                examples: &[Example { input: "nums = [2,3,1,1,4]", output: "2", explanation: "The minimum number of jumps to reach the last index is 2." }],
                constraints: &["1 <= nums.length <= 10^4"], leetcode_url: "https://leetcode.com/problems/jump-game-ii/",
                approaches: &[ApproachMeta { id: 0, name: "BFS Level Window Greed", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Tracking current jump window [l, r] and farthest reachable index computes min jumps in O(N) time.", description: "Track current level window and farthest reach." }],
            }),
        Problem::GasStation => Some(ProblemDetails {
                id: 134, title: "Gas Station", difficulty: Difficulty::Medium, category: Category::Greedy,
                statement: "There are n gas stations along a circular route. Return the starting gas station's index if you can travel around the circuit once in the clockwise direction, otherwise return -1.",
                examples: &[Example { input: "gas = [1,2,3,4,5], cost = [3,4,5,1,2]", output: "3", explanation: "Start at station 3 (index 3) and fill up with 4 unit of gas." }],
                constraints: &["n == gas.length == cost.length"], leetcode_url: "https://leetcode.com/problems/gas-station/",
                approaches: &[ApproachMeta { id: 0, name: "Total Balance & Reset Start Index", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "If sum(gas) >= sum(cost), a solution is guaranteed; resetting start index when total tank drops below 0 finds it in O(N).", description: "Reset start index when accumulated tank < 0." }],
            }),
        Problem::HandOfStraights => Some(ProblemDetails {
                id: 846, title: "Hand of Straights", difficulty: Difficulty::Medium, category: Category::Greedy,
                statement: "Alice has a card hand given as an integer array. Rearrange the cards into groups so that each group is of size groupSize, and consists of groupSize consecutive cards.",
                examples: &[Example { input: "hand = [1,2,3,6,2,3,4,7,8], groupSize = 3", output: "true", explanation: "Hand can be rearranged as [1,2,3],[2,3,4],[6,7,8]." }],
                constraints: &["1 <= hand.length <= 10^4"], leetcode_url: "https://leetcode.com/problems/hand-of-straights/",
                approaches: &[ApproachMeta { id: 0, name: "Frequency Min-Heap / Sorted Map Greed", time_complexity: "O(N log N)", space_complexity: "O(N)", rationale: "Starting from the smallest available card and forming consecutive groups of groupSize.", description: "Greedily form groups from smallest card available." }],
            }),
        Problem::MergeTriplets => Some(ProblemDetails {
                id: 1899, title: "Merge Triplets to Form Target Triplet", difficulty: Difficulty::Medium, category: Category::Greedy,
                statement: "Given a 2D integer array triplets and an integer array target, return true if it is possible to obtain target by merging triplets.",
                examples: &[Example { input: "triplets = [[2,5,3],[2,3,4],[1,2,5],[5,2,3]], target = [5,5,5]", output: "true", explanation: "Merge triplets to form [5,5,5]." }],
                constraints: &["1 <= triplets.length <= 10^5"], leetcode_url: "https://leetcode.com/problems/merge-triplets-to-form-target-triplet/",
                approaches: &[ApproachMeta { id: 0, name: "Filter Oversized Triplets & Match Target Values", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Ignoring triplets with any value > target[i] and checking if remaining triplets cover all target values.", description: "Filter out invalid triplets and check target match coverage." }],
            }),
        Problem::PartitionLabels => Some(ProblemDetails {
                id: 763, title: "Partition Labels", difficulty: Difficulty::Medium, category: Category::Greedy,
                statement: "You are given a string s. We want to partition the string into as many parts as possible so that each letter appears in at most one part.",
                examples: &[Example { input: "s = \"ababcbacadefegdehijhklij\"", output: "[9,7,8]", explanation: "Partitions are \"ababcbaca\", \"defegde\", \"hijhklij\"." }],
                constraints: &["1 <= s.length <= 500"], leetcode_url: "https://leetcode.com/problems/partition-labels/",
                approaches: &[ApproachMeta { id: 0, name: "Last Index Hash Map & Window End Tracking", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Tracking the last occurrences of each character and expanding partition boundary until end == current index.", description: "Expand partition end boundary to max last-occurrence index." }],
            }),
        Problem::ValidParenthesisString => Some(ProblemDetails {
                id: 678, title: "Valid Parenthesis String", difficulty: Difficulty::Medium, category: Category::Greedy,
                statement: "Given a string s containing only '(', ')' and '*', return true if s is valid.",
                examples: &[Example { input: "s = \"(*)\"", output: "true", explanation: "'*' can act as closing parenthesis." }],
                constraints: &["1 <= s.length <= 100"], leetcode_url: "https://leetcode.com/problems/valid-parenthesis-string/",
                approaches: &[ApproachMeta { id: 0, name: "Min/Max Open Count Range Greed", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Tracking minOpen and maxOpen count range handles flexible '*' wildcard matches in O(N) time.", description: "Maintain [minOpen, maxOpen] count range." }],
            }),
        _ => None,
    }
}

pub fn get_code_lines(problem: Problem, approach_id: usize) -> Option<Vec<(usize, &'static str)>> {
    match (problem, approach_id) {
        (Problem::MaximumSubarray, _) => Some(maximum_subarray_code_lines()),
        (Problem::JumpGame, _) => Some(jump_game_code_lines()),
        (Problem::JumpGameII, _) => Some(jump_game_ii_code_lines()),
        (Problem::GasStation, _) => Some(gas_station_code_lines()),
        (Problem::HandOfStraights, _) => Some(hand_of_straights_code_lines()),
        (Problem::MergeTriplets, _) => Some(merge_triplets_code_lines()),
        (Problem::PartitionLabels, _) => Some(partition_labels_code_lines()),
        (Problem::ValidParenthesisString, _) => Some(valid_parenthesis_string_code_lines()),
        _ => None,
    }
}

// ── Helper Code Line Generators ──

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
