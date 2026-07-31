use crate::model::problem::{ApproachMeta, Example, Problem, ProblemDetails};
use crate::model::taxonomy::{Category, Difficulty};

pub fn get_details(problem: Problem) -> Option<ProblemDetails> {
    match problem {
        Problem::MeetingRooms => Some(ProblemDetails {
                id: 252, title: "Meeting Rooms", difficulty: Difficulty::Easy, category: Category::Intervals,
                statement: "Given an array of meeting time intervals, determine if a person could attend all meetings.",
                examples: &[Example { input: "intervals = [[0,30],[5,10],[15,20]]", output: "false", explanation: "[0,30] and [5,10] overlap." }],
                constraints: &["0 <= intervals.length <= 10^4"], leetcode_url: "https://leetcode.com/problems/meeting-rooms/",
                approaches: &[ApproachMeta { id: 0, name: "Sort Intervals by Start Time", time_complexity: "O(N log N)", space_complexity: "O(1)", rationale: "Sorting interval start times in O(N log N) allows checking adjacent meeting overlaps in a single O(N) pass.", description: "Check adjacent overlap." }],
            }),
        Problem::InsertInterval => Some(ProblemDetails {
                id: 57, title: "Insert Interval", difficulty: Difficulty::Medium, category: Category::Intervals,
                statement: "You are given an array of non-overlapping intervals intervals where intervals[i] = [starti, endi] sorted in ascending order by starti. Insert newInterval into intervals such that intervals is still sorted.",
                examples: &[Example { input: "intervals = [[1,3],[6,9]], newInterval = [2,5]", output: "[[1,5],[6,9]]", explanation: "Merge newInterval [2,5] with [1,3] into [1,5]." }],
                constraints: &["0 <= intervals.length <= 10^4"], leetcode_url: "https://leetcode.com/problems/insert-interval/",
                approaches: &[ApproachMeta { id: 0, name: "Three-Phase Linear Scan", time_complexity: "O(N)", space_complexity: "O(N)", rationale: "Collecting left non-overlapping intervals, merging overlapping intervals with newInterval, and appending right non-overlapping intervals.", description: "3-phase scan: left, merge overlapping, right." }],
            }),
        Problem::MergeIntervals => Some(ProblemDetails {
                id: 56, title: "Merge Intervals", difficulty: Difficulty::Medium, category: Category::Intervals,
                statement: "Given an array of intervals where intervals[i] = [starti, endi], merge all overlapping intervals.",
                examples: &[Example { input: "intervals = [[1,3],[2,6],[8,10],[15,18]]", output: "[[1,6],[8,10],[15,18]]", explanation: "[1,3] and [2,6] overlap into [1,6]." }],
                constraints: &["1 <= intervals.length <= 10^4"], leetcode_url: "https://leetcode.com/problems/merge-intervals/",
                approaches: &[ApproachMeta { id: 0, name: "Sort by Start & Merge Adjacent", time_complexity: "O(N log N)", space_complexity: "O(N)", rationale: "Sorting by start time allows merging overlapping intervals in a single linear pass.", description: "Sort intervals by start time and merge adjacent overlaps." }],
            }),
        Problem::NonOverlappingIntervals => Some(ProblemDetails {
                id: 435, title: "Non-overlapping Intervals", difficulty: Difficulty::Medium, category: Category::Intervals,
                statement: "Given an array of intervals intervals where intervals[i] = [starti, endi], return the minimum number of intervals you need to remove to make the rest of the intervals non-overlapping.",
                examples: &[Example { input: "intervals = [[1,2],[2,3],[3,4],[1,3]]", output: "1", explanation: "[1,3] can be removed and the rest of the intervals are non-overlapping." }],
                constraints: &["1 <= intervals.length <= 10^5"], leetcode_url: "https://leetcode.com/problems/non-overlapping-intervals/",
                approaches: &[ApproachMeta { id: 0, name: "Greedy Earliest End Time Selection", time_complexity: "O(N log N)", space_complexity: "O(1)", rationale: "Sorting by end time and keeping interval with smaller end time minimizes overlaps.", description: "Sort intervals by start time; remove interval with larger end time when overlapping." }],
            }),
        Problem::MeetingRoomsII => Some(ProblemDetails {
                id: 253, title: "Meeting Rooms II", difficulty: Difficulty::Medium, category: Category::Intervals,
                statement: "Given an array of meeting time intervals intervals where intervals[i] = [starti, endi], return the minimum number of conference rooms required.",
                examples: &[Example { input: "intervals = [[0,30],[5,10],[15,20]]", output: "2", explanation: "Room 1: [0,30]; Room 2: [5,10], [15,20]." }],
                constraints: &["1 <= intervals.length <= 10^4"], leetcode_url: "https://leetcode.com/problems/meeting-rooms-ii/",
                approaches: &[ApproachMeta { id: 0, name: "Two Pointers / Min-Heap Active Meeting Count", time_complexity: "O(N log N)", space_complexity: "O(N)", rationale: "Sorting start and end times separately and using two pointers tracks simultaneous active meetings.", description: "Two pointers on sorted start and end time arrays." }],
            }),
        Problem::MinIntervalQuery => Some(ProblemDetails {
                id: 1851, title: "Minimum Interval to Include Each Query", difficulty: Difficulty::Hard, category: Category::Intervals,
                statement: "Given 2D integer array intervals and queries array, return smallest interval length containing each query.",
                examples: &[Example { input: "intervals = [[1,4],[2,4],[3,6],[4,4]], queries = [2,3,4,5]", output: "[3,3,1,4]", explanation: "Query 2: smallest interval is [2,4] length 3." }],
                constraints: &["1 <= intervals.length, queries.length <= 10^5"], leetcode_url: "https://leetcode.com/problems/minimum-interval-to-include-each-query/",
                approaches: &[ApproachMeta { id: 0, name: "Offline Query Sorting & Priority Queue", time_complexity: "O(N log N + Q log Q)", space_complexity: "O(N + Q)", rationale: "Sorting queries and pushing valid intervals into min-heap ordered by length.", description: "Process sorted queries with min-heap of active interval lengths." }],
            }),
        _ => None,
    }
}

