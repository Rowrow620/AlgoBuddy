use crate::model::visual_state::{Step, VisualState};

/// #143 Reorder List
pub fn generate_reorder_list_steps(nodes: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();
    if nodes.is_empty() {
        return vec![Step {
            description: "Empty list, no reordering needed.".to_string(),
            code_line: 1,
            visual: VisualState::Array1D {
                title: "Reorder List".to_string(),
                active_idx: None,
                secondary_idx: None,
                elements: vec![],
                pointers: vec![],
                status_message: "Empty list.".to_string(),
                is_success: None,
            },
        }];
    }

    let n = nodes.len();
    steps.push(Step {
        description: format!("Initial list of {} nodes.", n),
        code_line: 1,
        visual: VisualState::Array1D {
            title: "Reorder List".to_string(),
            active_idx: Some(0),
            secondary_idx: None,
            elements: nodes.to_vec(),
            pointers: vec![("head", 0)],
            status_message: "Starting reorder process.".to_string(),
            is_success: None,
        },
    });

    let mut reordered = Vec::new();
    let mut left = 0;
    let mut right = n - 1;

    while left <= right {
        if left == right {
            reordered.push(nodes[left]);
            steps.push(Step {
                description: format!("Append middle node val={}.", nodes[left]),
                code_line: 10,
                visual: VisualState::Array1D {
                    title: "Reorder List".to_string(),
                    active_idx: Some(left),
                    secondary_idx: None,
                    elements: reordered.clone(),
                    pointers: vec![("mid", reordered.len() - 1)],
                    status_message: format!("Appended val={}.", nodes[left]),
                    is_success: None,
                },
            });
            break;
        }

        reordered.push(nodes[left]);
        steps.push(Step {
            description: format!("Append left node L{} (val={}).", left, nodes[left]),
            code_line: 8,
            visual: VisualState::Array1D {
                title: "Reorder List".to_string(),
                active_idx: Some(reordered.len() - 1),
                secondary_idx: None,
                elements: reordered.clone(),
                pointers: vec![("left", reordered.len() - 1)],
                status_message: format!("Appended L{} (val={}).", left, nodes[left]),
                is_success: None,
            },
        });

        reordered.push(nodes[right]);
        steps.push(Step {
            description: format!("Append right node R{} (val={}).", right, nodes[right]),
            code_line: 9,
            visual: VisualState::Array1D {
                title: "Reorder List".to_string(),
                active_idx: Some(reordered.len() - 1),
                secondary_idx: None,
                elements: reordered.clone(),
                pointers: vec![("right", reordered.len() - 1)],
                status_message: format!("Appended R{} (val={}).", right, nodes[right]),
                is_success: None,
            },
        });

        left += 1;
        if right > 0 {
            right -= 1;
        }
    }

    steps.push(Step {
        description: "Reordering complete!".to_string(),
        code_line: 12,
        visual: VisualState::Array1D {
            title: "Reordered List Result".to_string(),
            active_idx: None,
            secondary_idx: None,
            elements: reordered,
            pointers: vec![("head", 0)],
            status_message: "Reordered list formed.".to_string(),
            is_success: Some(true),
        },
    });

    steps
}

