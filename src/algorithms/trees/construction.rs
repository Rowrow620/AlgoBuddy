use crate::model::{Step, VisualState};
use std::collections::{HashMap, HashSet, VecDeque};

/// #105 Construct Binary Tree from Preorder and Inorder Traversal
#[derive(Clone)]
struct ConstructedTreeNode {
    value: i32,
    left: Option<Box<ConstructedTreeNode>>,
    right: Option<Box<ConstructedTreeNode>>,
}

struct ConstructionEvent {
    preorder_idx: usize,
    value: i32,
    inorder_start: usize,
    inorder_end: usize,
    split_idx: usize,
    depth: i32,
}

fn build_from_traversals(
    preorder: &[i32],
    inorder_positions: &HashMap<i32, usize>,
    preorder_idx: &mut usize,
    inorder_start: usize,
    inorder_end: usize,
    depth: i32,
    events: &mut Vec<ConstructionEvent>,
) -> Result<Option<Box<ConstructedTreeNode>>, String> {
    if inorder_start >= inorder_end {
        return Ok(None);
    }

    let root_preorder_idx = *preorder_idx;
    let root_value = preorder
        .get(root_preorder_idx)
        .copied()
        .ok_or_else(|| "preorder ended before all inorder partitions were built".to_string())?;
    let split_idx = inorder_positions
        .get(&root_value)
        .copied()
        .ok_or_else(|| format!("value {root_value} is missing from inorder"))?;
    if split_idx < inorder_start || split_idx >= inorder_end {
        return Err(format!(
            "value {root_value} falls outside its required inorder partition"
        ));
    }

    *preorder_idx += 1;
    events.push(ConstructionEvent {
        preorder_idx: root_preorder_idx,
        value: root_value,
        inorder_start,
        inorder_end,
        split_idx,
        depth,
    });

    let left = build_from_traversals(
        preorder,
        inorder_positions,
        preorder_idx,
        inorder_start,
        split_idx,
        depth + 1,
        events,
    )?;
    let right = build_from_traversals(
        preorder,
        inorder_positions,
        preorder_idx,
        split_idx + 1,
        inorder_end,
        depth + 1,
        events,
    )?;

    Ok(Some(Box::new(ConstructedTreeNode {
        value: root_value,
        left,
        right,
    })))
}

fn constructed_tree_level_order(root: Option<&ConstructedTreeNode>) -> Vec<Option<i32>> {
    let Some(root) = root else {
        return Vec::new();
    };

    let mut nodes = Vec::new();
    let mut queue = VecDeque::from([Some(root)]);
    while let Some(node) = queue.pop_front() {
        match node {
            Some(node) => {
                nodes.push(Some(node.value));
                queue.push_back(node.left.as_deref());
                queue.push_back(node.right.as_deref());
            }
            None => nodes.push(None),
        }
    }
    while nodes.last() == Some(&None) {
        nodes.pop();
    }
    nodes
}

fn construction_snapshot(
    final_tree: &[Option<i32>],
    visible_values: &HashSet<i32>,
) -> Vec<Option<i32>> {
    let mut snapshot: Vec<Option<i32>> = final_tree
        .iter()
        .map(|node| node.filter(|value| visible_values.contains(value)))
        .collect();
    while snapshot.last() == Some(&None) {
        snapshot.pop();
    }
    snapshot
}

fn construction_failure(description: String) -> Vec<Step> {
    vec![Step {
        code_line: 3,
        description,
        visual: VisualState::TreeVisual {
            tree_nodes: Vec::new(),
            active_node_idx: None,
            secondary_node_idx: None,
            depth_val: None,
            max_diameter: None,
        },
    }]
}

