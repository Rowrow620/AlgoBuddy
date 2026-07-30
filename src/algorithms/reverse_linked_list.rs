use crate::model::{Step, VisualState};

pub fn generate_reverse_linked_list_steps(nodes: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();
    let nodes_vec = nodes.to_vec();
    let n = nodes.len();

    if nodes.is_empty() {
        steps.push(Step {
            code_line: 13,
            description: "Linked list is empty (head = None). Returning None.".to_string(),
            visual: VisualState::LinkedList {
                nodes: nodes_vec,
                prev_idx: None,
                curr_idx: None,
                next_idx: None,
                reversed_so_far: Vec::new(),
            },
        });
        return steps;
    }

    let mut prev: Option<usize> = None;
    let mut curr: Option<usize> = Some(0);
    let mut reversed_so_far: Vec<i32> = Vec::new();

    // 1. Pointer init (code_line 7)
    steps.push(Step {
        code_line: 7,
        description: format!(
            "Initialized prev = None and curr = index 0 (val={}).",
            nodes[0]
        ),
        visual: VisualState::LinkedList {
            nodes: nodes_vec.clone(),
            prev_idx: prev,
            curr_idx: curr,
            next_idx: if n > 1 { Some(1) } else { None },
            reversed_so_far: reversed_so_far.clone(),
        },
    });

    // 2. Loop while curr (code_line 8)
    while let Some(c_idx) = curr {
        let nxt = if c_idx + 1 < n { Some(c_idx + 1) } else { None };

        // Save nxt (code_line 9)
        steps.push(Step {
            code_line: 9,
            description: format!(
                "Saved next pointer nxt = {:?}.",
                nxt.map(|i| format!("idx {} (val={})", i, nodes[i]))
            ),
            visual: VisualState::LinkedList {
                nodes: nodes_vec.clone(),
                prev_idx: prev,
                curr_idx: Some(c_idx),
                next_idx: nxt,
                reversed_so_far: reversed_so_far.clone(),
            },
        });

        // Flip pointer (code_line 10)
        reversed_so_far.insert(0, nodes[c_idx]);
        steps.push(Step {
            code_line: 10,
            description: format!(
                "Reversed link: node[{}] (val={}) now points to prev ({:?}).",
                c_idx,
                nodes[c_idx],
                prev.map(|p| nodes[p])
            ),
            visual: VisualState::LinkedList {
                nodes: nodes_vec.clone(),
                prev_idx: prev,
                curr_idx: Some(c_idx),
                next_idx: nxt,
                reversed_so_far: reversed_so_far.clone(),
            },
        });

        // Shift prev = curr (code_line 11)
        prev = Some(c_idx);
        steps.push(Step {
            code_line: 11,
            description: format!(
                "Advanced prev = curr (prev now at node[{}] val={}).",
                c_idx, nodes[c_idx]
            ),
            visual: VisualState::LinkedList {
                nodes: nodes_vec.clone(),
                prev_idx: prev,
                curr_idx: Some(c_idx),
                next_idx: nxt,
                reversed_so_far: reversed_so_far.clone(),
            },
        });

        // Shift curr = nxt (code_line 12)
        curr = nxt;
        steps.push(Step {
            code_line: 12,
            description: format!(
                "Advanced curr = nxt ({:?}).",
                curr.map(|i| format!("idx {} (val={})", i, nodes[i]))
            ),
            visual: VisualState::LinkedList {
                nodes: nodes_vec.clone(),
                prev_idx: prev,
                curr_idx: curr,
                next_idx: if let Some(ci) = curr {
                    if ci + 1 < n {
                        Some(ci + 1)
                    } else {
                        None
                    }
                } else {
                    None
                },
                reversed_so_far: reversed_so_far.clone(),
            },
        });
    }

    steps.push(Step {
        code_line: 13,
        description: format!(
            "Reversal complete! New head is node val={}. Reversed list: {:?}.",
            reversed_so_far.first().unwrap_or(&0),
            reversed_so_far
        ),
        visual: VisualState::LinkedList {
            nodes: nodes_vec,
            prev_idx: prev,
            curr_idx: None,
            next_idx: None,
            reversed_so_far,
        },
    });

    steps
}
