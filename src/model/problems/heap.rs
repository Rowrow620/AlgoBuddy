use crate::model::problem::{ApproachMeta, Example, Problem, ProblemDetails};
use crate::model::taxonomy::{Category, Difficulty};

pub fn get_details(problem: Problem) -> Option<ProblemDetails> {
    match problem {
        Problem::KthLargestStream => Some(ProblemDetails {
                id: 703, title: "Kth Largest Element in a Stream", difficulty: Difficulty::Easy, category: Category::HeapPriorityQueue,
                statement: "Design a class to find the k-th largest element in a stream.",
                examples: &[Example { input: "k = 3, nums = [4, 5, 8, 2], val = 3", output: "4", explanation: "Min-heap of size k=3." }],
                constraints: &["1 <= k <= 10^4"], leetcode_url: "https://leetcode.com/problems/kth-largest-element-in-a-stream/",
                approaches: &[ApproachMeta { id: 0, name: "Min-Heap of Size k", time_complexity: "O(N log k)", space_complexity: "O(k)", rationale: "A min-heap of size k keeps the k largest elements at all times; the top element is always the k-th largest in O(log k) per add.", description: "Maintain min-heap of size k." }],
            }),
        Problem::LastStone => Some(ProblemDetails {
                id: 1046, title: "Last Stone Weight", difficulty: Difficulty::Easy, category: Category::HeapPriorityQueue,
                statement: "Smash two heaviest stones y and x until at most 1 stone remains.",
                examples: &[Example { input: "stones = [2, 7, 4, 1, 8, 1]", output: "1", explanation: "Smash 8 and 7, remaining 1." }],
                constraints: &["1 <= stones.length <= 30"], leetcode_url: "https://leetcode.com/problems/last-stone-weight/",
                approaches: &[ApproachMeta { id: 0, name: "Max-Heap Simulation", time_complexity: "O(N log N)", space_complexity: "O(N)", rationale: "A max-heap always provides the two heaviest stones in O(log N) time per smash iteration.", description: "Repeatedly smash top 2." }],
            }),
        Problem::KClosestPoints => Some(ProblemDetails {
                id: 973, title: "K Closest Points to Origin", difficulty: Difficulty::Medium, category: Category::HeapPriorityQueue,
                statement: "Given an array of points where points[i] = [xi, yi] and an integer k, return the k closest points to the origin (0, 0).",
                examples: &[Example { input: "points = [[1,3],[-2,2]], k = 1", output: "[[-2,2]]", explanation: "Distance of [1,3] is 10, distance of [-2,2] is 8. [-2,2] is closer." }],
                constraints: &["1 <= k <= points.length <= 10^4", "-10^4 <= xi, yi <= 10^4"], leetcode_url: "https://leetcode.com/problems/k-closest-points-to-origin/",
                approaches: &[ApproachMeta { id: 0, name: "Max-Heap of Size K", time_complexity: "O(N log K)", space_complexity: "O(K)", rationale: "Maintaining a max-heap of size K stores the smallest K distances seen so far in O(N log K) time.", description: "Push distances into max-heap of size K." }],
            }),
        Problem::TaskScheduler => Some(ProblemDetails {
                id: 621, title: "Task Scheduler", difficulty: Difficulty::Medium, category: Category::HeapPriorityQueue,
                statement: "Given a characters array tasks, representing the tasks a CPU needs to do, where each letter represents a different task. Tasks could be done in any order. Each task is done in one unit of time. For each unit of time, the CPU could have done a task or be idle. However, there is a non-negative integer n that represents the cooldown period between two same tasks.",
                examples: &[Example { input: "tasks = [\"A\",\"A\",\"A\",\"B\",\"B\",\"B\"], n = 2", output: "8", explanation: "A -> B -> idle -> A -> B -> idle -> A -> B." }],
                constraints: &["1 <= tasks.length <= 10^4", "0 <= n <= 100"], leetcode_url: "https://leetcode.com/problems/task-scheduler/",
                approaches: &[ApproachMeta { id: 0, name: "Max-Heap Frequency Priority Queue", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Always scheduling the highest frequency task via a Max-Heap minimizes CPU idle cooling cycles in O(N) time.", description: "Use Max-Heap of task counts to greedily schedule most frequent tasks." }],
            }),
        Problem::FindMedianDataStream => Some(ProblemDetails {
                id: 295, title: "Find Median from Data Stream", difficulty: Difficulty::Hard, category: Category::HeapPriorityQueue,
                statement: "The median is the middle value in an ordered integer list. If the size of the list is even, there is no middle value, and the median is the mean of the two middle values. Implement MedianFinder class.",
                examples: &[Example { input: "addNum(1), addNum(2), findMedian(), addNum(3), findMedian()", output: "1.5, 2.0", explanation: "Maintain small max-heap and large min-heap." }],
                constraints: &["-10^5 <= num <= 10^5", "At most 5 * 10^4 calls will be made to addNum and findMedian"], leetcode_url: "https://leetcode.com/problems/find-median-from-data-stream/",
                approaches: &[ApproachMeta { id: 0, name: "Two Heaps (Small Max-Heap & Large Min-Heap)", time_complexity: "O(log N) add, O(1) find", space_complexity: "O(N)", rationale: "Balancing two heaps keeps the middle elements accessible at the roots in O(1) time.", description: "Balance small max-heap and large min-heap." }],
            }),
        Problem::KthLargestArray => Some(ProblemDetails {
                id: 215, title: "Kth Largest Element in an Array", difficulty: Difficulty::Medium, category: Category::HeapPriorityQueue,
                statement: "Given an integer array nums and an integer k, return the kth largest element in the array.",
                examples: &[Example { input: "nums = [3,2,1,5,6,4], k = 2", output: "5", explanation: "Sorted in descending order: 6, 5, 4, 3, 2, 1. The 2nd largest is 5." }],
                constraints: &["1 <= k <= nums.length <= 10^5", "-10^4 <= nums[i] <= 10^4"], leetcode_url: "https://leetcode.com/problems/kth-largest-element-in-an-array/",
                approaches: &[ApproachMeta { id: 0, name: "Min-Heap of Size K / QuickSelect", time_complexity: "O(N log K)", space_complexity: "O(K)", rationale: "Maintaining a Min-Heap of size K leaves the Kth largest element at the root.", description: "Push into Min-Heap of size K." }],
            }),
        Problem::DesignTwitter => Some(ProblemDetails {
                id: 355, title: "Design Twitter", difficulty: Difficulty::Medium, category: Category::HeapPriorityQueue,
                statement: "Design a simplified version of Twitter where users can post tweets, follow/unfollow another user, and see the 10 most recent tweets in their news feed.",
                examples: &[Example { input: "postTweet(1, 5), getNewsFeed(1), follow(1, 2), postTweet(2, 6), getNewsFeed(1)", output: "[5], [6, 5]", explanation: "News feed retrieves top 10 recent tweets across followed users using a Max-Heap." }],
                constraints: &["1 <= userId, followerId, followeeId <= 500", "0 <= tweetId <= 10^4"], leetcode_url: "https://leetcode.com/problems/design-twitter/",
                approaches: &[ApproachMeta { id: 0, name: "Max-Heap Feed Merging", time_complexity: "O(K log N)", space_complexity: "O(N)", rationale: "Merging most recent tweets across followed users via Max-Heap returns news feed in O(K log N) time.", description: "Max-Heap merge of followed users' tweet lists." }],
            }),
        _ => None,
    }
}

pub fn get_code_lines(problem: Problem, approach_id: usize) -> Option<Vec<(usize, &'static str)>> {
    match (problem, approach_id) {
        (Problem::KthLargestStream, _) => Some(vec![
            (1, "class KthLargest:"),
            (2, "    def __init__(self, k: int, nums: List[int]):"),
            (3, "        self.minHeap, self.k = nums, k"),
            (4, "        heapq.heapify(self.minHeap)"),
            (
                5,
                "        while len(self.minHeap) > k: heapq.heappop(self.minHeap)",
            ),
            (6, "    def add(self, val: int) -> int:"),
            (7, "        heapq.heappush(self.minHeap, val)"),
            (
                8,
                "        if len(self.minHeap) > self.k: heapq.heappop(self.minHeap)",
            ),
            (9, "        return self.minHeap[0]"),
        ]),
        (Problem::LastStone, _) => Some(vec![
            (1, "class Solution:"),
            (
                2,
                "    def lastStoneWeight(self, stones: List[int]) -> int:",
            ),
            (3, "        stones = [-s for s in stones]"),
            (4, "        heapq.heapify(stones)"),
            (5, "        while len(stones) > 1:"),
            (
                6,
                "            first = heapq.heappop(stones); second = heapq.heappop(stones)",
            ),
            (
                7,
                "            if second > first: heapq.heappush(stones, first - second)",
            ),
            (8, "        stones.append(0)"),
            (9, "        return abs(stones[0])"),
        ]),
        (Problem::KClosestPoints, _) => Some(k_closest_points_code_lines()),
        (Problem::TaskScheduler, _) => Some(task_scheduler_code_lines()),
        (Problem::FindMedianDataStream, _) => Some(find_median_code_lines()),
        (Problem::KthLargestArray, _) => Some(kth_largest_array_code_lines()),
        (Problem::DesignTwitter, _) => Some(design_twitter_code_lines()),
        _ => None,
    }
}

// ── Helper Code Line Generators ──

pub fn kth_largest_array_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def findKthLargest(self, nums: List[int], k: int) -> int:",
        ),
        (3, "        heap = nums[:k]"),
        (4, "        heapq.heapify(heap)"),
        (5, "        for num in nums[k:]:"),
        (6, "            if num > heap[0]:"),
        (7, "                heapq.heappushpop(heap, num)"),
        (8, "        return heap[0]"),
    ]
}

