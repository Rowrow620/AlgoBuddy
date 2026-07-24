use crate::model::{Step, VisualState};

pub fn generate_diameter_tree_steps(tree: &[Option<i32>]) -> Vec<Step> {
    let mut steps = Vec::new();
    let current_tree = tree.to_vec();

    if current_tree.is_empty() || current_tree[0].is_none() {
        steps.push(Step {
            code_line: 6,
            description: "Root is empty (None). Diameter = 0.".to_string(),
            visual: VisualState::TreeVisual {
                tree_nodes: current_tree,
                active_node_idx: None,
                secondary_node_idx: None,
                depth_val: Some(0),
                max_diameter: Some(0),
            },
        });
        return steps;
    }

    steps.push(Step {
        code_line: 3,
        description: "Initialized res = 0 to track maximum path diameter across all nodes.".to_string(),
        visual: VisualState::TreeVisual {
            tree_nodes: current_tree.clone(),
            active_node_idx: Some(0),
            secondary_node_idx: None,
            depth_val: None,
            max_diameter: Some(0),
        },
    });

    let n = current_tree.len();
    let mut max_diam = 0;

    for i in 0..n {
        if current_tree[i].is_some() {
            let left_child = 2 * i + 1;
            let right_child = 2 * i + 2;

            let left_h = if left_child < n && current_tree[left_child].is_some() { 1 } else { 0 };
            let right_h = if right_child < n && current_tree[right_child].is_some() { 1 } else { 0 };
            let diam_at_node = left_h + right_h;
            max_diam = max_diam.max(diam_at_node);

            steps.push(Step {
                code_line: 9,
                description: format!("Node val={:?} at index {}: left_h={}, right_h={}. Path diameter through node = {}. Updated max res = {}.", current_tree[i], i, left_h, right_h, diam_at_node, max_diam),
                visual: VisualState::TreeVisual {
                    tree_nodes: current_tree.clone(),
                    active_node_idx: Some(i),
                    secondary_node_idx: if left_child < n { Some(left_child) } else if right_child < n { Some(right_child) } else { None },
                    depth_val: Some(1 + left_h.max(right_h)),
                    max_diameter: Some(max_diam),
                },
            });
        }
    }

    steps.push(Step {
        code_line: 12,
        description: format!("Completed DFS traversal. Maximum tree diameter (longest path in edges) = {}.", max_diam),
        visual: VisualState::TreeVisual {
            tree_nodes: current_tree,
            active_node_idx: None,
            secondary_node_idx: None,
            depth_val: None,
            max_diameter: Some(max_diam),
        },
    });

    steps
}
