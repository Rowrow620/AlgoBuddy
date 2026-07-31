use crate::model::problem::{ApproachMeta, Example, Problem, ProblemDetails};
use crate::model::taxonomy::{Category, Difficulty};

pub fn get_details(problem: Problem) -> Option<ProblemDetails> {
    match problem {
        Problem::ReconstructItinerary => Some(ProblemDetails {
                id: 332, title: "Reconstruct Itinerary", difficulty: Difficulty::Hard, category: Category::AdvancedGraphs,
                statement: "Reconstruct the itinerary in order from a list of airline tickets. All tickets belong to a man who departs from JFK.",
                examples: &[Example { input: "tickets = [[\"MUC\",\"LHR\"],[\"JFK\",\"MUC\"],[\"SFO\",\"SJC\"],[\"LHR\",\"SFO\"]]", output: "[\"JFK\",\"MUC\",\"LHR\",\"SFO\",\"SJC\"]", explanation: "Valid flight itinerary traversing all tickets." }],
                constraints: &["1 <= tickets.length <= 300"], leetcode_url: "https://leetcode.com/problems/reconstruct-itinerary/",
                approaches: &[ApproachMeta { id: 0, name: "Hierholzer's Eulerian Path Algorithm", time_complexity: "O(E log E)", space_complexity: "O(E)", rationale: "Greedily exploring smallest lexical airport destinations and post-order appending yields Eulerian path.", description: "Eulerian path post-order traversal." }],
            }),
        Problem::MinCostConnectPoints => Some(ProblemDetails {
                id: 1584, title: "Min Cost to Connect All Points", difficulty: Difficulty::Medium, category: Category::AdvancedGraphs,
                statement: "Return the minimum cost to make all points connected using Manhattan distance between points.",
                examples: &[Example { input: "points = [[0,0],[2,2],[3,10],[5,2],[7,0]]", output: "20", explanation: "Minimum Spanning Tree cost = 20." }],
                constraints: &["1 <= points.length <= 1000"], leetcode_url: "https://leetcode.com/problems/min-cost-to-connect-all-points/",
                approaches: &[ApproachMeta { id: 0, name: "Prim's Minimum Spanning Tree (MST)", time_complexity: "O(N^2)", space_complexity: "O(N)", rationale: "Growing an MST greedily by picking the minimum Manhattan distance edge to unvisited points guarantees minimal connection cost.", description: "Greedy MST edge selection." }],
            }),
        Problem::NetworkDelayTime => Some(ProblemDetails {
                id: 743, title: "Network Delay Time", difficulty: Difficulty::Medium, category: Category::AdvancedGraphs,
                statement: "You are given a network of n nodes labeled from 1 to n and times[i] = (ui, vi, wi). Return the minimum time it takes for all n nodes to receive a signal sent from node k.",
                examples: &[Example { input: "times = [[2,1,1],[2,3,1],[3,4,1]], n = 4, k = 2", output: "2", explanation: "Signal reaches node 4 at time t=2." }],
                constraints: &["1 <= k <= n <= 100"], leetcode_url: "https://leetcode.com/problems/network-delay-time/",
                approaches: &[ApproachMeta { id: 0, name: "Dijkstra's Shortest Path Algorithm", time_complexity: "O(E log V)", space_complexity: "O(V + E)", rationale: "Priority queue min-heap relaxation finds single-source shortest path arrival times to all nodes.", description: "Min-heap Dijkstra propagation." }],
            }),
        Problem::SwimInRisingWater => Some(ProblemDetails {
                id: 778, title: "Swim in Rising Water", difficulty: Difficulty::Hard, category: Category::AdvancedGraphs,
                statement: "You are given an n x n integer matrix grid where each cell represents the elevation at that point. Return the least time until you can reach the bottom right cell (n-1, n-1) from (0,0).",
                examples: &[Example { input: "grid = [[0,2],[1,3]]", output: "3", explanation: "At time 3, water level rises to 3 and top-left connects to bottom-right." }],
                constraints: &["n == grid.length"], leetcode_url: "https://leetcode.com/problems/swim-in-rising-water/",
                approaches: &[ApproachMeta { id: 0, name: "Dijkstra / Min-Heap Grid Expansion", time_complexity: "O(N^2 log N)", space_complexity: "O(N^2)", rationale: "Expanding paths by minimum required max-elevation bottleneck via Priority Queue reaches destination in least time.", description: "Bottleneck path Min-Heap." }],
            }),
        Problem::AlienDictionary => Some(ProblemDetails {
                id: 269, title: "Alien Dictionary", difficulty: Difficulty::Hard, category: Category::AdvancedGraphs,
                statement: "There is a new alien language that uses the English alphabet. Given a list of words from the alien dictionary, derive the order of letters in this language.",
                examples: &[Example { input: "words = [\"wrt\",\"wrf\",\"er\",\"ett\",\"rftt\"]", output: "\"wertf\"", explanation: "Alien character precedence order is wertf." }],
                constraints: &["1 <= words.length <= 100"], leetcode_url: "https://leetcode.com/problems/alien-dictionary/",
                approaches: &[ApproachMeta { id: 0, name: "Topological Sort / Post-Order DFS", time_complexity: "O(C)", space_complexity: "O(1)", rationale: "Constructing character precedence edges from adjacent word differences and detecting directed cycles yields valid alien alphabet.", description: "Directed character DAG topological sort." }],
            }),
        Problem::CheapestFlights => Some(ProblemDetails {
                id: 787, title: "Cheapest Flights Within K Stops", difficulty: Difficulty::Medium, category: Category::AdvancedGraphs,
                statement: "Return the cheapest price from src to dst with at most k stops. If there is no such route, return -1.",
                examples: &[Example { input: "n = 4, flights = [[0,1,100],[1,2,100],[2,0,100],[1,3,600],[2,3,200]], src = 0, dst = 3, k = 1", output: "700", explanation: "Cheapest route 0 -> 1 -> 3 cost 700 with 1 stop." }],
                constraints: &["1 <= n <= 100", "0 <= k < n"], leetcode_url: "https://leetcode.com/problems/cheapest-flights-within-k-stops/",
                approaches: &[ApproachMeta { id: 0, name: "Bellman-Ford Algorithm (K Iterations)", time_complexity: "O(K * E)", space_complexity: "O(V)", rationale: "Relaxing edge costs exactly K+1 times guarantees finding shortest path with at most K stops.", description: "K-step edge cost relaxation." }],
            }),
        _ => None,
    }
}

