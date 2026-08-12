use crate::model::{Step, VisualState};
use std::collections::VecDeque;

/// #124 Binary Tree Maximum Path Sum
fn compact_level_order_children(nodes: &[Option<i32>]) -> Vec<(Option<usize>, Option<usize>)> {
    let mut children = vec![(None, None); nodes.len()];
    if nodes.first().copied().flatten().is_none() {
        return children;
    }

    let mut parents = VecDeque::from([0]);
    let mut next_child = 1;
    while let Some(parent_idx) = parents.pop_front() {
        if next_child >= nodes.len() {
            break;
        }

        if nodes[next_child].is_some() {
            children[parent_idx].0 = Some(next_child);
            parents.push_back(next_child);
        }
        next_child += 1;

        if next_child < nodes.len() {
            if nodes[next_child].is_some() {
                children[parent_idx].1 = Some(next_child);
                parents.push_back(next_child);
            }
            next_child += 1;
        }
    }
    children
}

pub fn generate_tree_max_path_sum_steps(tree: &[Option<i32>]) -> Vec<Step> {
    let nodes = tree.to_vec();
    if nodes.first().copied().flatten().is_none() {
        return vec![Step {
            code_line: 12,
            description: "The tree is empty, so no non-empty path exists.".to_string(),
            visual: VisualState::TreeMaxPathVisual {
                tree_nodes: nodes,
                active_node_idx: None,
                secondary_node_idx: None,
                left_gain: None,
                right_gain: None,
                through_node_sum: None,
                returned_gain: None,
                max_path_sum: None,
            },
        }];
    }

    fn gain(
        idx: usize,
        nodes: &[Option<i32>],
        children: &[(Option<usize>, Option<usize>)],
        best: &mut i32,
        steps: &mut Vec<Step>,
    ) -> i32 {
        let Some(Some(value)) = nodes.get(idx).copied() else {
            return 0;
        };

        let (left_idx, right_idx) = children[idx];
        let left_gain = left_idx
            .map(|child_idx| gain(child_idx, nodes, children, best, steps))
            .unwrap_or(0)
            .max(0);
        let right_gain = right_idx
            .map(|child_idx| gain(child_idx, nodes, children, best, steps))
            .unwrap_or(0)
            .max(0);
        let through_node_sum = value + left_gain + right_gain;
        *best = (*best).max(through_node_sum);
        let return_gain = value + left_gain.max(right_gain);
        let secondary_node_idx = if left_gain >= right_gain && left_gain > 0 {
            left_idx
        } else if right_gain > 0 {
            right_idx
        } else {
            None
        };

        steps.push(Step {
            code_line: 9,
            description: format!(
                "Post-order node {value} at index {idx}: left gain = {left_gain}, right gain = {right_gain}, path through node = {through_node_sum}. Updated running maximum to {}.",
                *best
            ),
            visual: VisualState::TreeMaxPathVisual {
                tree_nodes: nodes.to_vec(),
                active_node_idx: Some(idx),
                secondary_node_idx,
                left_gain: Some(left_gain),
                right_gain: Some(right_gain),
                through_node_sum: Some(through_node_sum),
                returned_gain: None,
                max_path_sum: Some(*best),
            },
        });
        steps.push(Step {
            code_line: 10,
            description: format!(
                "Return one-sided gain {return_gain} from node {value} to its parent."
            ),
            visual: VisualState::TreeMaxPathVisual {
                tree_nodes: nodes.to_vec(),
                active_node_idx: Some(idx),
                secondary_node_idx,
                left_gain: Some(left_gain),
                right_gain: Some(right_gain),
                through_node_sum: None,
                returned_gain: Some(return_gain),
                max_path_sum: Some(*best),
            },
        });
        return_gain
    }

    let mut steps = vec![Step {
        code_line: 3,
        description:
            "Initialized the global maximum path sum to negative infinity before post-order DFS."
                .to_string(),
        visual: VisualState::TreeMaxPathVisual {
            tree_nodes: nodes.clone(),
            active_node_idx: None,
            secondary_node_idx: None,
            left_gain: None,
            right_gain: None,
            through_node_sum: None,
            returned_gain: None,
            max_path_sum: None,
        },
    }];
    let children = compact_level_order_children(&nodes);
    let mut best = i32::MIN;
    gain(0, &nodes, &children, &mut best, &mut steps);
    steps.push(Step {
        code_line: 12,
        description: format!("Post-order traversal complete. Return maximum path sum {best}."),
        visual: VisualState::TreeMaxPathVisual {
            tree_nodes: nodes,
            active_node_idx: None,
            secondary_node_idx: None,
            left_gain: None,
            right_gain: None,
            through_node_sum: None,
            returned_gain: None,
            max_path_sum: Some(best),
        },
    });
    steps
}
