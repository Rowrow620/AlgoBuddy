use crate::model::problem::{ApproachMeta, Example, Problem, ProblemDetails};
use crate::model::taxonomy::{Category, Difficulty};

pub fn get_details(problem: Problem) -> Option<ProblemDetails> {
    match problem {
        Problem::NumberIslands => Some(ProblemDetails {
                id: 200, title: "Number of Islands", difficulty: Difficulty::Medium, category: Category::Graphs,
                statement: "Given an m x n 2D binary grid grid which represents a map of '1's (land) and '0's (water), return the number of islands.",
                examples: &[Example { input: "grid = [[\"1\",\"1\",\"1\",\"1\",\"0\"],[\"1\",\"1\",\"0\",\"1\",\"0\"],[\"1\",\"1\",\"0\",\"0\",\"0\"],[\"0\",\"0\",\"0\",\"0\",\"0\"]]", output: "1", explanation: "1 connected land mass." }],
                constraints: &["m == grid.length", "n == grid[i].length", "1 <= m, n <= 300"], leetcode_url: "https://leetcode.com/problems/number-of-islands/",
                approaches: &[ApproachMeta { id: 0, name: "BFS / DFS Grid Traversal", time_complexity: "O(M * N)", space_complexity: "O(M * N)", rationale: "Traversing connected land components using BFS/DFS counts unique islands in O(M * N) time.", description: "DFS/BFS flood fill set land cells to '0'." }],
            }),
        Problem::MaxAreaIsland => Some(ProblemDetails {
                id: 695, title: "Max Area of Island", difficulty: Difficulty::Medium, category: Category::Graphs,
                statement: "You are given an m x n binary matrix grid. An island is a group of 1's (representing land) connected 4-directionally. Return the maximum area of an island in grid.",
                examples: &[Example { input: "grid = [[0,0,1,0,0],[0,0,0,0,0],[0,1,1,1,0],[0,0,0,0,0]]", output: "3", explanation: "Max area island has 3 connected land cells." }],
                constraints: &["m == grid.length", "n == grid[i].length"], leetcode_url: "https://leetcode.com/problems/max-area-of-island/",
                approaches: &[ApproachMeta { id: 0, name: "DFS Connected Component Area Sum", time_complexity: "O(M * N)", space_complexity: "O(M * N)", rationale: "Accumulating connected land cell counts during DFS exploration tracks max island area.", description: "Sum 1 + dfs(up) + dfs(down) + dfs(left) + dfs(right)." }],
            }),
        Problem::CloneGraph => Some(ProblemDetails {
                id: 133, title: "Clone Graph", difficulty: Difficulty::Medium, category: Category::Graphs,
                statement: "Given a reference of a node in a connected undirected graph. Return a deep copy (clone) of the graph.",
                examples: &[Example { input: "adjList = [[2,4],[1,3],[2,4],[1,3]]", output: "[[2,4],[1,3],[2,4],[1,3]]", explanation: "Deep copy connected node graph structure." }],
                constraints: &["The number of nodes in the graph is in the range [0, 100]."], leetcode_url: "https://leetcode.com/problems/clone-graph/",
                approaches: &[ApproachMeta { id: 0, name: "DFS / BFS Hash Map Node Mapping", time_complexity: "O(V + E)", space_complexity: "O(V)", rationale: "Using a hash map to map old nodes to new cloned nodes prevents infinite recursion and handles cycles.", description: "Map old_node -> new_node and recursively clone neighbors." }],
            }),
        Problem::WallsAndGates => Some(ProblemDetails {
                id: 286, title: "Walls and Gates", difficulty: Difficulty::Medium, category: Category::Graphs,
                statement: "Fill each empty room (INF) with the distance to its nearest gate (0). If it is impossible to reach a gate, it should be filled with INF.",
                examples: &[Example { input: "rooms = [[2147483647,-1,0,2147483647],[2147483647,2147483647,2147483647,-1]]", output: "[[3,-1,0,1],[2,2,1,-1]]", explanation: "Fill empty rooms with shortest distance to gate 0." }],
                constraints: &["m == rooms.length", "n == rooms[i].length"], leetcode_url: "https://leetcode.com/problems/walls-and-gates/",
                approaches: &[ApproachMeta { id: 0, name: "Multi-Source BFS from Gates", time_complexity: "O(M * N)", space_complexity: "O(M * N)", rationale: "Enqueuing all gates simultaneously and expanding level-by-level computes shortest distance to gates in O(M * N) time.", description: "Multi-source BFS queue initialized with all gate (0) coordinates." }],
            }),
        Problem::RottingOranges => Some(ProblemDetails {
                id: 994, title: "Rotting Oranges", difficulty: Difficulty::Medium, category: Category::Graphs,
                statement: "Return the minimum number of minutes that must elapse until no cell has a fresh orange. If this is impossible, return -1.",
                examples: &[Example { input: "grid = [[2,1,1],[1,1,0],[0,1,1]]", output: "4", explanation: "Fresh oranges rot in 4 minutes." }],
                constraints: &["m == grid.length", "n == grid[i].length"], leetcode_url: "https://leetcode.com/problems/rotting-oranges/",
                approaches: &[ApproachMeta { id: 0, name: "Multi-Source BFS Minute Level Tracking", time_complexity: "O(M * N)", space_complexity: "O(M * N)", rationale: "Multi-source BFS enqueuing rotten oranges (2) tracks minutes until all fresh oranges (1) turn rotten.", description: "Level-by-level BFS from all rotten orange positions." }],
            }),
        Problem::PacificAtlantic => Some(ProblemDetails {
                id: 417, title: "Pacific Atlantic Water Flow", difficulty: Difficulty::Medium, category: Category::Graphs,
                statement: "Find the list of grid coordinates where water can flow to both the Pacific and Atlantic oceans.",
                examples: &[Example { input: "heights = [[1,2,2,3,5],[3,2,3,4,4],[2,4,5,3,1],[6,7,1,4,5],[5,1,1,2,4]]", output: "[[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[4,0]]", explanation: "Coordinates where water flows outward to both oceans." }],
                constraints: &["m == heights.length", "n == heights[i].length"], leetcode_url: "https://leetcode.com/problems/pacific-atlantic-water-flow/",
                approaches: &[ApproachMeta { id: 0, name: "Reverse Ocean Boundary DFS", time_complexity: "O(M * N)", space_complexity: "O(M * N)", rationale: "Starting DFS inward from Pacific and Atlantic ocean edges finds reachable cells; intersection is the answer.", description: "Reverse DFS from ocean borders uphill." }],
            }),
        Problem::SurroundedRegions => Some(ProblemDetails {
                id: 130, title: "Surrounded Regions", difficulty: Difficulty::Medium, category: Category::Graphs,
                statement: "Given an m x n matrix board containing 'X' and 'O', capture all regions that are 4-directionally surrounded by 'X'.",
                examples: &[Example { input: "board = [[\"X\",\"X\",\"X\",\"X\"],[\"X\",\"O\",\"O\",\"X\"],[\"X\",\"X\",\"O\",\"X\"],[\"X\",\"O\",\"X\",\"X\"]]", output: "[[\"X\",\"X\",\"X\",\"X\"],[\"X\",\"X\",\"X\",\"X\"],[\"X\",\"X\",\"X\",\"X\"],[\"X\",\"O\",\"X\",\"X\"]]", explanation: "Capture non-border surrounded 'O' regions." }],
                constraints: &["m == board.length", "n == board[i].length"], leetcode_url: "https://leetcode.com/problems/surrounded-regions/",
                approaches: &[ApproachMeta { id: 0, name: "Unsurrounded Border DFS Capture", time_complexity: "O(M * N)", space_complexity: "O(M * N)", rationale: "Marking border-connected 'O's as temporary 'T' via DFS leaves remaining 'O's surrounded to flip to 'X'.", description: "Mark border 'O's as 'T', flip remaining 'O' to 'X', then 'T' back to 'O'." }],
            }),
        Problem::CourseSchedule => Some(ProblemDetails {
                id: 207, title: "Course Schedule", difficulty: Difficulty::Medium, category: Category::Graphs,
                statement: "There are a total of numCourses courses you have to take, labeled from 0 to numCourses - 1. Return true if you can finish all courses.",
                examples: &[Example { input: "numCourses = 2, prerequisites = [[1,0]]", output: "true", explanation: "To take course 1 you should have finished course 0. So it is possible." }],
                constraints: &["1 <= numCourses <= 2000"], leetcode_url: "https://leetcode.com/problems/course-schedule/",
                approaches: &[ApproachMeta { id: 0, name: "Kahn's Topological Sort / DFS Cycle Detection", time_complexity: "O(V + E)", space_complexity: "O(V + E)", rationale: "Detecting directed cycles in the prerequisite graph verifies if a valid course order exists.", description: "Detect directed graph cycles using DFS visit states or indegrees." }],
            }),
        Problem::CourseScheduleII => Some(ProblemDetails {
                id: 210, title: "Course Schedule II", difficulty: Difficulty::Medium, category: Category::Graphs,
                statement: "Return the ordering of courses you should take to finish all courses. If it is impossible to finish all courses, return an empty array.",
                examples: &[Example { input: "numCourses = 2, prerequisites = [[1,0]]", output: "[0,1]", explanation: "Course 0 then course 1." }],
                constraints: &["1 <= numCourses <= 2000"], leetcode_url: "https://leetcode.com/problems/course-schedule-ii/",
                approaches: &[ApproachMeta { id: 0, name: "Topological Sort Order", time_complexity: "O(V + E)", space_complexity: "O(V + E)", rationale: "Kahn's BFS queue or DFS post-order reversal yields valid course completion sequence.", description: "Append course nodes to topological order queue." }],
            }),
        Problem::GraphValidTree => Some(ProblemDetails {
                id: 261, title: "Graph Valid Tree", difficulty: Difficulty::Medium, category: Category::Graphs,
                statement: "Given n nodes labeled from 0 to n - 1 and a list of undirected edges, write a function to check whether these edges make up a valid tree.",
                examples: &[Example { input: "n = 5, edges = [[0,1],[0,2],[0,3],[1,4]]", output: "true", explanation: "Graph is fully connected and has no cycles." }],
                constraints: &["1 <= n <= 2000"], leetcode_url: "https://leetcode.com/problems/graph-valid-tree/",
                approaches: &[ApproachMeta { id: 0, name: "Union-Find / DFS Cycle & Component Check", time_complexity: "O(V + E)", space_complexity: "O(V)", rationale: "A graph is a valid tree if E == V - 1 and all nodes are connected in a single component without cycles.", description: "Verify edges == n - 1 and single connected component." }],
            }),
        Problem::ConnectedComponents => Some(ProblemDetails {
                id: 323, title: "Number of Connected Components in an Undirected Graph", difficulty: Difficulty::Medium, category: Category::Graphs,
                statement: "Given n nodes and an array of undirected edges, return the number of connected components in the graph.",
                examples: &[Example { input: "n = 5, edges = [[0,1],[1,2],[3,4]]", output: "2", explanation: "Components are {0,1,2} and {3,4}." }],
                constraints: &["1 <= n <= 2000"], leetcode_url: "https://leetcode.com/problems/number-of-connected-components-in-an-undirected-graph/",
                approaches: &[ApproachMeta { id: 0, name: "Union-Find Disjoint Set", time_complexity: "O(V + E * alpha(V))", space_complexity: "O(V)", rationale: "Union-Find decrements component count upon uniting distinct node sets.", description: "Initialize V components and decrement on union(u, v)." }],
            }),
        Problem::RedundantConnection => Some(ProblemDetails {
                id: 684, title: "Redundant Connection", difficulty: Difficulty::Medium, category: Category::Graphs,
                statement: "Return an edge that can be removed so that the resulting graph is a tree of n nodes.",
                examples: &[Example { input: "edges = [[1,2],[1,3],[2,3]]", output: "[2,3]", explanation: "[2,3] creates a cycle in the graph." }],
                constraints: &["n == edges.length", "3 <= n <= 1000"], leetcode_url: "https://leetcode.com/problems/redundant-connection/",
                approaches: &[ApproachMeta { id: 0, name: "Union-Find Cycle Edge Identification", time_complexity: "O(N * alpha(N))", space_complexity: "O(N)", rationale: "The first edge connecting two already-united nodes forms the redundant cycle edge.", description: "Return edge where find(u) == find(v)." }],
            }),
        Problem::WordLadder => Some(ProblemDetails {
                id: 127, title: "Word Ladder", difficulty: Difficulty::Hard, category: Category::Graphs,
                statement: "Given two words, beginWord and endWord, and a dictionary wordList, return the number of words in the shortest transformation sequence from beginWord to endWord, or 0 if no such sequence exists.",
                examples: &[Example { input: "beginWord = \"hit\", endWord = \"cog\", wordList = [\"hot\",\"dot\",\"dog\",\"lot\",\"log\",\"cog\"]", output: "5", explanation: "Shortest transformation: hit -> hot -> dot -> dog -> cog (5 words)." }],
                constraints: &["1 <= beginWord.length <= 10", "1 <= wordList.length <= 5000"], leetcode_url: "https://leetcode.com/problems/word-ladder/",
                approaches: &[ApproachMeta { id: 0, name: "BFS Shortest Path Transformation Graph", time_complexity: "O(N * M^2)", space_complexity: "O(N * M^2)", rationale: "Building wildcard pattern adjacency buckets and performing BFS guarantees shortest transformation path.", description: "BFS on single-character pattern buckets." }],
            }),
        _ => None,
    }
}

