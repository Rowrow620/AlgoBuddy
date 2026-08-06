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
