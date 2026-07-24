use crate::model::{Step, VisualState};

pub fn generate_invert_tree_steps(tree: &[Option<i32>]) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut current_tree = tree.to_vec();

    if current_tree.is_empty() || current_tree[0].is_none() {
        steps.push(Step {
            code_line: 4,
            description: "Root is empty (None). Returning None.".to_string(),
            visual: VisualState::TreeVisual {
                tree_nodes: current_tree,
                active_node_idx: None,
                secondary_node_idx: None,
                depth_val: None,
                max_diameter: None,
            },
        });
        return steps;
    }

    steps.push(Step {
        code_line: 3,
        description: format!("Initialized binary tree inversion for root node val={:?}.", current_tree[0]),
        visual: VisualState::TreeVisual {
            tree_nodes: current_tree.clone(),
            active_node_idx: Some(0),
            secondary_node_idx: None,
            depth_val: None,
            max_diameter: None,
        },
    });

    // Invert level-order nodes array
    let n = current_tree.len();
    for i in 0..n {
        if current_tree[i].is_some() {
            let left_child = 2 * i + 1;
            let right_child = 2 * i + 2;

            if left_child < n || right_child < n {
                let left_val = if left_child < n { current_tree[left_child] } else { None };
                let right_val = if right_child < n { current_tree[right_child] } else { None };

                steps.push(Step {
                    code_line: 6,
                    description: format!("Swapping left child ({:?}) and right child ({:?}) for node val={:?} at index {}.", left_val, right_val, current_tree[i], i),
                    visual: VisualState::TreeVisual {
                        tree_nodes: current_tree.clone(),
                        active_node_idx: Some(i),
                        secondary_node_idx: if left_child < n { Some(left_child) } else { None },
                        depth_val: None,
                        max_diameter: None,
                    },
                });

                if left_child < n && right_child < n {
                    current_tree.swap(left_child, right_child);
                }

                steps.push(Step {
                    code_line: 7,
                    description: format!("Subtrees swapped! Left child is now {:?}, right child is now {:?}.", current_tree.get(left_child).cloned().flatten(), current_tree.get(right_child).cloned().flatten()),
                    visual: VisualState::TreeVisual {
                        tree_nodes: current_tree.clone(),
                        active_node_idx: Some(i),
                        secondary_node_idx: if right_child < n { Some(right_child) } else { None },
                        depth_val: None,
                        max_diameter: None,
                    },
                });
            }
        }
    }

    steps.push(Step {
        code_line: 10,
        description: "Completed binary tree inversion. Returned inverted root.".to_string(),
        visual: VisualState::TreeVisual {
            tree_nodes: current_tree,
            active_node_idx: None,
            secondary_node_idx: None,
            depth_val: None,
            max_diameter: None,
        },
    });

    steps
}
