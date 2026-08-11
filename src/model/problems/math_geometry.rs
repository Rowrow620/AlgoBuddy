use crate::model::problem::{ApproachMeta, Example, Problem, ProblemDetails};
use crate::model::taxonomy::{Category, Difficulty};

pub fn get_details(problem: Problem) -> Option<ProblemDetails> {
    match problem {
        Problem::HappyNumber => Some(ProblemDetails {
                id: 202, title: "Happy Number", difficulty: Difficulty::Easy, category: Category::MathAndGeometry,
                statement: "Determine if a number n is happy (sum of square of digits reaches 1).",
                examples: &[Example { input: "n = 19", output: "true", explanation: "1^2+9^2=82 -> 68 -> 100 -> 1." }],
                constraints: &["1 <= n <= 2^31 - 1"], leetcode_url: "https://leetcode.com/problems/happy-number/",
                approaches: &[ApproachMeta { id: 0, name: "HashSet Cycle Detection", time_complexity: "O(log N)", space_complexity: "O(log N)", rationale: "A HashSet tracks previously seen digit sum results to detect infinite cycles in logarithmic time.", description: "Track seen square sums." }],
            }),
        Problem::PlusOne => Some(ProblemDetails {
                id: 66, title: "Plus One", difficulty: Difficulty::Easy, category: Category::MathAndGeometry,
                statement: "Increment the large integer represented as a digit array by one.",
                examples: &[Example { input: "digits = [1, 2, 3]", output: "[1, 2, 4]", explanation: "123 + 1 = 124." }],
                constraints: &["1 <= digits.length <= 100"], leetcode_url: "https://leetcode.com/problems/plus-one/",
                approaches: &[ApproachMeta { id: 0, name: "Right-to-Left Carry Pass", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Iterating backwards handles digit carry in O(N) time, adding a new leading 1 only if all digits were 9.", description: "Add 1 from right, carry overflow." }],
            }),
        Problem::RotateImage => Some(ProblemDetails {
                id: 48, title: "Rotate Image", difficulty: Difficulty::Medium, category: Category::MathAndGeometry,
                statement: "You are given an n x n 2D matrix representing an image, rotate the image by 90 degrees (clockwise) in-place.",
                examples: &[Example { input: "matrix = [[1,2,3],[4,5,6],[7,8,9]]", output: "[[7,4,1],[8,5,2],[9,6,3]]", explanation: "Rotate 90 degrees clockwise." }],
                constraints: &["n == matrix.length == matrix[i].length", "1 <= n <= 20"], leetcode_url: "https://leetcode.com/problems/rotate-image/",
                approaches: &[ApproachMeta { id: 0, name: "Matrix Transpose + Reverse Rows", time_complexity: "O(N^2)", space_complexity: "O(1)", rationale: "Transposing the matrix in-place and then reversing each row rotates the image 90 degrees clockwise in O(N^2) time.", description: "Transpose matrix across diagonal and reverse each row." }],
            }),
        Problem::SpiralMatrix => Some(ProblemDetails {
                id: 54, title: "Spiral Matrix", difficulty: Difficulty::Medium, category: Category::MathAndGeometry,
                statement: "Given an m x n matrix, return all elements of the matrix in spiral order.",
                examples: &[Example { input: "matrix = [[1,2,3],[4,5,6],[7,8,9]]", output: "[1,2,3,6,9,8,7,4,5]", explanation: "Traverse clockwise inward spiral." }],
                constraints: &["m == matrix.length", "n == matrix[i].length", "1 <= m, n <= 10"], leetcode_url: "https://leetcode.com/problems/spiral-matrix/",
                approaches: &[ApproachMeta { id: 0, name: "4-Boundary Shrinking Traversal", time_complexity: "O(M * N)", space_complexity: "O(1)", rationale: "Maintaining top, bottom, left, and right boundaries and traversing edges inward collects elements in spiral order.", description: "Shrink boundaries top/bottom/left/right while traversing outer edges." }],
            }),
        Problem::SetMatrixZeroes => Some(ProblemDetails {
                id: 73, title: "Set Matrix Zeroes", difficulty: Difficulty::Medium, category: Category::MathAndGeometry,
                statement: "Given an m x n integer matrix matrix, if an element is 0, set its entire row and column to 0's. You must do it in-place.",
                examples: &[Example { input: "matrix = [[1,1,1],[1,0,1],[1,1,1]]", output: "[[1,0,1],[0,0,0],[1,0,1]]", explanation: "Set row 1 and column 1 to all zeroes." }],
                constraints: &["m == matrix.length", "n == matrix[0].length", "1 <= m, n <= 200"], leetcode_url: "https://leetcode.com/problems/set-matrix-zeroes/",
                approaches: &[ApproachMeta { id: 0, name: "First Row/Column State Flags", time_complexity: "O(M * N)", space_complexity: "O(1)", rationale: "Using the matrix's first row and first column to store zero flags achieves in-place O(1) extra space complexity.", description: "Mark first row and col as zero flags." }],
            }),
        Problem::PowXN => Some(ProblemDetails {
                id: 50, title: "Pow(x, n)", difficulty: Difficulty::Medium, category: Category::MathAndGeometry,
                statement: "Implement pow(x, n), which calculates x raised to the power n (i.e., x^n).",
                examples: &[Example { input: "x = 2.00000, n = 10", output: "1024.00000", explanation: "2^10 = 1024." }],
                constraints: &["-100.0 < x < 100.0", "-2^31 <= n <= 2^31 - 1"], leetcode_url: "https://leetcode.com/problems/powx-n/",
                approaches: &[ApproachMeta { id: 0, name: "Binary Exponentiation (Divide & Conquer)", time_complexity: "O(log N)", space_complexity: "O(log N)", rationale: "Dividing n by 2 recursively computes x^n in logarithmic O(log N) time.", description: "Divide and conquer squaring x when n is even." }],
            }),
        Problem::MultiplyStrings => Some(ProblemDetails {
                id: 43, title: "Multiply Strings", difficulty: Difficulty::Medium, category: Category::MathAndGeometry,
                statement: "Given two non-negative integers num1 and num2 represented as strings, return the product of num1 and num2, also represented as a string.",
                examples: &[Example { input: "num1 = \"2\", num2 = \"3\"", output: "\"6\"", explanation: "2 * 3 = 6." }],
                constraints: &["1 <= num1.length, num2.length <= 200"], leetcode_url: "https://leetcode.com/problems/multiply-strings/",
                approaches: &[ApproachMeta { id: 0, name: "Positional Grade-School Digit Array", time_complexity: "O(N * M)", space_complexity: "O(N + M)", rationale: "Product of digit at num1[i] and num2[j] places result at res[i + j + 1].", description: "Digit-by-digit multiplication with carry array." }],
            }),
        Problem::DetectSquares => Some(ProblemDetails {
                id: 2013, title: "Detect Squares", difficulty: Difficulty::Medium, category: Category::MathAndGeometry,
                statement: "Design a data structure that accepts a stream of 2D points and counts the number of ways to form axis-aligned squares with a given query point.",
                examples: &[Example { input: "add([3, 10]), add([11, 2]), add([3, 2]), count([11, 10])", output: "1", explanation: "Query point [11, 10] forms 1 square of side length 8." }],
                constraints: &["0 <= x, y <= 1000"], leetcode_url: "https://leetcode.com/problems/detect-squares/",
                approaches: &[ApproachMeta { id: 0, name: "Point Frequency Map & Diagonal Search", time_complexity: "O(N) count", space_complexity: "O(N)", rationale: "Searching points with matching dx == dy diagonal distance counts valid 4-corner squares in O(N) time.", description: "Store point frequencies in hash map and count diagonal square matches." }],
            }),
        _ => None,
    }
}

pub fn get_code_lines(problem: Problem, approach_id: usize) -> Option<Vec<(usize, &'static str)>> {
    match (problem, approach_id) {
        (Problem::HappyNumber, _) => Some(vec![
            (1, "class Solution:"),
            (2, "    def isHappy(self, n: int) -> bool:"),
            (3, "        visit = set()"),
            (4, "        while n not in visit:"),
            (5, "            visit.add(n); n = self.sumOfSquares(n)"),
            (6, "            if n == 1: return True"),
            (7, "        return False"),
        ]),
        (Problem::PlusOne, _) => Some(vec![
            (1, "class Solution:"),
            (2, "    def plusOne(self, digits: List[int]) -> List[int]:"),
            (3, "        for i in range(len(digits) - 1, -1, -1):"),
            (
                4,
                "            if digits[i] < 9: digits[i] += 1; return digits",
            ),
            (5, "            digits[i] = 0"),
            (6, "        return [1] + digits"),
        ]),
        (Problem::RotateImage, _) => Some(rotate_image_code_lines()),
        (Problem::SpiralMatrix, _) => Some(spiral_matrix_code_lines()),
        (Problem::SetMatrixZeroes, _) => Some(set_matrix_zeroes_code_lines()),
        (Problem::PowXN, _) => Some(pow_xn_code_lines()),
        (Problem::MultiplyStrings, _) => Some(multiply_strings_code_lines()),
        (Problem::DetectSquares, _) => Some(detect_squares_code_lines()),
        _ => None,
    }
}

// ── Helper Code Line Generators ──

pub fn rotate_image_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def rotate(self, matrix: List[List[int]]) -> None:"),
        (3, "        n = len(matrix)"),
        (4, "        for row in range(n):"),
        (5, "            for col in range(row + 1, n):"),
        (6, "                # Transpose across the main diagonal"),
        (7, "                matrix[row][col], matrix[col][row] = matrix[col][row], matrix[row][col]"),
        (8, ""),
        (9, "        for row in matrix:"),
        (10, "            left, right = 0, n - 1"),
        (11, "            while left < right:"),
        (12, "                row[left], row[right] = row[right], row[left]"),
        (13, "                left, right = left + 1, right - 1"),
        (14, "        return None"),
    ]
}

pub fn spiral_matrix_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def spiralOrder(self, matrix: List[List[int]]) -> List[int]:",
        ),
        (
            3,
            "        res = []; left, right = 0, len(matrix[0]); top, bottom = 0, len(matrix)",
        ),
        (4, "        while left < right and top < bottom:"),
        (
            5,
            "            for i in range(left, right): res.append(matrix[top][i])",
        ),
        (6, "            top += 1"),
        (7, "            for i in range(top, bottom):"),
        (8, "                res.append(matrix[i][right - 1])"),
        (9, "            right -= 1"),
        (
            10,
            "            if not (left < right and top < bottom): break",
        ),
        (11, "            for i in range(right - 1, left - 1, -1):"),
        (12, "                res.append(matrix[bottom - 1][i])"),
        (13, "            bottom -= 1"),
        (14, "            for i in range(bottom - 1, top - 1, -1):"),
        (15, "                res.append(matrix[i][left])"),
        (16, "            left += 1"),
        (17, "        return res"),
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
