use crate::model::problem::{ApproachMeta, Example, Problem, ProblemDetails};
use crate::model::taxonomy::{Category, Difficulty};

pub fn get_details(problem: Problem) -> Option<ProblemDetails> {
    match problem {
        Problem::ValidPalindrome => Some(ProblemDetails {
                id: 125, title: "Valid Palindrome", difficulty: Difficulty::Easy, category: Category::TwoPointers,
                statement: "Given a string s, return true if it is a palindrome.",
                examples: &[Example { input: "s = \"Was it a car or a cat I saw?\"", output: "true", explanation: "Alphanumeric filter palindrome." }],
                constraints: &["1 <= s.length <= 1000"], leetcode_url: "https://leetcode.com/problems/valid-palindrome/",
                approaches: &[ApproachMeta { id: 0, name: "Two Pointers In-Place", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Two pointers converging from both ends check symmetry in O(N) time without allocating extra string storage (O(1) space).", description: "Left and right pointers." }],
            }),
        Problem::TwoSumII => Some(ProblemDetails {
                id: 167, title: "Two Sum II - Input Array Is Sorted", difficulty: Difficulty::Medium, category: Category::TwoPointers,
                statement: "Given a 1-indexed array of integers that is already sorted in non-decreasing order, find two numbers such that they add up to a specific target number.",
                examples: &[Example { input: "numbers = [2, 7, 11, 15], target = 9", output: "[1, 2]", explanation: "numbers[1] + numbers[2] = 9." }],
                constraints: &["2 <= numbers.length <= 3*10^4", "numbers is sorted in non-decreasing order"], leetcode_url: "https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/",
                approaches: &[ApproachMeta { id: 0, name: "Two Pointers (Sorted)", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Because the array is sorted, moving left pointer right increases sum and right pointer left decreases sum in O(N) time and O(1) space.", description: "Left/right pointers converge on target sum." }],
            }),
        Problem::ThreeSum => Some(ProblemDetails {
                id: 15, title: "3Sum", difficulty: Difficulty::Medium, category: Category::TwoPointers,
                statement: "Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.",
                examples: &[Example { input: "nums = [-1, 0, 1, 2, -1, -4]", output: "[[-1, -1, 2], [-1, 0, 1]]", explanation: "Two unique triplets sum to 0." }],
                constraints: &["3 <= nums.length <= 3000"], leetcode_url: "https://leetcode.com/problems/3sum/",
                approaches: &[ApproachMeta { id: 0, name: "Sort + Two Pointers", time_complexity: "O(N^2)", space_complexity: "O(1)", rationale: "Sorting the array and using two pointers for each fixed anchor avoids duplicate triplets in O(N^2) time and O(1) auxiliary space.", description: "Fix anchor, two pointers for remaining pair." }],
            }),
        Problem::ContainerWater => Some(ProblemDetails {
                id: 11, title: "Container With Most Water", difficulty: Difficulty::Medium, category: Category::TwoPointers,
                statement: "Given n non-negative integers representing n vertical lines, find two lines that together with the x-axis form a container that holds the most water.",
                examples: &[Example { input: "height = [1, 8, 6, 2, 5, 4, 8, 3, 7]", output: "49", explanation: "Lines at index 1 and 8 form container of area 49." }],
                constraints: &["n == height.length", "2 <= n <= 10^5"], leetcode_url: "https://leetcode.com/problems/container-with-most-water/",
                approaches: &[ApproachMeta { id: 0, name: "Two Pointers Greedy", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Moving the pointer with shorter height inward is the only way to potentially find a larger area, achieving O(N) time.", description: "Move the shorter line inward to maximize area." }],
            }),
        Problem::TrappingRain => Some(ProblemDetails {
                id: 42, title: "Trapping Rain Water", difficulty: Difficulty::Hard, category: Category::TwoPointers,
                statement: "Given n non-negative integers representing an elevation map where the width of each bar is 1, compute how much water it can trap after raining.",
                examples: &[Example { input: "height = [0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]", output: "6", explanation: "6 units of rain water are trapped." }],
                constraints: &["n == height.length", "0 <= n <= 2*10^4"], leetcode_url: "https://leetcode.com/problems/trapping-rain-water/",
                approaches: &[ApproachMeta { id: 0, name: "Two Pointers (leftMax / rightMax)", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Maintaining leftMax and rightMax bounds computes trapped water per column in a single O(N) pass with O(1) space.", description: "Track max heights from both sides." }],
            }),
        _ => None,
    }
}

pub fn get_code_lines(problem: Problem, approach_id: usize) -> Option<Vec<(usize, &'static str)>> {
    match (problem, approach_id) {
        (Problem::ValidPalindrome, 0) => Some(vec![
            (1, "class Solution:"),
            (2, "    def isPalindrome(self, s: str) -> bool:"),
            (3, "        l, r = 0, len(s) - 1"),
            (4, "        while l < r:"),
            (5, "            while l < r and not s[l].isalnum(): l += 1"),
            (6, "            while r > l and not s[r].isalnum(): r -= 1"),
            (7, "            if s[l].lower() != s[r].lower(): return False"),
            (8, "            l, r = l + 1, r - 1"),
            (9, "        return True"),
        ]),
        (Problem::TwoSumII, _) => Some(vec![
            (1, "class Solution:"),
            (2, "    def twoSum(self, numbers: List[int], target: int) -> List[int]:"),
            (3, "        l, r = 0, len(numbers) - 1"),
            (4, "        while l < r:"),
            (5, "            curSum = numbers[l] + numbers[r]"),
            (6, "            if curSum == target: return [l + 1, r + 1]"),
            (7, "            elif curSum < target: l += 1"),
            (8, "            else: r -= 1"),
            (9, "        return []"),
        ]),
        (Problem::ThreeSum, _) => Some(vec![
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
        ]),
        (Problem::ContainerWater, _) => Some(vec![
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
        ]),
        (Problem::TrappingRain, _) => Some(vec![
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
        ]),
        _ => None,
    }
}
