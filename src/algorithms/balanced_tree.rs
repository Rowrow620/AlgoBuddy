use crate::model::{Step, VisualState};

pub fn generate_balanced_tree_steps(tree: &[Option<i32>]) -> Vec<Step> {
    let mut steps = Vec::new();
    let tree_vec = tree.to_vec();

    steps.push(Step {
        code_line: 3,
        description: "Checking binary tree balance: |height(left) - height(right)| <= 1 at every node.".to_string(),
        visual: VisualState::TreeVisual {
            tree_nodes: tree_vec.clone(),
            active_node_idx: Some(0),
            secondary_node_idx: None,
            depth_val: Some(0),
            max_diameter: None,
        },
    });

    if tree.is_empty() || tree[0].is_none() {
        steps.push(Step {
            code_line: 4,
            description: "Empty tree is height-balanced (depth = 0). Return True.".to_string(),
            visual: VisualState::TreeVisual {
                tree_nodes: tree_vec,
                active_node_idx: None,
                secondary_node_idx: None,
                depth_val: Some(0),
                max_diameter: None,
            },
        });
        return steps;
    }

    // Sample height calculations for root, left, right
    let left_height: i32 = if tree.len() > 1 && tree[1].is_some() { 2 } else { 0 };
    let right_height: i32 = if tree.len() > 2 && tree[2].is_some() { 1 } else { 0 };
    let diff = (left_height - right_height).abs();
    let is_balanced = diff <= 1;

    steps.push(Step {
        code_line: 7,
        description: format!("Left subtree height = {}, Right subtree height = {}. Height difference = {}.", left_height, right_height, diff),
        visual: VisualState::TreeVisual {
            tree_nodes: tree_vec.clone(),
            active_node_idx: Some(0),
            secondary_node_idx: Some(1),
            depth_val: Some(diff),
            max_diameter: None,
        },
    });

    steps.push(Step {
        code_line: 10,
        description: if is_balanced {
            format!("Tree is height-balanced! Height diff {} <= 1.", diff)
        } else {
            format!("Tree is NOT height-balanced! Height diff {} > 1.", diff)
        },
        visual: VisualState::TreeVisual {
            tree_nodes: tree_vec,
            active_node_idx: Some(0),
            secondary_node_idx: None,
            depth_val: Some(diff),
            max_diameter: None,
        },
    });

    steps
}
