use super::*;
use std::collections::HashSet;

#[test]
fn audited_problem_catalog_has_complete_metadata_and_valid_traces() {
    let expected_category_counts = [
        (Category::ArraysAndHashing, 9),
        (Category::TwoPointers, 5),
        (Category::SlidingWindow, 6),
        (Category::Stack, 7),
        (Category::BinarySearch, 7),
        (Category::LinkedList, 11),
        (Category::Trees, 15),
        (Category::Tries, 3),
        (Category::HeapPriorityQueue, 7),
        (Category::Backtracking, 9),
        (Category::Graphs, 13),
        (Category::AdvancedGraphs, 6),
        (Category::OneDDp, 12),
        (Category::TwoDDp, 11),
        (Category::Greedy, 8),
        (Category::Intervals, 6),
        (Category::MathAndGeometry, 8),
        (Category::BitManipulation, 7),
    ];
    let mut failures = Vec::new();
    let mut problem_ids = HashSet::new();

    // These sets are a ratchet for pre-existing audit debt. New entries fail CI;
    // resolved entries also fail until they are deliberately removed here.
    let known_placeholder_traces: HashSet<_> = [
        Problem::ReorderList,
        Problem::RemoveNthNodeFromEnd,
        Problem::CopyListWithRandomPointer,
        Problem::AddTwoNumbers,
        Problem::FindDuplicateNumber,
        Problem::LruCache,
        Problem::MergeKSortedLists,
        Problem::ReverseNodesInKGroup,
        Problem::BinaryTreeLevelOrderTraversal,
        Problem::BinaryTreeRightSideView,
        Problem::CountGoodNodes,
        Problem::ConstructBinaryTreePreorderInorder,
        Problem::BinaryTreeMaxPathSum,
        Problem::SerializeDeserializeBinaryTree,
    ]
    .into_iter()
    .collect();
    let known_line_mismatches: HashSet<_> = [
        Problem::ValidAnagram,
        Problem::GroupAnagrams,
        Problem::ValidSudoku,
        Problem::LongestConsecutive,
        Problem::ValidPalindrome,
        Problem::BestTimeStock,
        Problem::ValidParentheses,
        Problem::BinarySearch,
        Problem::ReverseLinkedList,
        Problem::MergeTwoLists,
        Problem::LinkedListCycle,
        Problem::InvertTree,
        Problem::MaxDepthTree,
        Problem::DiameterTree,
        Problem::BalancedTree,
        Problem::SameTree,
        Problem::Subtree,
        Problem::ClimbingStairs,
        Problem::MinCostStairs,
        Problem::HouseRobberII,
        Problem::DecodeWays,
        Problem::MaxProductSubarray,
        Problem::LastStone,
        Problem::MeetingRooms,
        Problem::HappyNumber,
        Problem::SingleNumber,
        Problem::CountingBits,
        Problem::ReverseBits,
        Problem::MissingNumber,
        Problem::RotateImage,
        Problem::SpiralMatrix,
        Problem::MaxAreaIsland,
        Problem::EvalRPN,
        Problem::GenerateParentheses,
        Problem::DailyTemperatures,
        Problem::CarFleet,
        Problem::LargestRectangle,
        Problem::FindMinRotated,
        Problem::ReorderList,
        Problem::RemoveNthNodeFromEnd,
        Problem::CopyListWithRandomPointer,
        Problem::AddTwoNumbers,
        Problem::FindDuplicateNumber,
        Problem::LruCache,
        Problem::ReverseNodesInKGroup,
        Problem::BinaryTreeLevelOrderTraversal,
        Problem::BinaryTreeRightSideView,
        Problem::CountGoodNodes,
        Problem::SerializeDeserializeBinaryTree,
    ]
    .into_iter()
    .collect();
    let known_visual_state_debt: HashSet<_> = [
        Problem::MinStack,
        Problem::GenerateParentheses,
        Problem::BinaryTreeMaxPathSum,
    ]
    .into_iter()
    .collect();
    let mut placeholder_traces = HashSet::new();
    let mut line_mismatches = HashSet::new();
    let mut visual_state_debt = HashSet::new();

    for (category, expected) in expected_category_counts {
        let actual = Problem::all()
            .iter()
            .filter(|problem| problem.category() == category)
            .count();
        if actual != expected {
            failures.push(format!(
                "{}: expected {expected} problems, found {actual}",
                category.name()
            ));
        }
    }

    for &problem in Problem::all() {
        let details = problem.details();
        let context = format!("{problem:?} (#{} {})", details.id, details.title);

        if details.id == 0 {
            failures.push(format!("{context}: LeetCode ID must be nonzero"));
        }
        if !problem_ids.insert(details.id) {
            failures.push(format!("{context}: duplicate LeetCode ID {}", details.id));
        }
        for (field, value) in [
            ("title", details.title),
            ("statement", details.statement),
            ("LeetCode URL", details.leetcode_url),
        ] {
            if value.trim().is_empty() {
                failures.push(format!("{context}: {field} is empty"));
            }
        }
        if !details.leetcode_url.starts_with("https://leetcode.com/") {
            failures.push(format!("{context}: invalid LeetCode URL"));
        }
        if details.examples.is_empty() {
            failures.push(format!("{context}: no examples"));
        }
        for (index, example) in details.examples.iter().enumerate() {
            if example.input.trim().is_empty()
                || example.output.trim().is_empty()
                || example.explanation.trim().is_empty()
            {
                failures.push(format!(
                    "{context}: example {} has an empty field",
                    index + 1
                ));
            }
        }
        if details.constraints.is_empty() {
            failures.push(format!("{context}: no constraints"));
        }
        if details
            .constraints
            .iter()
            .any(|constraint| constraint.trim().is_empty())
        {
            failures.push(format!("{context}: contains an empty constraint"));
        }
        if details.approaches.is_empty() {
            failures.push(format!("{context}: no approaches"));
            continue;
        }

        let mut approach_ids = HashSet::new();
        for approach in details.approaches {
            if !approach_ids.insert(approach.id) {
                failures.push(format!("{context}: duplicate approach ID {}", approach.id));
            }
            for (field, value) in [
                ("approach name", approach.name),
                ("time complexity", approach.time_complexity),
                ("space complexity", approach.space_complexity),
                ("rationale", approach.rationale),
                ("approach description", approach.description),
            ] {
                if value.trim().is_empty() {
                    failures.push(format!(
                        "{context}, approach {}: {field} is empty",
                        approach.id
                    ));
                }
            }

            let code_lines = approach_code_lines(problem, approach.id);
            let placeholder = code_lines.len() == 1
                && code_lines[0].1.trim() == "# Approach implementation trace";
            if placeholder {
                placeholder_traces.insert(problem);
            }

            let mut displayed_lines = HashSet::new();
            for &(line_number, _) in &code_lines {
                if line_number == 0 {
                    failures.push(format!(
                        "{context}, approach {}: code line number is zero",
                        approach.id
                    ));
                }
                if !displayed_lines.insert(line_number) {
                    failures.push(format!(
                        "{context}, approach {}: duplicate code line {line_number}",
                        approach.id
                    ));
                }
            }
            if code_lines.windows(2).any(|pair| pair[0].0 >= pair[1].0) {
                failures.push(format!(
                    "{context}, approach {}: code lines are not strictly ordered",
                    approach.id
                ));
            }

            let mut app = VisualizerApp::default();
            app.current_problem = problem;
            app.selected_approach_id = approach.id;
            recompute_steps(&mut app);
            if app.steps.is_empty() {
                failures.push(format!(
                    "{context}, approach {}: generated no steps",
                    approach.id
                ));
            }

            for (step_idx, step) in app.steps.iter().enumerate() {
                let step_context =
                    format!("{context}, approach {}, step {}", approach.id, step_idx + 1);
                if step.description.trim().is_empty() {
                    failures.push(format!("{step_context}: empty description"));
                }
                if !displayed_lines.contains(&step.code_line) {
                    line_mismatches.insert(problem);
                }
                let mut step_visual_failures = Vec::new();
                validate_visual_state(&step.visual, &step_context, &mut step_visual_failures);
                if !step_visual_failures.is_empty() {
                    visual_state_debt.insert(problem);
                }
            }
        }
    }

    check_debt_set(
        "placeholder traces",
        &known_placeholder_traces,
        &placeholder_traces,
        &mut failures,
    );
    check_debt_set(
        "source-line mismatches",
        &known_line_mismatches,
        &line_mismatches,
        &mut failures,
    );
    check_debt_set(
        "visual-state violations",
        &known_visual_state_debt,
        &visual_state_debt,
        &mut failures,
    );

    assert_audit_clean(failures);
}