pub fn get_code_lines(problem: Problem, approach_id: usize) -> Option<Vec<(usize, &'static str)>> {
    match (problem, approach_id) {
        (Problem::NumberIslands, _) => Some(number_islands_code_lines()),
        (Problem::MaxAreaIsland, _) => Some(max_area_island_code_lines()),
        (Problem::CloneGraph, _) => Some(clone_graph_code_lines()),
        (Problem::WallsAndGates, _) => Some(walls_and_gates_code_lines()),
        (Problem::RottingOranges, _) => Some(rotting_oranges_code_lines()),
        (Problem::PacificAtlantic, _) => Some(pacific_atlantic_code_lines()),
        (Problem::SurroundedRegions, _) => Some(surrounded_regions_code_lines()),
        (Problem::CourseSchedule, _) => Some(course_schedule_code_lines()),
        (Problem::CourseScheduleII, _) => Some(course_schedule_ii_code_lines()),
        (Problem::GraphValidTree, _) => Some(graph_valid_tree_code_lines()),
        (Problem::ConnectedComponents, _) => Some(connected_components_code_lines()),
        (Problem::RedundantConnection, _) => Some(redundant_connection_code_lines()),
        (Problem::WordLadder, _) => Some(word_ladder_code_lines()),
        _ => None,
    }
}