pub fn generate_construct_tree_pre_in_steps(preorder: &[i32], inorder: &[i32]) -> Vec<Step> {
    if preorder.len() != inorder.len() {
        return construction_failure(format!(
            "Cannot reconstruct tree: preorder has {} values but inorder has {}.",
            preorder.len(),
            inorder.len()
        ));
    }
    if preorder.is_empty() {
        return vec![Step {
            code_line: 15,
            description: "Both traversals are empty; return an empty tree.".to_string(),
            visual: VisualState::TreeVisual {
                tree_nodes: Vec::new(),
                active_node_idx: None,
                secondary_node_idx: None,
                depth_val: None,
                max_diameter: None,
            },
        }];
    }

    let mut inorder_positions = HashMap::with_capacity(inorder.len());
    for (idx, &value) in inorder.iter().enumerate() {
        if inorder_positions.insert(value, idx).is_some() {
            return construction_failure(format!(
                "Cannot reconstruct tree: inorder contains duplicate value {value}."
            ));
        }
    }
    let mut preorder_values = HashSet::with_capacity(preorder.len());
    if let Some(&duplicate) = preorder
        .iter()
        .find(|&&value| !preorder_values.insert(value))
    {
        return construction_failure(format!(
            "Cannot reconstruct tree: preorder contains duplicate value {duplicate}."
        ));
    }

    let mut preorder_idx = 0;
    let mut events = Vec::with_capacity(preorder.len());
    let root = match build_from_traversals(
        preorder,
        &inorder_positions,
        &mut preorder_idx,
        0,
        inorder.len(),
        1,
        &mut events,
    ) {
        Ok(root) if preorder_idx == preorder.len() => root,
        Ok(_) => {
            return construction_failure(
                "Cannot reconstruct tree: not every preorder value was consumed.".to_string(),
            );
        }
        Err(message) => {
            return construction_failure(format!("Cannot reconstruct tree: {message}."));
        }
    };

    let final_tree = constructed_tree_level_order(root.as_deref());
    let mut steps = vec![Step {
        code_line: 3,
        description: format!(
            "Indexed all {} inorder values for O(1) splits. Set preorder_index = 0 before reconstruction.",
            inorder_positions.len()
        ),
        visual: VisualState::TreeVisual {
            tree_nodes: Vec::new(),
            active_node_idx: None,
            secondary_node_idx: None,
            depth_val: None,
            max_diameter: None,
        },
    }];

    let mut visible_values = HashSet::with_capacity(events.len());
    for event in events {
        let tree_before = construction_snapshot(&final_tree, &visible_values);
        steps.push(Step {
            code_line: 8,
            description: format!(
                "Read preorder[{}] = {} as the root value for inorder[{}..{}); the node has not been constructed yet.",
                event.preorder_idx, event.value, event.inorder_start, event.inorder_end
            ),
            visual: VisualState::TreeVisual {
                tree_nodes: tree_before,
                active_node_idx: None,
                secondary_node_idx: None,
                depth_val: None,
                max_diameter: None,
            },
        });

        visible_values.insert(event.value);
        let partial_tree = construction_snapshot(&final_tree, &visible_values);
        let active_idx = partial_tree
            .iter()
            .position(|node| *node == Some(event.value));
        let depth = Some(event.depth);

        steps.push(Step {
            code_line: 10,
            description: format!(
                "Constructed TreeNode({}) at recursion depth {}.",
                event.value, event.depth
            ),
            visual: VisualState::TreeVisual {
                tree_nodes: partial_tree.clone(),
                active_node_idx: active_idx,
                secondary_node_idx: None,
                depth_val: depth,
                max_diameter: None,
            },
        });
        steps.push(Step {
            code_line: 11,
            description: format!(
                "Value {} is at inorder index {}. Left partition = {:?}; right partition = {:?}.",
                event.value,
                event.split_idx,
                &inorder[event.inorder_start..event.split_idx],
                &inorder[event.split_idx + 1..event.inorder_end]
            ),
            visual: VisualState::TreeVisual {
                tree_nodes: partial_tree,
                active_node_idx: active_idx,
                secondary_node_idx: None,
                depth_val: depth,
                max_diameter: None,
            },
        });
    }

    steps.push(Step {
        code_line: 15,
        description: format!(
            "Construction complete using both traversals. Level-order tree = {:?}.",
            final_tree
        ),
        visual: VisualState::TreeVisual {
            tree_nodes: final_tree,
            active_node_idx: None,
            secondary_node_idx: None,
            depth_val: None,
            max_diameter: None,
        },
    });
    steps
}