fn validate_visual_state(visual: &VisualState, context: &str, failures: &mut Vec<String>) {
    match visual {
        VisualState::ContainsDuplicate {
            nums, active_idx, ..
        } => check_index("active index", *active_idx, nums.len(), context, failures),
        VisualState::GroupAnagrams {
            input_strs,
            active_idx,
            ..
        } => check_index(
            "active string index",
            *active_idx,
            input_strs.len(),
            context,
            failures,
        ),
        VisualState::TopK {
            nums,
            active_nums_idx,
            buckets,
            active_bucket_idx,
            ..
        } => {
            check_index(
                "active number index",
                *active_nums_idx,
                nums.len(),
                context,
                failures,
            );
            check_index(
                "active bucket index",
                *active_bucket_idx,
                buckets.len(),
                context,
                failures,
            );
        }
        VisualState::EncodeDecode {
            input_strs,
            active_str_idx,
            ..
        } => check_index(
            "active string index",
            *active_str_idx,
            input_strs.len(),
            context,
            failures,
        ),
        VisualState::Product {
            nums,
            output,
            active_idx,
            ..
        } => {
            check_index("active index", *active_idx, nums.len(), context, failures);
            if output.len() != nums.len() {
                failures.push(format!(
                    "{context}: output length {} does not match input length {}",
                    output.len(),
                    nums.len()
                ));
            }
        }
        VisualState::Trie {
            current_word,
            active_char_idx,
            ..
        } => check_index(
            "active character index",
            *active_char_idx,
            current_word.chars().count(),
            context,
            failures,
        ),
        VisualState::ValidSudoku {
            active_r,
            active_c,
            duplicate_pos,
            ..
        } => {
            check_index("active row", *active_r, 9, context, failures);
            check_index("active column", *active_c, 9, context, failures);
            if let Some((row, col)) = duplicate_pos {
                if *row >= 9 || *col >= 9 {
                    failures.push(format!(
                        "{context}: duplicate cell ({row}, {col}) is outside the board"
                    ));
                }
            }
        }
        VisualState::LongestConsecutive { .. } | VisualState::DecisionTreeVisual { .. } => {}
        VisualState::TwoSum {
            nums,
            active_idx,
            secondary_idx,
            found_indices,
            ..
        } => {
            check_index("active index", *active_idx, nums.len(), context, failures);
            check_index(
                "secondary index",
                *secondary_idx,
                nums.len(),
                context,
                failures,
            );
            if let Some((left, right)) = found_indices {
                check_required_index("result index", *left, nums.len(), context, failures);
                check_required_index("result index", *right, nums.len(), context, failures);
            }
        }
        VisualState::ValidAnagram {
            s,
            t,
            active_s_idx,
            active_t_idx,
            ..
        } => {
            check_index(
                "active s index",
                *active_s_idx,
                s.chars().count(),
                context,
                failures,
            );
            check_index(
                "active t index",
                *active_t_idx,
                t.chars().count(),
                context,
                failures,
            );
        }
        VisualState::TwoPointers {
            chars, left, right, ..
        } => {
            check_boundary("left pointer", *left, chars.len(), context, failures);
            check_boundary("right pointer", *right, chars.len(), context, failures);
        }
        VisualState::Stack {
            chars, active_idx, ..
        } => check_index("active index", *active_idx, chars.len(), context, failures),
        VisualState::BestTimeStock {
            prices,
            left_buy,
            right_sell,
            ..
        } => {
            check_required_index("buy index", *left_buy, prices.len(), context, failures);
            check_required_index("sell index", *right_sell, prices.len(), context, failures);
        }
        VisualState::BinarySearch {
            nums,
            left,
            right,
            mid,
            found_idx,
            ..
        } => {
            check_boundary("left bound", *left, nums.len(), context, failures);
            check_boundary("right bound", *right, nums.len(), context, failures);
            check_index("middle index", *mid, nums.len(), context, failures);
            check_index("result index", *found_idx, nums.len(), context, failures);
        }
        VisualState::LinkedList {
            nodes,
            prev_idx,
            curr_idx,
            next_idx,
            ..
        } => {
            check_index("previous index", *prev_idx, nodes.len(), context, failures);
            check_index("current index", *curr_idx, nodes.len(), context, failures);
            check_index("next index", *next_idx, nodes.len(), context, failures);
        }
        VisualState::MergeLinkedLists {
            list1,
            list2,
            p1_idx,
            p2_idx,
            ..
        } => {
            check_index("list 1 index", *p1_idx, list1.len(), context, failures);
            check_index("list 2 index", *p2_idx, list2.len(), context, failures);
        }
        VisualState::LinkedListCycle {
            nodes,
            cycle_target_idx,
            slow_idx,
            fast_idx,
            ..
        } => {
            check_index(
                "cycle target index",
                *cycle_target_idx,
                nodes.len(),
                context,
                failures,
            );
            check_index("slow index", *slow_idx, nodes.len(), context, failures);
            check_index("fast index", *fast_idx, nodes.len(), context, failures);
        }
        VisualState::TreeVisual {
            tree_nodes,
            active_node_idx,
            secondary_node_idx,
            depth_val,
            ..
        } => {
            check_index(
                "active node index",
                *active_node_idx,
                tree_nodes.len(),
                context,
                failures,
            );
            check_index(
                "secondary node index",
                *secondary_node_idx,
                tree_nodes.len(),
                context,
                failures,
            );
            if let Some(depth) = depth_val {
                let maximum_depth = if tree_nodes.is_empty() {
                    0
                } else {
                    (usize::BITS - tree_nodes.len().leading_zeros()) as i32
                };
                if *depth < 0 || *depth > maximum_depth {
                    failures.push(format!(
                        "{context}: displayed depth {depth} is outside 0..={maximum_depth}"
                    ));
                }
            }
        }
        VisualState::HeapVisual {
            heap_elements,
            active_idx,
            swapped_pair,
            ..
        } => {
            check_index(
                "active heap index",
                *active_idx,
                heap_elements.len(),
                context,
                failures,
            );
            if let Some((left, right)) = swapped_pair {
                check_required_index("swap index", *left, heap_elements.len(), context, failures);
                check_required_index("swap index", *right, heap_elements.len(), context, failures);
            }
        }
        VisualState::GridGraph {
            rows,
            cols,
            grid,
            active_cell,
            visited_cells,
            frontier_cells,
            ..
        } => {
            if grid.len() != *rows || grid.iter().any(|row| row.len() != *cols) {
                failures.push(format!(
                    "{context}: declared grid dimensions {rows}x{cols} do not match its data"
                ));
            }
            for &(row, col) in active_cell
                .iter()
                .chain(visited_cells.iter())
                .chain(frontier_cells.iter())
            {
                if row >= *rows || col >= *cols {
                    failures.push(format!(
                        "{context}: grid cell ({row}, {col}) is out of bounds"
                    ));
                }
            }
        }
        VisualState::NodeGraph {
            nodes,
            node_labels,
            edges,
            active_node,
            active_edge,
            visited_nodes,
            cycle_edges,
            topo_order,
            ..
        } => {
            if node_labels.len() != nodes.len() {
                failures.push(format!(
                    "{context}: {} graph nodes have {} labels",
                    nodes.len(),
                    node_labels.len()
                ));
            }
            let valid_nodes: HashSet<_> = nodes.iter().copied().collect();
            for node in active_node
                .iter()
                .chain(visited_nodes.iter())
                .chain(topo_order.iter())
            {
                if !valid_nodes.contains(node) {
                    failures.push(format!("{context}: graph references missing node {node}"));
                }
            }
            for &(from, to) in edges
                .iter()
                .chain(active_edge.iter())
                .chain(cycle_edges.iter())
            {
                if !valid_nodes.contains(&from) || !valid_nodes.contains(&to) {
                    failures.push(format!(
                        "{context}: graph edge {from} -> {to} references a missing node"
                    ));
                }
            }
        }
        VisualState::Array1D {
            elements,
            active_idx,
            secondary_idx,
            ..
        } => {
            check_index(
                "active index",
                *active_idx,
                elements.len(),
                context,
                failures,
            );
            check_index(
                "secondary index",
                *secondary_idx,
                elements.len(),
                context,
                failures,
            );
        }
    }
}