/// #19 Remove Nth Node From End of List
pub fn generate_remove_nth_node_steps(nodes: &[i32], n: usize) -> Vec<Step> {
    let mut steps = Vec::new();
    let sz = nodes.len();
    if sz == 0 || n == 0 || n > sz {
        return vec![Step {
            description: "Invalid input or empty list.".to_string(),
            code_line: 1,
            visual: VisualState::Array1D {
                title: "Remove Nth Node From End".to_string(),
                active_idx: None,
                secondary_idx: None,
                elements: nodes.to_vec(),
                pointers: vec![],
                status_message: "Invalid input.".to_string(),
                is_success: None,
            },
        }];
    }

    steps.push(Step {
        description: format!("Initialize fast and slow pointers to remove {}th node from end.", n),
        code_line: 2,
        visual: VisualState::Array1D {
            title: "Remove Nth Node From End".to_string(),
            active_idx: Some(0),
            secondary_idx: Some(0),
            elements: nodes.to_vec(),
            pointers: vec![("fast", 0), ("slow", 0)],
            status_message: format!("Target: {}th from end.", n),
            is_success: None,
        },
    });

    let target_idx = sz - n;
    let mut result = nodes.to_vec();
    result.remove(target_idx);

    steps.push(Step {
        description: format!("Advance fast pointer by {} nodes.", n),
        code_line: 4,
        visual: VisualState::Array1D {
            title: "Remove Nth Node From End".to_string(),
            active_idx: Some(n.min(sz - 1)),
            secondary_idx: Some(0),
            elements: nodes.to_vec(),
            pointers: vec![("fast", n.min(sz - 1)), ("slow", 0)],
            status_message: format!("Fast advanced by {}.", n),
            is_success: None,
        },
    });

    steps.push(Step {
        description: format!("Move both pointers until fast reaches end. Slow points to node at index {}.", target_idx),
        code_line: 6,
        visual: VisualState::Array1D {
            title: "Remove Nth Node From End".to_string(),
            active_idx: Some(target_idx),
            secondary_idx: None,
            elements: nodes.to_vec(),
            pointers: vec![("remove", target_idx)],
            status_message: format!("Node at index {} selected for removal.", target_idx),
            is_success: None,
        },
    });

    steps.push(Step {
        description: format!("Bypass node at index {} (val={}).", target_idx, nodes[target_idx]),
        code_line: 8,
        visual: VisualState::Array1D {
            title: "Resulting Linked List".to_string(),
            active_idx: None,
            secondary_idx: None,
            elements: result,
            pointers: vec![("head", 0)],
            status_message: "Node successfully removed.".to_string(),
            is_success: Some(true),
        },
    });

    steps
}

/// #138 Copy List With Random Pointer
pub fn generate_copy_list_random_steps(nodes: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();
    steps.push(Step {
        description: "Original list with random pointers initialized.".to_string(),
        code_line: 1,
        visual: VisualState::Array1D {
            title: "Copy List with Random Pointer".to_string(),
            active_idx: Some(0),
            secondary_idx: None,
            elements: nodes.to_vec(),
            pointers: vec![("head", 0)],
            status_message: "Original list.".to_string(),
            is_success: None,
        },
    });

    let mut copied = Vec::new();
    for (i, &val) in nodes.iter().enumerate() {
        copied.push(val);
        steps.push(Step {
            description: format!("Deep copy node at index {} with val={}.", i, val),
            code_line: 5,
            visual: VisualState::Array1D {
                title: "Copy List with Random Pointer".to_string(),
                active_idx: Some(i),
                secondary_idx: None,
                elements: copied.clone(),
                pointers: vec![("curr_copy", i)],
                status_message: format!("Created deep copy of node {}.", i),
                is_success: None,
            },
        });
    }

    steps.push(Step {
        description: "Deep copy with random pointers fully constructed.".to_string(),
        code_line: 12,
        visual: VisualState::Array1D {
            title: "Cloned List Result".to_string(),
            active_idx: None,
            secondary_idx: None,
            elements: copied,
            pointers: vec![("copy_head", 0)],
            status_message: "Cloned list ready.".to_string(),
            is_success: Some(true),
        },
    });

    steps
}

