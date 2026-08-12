use crate::model::{Step, VisualState};

const REVERSE_LINKED_LIST_TRACE_LIMIT: usize = 128;

pub fn generate_reverse_linked_list_steps(nodes: &[i32], approach_id: usize) -> Vec<Step> {
    if nodes.len() > REVERSE_LINKED_LIST_TRACE_LIMIT {
        return vec![Step {
            code_line: 3,
            description: format!(
                "Reverse Linked List visualization supports up to {} nodes; shorten the input to build the detailed trace.",
                REVERSE_LINKED_LIST_TRACE_LIMIT
            ),
            visual: VisualState::TraceUnavailable {
                message: format!(
                    "Detailed linked-list traces accept at most {} nodes because every step stores a list snapshot.",
                    REVERSE_LINKED_LIST_TRACE_LIMIT
                ),
            },
        }];
    }

    match approach_id {
        0 => generate_iterative_steps(nodes),
        1 => generate_recursive_steps(nodes),
        _ => Vec::new(),
    }
}

fn generate_iterative_steps(nodes: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();
    let nodes_vec = nodes.to_vec();
    let n = nodes.len();

    if nodes.is_empty() {
        steps.push(Step {
            code_line: 9,
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

    steps.push(Step {
        code_line: 3,
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

    while let Some(c_idx) = curr {
        let nxt = if c_idx + 1 < n { Some(c_idx + 1) } else { None };

        steps.push(Step {
            code_line: 5,
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

        reversed_so_far.insert(0, nodes[c_idx]);
        steps.push(Step {
            code_line: 6,
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

        prev = Some(c_idx);
        steps.push(Step {
            code_line: 7,
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

        curr = nxt;
        steps.push(Step {
            code_line: 8,
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
        code_line: 9,
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

fn generate_recursive_steps(nodes: &[i32]) -> Vec<Step> {
    let nodes_vec = nodes.to_vec();
    if nodes.is_empty() {
        return vec![Step {
            code_line: 3,
            description: "head is None, so the recursive base case returns None.".to_string(),
            visual: VisualState::LinkedList {
                nodes: nodes_vec,
                prev_idx: None,
                curr_idx: None,
                next_idx: None,
                reversed_so_far: Vec::new(),
            },
        }];
    }

    let mut steps = Vec::new();
    for index in 0..nodes.len().saturating_sub(1) {
        steps.push(Step {
            code_line: 4,
            description: format!(
                "Recurse from node[{}] = {} to node[{}] = {}.",
                index,
                nodes[index],
                index + 1,
                nodes[index + 1]
            ),
            visual: VisualState::LinkedList {
                nodes: nodes_vec.clone(),
                prev_idx: None,
                curr_idx: Some(index),
                next_idx: Some(index + 1),
                reversed_so_far: Vec::new(),
            },
        });
    }

    let tail_index = nodes.len() - 1;
    let mut reversed = vec![nodes[tail_index]];
    steps.push(Step {
        code_line: 3,
        description: format!(
            "node[{}] = {} has no next node, so it becomes the new head.",
            tail_index, nodes[tail_index]
        ),
        visual: VisualState::LinkedList {
            nodes: nodes_vec.clone(),
            prev_idx: None,
            curr_idx: Some(tail_index),
            next_idx: None,
            reversed_so_far: reversed.clone(),
        },
    });

    for index in (0..tail_index).rev() {
        reversed.push(nodes[index]);
        steps.push(Step {
            code_line: 5,
            description: format!(
                "Unwinding at node[{}] = {}: make its next node point back to it.",
                index, nodes[index]
            ),
            visual: VisualState::LinkedList {
                nodes: nodes_vec.clone(),
                prev_idx: Some(index + 1),
                curr_idx: Some(index),
                next_idx: Some(index + 1),
                reversed_so_far: reversed.clone(),
            },
        });
        steps.push(Step {
            code_line: 6,
            description: format!(
                "Clear the original next pointer from node[{}] to avoid a cycle.",
                index
            ),
            visual: VisualState::LinkedList {
                nodes: nodes_vec.clone(),
                prev_idx: Some(index + 1),
                curr_idx: Some(index),
                next_idx: None,
                reversed_so_far: reversed.clone(),
            },
        });
    }

    steps.push(Step {
        code_line: 7,
        description: format!("Recursive reversal complete: {:?}.", reversed),
        visual: VisualState::LinkedList {
            nodes: nodes_vec,
            prev_idx: Some(tail_index),
            curr_idx: None,
            next_idx: None,
            reversed_so_far: reversed,
        },
    });
    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    fn final_values(steps: &[Step]) -> Vec<i32> {
        match &steps.last().expect("trace has a final step").visual {
            VisualState::LinkedList {
                reversed_so_far, ..
            } => reversed_so_far.clone(),
            visual => panic!("unexpected final visual: {visual:?}"),
        }
    }

    #[test]
    fn both_reverse_approaches_return_the_same_values() {
        let nodes = [0, 1, 2, 3];
        assert_eq!(
            final_values(&generate_reverse_linked_list_steps(&nodes, 0)),
            vec![3, 2, 1, 0]
        );
        assert_eq!(
            final_values(&generate_reverse_linked_list_steps(&nodes, 1)),
            vec![3, 2, 1, 0]
        );
    }

    #[test]
    fn recursive_trace_rejects_inputs_that_are_too_large_to_snapshot() {
        let steps =
            generate_reverse_linked_list_steps(&vec![0; REVERSE_LINKED_LIST_TRACE_LIMIT + 1], 1);
        assert!(matches!(
            steps[0].visual,
            VisualState::TraceUnavailable { .. }
        ));
    }
}