fn check_index(
    label: &str,
    index: Option<usize>,
    len: usize,
    context: &str,
    failures: &mut Vec<String>,
) {
    if let Some(index) = index {
        check_required_index(label, index, len, context, failures);
    }
}

fn check_required_index(
    label: &str,
    index: usize,
    len: usize,
    context: &str,
    failures: &mut Vec<String>,
) {
    if index >= len {
        failures.push(format!(
            "{context}: {label} {index} is out of bounds for length {len}"
        ));
    }
}

fn check_boundary(
    label: &str,
    index: usize,
    len: usize,
    context: &str,
    failures: &mut Vec<String>,
) {
    if index > len {
        failures.push(format!("{context}: {label} {index} exceeds boundary {len}"));
    }
}

fn assert_audit_clean(failures: Vec<String>) {
    if !failures.is_empty() {
        panic!(
            "global problem audit found {} violation(s):\n{}",
            failures.len(),
            failures.join("\n")
        );
    }
}

fn check_debt_set(
    label: &str,
    expected: &HashSet<Problem>,
    actual: &HashSet<Problem>,
    failures: &mut Vec<String>,
) {
    if actual != expected {
        let mut added: Vec<_> = actual
            .difference(expected)
            .map(|p| format!("{p:?}"))
            .collect();
        let mut resolved: Vec<_> = expected
            .difference(actual)
            .map(|p| format!("{p:?}"))
            .collect();
        added.sort();
        resolved.sort();
        failures.push(format!(
            "{label} changed; new: [{}], resolved (remove from baseline): [{}]",
            added.join(", "),
            resolved.join(", ")
        ));
    }
}
