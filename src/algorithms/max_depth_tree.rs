use crate::model::{Step, VisualState};

pub fn generate_max_depth_tree_steps(tree: &[Option<i32>]) -> Vec<Step> {
    let mut steps = Vec::new();
    let current_tree = tree.to_vec();

    if current_tree.is_empty() || current_tree[0].is_none() {
        steps.push(Step {
            code_line: 4,
            description: "Root is empty (None). Depth = 0.".to_string(),
            visual: VisualState::TreeVisual {
                tree_nodes: current_tree,
                active_node_idx: None,
                secondary_node_idx: None,
                depth_val: Some(0),
                max_diameter: None,
            },
        });
        return steps;
    }

    steps.push(Step {
        code_line: 3,
        description: format!("Initiated DFS Max Depth calculation for root val={:?}.", current_tree[0]),
        visual: VisualState::TreeVisual {
            tree_nodes: current_tree.clone(),
            active_node_idx: Some(0),
            secondary_node_idx: None,
            depth_val: Some(1),
            max_diameter: None,
        },
    });

    let n = current_tree.len();
    let mut max_depth = 1;

    for i in 0..n {
        if current_tree[i].is_some() {
            let depth = (i as f64 + 1.0).log2().floor() as i32 + 1;
            max_depth = max_depth.max(depth);

            steps.push(Step {
                code_line: 7,
                description: format!("Visiting node val={:?} at index {} (level depth = {}). Running maxDepth = {}.", current_tree[i], i, depth, max_depth),
                visual: VisualState::TreeVisual {
                    tree_nodes: current_tree.clone(),
                    active_node_idx: Some(i),
                    secondary_node_idx: None,
                    depth_val: Some(max_depth),
                    max_diameter: None,
                },
            });
        }
    }

    steps.push(Step {
        code_line: 7,
        description: format!("DFS Traversal complete! Maximum tree depth = {}.", max_depth),
        visual: VisualState::TreeVisual {
            tree_nodes: current_tree,
            active_node_idx: None,
            secondary_node_idx: None,
            depth_val: Some(max_depth),
            max_diameter: None,
        },
    });

    steps
}