pub fn design_twitter_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Twitter:"),
        (2, "    def __init__(self):"),
        (3, "        self.count = 0; self.tweetMap = defaultdict(list); self.followMap = defaultdict(set)"),
        (4, "    def postTweet(self, userId: int, tweetId: int) -> None:"),
        (5, "        self.tweetMap[userId].append([self.count, tweetId]); self.count -= 1"),
        (6, "    def getNewsFeed(self, userId: int) -> List[int]:"),
        (7, "        res, minHeap = [], []"),
        (8, "        self.followMap[userId].add(userId)"),
        (9, "        for followeeId in self.followMap[userId]:"),
        (10, "            if followeeId in self.tweetMap:"),
        (11, "                index = len(self.tweetMap[followeeId]) - 1"),
        (12, "                count, tweetId = self.tweetMap[followeeId][index]"),
        (13, "                minHeap.append([count, tweetId, followeeId, index - 1])"),
        (14, "        heapq.heapify(minHeap)"),
        (15, "        while minHeap and len(res) < 10:"),
        (16, "            count, tweetId, followeeId, index = heapq.heappop(minHeap)"),
        (17, "            res.append(tweetId)"),
        (18, "            if index >= 0:"),
        (19, "                count, tweetId = self.tweetMap[followeeId][index]"),
        (20, "                heapq.heappush(minHeap, [count, tweetId, followeeId, index - 1])"),
        (21, "        return res"),
    ]
}

