use crate::model::{Step, VisualState};

/// #235 Lowest Common Ancestor of a Binary Search Tree
pub fn generate_lowest_common_ancestor_bst_steps(
    tree: &[Option<i32>],
    p: i32,
    q: i32,
) -> Vec<Step> {
    let nodes = tree.to_vec();
    let p_idx = nodes.iter().position(|node| *node == Some(p));
    let q_idx = nodes.iter().position(|node| *node == Some(q));
    let mut steps = Vec::new();
    let mut current_idx = 0;

    while let Some(Some(current)) = nodes.get(current_idx) {
        let current_depth = (usize::BITS - (current_idx + 1).leading_zeros()) as i32;
        steps.push(Step {
            code_line: 4,
            description: format!("Compare p = {p} and q = {q} with node {current}."),
            visual: VisualState::TreeVisual {
                tree_nodes: nodes.clone(),
                active_node_idx: Some(current_idx),
                secondary_node_idx: p_idx.or(q_idx),
                depth_val: Some(current_depth),
                max_diameter: None,
            },
        });

        if p > *current && q > *current {
            steps.last_mut().expect("step was just added").code_line = 5;
            steps.last_mut().expect("step was just added").description =
                format!("Both targets are greater than {current}; continue right.");
            current_idx = current_idx * 2 + 2;
        } else if p < *current && q < *current {
            steps.last_mut().expect("step was just added").code_line = 6;
            steps.last_mut().expect("step was just added").description =
                format!("Both targets are less than {current}; continue left.");
            current_idx = current_idx * 2 + 1;
        } else {
            steps.push(Step {
                code_line: 7,
                description: format!(
                    "The paths split at {current}; this is the lowest common ancestor."
                ),
                visual: VisualState::TreeVisual {
                    tree_nodes: nodes.clone(),
                    active_node_idx: Some(current_idx),
                    secondary_node_idx: q_idx,
                    depth_val: Some(current_depth),
                    max_diameter: None,
                },
            });
            break;
        }
    }

    if steps.is_empty() || steps.last().is_some_and(|step| step.code_line != 7) {
        steps.push(Step {
            code_line: 4,
            description: "No common ancestor was found for the selected values.".to_string(),
            visual: VisualState::TreeVisual {
                tree_nodes: nodes,
                active_node_idx: None,
                secondary_node_idx: None,
                depth_val: None,
                max_diameter: None,
            },
        });
    }

    steps
}

/// #98 Validate Binary Search Tree
pub fn generate_validate_bst_steps(tree: &[Option<i32>]) -> Vec<Step> {
    fn validate(
        nodes: &[Option<i32>],
        idx: usize,
        depth: i32,
        lower: i64,
        upper: i64,
        max_depth: &mut i32,
        steps: &mut Vec<Step>,
    ) -> bool {
        let Some(Some(value)) = nodes.get(idx) else {
            return true;
        };

        *max_depth = (*max_depth).max(depth);
        let valid = lower < i64::from(*value) && i64::from(*value) < upper;
        steps.push(Step {
            code_line: 5,
            description: format!(
                "Check node {value}: it must be strictly between {lower} and {upper}. {}",
                if valid { "Valid." } else { "Invalid." }
            ),
            visual: VisualState::TreeVisual {
                tree_nodes: nodes.to_vec(),
                active_node_idx: Some(idx),
                secondary_node_idx: None,
                depth_val: Some(depth),
                max_diameter: None,
            },
        });

        valid
            && validate(
                nodes,
                idx * 2 + 1,
                depth + 1,
                lower,
                i64::from(*value),
                max_depth,
                steps,
            )
            && validate(
                nodes,
                idx * 2 + 2,
                depth + 1,
                i64::from(*value),
                upper,
                max_depth,
                steps,
            )
    }

    let mut steps = Vec::new();
    let mut max_depth = 0;
    let valid = validate(tree, 0, 1, i64::MIN, i64::MAX, &mut max_depth, &mut steps);
    steps.push(Step {
        code_line: 7,
        description: format!("BST validation complete: {valid}."),
        visual: VisualState::TreeVisual {
            tree_nodes: tree.to_vec(),
            active_node_idx: None,
            secondary_node_idx: None,
            depth_val: Some(max_depth),
            max_diameter: None,
        },
    });
    steps
}

