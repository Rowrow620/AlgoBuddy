use crate::model::problem::{ApproachMeta, Example, Problem, ProblemDetails};
use crate::model::taxonomy::{Category, Difficulty};

pub fn get_details(problem: Problem) -> Option<ProblemDetails> {
    match problem {
        Problem::SingleNumber => Some(ProblemDetails {
                id: 136, title: "Single Number", difficulty: Difficulty::Easy, category: Category::BitManipulation,
                statement: "Given a non-empty array of integers where every element appears twice except for one, find it.",
                examples: &[Example { input: "nums = [4, 1, 2, 1, 2]", output: "4", explanation: "4 is non-duplicate." }],
                constraints: &["1 <= nums.length <= 3*10^4"], leetcode_url: "https://leetcode.com/problems/single-number/",
                approaches: &[
                    ApproachMeta { id: 0, name: "Bitwise XOR", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Bitwise XOR properties (a ^ a = 0 and a ^ 0 = a) cancel out paired numbers, isolating the single number in O(N) time and O(1) space.", description: "a ^ a = 0 cancels duplicates." },
                    ApproachMeta { id: 1, name: "Brute Force Frequency Scan", time_complexity: "O(N^2)", space_complexity: "O(1)", rationale: "Counting every candidate with a fresh scan avoids auxiliary data structures but repeats up to N comparisons for each of N values.", description: "For each value, scan the full array and return the one counted once." },
                ],
            }),
        Problem::CountingBits => Some(ProblemDetails {
                id: 338, title: "Counting Bits", difficulty: Difficulty::Easy, category: Category::BitManipulation,
                statement: "Given n, return an array ans of length n + 1 where ans[i] is the number of 1's in binary representation of i.",
                examples: &[Example { input: "n = 5", output: "[0,1,1,2,1,2]", explanation: "Bits for 0..5." }],
                constraints: &["0 <= n <= 10^5"], leetcode_url: "https://leetcode.com/problems/counting-bits/",
                approaches: &[
                    ApproachMeta { id: 0, name: "Dynamic Programming (Bit Shift / Offset)", time_complexity: "O(N)", space_complexity: "O(N)", rationale: "Using DP transition bits[i] = bits[i >> 1] + (i & 1) computes bit counts for 0..N in linear O(N) time.", description: "dp[i] = 1 + dp[i - offset]." },
                    ApproachMeta { id: 1, name: "Independent 32-Bit Scan", time_complexity: "O(32N)", space_complexity: "O(N)", rationale: "Scanning all 32 positions independently for every value is straightforward but does not reuse any previously computed bit counts.", description: "Count set bits from scratch for each integer from 0 through n." },
                ],
            }),
        Problem::ReverseBits => Some(ProblemDetails {
                id: 190, title: "Reverse Bits", difficulty: Difficulty::Easy, category: Category::BitManipulation,
                statement: "Reverse bits of a given 32-bit unsigned integer.",
                examples: &[Example { input: "n = 43261596 (00000010100101000001111010011100)", output: "964176192", explanation: "Reversed bits." }],
                constraints: &["32-bit integer"], leetcode_url: "https://leetcode.com/problems/reverse-bits/",
                approaches: &[
                    ApproachMeta { id: 0, name: "Bitwise Shift & Or", time_complexity: "O(1)", space_complexity: "O(1)", rationale: "Looping 32 bits and shifting the target bit to position (31 - i) reverses bit order in deterministic O(1) time.", description: "Shift bit i to 31 - i." },
                    ApproachMeta { id: 1, name: "Binary String Reverse", time_complexity: "O(1)", space_complexity: "O(1)", rationale: "Formatting a fixed-width 32-bit string, reversing its characters, and parsing it back is a literal representation-based baseline.", description: "Format 32 bits as text, reverse the text, and parse it as binary." },
                ],
            }),
        Problem::MissingNumber => Some(ProblemDetails {
                id: 268, title: "Missing Number", difficulty: Difficulty::Easy, category: Category::BitManipulation,
                statement: "Given an array containing n distinct numbers in range [0, n], return the missing number.",
                examples: &[Example { input: "nums = [3, 0, 1]", output: "2", explanation: "Range [0..3], 2 is missing." }],
                constraints: &["1 <= n <= 10^4"], leetcode_url: "https://leetcode.com/problems/missing-number/",
                approaches: &[
                    ApproachMeta { id: 0, name: "Gauss Sum Formula", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Gauss sum formula N*(N+1)/2 gives expected total; subtracting actual array sum finds missing number in O(N) time and O(1) space.", description: "expected_sum - actual_sum." },
                    ApproachMeta { id: 1, name: "Brute Force Membership Scan", time_complexity: "O(N^2)", space_complexity: "O(1)", rationale: "Trying every candidate in 0..N and scanning the array for membership uses no auxiliary set but can repeat N comparisons N times.", description: "Return the first candidate not found by a full-array scan." },
                ],
            }),
        Problem::Number1Bits => Some(ProblemDetails {
                id: 191, title: "Number of 1 Bits", difficulty: Difficulty::Easy, category: Category::BitManipulation,
                statement: "Write a function that takes the binary representation of a positive integer and returns the number of set bits it has (also known as Hamming weight).",
                examples: &[Example { input: "n = 11 (binary 00000000000000000000000000001011)", output: "3", explanation: "Total 3 set bits." }],
                constraints: &["1 <= n <= 2^31 - 1"], leetcode_url: "https://leetcode.com/problems/number-of-1-bits/",
                approaches: &[
                    ApproachMeta { id: 0, name: "Bitwise AND n & (n - 1) Clearing", time_complexity: "O(1)", space_complexity: "O(1)", rationale: "Repeatedly clearing the lowest set bit using n &= (n - 1) counts 1-bits in constant O(1) time.", description: "Loop while n != 0 executing n &= (n - 1)." },
                    ApproachMeta { id: 1, name: "Fixed 32-Position Scan", time_complexity: "O(1)", space_complexity: "O(1)", rationale: "Checking every one of the 32 bit positions is the direct baseline and performs the same fixed amount of work regardless of how many bits are set.", description: "Shift through all 32 positions and add each low bit to the count." },
                ],
            }),
        Problem::SumTwoIntegers => Some(ProblemDetails {
                id: 371, title: "Sum of Two Integers", difficulty: Difficulty::Medium, category: Category::BitManipulation,
                statement: "Given two integers a and b, return the sum of the two integers without using the operators + and -.",
                examples: &[Example { input: "a = 1, b = 2", output: "3", explanation: "Bitwise XOR sum and AND carry bit shifts." }],
                constraints: &["-1000 <= a, b <= 1000"], leetcode_url: "https://leetcode.com/problems/sum-of-two-integers/",
                approaches: &[ApproachMeta { id: 0, name: "Bitwise XOR and Shifted Carry", time_complexity: "O(1)", space_complexity: "O(1)", rationale: "(a ^ b) computes sum without carry; (a & b) << 1 computes carry bits until carry is 0.", description: "Bitwise XOR and left-shift AND carry loop." }],
            }),
        Problem::ReverseInteger => Some(ProblemDetails {
                id: 7, title: "Reverse Integer", difficulty: Difficulty::Medium, category: Category::BitManipulation,
                statement: "Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes the value to go outside the signed 32-bit integer range [-2^31, 2^31 - 1], then return 0.",
                examples: &[Example { input: "x = 123", output: "321", explanation: "Reverse digits of 123 to get 321." }],
                constraints: &["-2^31 <= x <= 2^31 - 1"], leetcode_url: "https://leetcode.com/problems/reverse-integer/",
                approaches: &[ApproachMeta { id: 0, name: "Modulo & 32-Bit Overflow Boundary Check", time_complexity: "O(log10 X)", space_complexity: "O(1)", rationale: "Extracting digits via x % 10 and checking 32-bit INT_MAX boundaries before multiplying.", description: "Extract digits with modulo 10 and check overflow." }],
            }),
        _ => None,
    }
}

pub fn get_code_lines(problem: Problem, approach_id: usize) -> Option<Vec<(usize, &'static str)>> {
    match (problem, approach_id) {
        (Problem::SingleNumber, 0) => Some(vec![
            (1, "class Solution:"),
            (2, "    def singleNumber(self, nums: List[int]) -> int:"),
            (3, "        res = 0"),
            (4, "        for n in nums: res ^= n"),
            (5, "        return res"),
        ]),
        (Problem::SingleNumber, 1) => Some(vec![
            (1, "class Solution:"),
            (2, "    def singleNumber(self, nums: List[int]) -> int:"),
            (3, "        for value in nums:"),
            (4, "            count = 0"),
            (5, "            for candidate in nums:"),
            (6, "                if candidate == value: count += 1"),
            (7, "            if count == 1: return value"),
            (
                8,
                "        raise ValueError(\"input has no single number\")",
            ),
        ]),
        (Problem::CountingBits, 0) => Some(vec![
            (1, "class Solution:"),
            (2, "    def countBits(self, n: int) -> List[int]:"),
            (3, "        dp = [0] * (n + 1); offset = 1"),
            (4, "        for i in range(1, n + 1):"),
            (5, "            if offset * 2 == i: offset = i"),
            (6, "            dp[i] = 1 + dp[i - offset]"),
            (7, "        return dp"),
        ]),
        (Problem::CountingBits, 1) => Some(vec![
            (1, "class Solution:"),
            (2, "    def countBits(self, n: int) -> List[int]:"),
            (3, "        answer = []"),
            (4, "        for value in range(n + 1):"),
            (5, "            count = 0"),
            (6, "            for bit in range(32):"),
            (7, "                count += (value >> bit) & 1"),
            (8, "            answer.append(count)"),
            (9, "        return answer"),
        ]),
        (Problem::ReverseBits, 0) => Some(vec![
            (1, "class Solution:"),
            (2, "    def reverseBits(self, n: int) -> int:"),
            (3, "        res = 0"),
            (4, "        for i in range(32):"),
            (
                5,
                "            bit = (n >> i) & 1; res |= (bit << (31 - i))",
            ),
            (6, "        return res"),
        ]),
        (Problem::ReverseBits, 1) => Some(vec![
            (1, "class Solution:"),
            (2, "    def reverseBits(self, n: int) -> int:"),
            (3, "        bits = f\"{n:032b}\""),
            (4, "        reversed_bits = bits[::-1]"),
            (5, "        return int(reversed_bits, 2)"),
        ]),
        (Problem::MissingNumber, 0) => Some(vec![
            (1, "class Solution:"),
            (2, "    def missingNumber(self, nums: List[int]) -> int:"),
            (3, "        n = len(nums)"),
            (4, "        expected = n * (n + 1) // 2"),
            (5, "        actual = sum(nums)"),
            (6, "        missing = expected - actual"),
            (7, "        return missing"),
        ]),
        (Problem::MissingNumber, 1) => Some(vec![
            (1, "class Solution:"),
            (2, "    def missingNumber(self, nums: List[int]) -> int:"),
            (3, "        for candidate in range(len(nums) + 1):"),
            (4, "            found = False"),
            (5, "            for value in nums:"),
            (
                6,
                "                if value == candidate: found = True; break",
            ),
            (7, "            if not found: return candidate"),
        ]),
        (Problem::Number1Bits, 0) => Some(number_1_bits_code_lines()),
        (Problem::Number1Bits, 1) => Some(vec![
            (1, "class Solution:"),
            (2, "    def hammingWeight(self, n: int) -> int:"),
            (3, "        count = 0"),
            (4, "        for bit in range(32):"),
            (5, "            count += (n >> bit) & 1"),
            (6, "        return count"),
        ]),
        (Problem::SumTwoIntegers, _) => Some(sum_two_integers_code_lines()),
        (Problem::ReverseInteger, _) => Some(reverse_integer_code_lines()),
        _ => None,
    }
}

// ── Helper Code Line Generators ──

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