/// #2 Add Two Numbers
pub fn generate_add_two_numbers_steps(l1: &[i32], l2: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut i = 0;
    let mut j = 0;
    let mut carry = 0;
    let mut result = Vec::new();

    steps.push(Step {
        description: "Initialize addition of two linked lists stored in reverse order.".to_string(),
        code_line: 1,
        visual: VisualState::Array1D {
            title: "Add Two Numbers".to_string(),
            active_idx: None,
            secondary_idx: None,
            elements: result.clone(),
            pointers: vec![],
            status_message: "Carry = 0.".to_string(),
            is_success: None,
        },
    });

    while i < l1.len() || j < l2.len() || carry > 0 {
        let v1 = if i < l1.len() { l1[i] } else { 0 };
        let v2 = if j < l2.len() { l2[j] } else { 0 };
        let sum = v1 + v2 + carry;
        let digit = sum % 10;
        carry = sum / 10;
        result.push(digit);

        steps.push(Step {
            description: format!("Add v1={} + v2={} + carry -> sum={}, digit={}, new carry={}.", v1, v2, sum, digit, carry),
            code_line: 6,
            visual: VisualState::Array1D {
                title: "Add Two Numbers".to_string(),
                active_idx: Some(result.len() - 1),
                secondary_idx: None,
                elements: result.clone(),
                pointers: vec![("digit", result.len() - 1)],
                status_message: format!("Sum digit: {}, Carry: {}", digit, carry),
                is_success: None,
            },
        });

        i += 1;
        j += 1;
    }

    steps.push(Step {
        description: "Sum list computation complete.".to_string(),
        code_line: 10,
        visual: VisualState::Array1D {
            title: "Sum Result List".to_string(),
            active_idx: None,
            secondary_idx: None,
            elements: result,
            pointers: vec![("res_head", 0)],
            status_message: "Addition complete.".to_string(),
            is_success: Some(true),
        },
    });

    steps
}

/// #287 Find The Duplicate Number
pub fn generate_find_duplicate_number_steps(nums: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();
    if nums.is_empty() { return steps; }

    steps.push(Step {
        description: "Floyd's Tortoise and Hare algorithm to find duplicate in O(1) space.".to_string(),
        code_line: 1,
        visual: VisualState::Array1D {
            title: "Find Duplicate Number".to_string(),
            active_idx: Some(0),
            secondary_idx: Some(0),
            elements: nums.to_vec(),
            pointers: vec![("slow", 0), ("fast", 0)],
            status_message: "Initialized slow & fast at index 0.".to_string(),
            is_success: None,
        },
    });

    let mut slow = nums[0] as usize;
    let mut fast = nums[nums[0] as usize] as usize;

    while slow != fast {
        steps.push(Step {
            description: format!("Advance slow -> {}, fast -> {}.", slow, fast),
            code_line: 5,
            visual: VisualState::Array1D {
                title: "Find Duplicate Number".to_string(),
                active_idx: Some(slow.min(nums.len() - 1)),
                secondary_idx: Some(fast.min(nums.len() - 1)),
                elements: nums.to_vec(),
                pointers: vec![("slow", slow.min(nums.len() - 1)), ("fast", fast.min(nums.len() - 1))],
                status_message: format!("slow={}, fast={}", slow, fast),
                is_success: None,
            },
        });
        slow = nums[slow] as usize;
        fast = nums[nums[fast] as usize] as usize;
    }

    let mut slow2 = 0;
    while slow != slow2 {
        slow = nums[slow] as usize;
        slow2 = nums[slow2] as usize;
    }

    steps.push(Step {
        description: format!("Found duplicate number val={}.", slow),
        code_line: 12,
        visual: VisualState::Array1D {
            title: "Duplicate Found".to_string(),
            active_idx: Some(slow.min(nums.len() - 1)),
            secondary_idx: None,
            elements: nums.to_vec(),
            pointers: vec![("duplicate", slow.min(nums.len() - 1))],
            status_message: format!("Duplicate is {}.", slow),
            is_success: Some(true),
        },
    });

    steps
}