// ── Helper Code Line Generators ──

pub fn number_islands_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def numIslands(self, grid: List[List[str]]) -> int:"),
        (3, "        if not grid: return 0"),
        (4, "        rows, cols = len(grid), len(grid[0]); visited = set(); islands = 0"),
        (5, "        def bfs(r, c):"),
        (6, "            q = collections.deque([(r, c)]); visited.add((r, c))"),
        (7, "            while q:"),
        (8, "                row, col = q.popleft()"),
        (9, "                for dr, dc in [[1,0],[-1,0],[0,1],[0,-1]]:"),
        (10, "                    r, c = row + dr, col + dc"),
        (11, "                    if 0 <= r < rows and 0 <= c < cols and grid[r][c] == '1' and (r,c) not in visited:"),
        (12, "                        q.append((r, c)); visited.add((r, c))"),
        (13, "        for r in range(rows):"),
        (14, "            for c in range(cols):"),
        (15, "                if grid[r][c] == '1' and (r, c) not in visited: bfs(r, c); islands += 1"),
        (16, "        return islands"),
    ]
}

pub fn max_area_island_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def maxAreaOfIsland(self, grid: List[List[int]]) -> int:"),
        (3, "        ROWS, COLS = len(grid), len(grid[0]); visit = set()"),
        (4, "        def dfs(r, c):"),
        (5, "            if r < 0 or r == ROWS or c < 0 or c == COLS or grid[r][c] == 0 or (r, c) in visit: return 0"),
        (6, "            visit.add((r, c))"),
        (7, "            return 1 + dfs(r + 1, c) + dfs(r - 1, c) + dfs(r, c + 1) + dfs(r, c - 1)"),
        (8, "        area = 0"),
        (9, "        for r in range(ROWS):"),
        (10, "            for c in range(COLS): area = max(area, dfs(r, c))"),
        (11, "        return area"),
    ]
}

