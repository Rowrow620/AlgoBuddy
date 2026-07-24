use crate::model::{Step, VisualState};

pub fn generate_same_tree_steps(tree1: &[Option<i32>], tree2: &[Option<i32>]) -> Vec<Step> {
    let mut steps = Vec::new();
    let t1 = tree1.to_vec();
    let t2 = tree2.to_vec();

    steps.push(Step {
        code_line: 3,
        description: "Comparing Tree p and Tree q for structural and value equality.".to_string(),
        visual: VisualState::TreeVisual {
            tree_nodes: t1.clone(),
            active_node_idx: Some(0),
            secondary_node_idx: None,
            depth_val: None,
            max_diameter: None,
        },
    });

    let max_len = t1.len().max(t2.len());

    for i in 0..max_len {
        let v1 = t1.get(i).cloned().flatten();
        let v2 = t2.get(i).cloned().flatten();

        steps.push(Step {
            code_line: 6,
            description: format!("Comparing node at index {}: Tree p = {:?} vs Tree q = {:?}.", i, v1, v2),
            visual: VisualState::TreeVisual {
                tree_nodes: t1.clone(),
                active_node_idx: Some(i),
                secondary_node_idx: None,
                depth_val: None,
                max_diameter: None,
            },
        });

        if v1 != v2 {
            steps.push(Step {
                code_line: 8,
                description: format!("Mismatch found at index {}! {:?} != {:?}. Return False.", i, v1, v2),
                visual: VisualState::TreeVisual {
                    tree_nodes: t1.clone(),
                    active_node_idx: Some(i),
                    secondary_node_idx: None,
                    depth_val: None,
                    max_diameter: None,
                },
            });
            return steps;
        }
    }

    steps.push(Step {
        code_line: 10,
        description: "All nodes and structures match perfectly! Return True.".to_string(),
        visual: VisualState::TreeVisual {
            tree_nodes: t1,
            active_node_idx: None,
            secondary_node_idx: None,
            depth_val: None,
            max_diameter: None,
        },
    });

    steps
}