/// #230 Kth Smallest Element in a BST
pub fn generate_kth_smallest_bst_steps(tree: &[Option<i32>], k: usize) -> Vec<Step> {
    let nodes = tree.to_vec();
    let mut steps = Vec::new();
    let mut stack: Vec<usize> = Vec::new();
    let mut current = (!nodes.is_empty()).then_some(0);
    let mut remaining = k;

    steps.push(Step {
        code_line: 3,
        description: format!("Initialize at the root with k = {k} and an empty stack."),
        visual: VisualState::TreeVisual {
            tree_nodes: nodes.clone(),
            active_node_idx: current,
            secondary_node_idx: None,
            depth_val: current.map(tree_depth),
            max_diameter: None,
        },
    });

    while current.is_some() || !stack.is_empty() {
        while let Some(idx) = current.filter(|&idx| nodes.get(idx).is_some_and(Option::is_some)) {
            stack.push(idx);
            let next = idx * 2 + 1;
            current = Some(next);
            steps.push(Step {
                code_line: 7,
                description: format!(
                    "Push {} and traverse left. Stack: {}.",
                    nodes[idx].expect("current index contains a node"),
                    format_tree_stack(&nodes, &stack)
                ),
                visual: VisualState::TreeVisual {
                    tree_nodes: nodes.clone(),
                    active_node_idx: (next < nodes.len()).then_some(next),
                    secondary_node_idx: None,
                    depth_val: Some(tree_depth(next)),
                    max_diameter: None,
                },
            });
        }

        let Some(idx) = stack.pop() else {
            break;
        };
        let value = nodes[idx].expect("stack only contains tree nodes");
        steps.push(Step {
            code_line: 8,
            description: format!(
                "Backtrack to {value}. Stack: {}.",
                format_tree_stack(&nodes, &stack)
            ),
            visual: VisualState::TreeVisual {
                tree_nodes: nodes.clone(),
                active_node_idx: Some(idx),
                secondary_node_idx: None,
                depth_val: Some(tree_depth(idx)),
                max_diameter: None,
            },
        });

        remaining = remaining.saturating_sub(1);
        if remaining == 0 && k > 0 {
            steps.push(Step {
                code_line: 10,
                description: format!("Visit {value}; it is the {} smallest element.", ordinal(k)),
                visual: VisualState::TreeVisual {
                    tree_nodes: nodes,
                    active_node_idx: Some(idx),
                    secondary_node_idx: None,
                    depth_val: Some(tree_depth(idx)),
                    max_diameter: None,
                },
            });
            return steps;
        }

        let next = idx * 2 + 2;
        current = Some(next);
        steps.push(Step {
            code_line: 11,
            description: format!("Visit {value}; {remaining} more node(s) needed. Continue right."),
            visual: VisualState::TreeVisual {
                tree_nodes: nodes.clone(),
                active_node_idx: (next < nodes.len()).then_some(next),
                secondary_node_idx: None,
                depth_val: Some(tree_depth(next)),
                max_diameter: None,
            },
        });
    }

    steps.push(Step {
        code_line: 4,
        description: format!("The tree does not contain a {k}th smallest element."),
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

fn ordinal(value: usize) -> String {
    let suffix = if (11..=13).contains(&(value % 100)) {
        "th"
    } else {
        match value % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        }
    };
    format!("{value}{suffix}")
}

fn tree_depth(idx: usize) -> i32 {
    (usize::BITS - (idx + 1).leading_zeros()) as i32
}

fn format_tree_stack(nodes: &[Option<i32>], stack: &[usize]) -> String {
    let values: Vec<String> = stack
        .iter()
        .filter_map(|&idx| nodes.get(idx).copied().flatten())
        .map(|value| value.to_string())
        .collect();
    format!("[{}]", values.join(", "))
}