pub fn clone_graph_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def cloneGraph(self, node: 'Node') -> 'Node':"),
        (3, "        oldToNew = {}"),
        (4, "        def dfs(node):"),
        (5, "            if not node: return None"),
        (6, "            if node in oldToNew: return oldToNew[node]"),
        (
            7,
            "            copy = Node(node.val); oldToNew[node] = copy",
        ),
        (
            8,
            "            for nei in node.neighbors: copy.neighbors.append(dfs(nei))",
        ),
        (9, "            return copy"),
        (10, "        return dfs(node)"),
    ]
}

pub fn walls_and_gates_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def wallsAndGates(self, rooms: List[List[int]]) -> None:"),
        (3, "        ROWS, COLS = len(rooms), len(rooms[0]); q = collections.deque()"),
        (4, "        for r in range(ROWS):"),
        (5, "            for c in range(COLS):"),
        (6, "                if rooms[r][c] == 0: q.append((r, c))"),
        (7, "        dist = 0"),
        (8, "        while q:"),
        (9, "            for i in range(len(q)):"),
        (10, "                r, c = q.popleft()"),
        (11, "                rooms[r][c] = dist"),
        (12, "                for dr, dc in [[1,0],[-1,0],[0,1],[0,-1]]:"),
        (13, "                    nr, nc = r + dr, c + dc"),
        (14, "                    if 0 <= nr < ROWS and 0 <= nc < COLS and rooms[nr][nc] == 2147483647:"),
        (15, "                        q.append((nr, nc))"),
        (16, "            dist += 1"),
    ]
}

