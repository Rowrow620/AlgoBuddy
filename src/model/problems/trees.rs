use crate::model::problem::{ApproachMeta, Example, Problem, ProblemDetails};
use crate::model::taxonomy::{Category, Difficulty};

pub fn get_details(problem: Problem) -> Option<ProblemDetails> {
    match problem {
        Problem::InvertTree => Some(ProblemDetails {
                id: 226, title: "Invert Binary Tree", difficulty: Difficulty::Easy, category: Category::Trees,
                statement: "Invert a binary tree (swap left and right subtrees for every node).",
                examples: &[Example { input: "root = [1, 2, 3, 4, 5, 6, 7]", output: "[1, 3, 2, 7, 6, 5, 4]", explanation: "Subtrees swapped." }],
                constraints: &["0 <= nodes <= 100"], leetcode_url: "https://leetcode.com/problems/invert-binary-tree/",
                approaches: &[ApproachMeta { id: 0, name: "Recursive DFS", time_complexity: "O(N)", space_complexity: "O(H)", rationale: "Swapping left and right child pointers recursively visits all N tree nodes in O(N) time.", description: "Post-order swap." }],
            }),
        Problem::MaxDepthTree => Some(ProblemDetails {
                id: 104, title: "Maximum Depth of Binary Tree", difficulty: Difficulty::Easy, category: Category::Trees,
                statement: "Return the maximum depth of a binary tree.",
                examples: &[Example { input: "root = [1, 2, 3, null, null, 4]", output: "3", explanation: "Longest path is 3 nodes." }],
                constraints: &["0 <= nodes <= 100"], leetcode_url: "https://leetcode.com/problems/maximum-depth-of-binary-tree/",
                approaches: &[ApproachMeta { id: 0, name: "Recursive DFS", time_complexity: "O(N)", space_complexity: "O(H)", rationale: "Depth-first search computes subtree heights recursively as 1 + max(left, right) in O(N) time.", description: "1 + max(left, right)." }],
            }),
        Problem::DiameterTree => Some(ProblemDetails {
                id: 543, title: "Diameter of Binary Tree", difficulty: Difficulty::Easy, category: Category::Trees,
                statement: "Return length of longest path between any two nodes.",
                examples: &[Example { input: "root = [1, null, 2, 3, 4, 5]", output: "3", explanation: "Longest path has 3 edges." }],
                constraints: &["1 <= nodes <= 100"], leetcode_url: "https://leetcode.com/problems/diameter-of-binary-tree/",
                approaches: &[ApproachMeta { id: 0, name: "Post-order Depth DFS", time_complexity: "O(N)", space_complexity: "O(H)", rationale: "Calculating longest left + right depth path at each node during DFS finds the global diameter in O(N) time.", description: "Left height + right height." }],
            }),
        Problem::BalancedTree => Some(ProblemDetails {
                id: 110, title: "Balanced Binary Tree", difficulty: Difficulty::Easy, category: Category::Trees,
                statement: "Determine if a binary tree is height-balanced (|height(left) - height(right)| <= 1).",
                examples: &[Example { input: "root = [3, 9, 20, null, null, 15, 7]", output: "true", explanation: "Balanced heights." }],
                constraints: &["0 <= nodes <= 5000"], leetcode_url: "https://leetcode.com/problems/balanced-binary-tree/",
                approaches: &[ApproachMeta { id: 0, name: "Bottom-Up Height DFS", time_complexity: "O(N)", space_complexity: "O(H)", rationale: "Bottom-up DFS returns -1 immediately upon detecting an unbalanced subtree, pruning unnecessary calculations in O(N) time.", description: "Check height difference at each node." }],
            }),
        Problem::SameTree => Some(ProblemDetails {
                id: 100, title: "Same Tree", difficulty: Difficulty::Easy, category: Category::Trees,
                statement: "Given roots of two binary trees p and q, return true if they are structural and value identical.",
                examples: &[Example { input: "p = [1, 2, 3], q = [1, 2, 3]", output: "true", explanation: "Trees match." }],
                constraints: &["0 <= nodes <= 100"], leetcode_url: "https://leetcode.com/problems/same-tree/",
                approaches: &[ApproachMeta { id: 0, name: "Recursive DFS Comparison", time_complexity: "O(N)", space_complexity: "O(H)", rationale: "Recursive DFS verifies value match and structural equality across both trees simultaneously in O(N) time.", description: "Check p.val == q.val and recurse." }],
            }),
        Problem::Subtree => Some(ProblemDetails {
                id: 572, title: "Subtree of Another Tree", difficulty: Difficulty::Easy, category: Category::Trees,
                statement: "Return true if there is a subtree of root with the same structure and node values as subRoot.",
                examples: &[Example { input: "root = [3, 4, 5, 1, 2], subRoot = [4, 1, 2]", output: "true", explanation: "Subtree matches." }],
                constraints: &["0 <= nodes <= 2000"], leetcode_url: "https://leetcode.com/problems/subtree-of-another-tree/",
                approaches: &[ApproachMeta { id: 0, name: "Recursive Tree Matching", time_complexity: "O(N * M)", space_complexity: "O(H)", rationale: "Comparing subtree matches recursively at each root node checks structural identity in O(N * M) time.", description: "Compare root node with subRoot recursively." }],
            }),
        Problem::BinaryTreeLevelOrderTraversal => Some(ProblemDetails {
                id: 102, title: "Binary Tree Level Order Traversal", difficulty: Difficulty::Medium, category: Category::Trees,
                statement: "Given the root of a binary tree, return the level order traversal of its nodes' values (i.e. from left to right, level by level).",
                examples: &[Example { input: "root = [3,9,20,null,null,15,7]", output: "[[3],[9,20],[15,7]]", explanation: "Nodes grouped by depth level." }],
                constraints: &["0 <= n <= 2000"], leetcode_url: "https://leetcode.com/problems/binary-tree-level-order-traversal/",
                approaches: &[ApproachMeta { id: 0, name: "BFS Queue Level Traversal", time_complexity: "O(N)", space_complexity: "O(N)", rationale: "Process tree level by level using a Queue.", description: "Queue BFS level collection." }],
            }),
        Problem::BinaryTreeRightSideView => Some(ProblemDetails {
                id: 199, title: "Binary Tree Right Side View", difficulty: Difficulty::Medium, category: Category::Trees,
                statement: "Given the root of a binary tree, imagine yourself standing on the right side of it, return the values of the nodes you can see ordered from top to bottom.",
                examples: &[Example { input: "root = [1,2,3,null,5,null,4]", output: "[1,3,4]", explanation: "Rightmost node visible per level." }],
                constraints: &["0 <= n <= 100"], leetcode_url: "https://leetcode.com/problems/binary-tree-right-side-view/",
                approaches: &[ApproachMeta { id: 0, name: "Right-First DFS / BFS", time_complexity: "O(N)", space_complexity: "O(H)", rationale: "Traverse right subtree first to capture the first node seen per level depth.", description: "Right-child prior DFS depth map." }],
            }),
        Problem::CountGoodNodes => Some(ProblemDetails {
                id: 1448, title: "Count Good Nodes in Binary Tree", difficulty: Difficulty::Medium, category: Category::Trees,
                statement: "Given a binary tree root, a node X in the tree is named good if in the path from root to X there are no nodes with a value greater than X.",
                examples: &[Example { input: "root = [3,1,4,3,null,1,5]", output: "4", explanation: "4 nodes satisfy path max condition." }],
                constraints: &["1 <= n <= 10^5"], leetcode_url: "https://leetcode.com/problems/count-good-nodes-in-binary-tree/",
                approaches: &[ApproachMeta { id: 0, name: "Pre-order DFS with Path Maximum", time_complexity: "O(N)", space_complexity: "O(H)", rationale: "Pass max_so_far along DFS recursive calls to validate good node criteria.", description: "DFS max path value tracking." }],
            }),
        Problem::KthSmallestElementBst => Some(ProblemDetails {
                id: 230, title: "Kth Smallest Element in a BST", difficulty: Difficulty::Medium, category: Category::Trees,
                statement: "Given the root of a binary search tree, and an integer k, return the kth smallest value (1-indexed) of all the values of the nodes in the tree.",
                examples: &[Example { input: "root = [3,1,4,null,2], k = 1", output: "1", explanation: "The 1st smallest element in BST is 1." }],
                constraints: &["1 <= k <= n <= 10^4"], leetcode_url: "https://leetcode.com/problems/kth-smallest-element-in-a-bst/",
                approaches: &[ApproachMeta { id: 0, name: "In-Order Traversal", time_complexity: "O(H + K)", space_complexity: "O(H)", rationale: "In-order traversal visits BST nodes in sorted ascending order; stop at k-th element.", description: "In-order DFS iteration." }],
            }),
        Problem::ConstructBinaryTreePreorderInorder => Some(ProblemDetails {
                id: 105, title: "Construct Binary Tree from Preorder and Inorder Traversal", difficulty: Difficulty::Medium, category: Category::Trees,
                statement: "Given two integer arrays preorder and inorder where preorder is the preorder traversal of a binary tree and inorder is the inorder traversal of the same tree, construct and return the binary tree.",
                examples: &[Example { input: "preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]", output: "[3,9,20,null,null,15,7]", explanation: "Tree constructed from traversals." }],
                constraints: &["1 <= n <= 3000"], leetcode_url: "https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/",
                approaches: &[ApproachMeta { id: 0, name: "Recursive Root Pick & Subtree Partitioning", time_complexity: "O(N)", space_complexity: "O(N)", rationale: "First element of preorder is root; split inorder array around root index to build left and right subtrees.", description: "Preorder root pick + Inorder partition." }],
            }),
        Problem::BinaryTreeMaxPathSum => Some(ProblemDetails {
                id: 124, title: "Binary Tree Maximum Path Sum", difficulty: Difficulty::Hard, category: Category::Trees,
                statement: "A path in a binary tree is a sequence of nodes where each pair of adjacent nodes has an edge connecting them. Return the maximum path sum of any non-empty path.",
                examples: &[Example { input: "root = [-10,9,20,null,null,15,7]", output: "42", explanation: "Path 15 -> 20 -> 7 gives maximum sum 42." }],
                constraints: &["1 <= n <= 3 * 10^4"], leetcode_url: "https://leetcode.com/problems/binary-tree-maximum-path-sum/",
                approaches: &[ApproachMeta { id: 0, name: "Post-Order DFS Bottleneck Path Sum", time_complexity: "O(N)", space_complexity: "O(H)", rationale: "At each node, compute max single path gain from left and right children and update global max path sum.", description: "Post-order DFS max split path sum." }],
            }),
        Problem::SerializeDeserializeBinaryTree => Some(ProblemDetails {
                id: 297, title: "Serialize and Deserialize Binary Tree", difficulty: Difficulty::Hard, category: Category::Trees,
                statement: "Design an algorithm to serialize and deserialize a binary tree into a string and back into the original tree structure.",
                examples: &[Example { input: "root = [1,2,3,null,null,4,5]", output: "[1,2,3,null,null,4,5]", explanation: "Tree string representation converted back to tree." }],
                constraints: &["0 <= n <= 10^4"], leetcode_url: "https://leetcode.com/problems/serialize-and-deserialize-binary-tree/",
                approaches: &[ApproachMeta { id: 0, name: "Pre-order DFS with Null Markers", time_complexity: "O(N)", space_complexity: "O(N)", rationale: "Pre-order DFS string join with 'N' markers allows unambiguous reconstruction.", description: "Pre-order string join and reconstruction." }],
            }),
        _ => None,
    }
}

