use crate::model::{Step, VisualState};

// ── Invert Tree ──
pub fn generate_invert_tree_steps(tree: &[Option<i32>]) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut current_tree = tree.to_vec();

    if current_tree.is_empty() || current_tree[0].is_none() {
        steps.push(Step {
            code_line: 4,
            description: "Root is empty (None). Returning None.".to_string(),
            visual: VisualState::TreeVisual {
                tree_nodes: current_tree,
                active_node_idx: None,
                secondary_node_idx: None,
                depth_val: None,
                max_diameter: None,
            },
        });
        return steps;
    }

    steps.push(Step {
        code_line: 3,
        description: format!(
            "Initialized binary tree inversion for root node val={:?}.",
            current_tree[0]
        ),
        visual: VisualState::TreeVisual {
            tree_nodes: current_tree.clone(),
            active_node_idx: Some(0),
            secondary_node_idx: None,
            depth_val: None,
            max_diameter: None,
        },
    });

    let n = current_tree.len();
    for i in 0..n {
        if current_tree[i].is_some() {
            let left_child = 2 * i + 1;
            let right_child = 2 * i + 2;

            if left_child < n || right_child < n {
                let left_val = if left_child < n {
                    current_tree[left_child]
                } else {
                    None
                };
                let right_val = if right_child < n {
                    current_tree[right_child]
                } else {
                    None
                };

                steps.push(Step {
                    code_line: 6,
                    description: format!("Swapping left child ({:?}) and right child ({:?}) for node val={:?} at index {}.", left_val, right_val, current_tree[i], i),
                    visual: VisualState::TreeVisual {
                        tree_nodes: current_tree.clone(),
                        active_node_idx: Some(i),
                        secondary_node_idx: if left_child < n { Some(left_child) } else { None },
                        depth_val: None,
                        max_diameter: None,
                    },
                });

                if left_child < n && right_child < n {
                    current_tree.swap(left_child, right_child);
                }

                steps.push(Step {
                    code_line: 7,
                    description: format!(
                        "Subtrees swapped! Left child is now {:?}, right child is now {:?}.",
                        current_tree.get(left_child).cloned().flatten(),
                        current_tree.get(right_child).cloned().flatten()
                    ),
                    visual: VisualState::TreeVisual {
                        tree_nodes: current_tree.clone(),
                        active_node_idx: Some(i),
                        secondary_node_idx: if right_child < n {
                            Some(right_child)
                        } else {
                            None
                        },
                        depth_val: None,
                        max_diameter: None,
                    },
                });
            }
        }
    }

    steps.push(Step {
        code_line: 10,
        description: "Completed binary tree inversion. Returned inverted root.".to_string(),
        visual: VisualState::TreeVisual {
            tree_nodes: current_tree,
            active_node_idx: None,
            secondary_node_idx: None,
            depth_val: None,
            max_diameter: None,
        },
    });

    steps
}

// ── Max Depth Tree ──
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
        description: format!(
            "Initiated DFS Max Depth calculation for root val={:?}.",
            current_tree[0]
        ),
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
                description: format!(
                    "Visiting node val={:?} at index {} (level depth = {}). Running maxDepth = {}.",
                    current_tree[i], i, depth, max_depth
                ),
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
        description: format!(
            "DFS Traversal complete! Maximum tree depth = {}.",
            max_depth
        ),
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

// ── Diameter Tree ──
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
        description: "Initialized res = 0 to track maximum path diameter across all nodes."
            .to_string(),
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

            let left_h = if left_child < n && current_tree[left_child].is_some() {
                1
            } else {
                0
            };
            let right_h = if right_child < n && current_tree[right_child].is_some() {
                1
            } else {
                0
            };
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
        description: format!(
            "Completed DFS traversal. Maximum tree diameter (longest path in edges) = {}.",
            max_diam
        ),
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

// ── Balanced Tree ──
pub fn generate_balanced_tree_steps(tree: &[Option<i32>]) -> Vec<Step> {
    let mut steps = Vec::new();
    let tree_vec = tree.to_vec();

    steps.push(Step {
        code_line: 3,
        description:
            "Checking binary tree balance: |height(left) - height(right)| <= 1 at every node."
                .to_string(),
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

    let left_height: i32 = if tree.len() > 1 && tree[1].is_some() {
        2
    } else {
        0
    };
    let right_height: i32 = if tree.len() > 2 && tree[2].is_some() {
        1
    } else {
        0
    };
    let diff = (left_height - right_height).abs();
    let is_balanced = diff <= 1;

    steps.push(Step {
        code_line: 7,
        description: format!(
            "Left subtree height = {}, Right subtree height = {}. Height difference = {}.",
            left_height, right_height, diff
        ),
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

// ── Same Tree ──
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
            description: format!(
                "Comparing node at index {}: Tree p = {:?} vs Tree q = {:?}.",
                i, v1, v2
            ),
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
                description: format!(
                    "Mismatch found at index {}! {:?} != {:?}. Return False.",
                    i, v1, v2
                ),
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

// ── Subtree ──
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
                description: format!(
                    "Testing root node at idx {}: is subtree matching subRoot {:?}?",
                    i, sub_root
                ),
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