pub fn get_code_lines(problem: Problem, approach_id: usize) -> Option<Vec<(usize, &'static str)>> {
    match (problem, approach_id) {
        (Problem::ReconstructItinerary, _) => Some(reconstruct_itinerary_code_lines()),
        (Problem::MinCostConnectPoints, _) => Some(min_cost_points_code_lines()),
        (Problem::NetworkDelayTime, _) => Some(network_delay_code_lines()),
        (Problem::SwimInRisingWater, _) => Some(swim_rising_water_code_lines()),
        (Problem::AlienDictionary, _) => Some(alien_dictionary_code_lines()),
        (Problem::CheapestFlights, _) => Some(cheapest_flights_code_lines()),
        _ => None,
    }
}

// ── Helper Code Line Generators ──

pub fn reconstruct_itinerary_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def findItinerary(self, tickets: List[List[str]]) -> List[str]:",
        ),
        (3, "        adj = { src: [] for src, dst in tickets }"),
        (4, "        tickets.sort()"),
        (5, "        for src, dst in tickets: adj[src].append(dst)"),
        (6, "        res = []"),
        (7, "        def dfs(src):"),
        (8, "            if src in adj:"),
        (9, "                while adj[src]:"),
        (10, "                    next_dest = adj[src].pop(0)"),
        (11, "                    dfs(next_dest)"),
        (12, "            res.append(src)"),
        (13, "        dfs(\"JFK\")"),
        (14, "        return res[::-1]"),
    ]
}