pub fn rotting_oranges_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def orangesRotting(self, grid: List[List[int]]) -> int:",
        ),
        (3, "        q = collections.deque(); time, fresh = 0, 0"),
        (4, "        ROWS, COLS = len(grid), len(grid[0])"),
        (5, "        for r in range(ROWS):"),
        (6, "            for c in range(COLS):"),
        (7, "                if grid[r][c] == 1: fresh += 1"),
        (8, "                if grid[r][c] == 2: q.append([r, c])"),
        (9, "        while q and fresh > 0:"),
        (10, "            for i in range(len(q)):"),
        (11, "                r, c = q.popleft()"),
        (
            12,
            "                for dr, dc in [[1,0],[-1,0],[0,1],[0,-1]]:",
        ),
        (13, "                    row, col = r + dr, c + dc"),
        (
            14,
            "                    if 0 <= row < ROWS and 0 <= col < COLS and grid[row][col] == 1:",
        ),
        (
            15,
            "                        grid[row][col] = 2; q.append([row, col]); fresh -= 1",
        ),
        (16, "            time += 1"),
        (17, "        return time if fresh == 0 else -1"),
    ]
}

pub fn pacific_atlantic_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def pacificAtlantic(self, heights: List[List[int]]) -> List[List[int]]:"),
        (3, "        ROWS, COLS = len(heights), len(heights[0]); pac, atl = set(), set()"),
        (4, "        def dfs(r, c, visit, prevHeight):"),
        (5, "            if ((r, c) in visit or r < 0 or c < 0 or r == ROWS or c == COLS or heights[r][c] < prevHeight): return"),
        (6, "            visit.add((r, c))"),
        (7, "            for dr, dc in [[1,0],[-1,0],[0,1],[0,-1]]: dfs(r + dr, c + dc, visit, heights[r][c])"),
        (8, "        for c in range(COLS): dfs(0, c, pac, heights[0][c]); dfs(ROWS - 1, c, atl, heights[ROWS - 1][c])"),
        (9, "        for r in range(ROWS): dfs(r, 0, pac, heights[r][0]); dfs(r, COLS - 1, atl, heights[r][COLS - 1])"),
        (10, "        return list(pac & atl)"),
    ]
}