/// #146 LRU Cache
pub fn generate_lru_cache_steps(capacity: usize, ops: &[(&str, i32, i32)]) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut cache = Vec::new(); // (key, val)

    steps.push(Step {
        description: format!("Initialize LRU Cache with capacity = {}.", capacity),
        code_line: 1,
        visual: VisualState::Array1D {
            title: "LRU Cache State".to_string(),
            active_idx: None,
            secondary_idx: None,
            elements: vec![],
            pointers: vec![],
            status_message: format!("Capacity: {}", capacity),
            is_success: None,
        },
    });

    for op in ops {
        match op.0 {
            "put" => {
                let key = op.1;
                let val = op.2;
                cache.retain(|&(k, _)| k != key);
                cache.insert(0, (key, val));
                if cache.len() > capacity {
                    cache.pop();
                }
                let vals: Vec<i32> = cache.iter().map(|&(_, v)| v).collect();
                steps.push(Step {
                    description: format!("Put key={}, val={}. Cache size={}/{}.", key, val, cache.len(), capacity),
                    code_line: 6,
                    visual: VisualState::Array1D {
                        title: "LRU Cache State".to_string(),
                        active_idx: Some(0),
                        secondary_idx: None,
                        elements: vals,
                        pointers: vec![("MRU", 0)],
                        status_message: format!("Put ({}, {})", key, val),
                        is_success: None,
                    },
                });
            }
            "get" => {
                let key = op.1;
                let found = cache.iter().position(|&(k, _)| k == key);
                if let Some(idx) = found {
                    let item = cache.remove(idx);
                    cache.insert(0, item);
                    let vals: Vec<i32> = cache.iter().map(|&(_, v)| v).collect();
                    steps.push(Step {
                        description: format!("Get key={} -> HIT (val={}). Promoted to MRU.", key, item.1),
                        code_line: 12,
                        visual: VisualState::Array1D {
                            title: "LRU Cache State".to_string(),
                            active_idx: Some(0),
                            secondary_idx: None,
                            elements: vals,
                            pointers: vec![("HIT", 0)],
                            status_message: format!("Get {} -> {}", key, item.1),
                            is_success: Some(true),
                        },
                    });
                } else {
                    let vals: Vec<i32> = cache.iter().map(|&(_, v)| v).collect();
                    steps.push(Step {
                        description: format!("Get key={} -> MISS (-1).", key),
                        code_line: 14,
                        visual: VisualState::Array1D {
                            title: "LRU Cache State".to_string(),
                            active_idx: None,
                            secondary_idx: None,
                            elements: vals,
                            pointers: vec![],
                            status_message: format!("Get {} -> -1", key),
                            is_success: Some(false),
                        },
                    });
                }
            }
            _ => {}
        }
    }

    steps
}

/// #23 Merge K Sorted Lists
pub fn generate_merge_k_lists_steps(lists: &[Vec<i32>]) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut merged = Vec::new();
    for l in lists {
        merged.extend(l.clone());
    }
    merged.sort_unstable();

    steps.push(Step {
        description: format!("Merge {} sorted lists using Min-Heap / Divide & Conquer.", lists.len()),
        code_line: 1,
        visual: VisualState::Array1D {
            title: "Merge K Sorted Lists".to_string(),
            active_idx: None,
            secondary_idx: None,
            elements: merged.clone(),
            pointers: vec![("merged", 0)],
            status_message: format!("Merged list of {} elements.", merged.len()),
            is_success: Some(true),
        },
    });

    steps
}

/// #25 Reverse Nodes In K-Group
pub fn generate_reverse_k_group_steps(nodes: &[i32], k: usize) -> Vec<Step> {
    let mut steps = Vec::new();
    if nodes.is_empty() || k <= 1 {
        return vec![Step {
            description: "No reversing needed.".to_string(),
            code_line: 1,
            visual: VisualState::Array1D {
                title: "Reverse Nodes in K-Group".to_string(),
                active_idx: None,
                secondary_idx: None,
                elements: nodes.to_vec(),
                pointers: vec![],
                status_message: "Done.".to_string(),
                is_success: None,
            },
        }];
    }

    let mut result = Vec::new();
    let chunks = nodes.chunks(k);
    for chunk in chunks {
        if chunk.len() == k {
            let mut rev = chunk.to_vec();
            rev.reverse();
            result.extend(rev);
        } else {
            result.extend(chunk);
        }
    }

    steps.push(Step {
        description: format!("Reverse nodes in groups of k={}.", k),
        code_line: 5,
        visual: VisualState::Array1D {
            title: "K-Group Reversal Result".to_string(),
            active_idx: None,
            secondary_idx: None,
            elements: result,
            pointers: vec![("head", 0)],
            status_message: format!("k-group reversed (k={}).", k),
            is_success: Some(true),
        },
    });

    steps
}
