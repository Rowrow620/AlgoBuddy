use super::*;
use std::collections::{HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq)]
struct SourceLineMismatch {
    problem: Problem,
    problem_id: u32,
    problem_title: &'static str,
    approach_id: usize,
    approach_name: &'static str,
    step_number: usize,
    referenced_line: usize,
    available_lines: Vec<usize>,
}

#[test]
fn problem_catalog_has_complete_metadata_and_valid_traces() {
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

    // These sets are a ratchet for pre-existing validation debt. New entries fail CI;
    // resolved entries also fail until they are deliberately removed here.
    let known_placeholder_traces = HashSet::<Problem>::new();
    let known_line_mismatches = HashSet::new();
    let known_visual_state_debt = HashSet::<Problem>::new();
    let mut placeholder_traces = HashSet::new();
    let mut line_mismatches = HashSet::new();
    let mut line_mismatch_details = Vec::new();
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
                    line_mismatch_details.push(SourceLineMismatch {
                        problem,
                        problem_id: details.id,
                        problem_title: details.title,
                        approach_id: approach.id,
                        approach_name: approach.name,
                        step_number: step_idx + 1,
                        referenced_line: step.code_line,
                        available_lines: code_lines.iter().map(|(line, _)| *line).collect(),
                    });
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
    let line_mismatch_debt_changed = line_mismatches != known_line_mismatches;
    check_debt_set(
        "source-line mismatches",
        &known_line_mismatches,
        &line_mismatches,
        &mut failures,
    );
    if line_mismatch_debt_changed {
        failures.push(format_source_line_mismatch_report(&line_mismatch_details));
    }
    check_debt_set(
        "visual-state violations",
        &known_visual_state_debt,
        &visual_state_debt,
        &mut failures,
    );

    assert_catalog_clean(failures);
}

#[test]
#[ignore = "diagnostic report; run with --ignored --nocapture"]
fn source_line_mismatch_report() {
    let mut mismatches = Vec::new();

    for &problem in Problem::all() {
        let details = problem.details();
        for approach in details.approaches {
            let code_lines = approach_code_lines(problem, approach.id);
            let available_lines: Vec<_> = code_lines.iter().map(|(line, _)| *line).collect();

            let mut app = VisualizerApp::default();
            app.current_problem = problem;
            app.selected_approach_id = approach.id;
            recompute_steps(&mut app);

            for (step_idx, step) in app.steps.iter().enumerate() {
                if !available_lines.contains(&step.code_line) {
                    mismatches.push(SourceLineMismatch {
                        problem,
                        problem_id: details.id,
                        problem_title: details.title,
                        approach_id: approach.id,
                        approach_name: approach.name,
                        step_number: step_idx + 1,
                        referenced_line: step.code_line,
                        available_lines: available_lines.clone(),
                    });
                }
            }
        }
    }

    println!("{}", format_source_line_mismatch_report(&mismatches));
}

fn format_source_line_mismatch_report(mismatches: &[SourceLineMismatch]) -> String {
    if mismatches.is_empty() {
        return "Source-line mismatch report: no mismatches found.".to_owned();
    }

    let affected_problems: HashSet<_> =
        mismatches.iter().map(|mismatch| mismatch.problem).collect();
    let details = mismatches
        .iter()
        .map(|mismatch| {
            format!(
                "- {:?} (#{} {}) | approach {} ({}) | step {} | referenced line {} | available lines {:?}",
                mismatch.problem,
                mismatch.problem_id,
                mismatch.problem_title,
                mismatch.approach_id,
                mismatch.approach_name,
                mismatch.step_number,
                mismatch.referenced_line,
                mismatch.available_lines,
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "Source-line mismatch report: {} mismatched step(s) across {} problem(s):\n{details}",
        mismatches.len(),
        affected_problems.len(),
    )
}

#[test]
fn source_line_mismatch_report_includes_repair_context() {
    let report = format_source_line_mismatch_report(&[SourceLineMismatch {
        problem: Problem::BinarySearch,
        problem_id: 704,
        problem_title: "Binary Search",
        approach_id: 2,
        approach_name: "Iterative Binary Search",
        step_number: 3,
        referenced_line: 8,
        available_lines: vec![1, 2, 3, 5],
    }]);

    assert_eq!(
        report,
        "Source-line mismatch report: 1 mismatched step(s) across 1 problem(s):\n\
- BinarySearch (#704 Binary Search) | approach 2 (Iterative Binary Search) | step 3 | referenced line 8 | available lines [1, 2, 3, 5]"
    );
}

fn compact_level_order_max_depth(tree_nodes: &[Option<i32>]) -> i32 {
    if tree_nodes.first().copied().flatten().is_none() {
        return 0;
    }

    let mut pending_depths = VecDeque::from([1]);
    let mut maximum_depth = 0;
    for node in tree_nodes {
        let Some(depth) = pending_depths.pop_front() else {
            break;
        };
        if node.is_some() {
            maximum_depth = maximum_depth.max(depth);
            pending_depths.push_back(depth + 1);
            pending_depths.push_back(depth + 1);
        }
    }
    maximum_depth
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
                let maximum_depth = compact_level_order_max_depth(tree_nodes);
                if *depth < 0 || *depth > maximum_depth {
                    failures.push(format!(
                        "{context}: displayed depth {depth} is outside 0..={maximum_depth}"
                    ));
                }
            }
        }
        VisualState::TreeMaxPathVisual {
            tree_nodes,
            active_node_idx,
            secondary_node_idx,
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

fn assert_catalog_clean(failures: Vec<String>) {
    if !failures.is_empty() {
        panic!(
            "global problem catalog validation found {} violation(s):\n{}",
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
