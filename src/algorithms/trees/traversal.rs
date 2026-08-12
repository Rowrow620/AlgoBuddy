use crate::model::{Step, VisualState};

/// #102 Binary Tree Level Order Traversal
pub fn generate_level_order_traversal_steps(tree: &[Option<i32>]) -> Vec<Step> {
    let mut steps = Vec::new();
    let nodes = tree.to_vec();
    steps.push(Step {
        code_line: 1,
        description: "Perform BFS Level Order Traversal using a Queue.".to_string(),
        visual: VisualState::TreeVisual {
            tree_nodes: nodes.clone(),
            active_node_idx: Some(0),
            secondary_node_idx: None,
            depth_val: Some(1),
            max_diameter: None,
        },
    });

    for (i, node) in nodes.iter().enumerate() {
        if node.is_some() {
            steps.push(Step {
                code_line: 5,
                description: format!("Process node val={:?} at index {}.", node.unwrap(), i),
                visual: VisualState::TreeVisual {
                    tree_nodes: nodes.clone(),
                    active_node_idx: Some(i),
                    secondary_node_idx: None,
                    depth_val: Some((i as f32 + 1.0).log2().floor() as i32 + 1),
                    max_diameter: None,
                },
            });
        }
    }

    steps.push(Step {
        code_line: 12,
        description: "Level order traversal complete.".to_string(),
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

/// #199 Binary Tree Right Side View
pub fn generate_right_side_view_steps(tree: &[Option<i32>]) -> Vec<Step> {
    let mut steps = Vec::new();
    let nodes = tree.to_vec();
    steps.push(Step {
        code_line: 1,
        description: "Traverse tree levels collecting the rightmost visible node.".to_string(),
        visual: VisualState::TreeVisual {
            tree_nodes: nodes.clone(),
            active_node_idx: Some(0),
            secondary_node_idx: None,
            depth_val: Some(1),
            max_diameter: None,
        },
    });

    steps.push(Step {
        code_line: 8,
        description: "Right side view nodes collected.".to_string(),
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

/// #1448 Count Good Nodes in Binary Tree
pub fn generate_count_good_nodes_steps(tree: &[Option<i32>]) -> Vec<Step> {
    let mut steps = Vec::new();
    let nodes = tree.to_vec();
    let mut good_count = 0;
    let mut max_so_far = i32::MIN;

    for (i, node) in nodes.iter().enumerate() {
        if let Some(val) = *node {
            if val >= max_so_far {
                good_count += 1;
                max_so_far = max_so_far.max(val);
                steps.push(Step {
                    code_line: 5,
                    description: format!(
                        "Node val={} >= max_so_far ({}). GOOD node! Total good = {}.",
                        val, max_so_far, good_count
                    ),
                    visual: VisualState::TreeVisual {
                        tree_nodes: nodes.clone(),
                        active_node_idx: Some(i),
                        secondary_node_idx: None,
                        depth_val: Some(good_count),
                        max_diameter: None,
                    },
                });
            }
        }
    }

    steps
}