pub fn surrounded_regions_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def solve(self, board: List[List[str]]) -> None:"),
        (3, "        ROWS, COLS = len(board), len(board[0])"),
        (4, "        def capture(r, c):"),
        (
            5,
            "            if r < 0 or c < 0 or r == ROWS or c == COLS or board[r][c] != 'O': return",
        ),
        (6, "            board[r][c] = 'T'"),
        (
            7,
            "            for dr, dc in [[1,0],[-1,0],[0,1],[0,-1]]: capture(r + dr, c + dc)",
        ),
        (
            8,
            "        for r in range(ROWS): capture(r, 0); capture(r, COLS - 1)",
        ),
        (
            9,
            "        for c in range(COLS): capture(0, c); capture(ROWS - 1, c)",
        ),
        (10, "        for r in range(ROWS):"),
        (11, "            for c in range(COLS):"),
        (
            12,
            "                if board[r][c] == 'O': board[r][c] = 'X'",
        ),
        (
            13,
            "                elif board[r][c] == 'T': board[r][c] = 'O'",
        ),
    ]
}

pub fn course_schedule_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def canFinish(self, numCourses: int, prerequisites: List[List[int]]) -> bool:",
        ),
        (3, "        preMap = { i: [] for i in range(numCourses) }"),
        (
            4,
            "        for crs, pre in prerequisites: preMap[crs].append(pre)",
        ),
        (5, "        visitSet = set()"),
        (6, "        def dfs(crs):"),
        (7, "            if crs in visitSet: return False"),
        (8, "            if preMap[crs] == []: return True"),
        (9, "            visitSet.add(crs)"),
        (10, "            for pre in preMap[crs]:"),
        (11, "                if not dfs(pre): return False"),
        (12, "            visitSet.remove(crs); preMap[crs] = []"),
        (13, "            return True"),
        (14, "        for crs in range(numCourses):"),
        (15, "            if not dfs(crs): return False"),
        (16, "        return True"),
    ]
}

pub fn course_schedule_ii_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def findOrder(self, numCourses: int, prerequisites: List[List[int]]) -> List[int]:"),
        (3, "        prereq = { i: [] for i in range(numCourses) }"),
        (4, "        for crs, pre in prerequisites: prereq[crs].append(pre)"),
        (5, "        output = []; visit, cycle = set(), set()"),
        (6, "        def dfs(crs):"),
        (7, "            if crs in cycle: return False"),
        (8, "            if crs in visit: return True"),
        (9, "            cycle.add(crs)"),
        (10, "            for pre in prereq[crs]:"),
        (11, "                if not dfs(pre): return False"),
        (12, "            cycle.remove(crs); visit.add(crs); output.append(crs)"),
        (13, "            return True"),
        (14, "        for c in range(numCourses):"),
        (15, "            if not dfs(c): return []"),
        (16, "        return output"),
    ]
}

