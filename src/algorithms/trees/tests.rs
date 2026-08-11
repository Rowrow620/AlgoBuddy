use super::{generate_construct_tree_pre_in_steps, generate_tree_max_path_sum_steps};
use crate::model::{Step, VisualState};

fn final_constructed_tree(steps: &[Step]) -> &[Option<i32>] {
    match &steps.last().expect("construction trace must finish").visual {
        VisualState::TreeVisual { tree_nodes, .. } => tree_nodes,
        _ => panic!("expected tree visualization"),
    }
}

#[test]
fn construct_tree_uses_both_traversals_and_builds_expected_shape() {
    let preorder = [3, 9, 20, 15, 7];
    let inorder = [9, 3, 15, 20, 7];
    let steps = generate_construct_tree_pre_in_steps(&preorder, &inorder);

    assert_eq!(
        final_constructed_tree(&steps),
        [Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]
    );
    assert_eq!(steps.len(), 17);
    assert_eq!(steps.first().expect("initial step").code_line, 3);
    assert_eq!(steps.last().expect("final step").code_line, 15);
    assert!(steps
        .iter()
        .any(|step| step.description.contains("Left partition = [15]")));

    for node_steps in steps[1..steps.len() - 1].chunks_exact(3) {
        assert_eq!(
            node_steps
                .iter()
                .map(|step| step.code_line)
                .collect::<Vec<_>>(),
            [8, 10, 11]
        );
        let VisualState::TreeVisual {
            tree_nodes: before_nodes,
            active_node_idx: before_active,
            depth_val: before_depth,
            ..
        } = &node_steps[0].visual
        else {
            panic!("expected pre-construction tree state");
        };
        assert_eq!(*before_active, None);
        assert_eq!(*before_depth, None);

        let VisualState::TreeVisual {
            tree_nodes: after_nodes,
            active_node_idx: Some(active_idx),
            depth_val: Some(depth),
            ..
        } = &node_steps[1].visual
        else {
            panic!("expected node-construction tree state");
        };
        let constructed_value = after_nodes[*active_idx].expect("active node must exist");
        assert!(!before_nodes.contains(&Some(constructed_value)));
        assert!(*depth >= 1);
    }

    let balanced = generate_construct_tree_pre_in_steps(&[1, 2, 3], &[2, 1, 3]);
    let right_skewed = generate_construct_tree_pre_in_steps(&[1, 2, 3, 4], &[1, 2, 3, 4]);
    assert_eq!(
        final_constructed_tree(&balanced),
        [Some(1), Some(2), Some(3)]
    );
    assert_eq!(
        final_constructed_tree(&right_skewed),
        [Some(1), None, Some(2), None, Some(3), None, Some(4)]
    );
    let deepest_creation = right_skewed
        .iter()
        .find(|step| {
            step.code_line == 10
                && matches!(
                    &step.visual,
                    VisualState::TreeVisual {
                        tree_nodes,
                        active_node_idx: Some(active_idx),
                        ..
                    } if tree_nodes[*active_idx] == Some(4)
                )
        })
        .expect("fourth right-skewed node must be constructed");
    assert!(matches!(
        &deepest_creation.visual,
        VisualState::TreeVisual {
            depth_val: Some(4),
            ..
        }
    ));
}

#[test]
fn maximum_path_sum_uses_postorder_gains_and_returns_42() {
    let steps = generate_tree_max_path_sum_steps(&[
        Some(-10),
        Some(9),
        Some(20),
        None,
        None,
        Some(15),
        Some(7),
    ]);

    let visited: Vec<usize> = steps
        .iter()
        .filter(|step| step.code_line == 9)
        .filter_map(|step| match &step.visual {
            VisualState::TreeMaxPathVisual {
                active_node_idx, ..
            } => *active_node_idx,
            _ => None,
        })
        .collect();
    assert_eq!(visited, [1, 5, 6, 2, 0]);
    assert_eq!(steps.len(), 12);
    for node_steps in steps[1..steps.len() - 1].chunks_exact(2) {
        assert_eq!(node_steps[0].code_line, 9);
        assert_eq!(node_steps[1].code_line, 10);
        assert!(node_steps[0]
            .description
            .contains("Updated running maximum"));
        assert!(!node_steps[0].description.contains("Return one-sided gain"));
        assert!(node_steps[1].description.contains("Return one-sided gain"));
        assert!(matches!(
            &node_steps[0].visual,
            VisualState::TreeMaxPathVisual {
                through_node_sum: Some(_),
                returned_gain: None,
                ..
            }
        ));
        assert!(matches!(
            &node_steps[1].visual,
            VisualState::TreeMaxPathVisual {
                through_node_sum: None,
                returned_gain: Some(_),
                ..
            }
        ));
    }
    match &steps.last().expect("maximum path trace must finish").visual {
        VisualState::TreeMaxPathVisual { max_path_sum, .. } => {
            assert_eq!(*max_path_sum, Some(42));
        }
        _ => panic!("expected maximum path visualization"),
    }
    assert!(steps
        .iter()
        .any(|step| step.description.contains("path through node = 42")));
    assert!(steps
        .last()
        .expect("final step")
        .description
        .contains("maximum path sum 42"));
}

#[test]
fn maximum_path_sum_keeps_the_best_negative_node() {
    let steps = generate_tree_max_path_sum_steps(&[Some(-10), Some(-20), Some(-3)]);
    match &steps.last().expect("maximum path trace must finish").visual {
        VisualState::TreeMaxPathVisual { max_path_sum, .. } => {
            assert_eq!(*max_path_sum, Some(-3));
        }
        _ => panic!("expected maximum path visualization"),
    }
}

#[test]
fn maximum_path_sum_decodes_compact_level_order_children() {
    let steps = generate_tree_max_path_sum_steps(&[Some(1), None, Some(2), Some(3)]);

    let visited: Vec<usize> = steps
        .iter()
        .filter(|step| step.code_line == 9)
        .filter_map(|step| match &step.visual {
            VisualState::TreeMaxPathVisual {
                active_node_idx, ..
            } => *active_node_idx,
            _ => None,
        })
        .collect();
    assert_eq!(visited, [3, 2, 0]);

    let node_two_update = steps
        .iter()
        .find(|step| {
            step.code_line == 9
                && matches!(
                    &step.visual,
                    VisualState::TreeMaxPathVisual {
                        active_node_idx: Some(2),
                        secondary_node_idx: Some(3),
                        left_gain: Some(3),
                        right_gain: Some(0),
                        through_node_sum: Some(5),
                        ..
                    }
                )
        })
        .expect("node 3 must be decoded as node 2's left child");
    assert!(node_two_update
        .description
        .contains("path through node = 5"));

    assert!(matches!(
        &steps.last().expect("maximum path trace must finish").visual,
        VisualState::TreeMaxPathVisual {
            max_path_sum: Some(6),
            ..
        }
    ));
}