pub fn find_median_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class MedianFinder:"),
        (2, "    def __init__(self):"),
        (3, "        self.small, self.large = [], []"),
        (4, "    def addNum(self, num: int) -> None:"),
        (5, "        heapq.heappush(self.small, -1 * num)"),
        (
            6,
            "        if self.small and self.large and (-1 * self.small[0]) > self.large[0]:",
        ),
        (7, "            val = -1 * heapq.heappop(self.small)"),
        (8, "            heapq.heappush(self.large, val)"),
        (9, "        if len(self.small) > len(self.large) + 1:"),
        (10, "            val = -1 * heapq.heappop(self.small)"),
        (11, "            heapq.heappush(self.large, val)"),
        (12, "        if len(self.large) > len(self.small) + 1:"),
        (13, "            val = heapq.heappop(self.large)"),
        (14, "            heapq.heappush(self.small, -1 * val)"),
        (15, "    def findMedian(self) -> float:"),
        (
            16,
            "        if len(self.small) > len(self.large): return -1 * self.small[0]",
        ),
        (
            17,
            "        if len(self.large) > len(self.small): return self.large[0]",
        ),
        (
            18,
            "        return (-1 * self.small[0] + self.large[0]) / 2.0",
        ),
    ]
}

pub fn k_closest_points_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def kClosest(self, points: List[List[int]], k: int) -> List[List[int]]:",
        ),
        (3, "        minHeap = []"),
        (4, "        for x, y in points:"),
        (5, "            dist = (x ** 2) + (y ** 2)"),
        (6, "            minHeap.append([dist, x, y])"),
        (7, "        heapq.heapify(minHeap)"),
        (8, "        res = []"),
        (9, "        for _ in range(k):"),
        (10, "            dist, x, y = heapq.heappop(minHeap)"),
        (11, "            res.append([x, y])"),
        (12, "        return res"),
    ]
}

pub fn task_scheduler_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def leastInterval(self, tasks: List[str], n: int) -> int:",
        ),
        (3, "        count = Counter(tasks)"),
        (4, "        maxHeap = [-cnt for cnt in count.values()]"),
        (5, "        heapq.heapify(maxHeap)"),
        (6, "        time = 0"),
        (7, "        q = deque()"),
        (8, "        while maxHeap or q:"),
        (9, "            time += 1"),
        (10, "            if maxHeap:"),
        (11, "                cnt = 1 + heapq.heappop(maxHeap)"),
        (12, "                if cnt: q.append([cnt, time + n])"),
        (13, "            if q and q[0][1] == time:"),
        (
            14,
            "                heapq.heappush(maxHeap, q.popleft()[0])",
        ),
        (15, "        return time"),
    ]
}
