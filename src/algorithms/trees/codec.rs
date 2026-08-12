use crate::model::{Step, VisualState};

/// #297 Serialize and Deserialize Binary Tree
pub fn generate_serialize_deserialize_tree_steps(tree: &[Option<i32>]) -> Vec<Step> {
    let mut steps = Vec::new();
    let nodes = tree.to_vec();

    steps.push(Step {
        code_line: 1,
        description: "Serialize binary tree into preorder string string representation."
            .to_string(),
        visual: VisualState::TreeVisual {
            tree_nodes: nodes.clone(),
            active_node_idx: Some(0),
            secondary_node_idx: None,
            depth_val: None,
            max_diameter: None,
        },
    });

    steps.push(Step {
        code_line: 9,
        description: "Deserialize string back into original binary tree structure.".to_string(),
        visual: VisualState::TreeVisual {
            tree_nodes: nodes,
            active_node_idx: None,
            secondary_node_idx: None,
            depth_val: None,
            max_diameter: None,
        },
    });

    steps
}
