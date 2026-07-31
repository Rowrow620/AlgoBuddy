use crate::model::problem::{ApproachMeta, Example, Problem, ProblemDetails};
use crate::model::taxonomy::{Category, Difficulty};

pub fn get_details(problem: Problem) -> Option<ProblemDetails> {
    match problem {
        Problem::ImplementTrie => Some(ProblemDetails {
                id: 208, title: "Implement Trie (Prefix Tree)", difficulty: Difficulty::Medium, category: Category::Tries,
                statement: "A trie (prefix tree) is a tree data structure used to efficiently store and retrieve keys in a dataset of strings.",
                examples: &[Example { input: "insert(\"apple\"), search(\"apple\"), startsWith(\"app\")", output: "[null, true, true]", explanation: "Word and prefix found." }],
                constraints: &["1 <= word.length <= 2000"], leetcode_url: "https://leetcode.com/problems/implement-trie-prefix-tree/",
                approaches: &[ApproachMeta { id: 0, name: "TrieNode Hash/Array", time_complexity: "O(N)", space_complexity: "O(N * 26)", rationale: "Navigating child nodes by character code provides O(L) lookup independent of the total number of stored words.", description: "N-ary tree with character map and is_end flag." }],
            }),
        Problem::WordDictionary => Some(ProblemDetails {
                id: 211, title: "Design Add and Search Words Data Structure", difficulty: Difficulty::Medium, category: Category::Tries,
                statement: "Design a data structure that supports adding new words and searching if a string matches any previously added string (supporting '.' wildcards).",
                examples: &[Example { input: "addWord(\"bad\"), search(\".ad\")", output: "[null, true]", explanation: "'.' matches 'b'." }],
                constraints: &["1 <= word.length <= 25"], leetcode_url: "https://leetcode.com/problems/design-add-and-search-words-data-structure/",
                approaches: &[ApproachMeta { id: 0, name: "Trie DFS Wildcard Match", time_complexity: "O(N * 26^M)", space_complexity: "O(N)", rationale: "Trie DFS branches across 26 child nodes only when encountering wildcard ('.'), efficiently searching word patterns.", description: "DFS traversal branching on wildcard '.'." }],
            }),
        Problem::WordSearchII => Some(ProblemDetails {
                id: 212, title: "Word Search II", difficulty: Difficulty::Hard, category: Category::Tries,
                statement: "Given an m x n board of characters and a list of strings words, return all words on the board.",
                examples: &[Example { input: "board = [[\"o\",\"a\",\"a\",\"n\"],[\"e\",\"t\",\"a\",\"e\"]], words = [\"oath\",\"pea\",\"eat\",\"rain\"]", output: "[\"oath\",\"eat\"]", explanation: "Words found on grid." }],
                constraints: &["1 <= words.length <= 3 * 10^4"], leetcode_url: "https://leetcode.com/problems/word-search-ii/",
                approaches: &[ApproachMeta { id: 0, name: "Trie Grid Backtracking DFS", time_complexity: "O(M * N * 4^L)", space_complexity: "O(W * L)", rationale: "Building a Trie from dictionary words allows early pruning of grid DFS paths that do not form valid prefixes.", description: "Prune grid DFS using dictionary Trie." }],
            }),
        _ => None,
    }
}

pub fn get_code_lines(problem: Problem, approach_id: usize) -> Option<Vec<(usize, &'static str)>> {
    match (problem, approach_id) {
        (Problem::ImplementTrie, _) => Some(implement_trie_code_lines()),
        (Problem::WordDictionary, _) => Some(word_dictionary_code_lines()),
        (Problem::WordSearchII, _) => Some(word_search_ii_code_lines()),
        _ => None,
    }
}

// ── Helper Code Line Generators ──

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