pub fn get_code_lines(problem: Problem, approach_id: usize) -> Option<Vec<(usize, &'static str)>> {
    match (problem, approach_id) {
        (Problem::MeetingRooms, _) => Some(vec![
            (1, "class Solution:"),
            (
                2,
                "    def canAttendMeetings(self, intervals: List[Interval]) -> bool:",
            ),
            (3, "        intervals.sort(key=lambda i: i.start)"),
            (4, "        for i in range(1, len(intervals)):"),
            (
                5,
                "            if intervals[i].start < intervals[i - 1].end: return False",
            ),
            (6, "        return True"),
        ]),
        (Problem::InsertInterval, _) => Some(insert_interval_code_lines()),
        (Problem::MergeIntervals, _) => Some(merge_intervals_code_lines()),
        (Problem::NonOverlappingIntervals, _) => Some(non_overlapping_intervals_code_lines()),
        (Problem::MeetingRoomsII, _) => Some(meeting_rooms_ii_code_lines()),
        (Problem::MinIntervalQuery, _) => Some(min_interval_query_code_lines()),
        _ => None,
    }
}

// ── Helper Code Line Generators ──

pub fn insert_interval_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def insert(self, intervals: List[List[int]], newInterval: List[int]) -> List[List[int]]:"),
        (3, "        res = []"),
        (4, "        for i in range(len(intervals)):"),
        (5, "            if newInterval[1] < intervals[i][0]: res.append(newInterval); return res + intervals[i:]"),
        (6, "            elif newInterval[0] > intervals[i][1]: res.append(intervals[i])"),
        (7, "            else: newInterval = [min(newInterval[0], intervals[i][0]), max(newInterval[1], intervals[i][1])]"),
        (8, "        res.append(newInterval)"),
        (9, "        return res"),
    ]
}

pub fn merge_intervals_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def merge(self, intervals: List[List[int]]) -> List[List[int]]:",
        ),
        (
            3,
            "        intervals.sort(key=lambda i: i[0]); output = [intervals[0]]",
        ),
        (4, "        for start, end in intervals[1:]:"),
        (5, "            lastEnd = output[-1][1]"),
        (
            6,
            "            if start <= lastEnd: output[-1][1] = max(lastEnd, end)",
        ),
        (7, "            else: output.append([start, end])"),
        (8, "        return output"),
    ]
}

pub fn non_overlapping_intervals_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def eraseOverlapIntervals(self, intervals: List[List[int]]) -> int:",
        ),
        (
            3,
            "        intervals.sort(key=lambda x: x[0]); res = 0; prevEnd = intervals[0][1]",
        ),
        (4, "        for start, end in intervals[1:]:"),
        (5, "            if start >= prevEnd: prevEnd = end"),
        (6, "            else: res += 1; prevEnd = min(end, prevEnd)"),
        (7, "        return res"),
    ]
}

pub fn meeting_rooms_ii_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (
            2,
            "    def minMeetingRooms(self, intervals: List[List[int]]) -> int:",
        ),
        (3, "        start = sorted([i[0] for i in intervals])"),
        (4, "        end = sorted([i[1] for i in intervals])"),
        (5, "        res = count = s = e = 0"),
        (6, "        while s < len(intervals):"),
        (7, "            if start[s] < end[e]: s += 1; count += 1"),
        (8, "            else: e += 1; count -= 1"),
        (9, "            res = max(res, count)"),
        (10, "        return res"),
    ]
}

pub fn min_interval_query_code_lines() -> Vec<(usize, &'static str)> {
    vec![
        (1, "class Solution:"),
        (2, "    def minInterval(self, intervals: List[List[int]], queries: List[int]) -> List[int]:"),
        (3, "        intervals.sort(); minHeap = []; res = {}; i = 0"),
        (4, "        for q in sorted(queries):"),
        (5, "            while i < len(intervals) and intervals[i][0] <= q:"),
        (6, "                l, r = intervals[i]"),
        (7, "                heapq.heappush(minHeap, (r - l + 1, r)); i += 1"),
        (8, "            while minHeap and minHeap[0][1] < q: heapq.heappop(minHeap)"),
        (9, "            res[q] = minHeap[0][0] if minHeap else -1"),
        (10, "        return [res[q] for q in queries]"),
    ]
}