pub fn get_code_lines(problem: Problem, approach_id: usize) -> Option<Vec<(usize, &'static str)>> {
    match (problem, approach_id) {
        (Problem::InvertTree, _) => Some(vec![
            (1, "class Solution:"),
            (2, "    def invertTree(self, root: Optional[TreeNode]) -> Optional[TreeNode]:"),
            (3, "        if not root: return None"),
            (4, "        tmp = root.left; root.left = root.right; root.right = tmp"),
            (5, "        self.invertTree(root.left); self.invertTree(root.right)"),
            (6, "        return root"),
        ]),
        (Problem::MaxDepthTree, _) => Some(vec![
            (1, "class Solution:"),
            (2, "    def maxDepth(self, root: Optional[TreeNode]) -> int:"),
            (3, "        if not root: return 0"),
            (4, "        return 1 + max(self.maxDepth(root.left), self.maxDepth(root.right))"),
        ]),
        (Problem::DiameterTree, _) => Some(vec![
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
        ]),
        (Problem::BalancedTree, _) => Some(vec![
            (1, "class Solution:"),
            (2, "    def isBalanced(self, root: Optional[TreeNode]) -> bool:"),
            (3, "        def dfs(root):"),
            (4, "            if not root: return [True, 0]"),
            (5, "            left, right = dfs(root.left), dfs(root.right)"),
            (6, "            balanced = left[0] and right[0] and abs(left[1] - right[1]) <= 1"),
            (7, "            return [balanced, 1 + max(left[1], right[1])]"),
            (8, "        return dfs(root)[0]"),
        ]),
        (Problem::SameTree, _) => Some(vec![
            (1, "class Solution:"),
            (2, "    def isSameTree(self, p: Optional[TreeNode], q: Optional[TreeNode]) -> bool:"),
            (3, "        if not p and not q: return True"),
            (4, "        if not p or not q or p.val != q.val: return False"),
            (5, "        return self.isSameTree(p.left, q.left) and self.isSameTree(p.right, q.right)"),
        ]),
        (Problem::Subtree, _) => Some(vec![
            (1, "class Solution:"),
            (2, "    def isSubtree(self, root: Optional[TreeNode], subRoot: Optional[TreeNode]) -> bool:"),
            (3, "        if not subRoot: return True"),
            (4, "        if not root: return False"),
            (5, "        if self.sameTree(root, subRoot): return True"),
            (6, "        return self.isSubtree(root.left, subRoot) or self.isSubtree(root.right, subRoot)"),
        ]),
        _ => None,
    }
}