pub fn graph_valid_tree_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def validTree(self, n: int, edges: List[List[int]]) -> bool:",
        ),
        (3, "        if not n: return True"),
        (4, "        adj = { i: [] for i in range(n) }"),
        (
            5,
            "        for n1, n2 in edges: adj[n1].append(n2); adj[n2].append(n1)",
        ),
        (6, "        visit = set()"),
        (7, "        def dfs(i, prev):"),
        (8, "            if i in visit: return False"),
        (9, "            visit.add(i)"),
        (10, "            for j in adj[i]:"),
        (11, "                if j == prev: continue"),
        (12, "                if not dfs(j, i): return False"),
        (13, "            return True"),
        (14, "        return dfs(0, -1) and len(visit) == n"),
    ]
}

pub fn connected_components_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def countComponents(self, n: int, edges: List[List[int]]) -> int:",
        ),
        (3, "        par = [i for i in range(n)]; rank = [1] * n"),
        (4, "        def find(n1):"),
        (5, "            res = n1"),
        (
            6,
            "            while res != par[res]: par[res] = par[par[res]]; res = par[res]",
        ),
        (7, "            return res"),
        (8, "        def union(n1, n2):"),
        (9, "            p1, p2 = find(n1), find(n2)"),
        (10, "            if p1 == p2: return 0"),
        (
            11,
            "            if rank[p2] > rank[p1]: par[p1] = p2; rank[p2] += rank[p1]",
        ),
        (12, "            else: par[p2] = p1; rank[p1] += rank[p2]"),
        (13, "            return 1"),
        (14, "        res = n"),
        (15, "        for n1, n2 in edges: res -= union(n1, n2)"),
        (16, "        return res"),
    ]
}

pub fn redundant_connection_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def findRedundantConnection(self, edges: List[List[int]]) -> List[int]:",
        ),
        (
            3,
            "        par = [i for i in range(len(edges) + 1)]; rank = [1] * (len(edges) + 1)",
        ),
        (4, "        def find(n):"),
        (5, "            p = par[n]"),
        (
            6,
            "            while p != par[p]: par[p] = par[par[p]]; p = par[p]",
        ),
        (7, "            return p"),
        (8, "        def union(n1, n2):"),
        (9, "            p1, p2 = find(n1), find(n2)"),
        (10, "            if p1 == p2: return False"),
        (
            11,
            "            if rank[p1] > rank[p2]: par[p2] = p1; rank[p1] += rank[p2]",
        ),
        (12, "            else: par[p1] = p2; rank[p2] += rank[p1]"),
        (13, "            return True"),
        (14, "        for n1, n2 in edges:"),
        (15, "            if not union(n1, n2): return [n1, n2]"),
    ]
}

pub fn word_ladder_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def ladderLength(self, beginWord: str, endWord: str, wordList: List[str]) -> int:"),
        (3, "        if endWord not in wordList: return 0"),
        (4, "        nei = collections.defaultdict(list); wordList.append(beginWord)"),
        (5, "        for word in wordList:"),
        (6, "            for j in range(len(word)):"),
        (7, "                pattern = word[:j] + \"*\" + word[j+1:]"),
        (8, "                nei[pattern].append(word)"),
        (9, "        visit = set([beginWord]); q = collections.deque([beginWord]); res = 1"),
        (10, "        while q:"),
        (11, "            for i in range(len(q)):"),
        (12, "                word = q.popleft()"),
        (13, "                if word == endWord: return res"),
        (14, "                for j in range(len(word)):"),
        (15, "                    pattern = word[:j] + \"*\" + word[j+1:]"),
        (16, "                    for neiWord in nei[pattern]:"),
        (17, "                        if neiWord not in visit: visit.add(neiWord); q.append(neiWord)"),
        (18, "            res += 1"),
        (19, "        return 0"),
    ]
}