pub fn min_cost_points_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def minCostConnectPoints(self, points: List[List[int]]) -> int:",
        ),
        (3, "        N = len(points)"),
        (4, "        adj = { i: [] for i in range(N) }"),
        (5, "        for i in range(N):"),
        (6, "            x1, y1 = points[i]"),
        (7, "            for j in range(i + 1, N):"),
        (
            8,
            "                x2, y2 = points[j]; dist = abs(x1 - x2) + abs(y1 - y2)",
        ),
        (
            9,
            "                adj[i].append([dist, j]); adj[j].append([dist, i])",
        ),
        (10, "        res = 0; visit = set(); minH = [[0, 0]]"),
        (11, "        while len(visit) < N:"),
        (12, "            cost, i = heapq.heappop(minH)"),
        (13, "            if i in visit: continue"),
        (14, "            res += cost; visit.add(i)"),
        (15, "            for neiCost, nei in adj[i]:"),
        (
            16,
            "                if nei not in visit: heapq.heappush(minH, [neiCost, nei])",
        ),
        (17, "        return res"),
    ]
}

pub fn network_delay_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def networkDelayTime(self, times: List[List[int]], n: int, k: int) -> int:",
        ),
        (3, "        edges = collections.defaultdict(list)"),
        (4, "        for u, v, w in times: edges[u].append((v, w))"),
        (5, "        minHeap = [(0, k)]; visit = set(); t = 0"),
        (6, "        while minHeap:"),
        (7, "            w1, n1 = heapq.heappop(minHeap)"),
        (8, "            if n1 in visit: continue"),
        (9, "            visit.add(n1); t = w1"),
        (10, "            for n2, w2 in edges[n1]:"),
        (
            11,
            "                if n2 not in visit: heapq.heappush(minHeap, (w1 + w2, n2))",
        ),
        (12, "        return t if len(visit) == n else -1"),
    ]
}

pub fn swim_rising_water_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def swimInWater(self, grid: List[List[int]]) -> int:",
        ),
        (
            3,
            "        N = len(grid); visit = set(); minH = [[grid[0][0], 0, 0]]",
        ),
        (4, "        directions = [[0, 1], [0, -1], [1, 0], [-1, 0]]"),
        (5, "        visit.add((0, 0))"),
        (6, "        while minH:"),
        (7, "            t, r, c = heapq.heappop(minH)"),
        (8, "            if r == N - 1 and c == N - 1: return t"),
        (9, "            for dr, dc in directions:"),
        (10, "                row, col = r + dr, c + dc"),
        (
            11,
            "                if 0 <= row < N and 0 <= col < N and (row, col) not in visit:",
        ),
        (12, "                    visit.add((row, col))"),
        (
            13,
            "                    heapq.heappush(minH, [max(t, grid[row][col]), row, col])",
        ),
    ]
}

pub fn alien_dictionary_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def alienOrder(self, words: List[str]) -> str:"),
        (3, "        adj = { c: set() for w in words for c in w }"),
        (4, "        for i in range(len(words) - 1):"),
        (
            5,
            "            w1, w2 = words[i], words[i + 1]; minLen = min(len(w1), len(w2))",
        ),
        (
            6,
            "            if len(w1) > len(w2) and w1[:minLen] == w2[:minLen]: return \"\"",
        ),
        (7, "            for j in range(minLen):"),
        (
            8,
            "                if w1[j] != w2[j]: adj[w1[j]].add(w2[j]); break",
        ),
        (9, "        visit = {}; res = []"),
        (10, "        def dfs(c):"),
        (11, "            if c in visit: return visit[c]"),
        (12, "            visit[c] = True"),
        (13, "            for nei in adj[c]:"),
        (14, "                if dfs(nei): return True"),
        (15, "            visit[c] = False; res.append(c)"),
        (16, "        for c in adj:"),
        (17, "            if dfs(c): return \"\""),
        (18, "        return \"\".join(res[::-1])"),
    ]
}

pub fn cheapest_flights_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def findCheapestPrice(self, n: int, flights: List[List[int]], src: int, dst: int, k: int) -> int:"),
        (3, "        prices = [float(\"inf\")] * n; prices[src] = 0"),
        (4, "        for i in range(k + 1):"),
        (5, "            tmpPrices = list(prices)"),
        (6, "            for s, d, p in flights:"),
        (7, "                if prices[s] == float(\"inf\"): continue"),
        (8, "                if prices[s] + p < tmpPrices[d]: tmpPrices[d] = prices[s] + p"),
        (9, "            prices = tmpPrices"),
        (10, "        return prices[dst] if prices[dst] != float(\"inf\") else -1"),
    ]
}
