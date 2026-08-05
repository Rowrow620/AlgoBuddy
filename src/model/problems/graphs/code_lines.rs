use crate::model::problem::Problem;

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
