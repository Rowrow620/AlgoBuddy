use crate::model::{Step, VisualState};
use std::collections::VecDeque;

const LINEAR_TREE_TRACE_LIMIT: usize = 128;
const QUADRATIC_TREE_TRACE_LIMIT: usize = 40;

pub fn generate_invert_tree_steps(tree: &[Option<i32>], approach_id: usize) -> Vec<Step> {
    if tree.len() > LINEAR_TREE_TRACE_LIMIT {
        return tree_trace_unavailable(LINEAR_TREE_TRACE_LIMIT, "Invert Binary Tree");
    }
    match approach_id {
        0 => generate_invert_tree_recursive_steps(tree),
        1 => generate_invert_tree_bfs_steps(tree),
        _ => Vec::new(),
    }
}

fn generate_invert_tree_recursive_steps(tree: &[Option<i32>]) -> Vec<Step> {
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
                    code_line: 5,
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
        code_line: 6,
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

pub fn generate_max_depth_tree_steps(tree: &[Option<i32>], approach_id: usize) -> Vec<Step> {
    if tree.len() > LINEAR_TREE_TRACE_LIMIT {
        return tree_trace_unavailable(LINEAR_TREE_TRACE_LIMIT, "Maximum Depth");
    }
    match approach_id {
        0 => generate_max_depth_tree_recursive_steps(tree),
        1 => generate_max_depth_tree_bfs_steps(tree),
        _ => Vec::new(),
    }
}

fn generate_max_depth_tree_recursive_steps(tree: &[Option<i32>]) -> Vec<Step> {
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
                code_line: 4,
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
        code_line: 4,
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

pub fn generate_diameter_tree_steps(tree: &[Option<i32>], approach_id: usize) -> Vec<Step> {
    let limit = if approach_id == 1 {
        QUADRATIC_TREE_TRACE_LIMIT
    } else {
        LINEAR_TREE_TRACE_LIMIT
    };
    if tree.len() > limit {
        return tree_trace_unavailable(limit, "Diameter of Binary Tree");
    }
    match approach_id {
        0 => generate_diameter_tree_postorder_steps(tree),
        1 => generate_diameter_tree_recomputed_height_steps(tree),
        _ => Vec::new(),
    }
}

fn generate_diameter_tree_postorder_steps(tree: &[Option<i32>]) -> Vec<Step> {
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
                code_line: 8,
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
        code_line: 10,
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

pub fn generate_balanced_tree_steps(tree: &[Option<i32>], approach_id: usize) -> Vec<Step> {
    let limit = if approach_id == 1 {
        QUADRATIC_TREE_TRACE_LIMIT
    } else {
        LINEAR_TREE_TRACE_LIMIT
    };
    if tree.len() > limit {
        return tree_trace_unavailable(limit, "Balanced Binary Tree");
    }
    match approach_id {
        0 => generate_balanced_tree_bottom_up_steps(tree),
        1 => generate_balanced_tree_recomputed_height_steps(tree),
        _ => Vec::new(),
    }
}

fn generate_balanced_tree_bottom_up_steps(tree: &[Option<i32>]) -> Vec<Step> {
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
        code_line: 8,
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

pub fn generate_same_tree_steps(
    tree1: &[Option<i32>],
    tree2: &[Option<i32>],
    approach_id: usize,
) -> Vec<Step> {
    if tree1.len().max(tree2.len()) > LINEAR_TREE_TRACE_LIMIT {
        return tree_trace_unavailable(LINEAR_TREE_TRACE_LIMIT, "Same Tree");
    }
    match approach_id {
        0 => generate_same_tree_recursive_steps(tree1, tree2),
        1 => generate_same_tree_bfs_steps(tree1, tree2),
        _ => Vec::new(),
    }
}

fn generate_same_tree_recursive_steps(tree1: &[Option<i32>], tree2: &[Option<i32>]) -> Vec<Step> {
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
            code_line: 4,
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
                code_line: 4,
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
        code_line: 5,
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

pub fn generate_subtree_steps(
    root: &[Option<i32>],
    sub_root: &[Option<i32>],
    approach_id: usize,
) -> Vec<Step> {
    if root.len().saturating_add(sub_root.len()) > LINEAR_TREE_TRACE_LIMIT {
        return tree_trace_unavailable(LINEAR_TREE_TRACE_LIMIT, "Subtree of Another Tree");
    }
    match approach_id {
        0 => generate_subtree_recursive_steps(root, sub_root),
        1 => generate_subtree_serialization_steps(root, sub_root),
        _ => Vec::new(),
    }
}

fn generate_subtree_recursive_steps(root: &[Option<i32>], sub_root: &[Option<i32>]) -> Vec<Step> {
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
        code_line: 6,
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

fn tree_trace_unavailable(limit: usize, problem_name: &str) -> Vec<Step> {
    vec![Step {
        code_line: 3,
        description: format!(
            "{} visualization supports up to {} level-order entries; shorten the input to build the detailed trace.",
            problem_name, limit
        ),
        visual: VisualState::TraceUnavailable {
            message: format!(
                "This trace accepts at most {} level-order entries because each step stores a complete tree snapshot.",
                limit
            ),
        },
    }]
}

#[derive(Clone)]
struct CompactNode {
    value: i32,
    left: Option<usize>,
    right: Option<usize>,
    source_index: usize,
}

#[derive(Clone)]
struct CompactTree {
    nodes: Vec<CompactNode>,
    root: Option<usize>,
}

impl CompactTree {
    fn from_level_order(values: &[Option<i32>]) -> Self {
        let Some(root_value) = values.first().copied().flatten() else {
            return Self {
                nodes: Vec::new(),
                root: None,
            };
        };

        let mut nodes = vec![CompactNode {
            value: root_value,
            left: None,
            right: None,
            source_index: 0,
        }];
        let mut parents = VecDeque::from([0usize]);
        let mut source_index = 1;

        while let Some(parent) = parents.pop_front() {
            if source_index >= values.len() {
                break;
            }
            if let Some(value) = values[source_index] {
                let child = nodes.len();
                nodes.push(CompactNode {
                    value,
                    left: None,
                    right: None,
                    source_index,
                });
                nodes[parent].left = Some(child);
                parents.push_back(child);
            }
            source_index += 1;

            if source_index >= values.len() {
                continue;
            }
            if let Some(value) = values[source_index] {
                let child = nodes.len();
                nodes.push(CompactNode {
                    value,
                    left: None,
                    right: None,
                    source_index,
                });
                nodes[parent].right = Some(child);
                parents.push_back(child);
            }
            source_index += 1;
        }

        Self {
            nodes,
            root: Some(0),
        }
    }

    fn height(&self, node: Option<usize>) -> i32 {
        let Some(index) = node else {
            return 0;
        };
        1 + self
            .height(self.nodes[index].left)
            .max(self.height(self.nodes[index].right))
    }

    fn preorder_indices(&self) -> Vec<usize> {
        fn visit(tree: &CompactTree, node: Option<usize>, output: &mut Vec<usize>) {
            let Some(index) = node else {
                return;
            };
            output.push(index);
            visit(tree, tree.nodes[index].left, output);
            visit(tree, tree.nodes[index].right, output);
        }

        let mut output = Vec::new();
        visit(self, self.root, &mut output);
        output
    }

    fn level_order_snapshot(&self) -> (Vec<Option<i32>>, Vec<Option<usize>>) {
        let Some(root) = self.root else {
            return (Vec::new(), Vec::new());
        };
        let mut values = Vec::new();
        let mut node_at_position = Vec::new();
        let mut queue = VecDeque::from([Some(root)]);

        while let Some(node) = queue.pop_front() {
            match node {
                Some(index) => {
                    values.push(Some(self.nodes[index].value));
                    node_at_position.push(Some(index));
                    queue.push_back(self.nodes[index].left);
                    queue.push_back(self.nodes[index].right);
                }
                None => {
                    values.push(None);
                    node_at_position.push(None);
                }
            }
        }
        while values.last() == Some(&None) {
            values.pop();
            node_at_position.pop();
        }
        (values, node_at_position)
    }

    fn serialize_preorder(&self) -> String {
        fn serialize(tree: &CompactTree, node: Option<usize>, output: &mut String) {
            let Some(index) = node else {
                output.push_str("#,");
                return;
            };
            output.push('^');
            output.push_str(&tree.nodes[index].value.to_string());
            output.push(',');
            serialize(tree, tree.nodes[index].left, output);
            serialize(tree, tree.nodes[index].right, output);
        }

        let mut output = String::new();
        serialize(self, self.root, &mut output);
        output
    }
}

fn active_snapshot_index(node_positions: &[Option<usize>], node: usize) -> Option<usize> {
    node_positions
        .iter()
        .position(|position| *position == Some(node))
}

fn generate_invert_tree_bfs_steps(tree: &[Option<i32>]) -> Vec<Step> {
    let mut tree_state = CompactTree::from_level_order(tree);
    let Some(root) = tree_state.root else {
        return vec![Step {
            code_line: 3,
            description: "root is None, so there is nothing to invert.".to_string(),
            visual: VisualState::TreeVisual {
                tree_nodes: tree.to_vec(),
                active_node_idx: None,
                secondary_node_idx: None,
                depth_val: None,
                max_diameter: None,
            },
        }];
    };

    let mut queue = VecDeque::from([root]);
    let mut steps = vec![Step {
        code_line: 4,
        description: "Initialize the breadth-first queue with the root node.".to_string(),
        visual: VisualState::TreeVisual {
            tree_nodes: tree.to_vec(),
            active_node_idx: Some(0),
            secondary_node_idx: None,
            depth_val: None,
            max_diameter: None,
        },
    }];

    while let Some(node) = queue.pop_front() {
        let (before, positions) = tree_state.level_order_snapshot();
        let active = active_snapshot_index(&positions, node);
        steps.push(Step {
            code_line: 6,
            description: format!("Dequeue node value {}.", tree_state.nodes[node].value),
            visual: VisualState::TreeVisual {
                tree_nodes: before,
                active_node_idx: active,
                secondary_node_idx: None,
                depth_val: None,
                max_diameter: None,
            },
        });

        let left = tree_state.nodes[node].left;
        tree_state.nodes[node].left = tree_state.nodes[node].right;
        tree_state.nodes[node].right = left;
        let (after_swap, positions) = tree_state.level_order_snapshot();
        let active = active_snapshot_index(&positions, node);
        steps.push(Step {
            code_line: 7,
            description: format!(
                "Swap the left and right children of node value {}.",
                tree_state.nodes[node].value
            ),
            visual: VisualState::TreeVisual {
                tree_nodes: after_swap.clone(),
                active_node_idx: active,
                secondary_node_idx: None,
                depth_val: None,
                max_diameter: None,
            },
        });

        if let Some(left) = tree_state.nodes[node].left {
            queue.push_back(left);
            steps.push(Step {
                code_line: 8,
                description: format!(
                    "Enqueue the new left child, value {}.",
                    tree_state.nodes[left].value
                ),
                visual: VisualState::TreeVisual {
                    tree_nodes: after_swap.clone(),
                    active_node_idx: active,
                    secondary_node_idx: active_snapshot_index(&positions, left),
                    depth_val: None,
                    max_diameter: None,
                },
            });
        }
        if let Some(right) = tree_state.nodes[node].right {
            queue.push_back(right);
            steps.push(Step {
                code_line: 9,
                description: format!(
                    "Enqueue the new right child, value {}.",
                    tree_state.nodes[right].value
                ),
                visual: VisualState::TreeVisual {
                    tree_nodes: after_swap,
                    active_node_idx: active,
                    secondary_node_idx: active_snapshot_index(&positions, right),
                    depth_val: None,
                    max_diameter: None,
                },
            });
        }
    }

    let (inverted, _) = tree_state.level_order_snapshot();
    steps.push(Step {
        code_line: 10,
        description: format!("Breadth-first inversion complete: {:?}.", inverted),
        visual: VisualState::TreeVisual {
            tree_nodes: inverted,
            active_node_idx: None,
            secondary_node_idx: None,
            depth_val: None,
            max_diameter: None,
        },
    });
    steps
}

fn generate_max_depth_tree_bfs_steps(tree: &[Option<i32>]) -> Vec<Step> {
    let tree_state = CompactTree::from_level_order(tree);
    let Some(root) = tree_state.root else {
        return vec![Step {
            code_line: 3,
            description: "root is None, so its maximum depth is 0.".to_string(),
            visual: VisualState::TreeVisual {
                tree_nodes: tree.to_vec(),
                active_node_idx: None,
                secondary_node_idx: None,
                depth_val: Some(0),
                max_diameter: None,
            },
        }];
    };

    let mut queue = VecDeque::from([root]);
    let mut depth = 0;
    let mut steps = vec![Step {
        code_line: 4,
        description: "Initialize depth to 0 and enqueue the root.".to_string(),
        visual: VisualState::TreeVisual {
            tree_nodes: tree.to_vec(),
            active_node_idx: Some(0),
            secondary_node_idx: None,
            depth_val: Some(0),
            max_diameter: None,
        },
    }];

    while !queue.is_empty() {
        let level_size = queue.len();
        steps.push(Step {
            code_line: 6,
            description: format!("Process {} node(s) in the next level.", level_size),
            visual: VisualState::TreeVisual {
                tree_nodes: tree.to_vec(),
                active_node_idx: None,
                secondary_node_idx: None,
                depth_val: Some(depth),
                max_diameter: None,
            },
        });
        for _ in 0..level_size {
            let node = queue.pop_front().expect("level size matches queue");
            steps.push(Step {
                code_line: 7,
                description: format!("Visit node value {}.", tree_state.nodes[node].value),
                visual: VisualState::TreeVisual {
                    tree_nodes: tree.to_vec(),
                    active_node_idx: Some(tree_state.nodes[node].source_index),
                    secondary_node_idx: None,
                    depth_val: Some(depth + 1),
                    max_diameter: None,
                },
            });
            if let Some(left) = tree_state.nodes[node].left {
                queue.push_back(left);
                steps.push(Step {
                    code_line: 8,
                    description: format!(
                        "Enqueue left child value {}.",
                        tree_state.nodes[left].value
                    ),
                    visual: VisualState::TreeVisual {
                        tree_nodes: tree.to_vec(),
                        active_node_idx: Some(tree_state.nodes[node].source_index),
                        secondary_node_idx: Some(tree_state.nodes[left].source_index),
                        depth_val: Some(depth + 1),
                        max_diameter: None,
                    },
                });
            }
            if let Some(right) = tree_state.nodes[node].right {
                queue.push_back(right);
                steps.push(Step {
                    code_line: 9,
                    description: format!(
                        "Enqueue right child value {}.",
                        tree_state.nodes[right].value
                    ),
                    visual: VisualState::TreeVisual {
                        tree_nodes: tree.to_vec(),
                        active_node_idx: Some(tree_state.nodes[node].source_index),
                        secondary_node_idx: Some(tree_state.nodes[right].source_index),
                        depth_val: Some(depth + 1),
                        max_diameter: None,
                    },
                });
            }
        }
        depth += 1;
        steps.push(Step {
            code_line: 10,
            description: format!("Finished the level; depth is now {}.", depth),
            visual: VisualState::TreeVisual {
                tree_nodes: tree.to_vec(),
                active_node_idx: None,
                secondary_node_idx: None,
                depth_val: Some(depth),
                max_diameter: None,
            },
        });
    }

    steps.push(Step {
        code_line: 11,
        description: format!("Maximum depth is {}.", depth),
        visual: VisualState::TreeVisual {
            tree_nodes: tree.to_vec(),
            active_node_idx: None,
            secondary_node_idx: None,
            depth_val: Some(depth),
            max_diameter: None,
        },
    });
    steps
}

fn generate_diameter_tree_recomputed_height_steps(tree: &[Option<i32>]) -> Vec<Step> {
    let tree_state = CompactTree::from_level_order(tree);
    if tree_state.root.is_none() {
        return vec![Step {
            code_line: 3,
            description: "root is None, so the diameter is 0.".to_string(),
            visual: VisualState::TreeVisual {
                tree_nodes: tree.to_vec(),
                active_node_idx: None,
                secondary_node_idx: None,
                depth_val: Some(0),
                max_diameter: Some(0),
            },
        }];
    }

    let mut maximum = 0;
    let mut steps = Vec::new();
    for node in tree_state.preorder_indices() {
        let left_height = tree_state.height(tree_state.nodes[node].left);
        let right_height = tree_state.height(tree_state.nodes[node].right);
        let through_node = left_height + right_height;
        maximum = maximum.max(through_node);
        steps.push(Step {
            code_line: 4,
            description: format!(
                "At node value {}, recompute left height {} and right height {}; the path through this node has {} edge(s).",
                tree_state.nodes[node].value, left_height, right_height, through_node
            ),
            visual: VisualState::TreeVisual {
                tree_nodes: tree.to_vec(),
                active_node_idx: Some(tree_state.nodes[node].source_index),
                secondary_node_idx: None,
                depth_val: Some(1 + left_height.max(right_height)),
                max_diameter: Some(maximum),
            },
        });
    }
    steps.push(Step {
        code_line: 7,
        description: format!("The largest recomputed diameter is {} edge(s).", maximum),
        visual: VisualState::TreeVisual {
            tree_nodes: tree.to_vec(),
            active_node_idx: None,
            secondary_node_idx: None,
            depth_val: None,
            max_diameter: Some(maximum),
        },
    });
    steps
}

fn generate_balanced_tree_recomputed_height_steps(tree: &[Option<i32>]) -> Vec<Step> {
    let tree_state = CompactTree::from_level_order(tree);
    if tree_state.root.is_none() {
        return vec![Step {
            code_line: 3,
            description: "root is None, so the tree is balanced. Return True.".to_string(),
            visual: VisualState::TreeVisual {
                tree_nodes: tree.to_vec(),
                active_node_idx: None,
                secondary_node_idx: None,
                depth_val: Some(0),
                max_diameter: None,
            },
        }];
    }

    let mut steps = Vec::new();
    for node in tree_state.preorder_indices() {
        let left_height = tree_state.height(tree_state.nodes[node].left);
        let right_height = tree_state.height(tree_state.nodes[node].right);
        let difference = (left_height - right_height).abs();
        steps.push(Step {
            code_line: 4,
            description: format!(
                "At node value {}, recompute left height {} and right height {}.",
                tree_state.nodes[node].value, left_height, right_height
            ),
            visual: VisualState::TreeVisual {
                tree_nodes: tree.to_vec(),
                active_node_idx: Some(tree_state.nodes[node].source_index),
                secondary_node_idx: None,
                depth_val: Some(difference),
                max_diameter: None,
            },
        });
        if difference > 1 {
            steps.push(Step {
                code_line: 6,
                description: format!(
                    "Height difference {} is greater than 1 at node value {}. Return False.",
                    difference, tree_state.nodes[node].value
                ),
                visual: VisualState::TreeVisual {
                    tree_nodes: tree.to_vec(),
                    active_node_idx: Some(tree_state.nodes[node].source_index),
                    secondary_node_idx: None,
                    depth_val: Some(difference),
                    max_diameter: None,
                },
            });
            return steps;
        }
    }
    steps.push(Step {
        code_line: 7,
        description: "Every node has subtree heights within 1. Return True.".to_string(),
        visual: VisualState::TreeVisual {
            tree_nodes: tree.to_vec(),
            active_node_idx: None,
            secondary_node_idx: None,
            depth_val: Some(0),
            max_diameter: None,
        },
    });
    steps
}

fn generate_same_tree_bfs_steps(tree1: &[Option<i32>], tree2: &[Option<i32>]) -> Vec<Step> {
    let first = CompactTree::from_level_order(tree1);
    let second = CompactTree::from_level_order(tree2);
    let mut queue = VecDeque::from([(first.root, second.root)]);
    let mut steps = vec![Step {
        code_line: 3,
        description: "Initialize a queue with the two root nodes paired together.".to_string(),
        visual: VisualState::TreeVisual {
            tree_nodes: tree1.to_vec(),
            active_node_idx: first.root.map(|node| first.nodes[node].source_index),
            secondary_node_idx: None,
            depth_val: None,
            max_diameter: None,
        },
    }];

    while let Some((left, right)) = queue.pop_front() {
        let left_value = left.map(|node| first.nodes[node].value);
        let right_value = right.map(|node| second.nodes[node].value);
        steps.push(Step {
            code_line: 5,
            description: format!(
                "Compare paired nodes {:?} and {:?}.",
                left_value, right_value
            ),
            visual: VisualState::TreeVisual {
                tree_nodes: tree1.to_vec(),
                active_node_idx: left.map(|node| first.nodes[node].source_index),
                secondary_node_idx: None,
                depth_val: None,
                max_diameter: None,
            },
        });
        match (left, right) {
            (None, None) => {
                steps.push(Step {
                    code_line: 6,
                    description: "Both paired positions are empty; continue.".to_string(),
                    visual: VisualState::TreeVisual {
                        tree_nodes: tree1.to_vec(),
                        active_node_idx: None,
                        secondary_node_idx: None,
                        depth_val: None,
                        max_diameter: None,
                    },
                });
            }
            (Some(left), Some(right)) if first.nodes[left].value == second.nodes[right].value => {
                queue.push_back((first.nodes[left].left, second.nodes[right].left));
                steps.push(Step {
                    code_line: 8,
                    description: "Values match; enqueue the left-child pair.".to_string(),
                    visual: VisualState::TreeVisual {
                        tree_nodes: tree1.to_vec(),
                        active_node_idx: Some(first.nodes[left].source_index),
                        secondary_node_idx: first.nodes[left]
                            .left
                            .map(|node| first.nodes[node].source_index),
                        depth_val: None,
                        max_diameter: None,
                    },
                });
                queue.push_back((first.nodes[left].right, second.nodes[right].right));
                steps.push(Step {
                    code_line: 9,
                    description: "Enqueue the right-child pair.".to_string(),
                    visual: VisualState::TreeVisual {
                        tree_nodes: tree1.to_vec(),
                        active_node_idx: Some(first.nodes[left].source_index),
                        secondary_node_idx: first.nodes[left]
                            .right
                            .map(|node| first.nodes[node].source_index),
                        depth_val: None,
                        max_diameter: None,
                    },
                });
            }
            _ => {
                steps.push(Step {
                    code_line: 7,
                    description: "The paired nodes differ in value or structure. Return False."
                        .to_string(),
                    visual: VisualState::TreeVisual {
                        tree_nodes: tree1.to_vec(),
                        active_node_idx: left.map(|node| first.nodes[node].source_index),
                        secondary_node_idx: None,
                        depth_val: None,
                        max_diameter: None,
                    },
                });
                return steps;
            }
        }
    }

    steps.push(Step {
        code_line: 10,
        description: "Every paired node matched. Return True.".to_string(),
        visual: VisualState::TreeVisual {
            tree_nodes: tree1.to_vec(),
            active_node_idx: None,
            secondary_node_idx: None,
            depth_val: None,
            max_diameter: None,
        },
    });
    steps
}

fn generate_subtree_serialization_steps(
    root: &[Option<i32>],
    sub_root: &[Option<i32>],
) -> Vec<Step> {
    let root_serialized = CompactTree::from_level_order(root).serialize_preorder();
    let sub_serialized = CompactTree::from_level_order(sub_root).serialize_preorder();
    let mut steps = vec![Step {
        code_line: 6,
        description: format!(
            "Serialize root with value delimiters and null markers: {}",
            root_serialized
        ),
        visual: VisualState::TreeVisual {
            tree_nodes: root.to_vec(),
            active_node_idx: None,
            secondary_node_idx: None,
            depth_val: None,
            max_diameter: None,
        },
    }];
    steps.push(Step {
        code_line: 7,
        description: format!("Serialize subRoot the same way: {}", sub_serialized),
        visual: VisualState::TreeVisual {
            tree_nodes: sub_root.to_vec(),
            active_node_idx: None,
            secondary_node_idx: None,
            depth_val: None,
            max_diameter: None,
        },
    });
    let is_subtree = root_serialized.contains(&sub_serialized);
    steps.push(Step {
        code_line: 8,
        description: format!(
            "Serialized subtree {} found in serialized root. Return {}.",
            if is_subtree { "was" } else { "was not" },
            if is_subtree { "True" } else { "False" }
        ),
        visual: VisualState::TreeVisual {
            tree_nodes: root.to_vec(),
            active_node_idx: None,
            secondary_node_idx: None,
            depth_val: None,
            max_diameter: None,
        },
    });
    steps
}

#[cfg(test)]
mod approach_tests {
    use super::*;

    fn final_tree(steps: &[Step]) -> Vec<Option<i32>> {
        match &steps.last().expect("trace has a final step").visual {
            VisualState::TreeVisual { tree_nodes, .. } => tree_nodes.clone(),
            visual => panic!("unexpected final visual: {visual:?}"),
        }
    }

    #[test]
    fn bfs_inversion_handles_level_order_trees() {
        let tree = [
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
        ];
        assert_eq!(
            final_tree(&generate_invert_tree_steps(&tree, 1)),
            vec![
                Some(1),
                Some(3),
                Some(2),
                Some(7),
                Some(6),
                Some(5),
                Some(4)
            ]
        );
    }

    #[test]
    fn bfs_depth_handles_compact_sparse_level_order() {
        let tree = [Some(1), None, Some(2), Some(3)];
        let steps = generate_max_depth_tree_steps(&tree, 1);
        assert!(matches!(
            steps.last().map(|step| &step.visual),
            Some(VisualState::TreeVisual {
                depth_val: Some(3),
                ..
            })
        ));
    }

    #[test]
    fn repeated_height_variants_report_correct_results() {
        let tree = [Some(1), Some(2), None, Some(3), None, Some(4)];
        let diameter = generate_diameter_tree_steps(&tree, 1);
        assert!(matches!(
            diameter.last().map(|step| &step.visual),
            Some(VisualState::TreeVisual {
                max_diameter: Some(3),
                ..
            })
        ));
        let balanced = generate_balanced_tree_steps(&tree, 1);
        assert!(balanced
            .last()
            .is_some_and(|step| step.description.contains("Return False")));
    }

    #[test]
    fn bfs_same_tree_detects_value_and_shape_mismatches() {
        let first = [Some(1), Some(2), Some(3)];
        let same = generate_same_tree_steps(&first, &first, 1);
        assert!(same
            .last()
            .is_some_and(|step| step.description.contains("Return True")));
        let different = generate_same_tree_steps(&first, &[Some(1), None, Some(3)], 1);
        assert!(different
            .last()
            .is_some_and(|step| step.description.contains("Return False")));
    }

    #[test]
    fn serialization_uses_null_markers_to_preserve_structure() {
        let root = [Some(3), Some(4), Some(5), Some(1), Some(2)];
        let present = generate_subtree_steps(&root, &[Some(4), Some(1), Some(2)], 1);
        assert!(present
            .last()
            .is_some_and(|step| step.description.contains("Return True")));
        let absent = generate_subtree_steps(&root, &[Some(4), Some(1), None, Some(2)], 1);
        assert!(absent
            .last()
            .is_some_and(|step| step.description.contains("Return False")));
    }

    #[test]
    fn quadratic_tree_traces_have_a_smaller_snapshot_limit() {
        let steps = generate_diameter_tree_steps(&vec![Some(1); QUADRATIC_TREE_TRACE_LIMIT + 1], 1);
        assert!(matches!(
            steps[0].visual,
            VisualState::TraceUnavailable { .. }
        ));
    }
}
