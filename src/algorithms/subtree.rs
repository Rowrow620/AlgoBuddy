use crate::model::{Step, VisualState};

pub fn generate_subtree_steps(root: &[Option<i32>], sub_root: &[Option<i32>]) -> Vec<Step> {
    let mut steps = Vec::new();
    let r_vec = root.to_vec();

    steps.push(Step {
        code_line: 3,
        description: "Checking if `subRoot` is a valid subtree of `root`.".to_string(),
        visual: VisualState::TreeVisual {
            tree_nodes: r_vec.clone(),
            active_node_idx: Some(0),
            secondary_node_idx: None,
            depth_val: None,
            max_diameter: None,
        },
    });

    for (i, node_opt) in root.iter().enumerate() {
        if node_opt.is_some() {
            steps.push(Step {
                code_line: 6,
                description: format!("Testing root node at idx {}: is subtree matching subRoot {:?}?", i, sub_root),
                visual: VisualState::TreeVisual {
                    tree_nodes: r_vec.clone(),
                    active_node_idx: Some(i),
                    secondary_node_idx: None,
                    depth_val: None,
                    max_diameter: None,
                },
            });
        }
    }

    steps.push(Step {
        code_line: 9,
        description: "Subtree matching completed!".to_string(),
        visual: VisualState::TreeVisual {
            tree_nodes: r_vec,
            active_node_idx: None,
            secondary_node_idx: None,
            depth_val: None,
            max_diameter: None,
        },
    });

    steps
}
