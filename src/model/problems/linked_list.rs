use crate::model::problem::{ApproachMeta, Example, Problem, ProblemDetails};
use crate::model::taxonomy::{Category, Difficulty};

pub fn get_details(problem: Problem) -> Option<ProblemDetails> {
    match problem {
        Problem::ReverseLinkedList => Some(ProblemDetails {
                id: 206, title: "Reverse Linked List", difficulty: Difficulty::Easy, category: Category::LinkedList,
                statement: "Reverse a singly linked list.",
                examples: &[Example { input: "head = [0, 1, 2, 3]", output: "[3, 2, 1, 0]", explanation: "Next pointers flipped." }],
                constraints: &["0 <= length <= 1000"], leetcode_url: "https://leetcode.com/problems/reverse-linked-list/",
                approaches: &[ApproachMeta { id: 0, name: "Iterative Pointers (prev, curr)", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Reversing link pointers iteratively requires only 3 pointer variables (prev, curr, nxt), achieving O(N) time and O(1) space.", description: "Flip next pointers." }],
            }),
        Problem::MergeTwoLists => Some(ProblemDetails {
                id: 21, title: "Merge Two Sorted Linked Lists", difficulty: Difficulty::Easy, category: Category::LinkedList,
                statement: "Merge two sorted linked lists into one sorted list.",
                examples: &[Example { input: "list1 = [1, 2, 4], list2 = [1, 3, 5]", output: "[1, 1, 2, 3, 4, 5]", explanation: "Merged in order." }],
                constraints: &["0 <= list1.length <= 100"], leetcode_url: "https://leetcode.com/problems/merge-two-sorted-lists/",
                approaches: &[ApproachMeta { id: 0, name: "Two Pointers Merge", time_complexity: "O(N + M)", space_complexity: "O(1)", rationale: "Splicing existing list nodes together using two pointers merges sorted lists in O(N + M) time with zero extra allocations.", description: "Tail node attachments." }],
            }),
        Problem::LinkedListCycle => Some(ProblemDetails {
                id: 141, title: "Linked List Cycle Detection", difficulty: Difficulty::Easy, category: Category::LinkedList,
                statement: "Return true if there is a cycle in the linked list.",
                examples: &[Example { input: "head = [1, 2, 3, 4], index = 1", output: "true", explanation: "Tail connects to index 1." }],
                constraints: &["0 <= length <= 1000"], leetcode_url: "https://leetcode.com/problems/linked-list-cycle/",
                approaches: &[ApproachMeta { id: 0, name: "Floyd's Tortoise & Hare", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Floyd's fast pointer moves at 2x speed; if a cycle exists, the distance between slow and fast decreases by 1 each step (O(N) catch-up).", description: "Slow and fast pointers." }],
            }),
        Problem::ReorderList => Some(ProblemDetails {
                id: 143, title: "Reorder List", difficulty: Difficulty::Medium, category: Category::LinkedList,
                statement: "Reorder the linked list to be in the order: L0 -> Ln -> L1 -> Ln-1 -> L2 -> Ln-2 -> ...",
                examples: &[Example { input: "head = [1,2,3,4]", output: "[1,4,2,3]", explanation: "Nodes alternate from front and back." }],
                constraints: &["1 <= n <= 5 * 10^4"], leetcode_url: "https://leetcode.com/problems/reorder-list/",
                approaches: &[ApproachMeta { id: 0, name: "Fast/Slow + Reverse Second Half + Merge", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Split list in half, reverse second half, and interleave nodes in-place.", description: "Split, reverse second half, interleave." }],
            }),
        Problem::RemoveNthNodeFromEnd => Some(ProblemDetails {
                id: 19, title: "Remove Nth Node From End of List", difficulty: Difficulty::Medium, category: Category::LinkedList,
                statement: "Given the head of a linked list, remove the nth node from the end of the list and return its head.",
                examples: &[Example { input: "head = [1,2,3,4,5], n = 2", output: "[1,2,3,5]", explanation: "The 2nd node from end (4) is removed." }],
                constraints: &["1 <= sz <= 30", "1 <= n <= sz"], leetcode_url: "https://leetcode.com/problems/remove-nth-node-from-end-of-list/",
                approaches: &[ApproachMeta { id: 0, name: "Two Pointers (Fast & Slow Gap)", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Maintain an n-node gap between fast and slow pointers; when fast hits end, slow is right before target.", description: "Fast/slow pointer gap traversal." }],
            }),
        Problem::CopyListWithRandomPointer => Some(ProblemDetails {
                id: 138, title: "Copy List with Random Pointer", difficulty: Difficulty::Medium, category: Category::LinkedList,
                statement: "Construct a deep copy of a linked list where each node contains an additional random pointer.",
                examples: &[Example { input: "head = [[7,null],[13,0],[11,4],[10,2],[1,0]]", output: "Deep copy list", explanation: "Original nodes cloned with random pointer references." }],
                constraints: &["0 <= n <= 1000"], leetcode_url: "https://leetcode.com/problems/copy-list-with-random-pointer/",
                approaches: &[ApproachMeta { id: 0, name: "Hash Map / Interleaved Node Duplication", time_complexity: "O(N)", space_complexity: "O(N)", rationale: "Map old nodes to new cloned nodes to resolve random pointers in two passes.", description: "Old->New pointer mapping." }],
            }),
        Problem::AddTwoNumbers => Some(ProblemDetails {
                id: 2, title: "Add Two Numbers", difficulty: Difficulty::Medium, category: Category::LinkedList,
                statement: "Add two numbers represented as linked lists in reverse digit order and return the sum as a linked list.",
                examples: &[Example { input: "l1 = [2,4,3], l2 = [5,6,4]", output: "[7,0,8]", explanation: "342 + 465 = 807 (stored in reverse: 7->0->8)." }],
                constraints: &["1 <= n <= 100"], leetcode_url: "https://leetcode.com/problems/add-two-numbers/",
                approaches: &[ApproachMeta { id: 0, name: "Simultaneous Iteration with Carry", time_complexity: "O(max(N, M))", space_complexity: "O(max(N, M))", rationale: "Iterate digit nodes alongside carry variable to construct sum list.", description: "Single-pass digit sum with carry." }],
            }),
        Problem::FindDuplicateNumber => Some(ProblemDetails {
                id: 287, title: "Find the Duplicate Number", difficulty: Difficulty::Medium, category: Category::LinkedList,
                statement: "Given an array of integers nums containing n + 1 integers where each integer is in the range [1, n] inclusive, find the duplicate number without modifying array in O(1) extra space.",
                examples: &[Example { input: "nums = [1,3,4,2,2]", output: "2", explanation: "Number 2 appears twice." }],
                constraints: &["1 <= n <= 10^5"], leetcode_url: "https://leetcode.com/problems/find-the-duplicate-number/",
                approaches: &[ApproachMeta { id: 0, name: "Floyd's Tortoise and Hare (Cycle Detection)", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Treat array values as next pointers; duplicate forms cycle entrance.", description: "Cycle detection fast/slow pointer." }],
            }),
        Problem::LruCache => Some(ProblemDetails {
                id: 146, title: "LRU Cache", difficulty: Difficulty::Medium, category: Category::LinkedList,
                statement: "Design a data structure that follows the constraints of a Least Recently Used (LRU) cache with O(1) get and put operations.",
                examples: &[Example { input: "put(1,1), put(2,2), get(1), put(3,3), get(2)", output: "[null, null, 1, null, -1]", explanation: "Key 2 evicted when key 3 is added." }],
                constraints: &["1 <= capacity <= 3000"], leetcode_url: "https://leetcode.com/problems/lru-cache/",
                approaches: &[ApproachMeta { id: 0, name: "Doubly Linked List + Hash Map", time_complexity: "O(1)", space_complexity: "O(capacity)", rationale: "Hash map provides O(1) key lookup while Doubly Linked List maintains recency ordering.", description: "DLL for recency + Hash map for lookup." }],
            }),
        Problem::MergeKSortedLists => Some(ProblemDetails {
                id: 23, title: "Merge k Sorted Lists", difficulty: Difficulty::Hard, category: Category::LinkedList,
                statement: "You are given an array of k linked-lists lists, each linked-list is sorted in ascending order. Merge all the linked-lists into one sorted linked-list and return it.",
                examples: &[Example { input: "lists = [[1,4,5],[1,3,4],[2,6]]", output: "[1,1,2,3,4,4,5,6]", explanation: "Merged into a single sorted list." }],
                constraints: &["0 <= k <= 10^4"], leetcode_url: "https://leetcode.com/problems/merge-k-sorted-lists/",
                approaches: &[ApproachMeta { id: 0, name: "Min-Heap / Divide and Conquer", time_complexity: "O(N log K)", space_complexity: "O(K)", rationale: "Min-Heap tracks current minimum node head across all K lists.", description: "Priority queue min-heap extraction." }],
            }),
        Problem::ReverseNodesInKGroup => Some(ProblemDetails {
                id: 25, title: "Reverse Nodes in k-Group", difficulty: Difficulty::Hard, category: Category::LinkedList,
                statement: "Given the head of a linked list, reverse the nodes of a list k at a time, and return its modified list.",
                examples: &[Example { input: "head = [1,2,3,4,5], k = 2", output: "[2,1,4,3,5]", explanation: "Nodes reversed in pairs." }],
                constraints: &["1 <= k <= n <= 5000"], leetcode_url: "https://leetcode.com/problems/reverse-nodes-in-k-group/",
                approaches: &[ApproachMeta { id: 0, name: "K-Node Pointer Reversal", time_complexity: "O(N)", space_complexity: "O(1)", rationale: "Check if k nodes exist, reverse subsegment, and reconnect pointers.", description: "In-place k-group node reversal." }],
            }),
        _ => None,
    }
}

pub fn get_code_lines(problem: Problem, approach_id: usize) -> Option<Vec<(usize, &'static str)>> {
    match (problem, approach_id) {
        (Problem::ReverseLinkedList, _) => Some(vec![
            (1, "class Solution:"),
            (2, "    def reverseList(self, head: Optional[ListNode]) -> Optional[ListNode]:"),
            (3, "        prev, curr = None, head"),
            (4, "        while curr:"),
            (5, "            nxt = curr.next; curr.next = prev; prev = curr; curr = nxt"),
            (6, "        return prev"),
        ]),
        (Problem::MergeTwoLists, _) => Some(vec![
            (1, "class Solution:"),
            (2, "    def mergeTwoLists(self, list1: Optional[ListNode], list2: Optional[ListNode]) -> Optional[ListNode]:"),
            (3, "        dummy = tail = ListNode()"),
            (4, "        while list1 and list2:"),
            (5, "            if list1.val < list2.val: tail.next = list1; list1 = list1.next"),
            (6, "            else: tail.next = list2; list2 = list2.next"),
            (7, "            tail = tail.next"),
            (8, "        tail.next = list1 if list1 else list2"),
            (9, "        return dummy.next"),
        ]),
        (Problem::LinkedListCycle, _) => Some(vec![
            (1, "class Solution:"),
            (2, "    def hasCycle(self, head: Optional[ListNode]) -> bool:"),
            (3, "        slow, fast = head, head"),
            (4, "        while fast and fast.next:"),
            (5, "            slow = slow.next; fast = fast.next.next"),
            (6, "            if slow == fast: return True"),
            (7, "        return False"),
        ]),
        _ => None,
    }
}
