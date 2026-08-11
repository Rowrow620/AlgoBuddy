use super::*;
use std::collections::HashSet;

#[test]
fn recompute_steps_resets_playback_state() {
    let mut app = VisualizerApp::default();
    app.current_problem = Problem::TwoSum;
    app.current_step_idx = 99;
    app.last_focused_step_idx = Some(4);
    app.is_playing = true;

    recompute_steps(&mut app);

    assert_eq!(app.current_step_idx, 0);
    assert_eq!(app.last_focused_step_idx, None);
    assert!(!app.is_playing);
    assert!(!app.steps.is_empty());
}

#[test]
fn empty_numeric_input_keeps_existing_fallback() {
    let mut app = VisualizerApp::default();
    app.current_problem = Problem::ContainsDuplicate;
    app.set_input_str(Problem::ContainsDuplicate, "nums", "");

    recompute_steps(&mut app);

    let last = app.steps.last().expect("fallback must generate steps");
    match &last.visual {
        VisualState::ContainsDuplicate {
            nums,
            duplicate_val,
            has_duplicate,
            ..
        } => {
            assert_eq!(nums, &[1, 2, 3, 1]);
            assert_eq!(*duplicate_val, Some(1));
            assert_eq!(*has_duplicate, Some(true));
        }
        _ => panic!("expected contains-duplicate state"),
    }
}

#[test]
fn approach_selection_rebuilds_the_trace_and_preserves_input() {
    let mut app = VisualizerApp::default();
    app.current_problem = Problem::ContainsDuplicate;
    app.set_input_str(Problem::ContainsDuplicate, "nums", "3, 1, 3");

    recompute_steps(&mut app);
    assert!(app.steps[0].description.contains("HashSet"));

    app.current_step_idx = 2;
    app.last_focused_step_idx = Some(2);
    app.is_playing = true;

    assert!(select_approach(&mut app, 1));
    assert_eq!(app.selected_approach_id, 1);
    assert_eq!(app.current_step_idx, 0);
    assert_eq!(app.last_focused_step_idx, None);
    assert!(!app.is_playing);
    assert_eq!(
        app.get_input_str(Problem::ContainsDuplicate, "nums", ""),
        "3, 1, 3"
    );
    assert!(app.steps[0].description.contains("Sorted array"));
}

#[test]
fn approach_selection_rejects_active_and_unknown_ids() {
    let mut app = VisualizerApp::default();
    app.current_problem = Problem::TwoSum;
    recompute_steps(&mut app);
    app.current_step_idx = 1;
    app.is_playing = true;

    assert!(!select_approach(&mut app, 0));
    assert!(!select_approach(&mut app, usize::MAX));
    assert_eq!(app.selected_approach_id, 0);
    assert_eq!(app.current_step_idx, 1);
    assert!(app.is_playing);
}

#[test]
fn recompute_steps_restores_the_default_for_an_unknown_approach() {
    let mut app = VisualizerApp::default();
    app.current_problem = Problem::ValidAnagram;
    app.selected_approach_id = usize::MAX;

    recompute_steps(&mut app);

    assert_eq!(app.selected_approach_id, 0);
    assert!(!app.steps.is_empty());
}

#[test]
fn selecting_a_new_problem_uses_its_declared_default_approach() {
    let mut app = VisualizerApp::default();
    app.current_problem = Problem::TwoSum;
    assert!(select_approach(&mut app, 1));

    select_problem(&mut app, Problem::ValidAnagram);

    assert_eq!(
        app.selected_approach_id,
        Problem::ValidAnagram.details().default_approach_id()
    );
    assert_eq!(app.current_step_idx, 0);
    assert!(!app.steps.is_empty());
}

#[test]
fn top_k_clamping_keeps_input_state_normalized() {
    let mut app = VisualizerApp::default();
    app.current_problem = Problem::TopKFrequent;
    app.set_input_str(Problem::TopKFrequent, "nums", "1, 1, 2");
    app.set_input_int(Problem::TopKFrequent, "k", 99);

    recompute_steps(&mut app);

    assert_eq!(app.get_input_int(Problem::TopKFrequent, "k", 0), 2);
    assert!(!app.steps.is_empty());
}

#[test]
fn problem_registry_contains_150_distinct_problems() {
    let problems = Problem::all();
    let unique: HashSet<_> = problems.iter().copied().collect();

    assert_eq!(problems.len(), 150);
    assert_eq!(unique.len(), 150);
}

#[test]
fn neetcode_tree_and_bit_category_counts_match() {
    let tree_count = Problem::all()
        .iter()
        .filter(|problem| problem.category() == Category::Trees)
        .count();
    let bit_count = Problem::all()
        .iter()
        .filter(|problem| problem.category() == Category::BitManipulation)
        .count();

    assert_eq!(tree_count, 15);
    assert_eq!(bit_count, 7);
}

#[test]
fn new_bst_visualizers_compute_expected_results() {
    let lca_steps = crate::algorithms::trees::generate_lowest_common_ancestor_bst_steps(
        &[
            Some(6),
            Some(2),
            Some(8),
            Some(0),
            Some(4),
            Some(7),
            Some(9),
            None,
            None,
            Some(3),
            Some(5),
        ],
        2,
        8,
    );
    assert!(lca_steps
        .last()
        .expect("LCA trace must have a result")
        .description
        .contains("lowest common ancestor"));
    match &lca_steps
        .last()
        .expect("LCA trace must have a result")
        .visual
    {
        VisualState::TreeVisual { depth_val, .. } => assert_eq!(*depth_val, Some(1)),
        _ => panic!("expected tree visualization"),
    }

    let valid_steps =
        crate::algorithms::trees::generate_validate_bst_steps(&[Some(2), Some(1), Some(3)]);
    assert!(valid_steps
        .last()
        .expect("validation trace must have a result")
        .description
        .ends_with("true."));
    match &valid_steps
        .last()
        .expect("validation trace must have a result")
        .visual
    {
        VisualState::TreeVisual { depth_val, .. } => assert_eq!(*depth_val, Some(2)),
        _ => panic!("expected tree visualization"),
    }

    let invalid_steps = crate::algorithms::trees::generate_validate_bst_steps(&[
        Some(5),
        Some(1),
        Some(4),
        None,
        None,
        Some(3),
        Some(6),
    ]);
    assert!(invalid_steps
        .last()
        .expect("validation trace must have a result")
        .description
        .ends_with("false."));
}

#[test]
fn kth_smallest_default_example_has_complete_inorder_trace() {
    let steps = crate::algorithms::trees::generate_kth_smallest_bst_steps(
        &[Some(3), Some(1), Some(4), None, Some(2)],
        1,
    );

    assert_eq!(steps.len(), 5);
    assert_eq!(
        steps.iter().map(|step| step.code_line).collect::<Vec<_>>(),
        vec![3, 7, 7, 8, 10]
    );
    assert!(steps[1].description.contains("Stack: [3]"));
    assert!(steps[2].description.contains("Stack: [3, 1]"));
    assert!(steps[3].description.contains("Backtrack to 1"));
    assert!(steps[4].description.contains("1st smallest element"));
    match &steps[4].visual {
        VisualState::TreeVisual {
            active_node_idx,
            depth_val,
            ..
        } => {
            assert_eq!(*active_node_idx, Some(1));
            assert_eq!(*depth_val, Some(2));
        }
        _ => panic!("expected tree visualization"),
    }
}
