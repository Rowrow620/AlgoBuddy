use crate::model::{Step, VisualState};
use std::collections::BTreeSet;

const LINKED_LIST_CYCLE_TRACE_LIMIT: usize = 128;

pub fn generate_linked_list_cycle_steps(
    nodes: &[i32],
    cycle_index: i32,
    approach_id: usize,
) -> Vec<Step> {
    if nodes.len() > LINKED_LIST_CYCLE_TRACE_LIMIT {
        return vec![Step {
            code_line: 3,
            description: format!(
                "Linked List Cycle visualization supports up to {} nodes; shorten the input to build the detailed trace.",
                LINKED_LIST_CYCLE_TRACE_LIMIT
            ),
            visual: VisualState::TraceUnavailable {
                message: format!(
                    "Detailed cycle traces accept at most {} nodes because every pointer move stores a list snapshot.",
                    LINKED_LIST_CYCLE_TRACE_LIMIT
                ),
            },
        }];
    }

    match approach_id {
        0 => generate_floyd_steps(nodes, cycle_index),
        1 => generate_visited_set_steps(nodes, cycle_index),
        _ => Vec::new(),
    }
}

fn generate_floyd_steps(nodes: &[i32], cycle_index: i32) -> Vec<Step> {
    let mut steps = Vec::new();
    let nodes_vec = nodes.to_vec();
    let n = nodes.len();

    let cycle_target = if cycle_index >= 0 && (cycle_index as usize) < n {
        Some(cycle_index as usize)
    } else {
        None
    };

    if n == 0 {
        steps.push(Step {
            code_line: 7,
            description: "Empty list has no cycle. Return False.".to_string(),
            visual: VisualState::LinkedListCycle {
                nodes: nodes_vec,
                cycle_target_idx: None,
                slow_idx: None,
                fast_idx: None,
                visited_indices: BTreeSet::new(),
                has_cycle: Some(false),
            },
        });
        return steps;
    }

    let mut slow = 0;
    let mut fast = 0;

    // 1. Pointer init (code_line 3)
    steps.push(Step {
        code_line: 3,
        description: format!(
            "Initialized slow = index 0 (val={}) and fast = index 0 (val={}). Cycle target = {:?}.",
            nodes[0], nodes[0], cycle_target
        ),
        visual: VisualState::LinkedListCycle {
            nodes: nodes_vec.clone(),
            cycle_target_idx: cycle_target,
            slow_idx: Some(slow),
            fast_idx: Some(fast),
            visited_indices: BTreeSet::new(),
            has_cycle: None,
        },
    });

    let mut step_count = 0;
    let max_steps = n * 2 + 4;

    // Helper to get next node index considering cycle
    let get_next = |curr: usize| -> Option<usize> {
        if curr + 1 < n {
            Some(curr + 1)
        } else {
            cycle_target
        }
    };

    while step_count < max_steps {
        step_count += 1;

        // Advance slow 1 step
        let next_slow = match get_next(slow) {
            Some(s) => s,
            None => break,
        };
        slow = next_slow;

        // Advance fast 2 steps
        let fast_1 = match get_next(fast) {
            Some(f) => f,
            None => break,
        };
        let fast_2 = match get_next(fast_1) {
            Some(f) => f,
            None => break,
        };
        fast = fast_2;

        steps.push(Step {
            code_line: 5,
            description: format!("Advanced slow by 1 step to index {} (val={}) and fast by 2 steps to index {} (val={}).", slow, nodes[slow], fast, nodes[fast]),
            visual: VisualState::LinkedListCycle {
                nodes: nodes_vec.clone(),
                cycle_target_idx: cycle_target,
                slow_idx: Some(slow),
                fast_idx: Some(fast),
                visited_indices: BTreeSet::new(),
                has_cycle: None,
            },
        });

        if slow == fast {
            steps.push(Step {
                code_line: 6,
                description: format!(
                    "Pointers met! slow == fast at index {} (val={}). Cycle detected! Return True.",
                    slow, nodes[slow]
                ),
                visual: VisualState::LinkedListCycle {
                    nodes: nodes_vec.clone(),
                    cycle_target_idx: cycle_target,
                    slow_idx: Some(slow),
                    fast_idx: Some(fast),
                    visited_indices: BTreeSet::new(),
                    has_cycle: Some(true),
                },
            });
            return steps;
        }
    }

    steps.push(Step {
        code_line: 7,
        description: "Fast pointer reached null (end of list). No cycle exists. Return False."
            .to_string(),
        visual: VisualState::LinkedListCycle {
            nodes: nodes_vec,
            cycle_target_idx: cycle_target,
            slow_idx: Some(slow),
            fast_idx: Some(fast),
            visited_indices: BTreeSet::new(),
            has_cycle: Some(false),
        },
    });

    steps
}

fn generate_visited_set_steps(nodes: &[i32], cycle_index: i32) -> Vec<Step> {
    let nodes_vec = nodes.to_vec();
    let mut seen = BTreeSet::new();
    let cycle_target = usize::try_from(cycle_index)
        .ok()
        .filter(|&index| index < nodes.len());
    let next_index = |current: usize| {
        if current + 1 < nodes.len() {
            Some(current + 1)
        } else {
            cycle_target
        }
    };

    let mut steps = vec![Step {
        code_line: 3,
        description: "Initialize an empty set of visited node identities.".to_string(),
        visual: VisualState::LinkedListCycle {
            nodes: nodes_vec.clone(),
            cycle_target_idx: cycle_target,
            slow_idx: nodes.first().map(|_| 0),
            fast_idx: None,
            visited_indices: seen.clone(),
            has_cycle: None,
        },
    }];
    steps.push(Step {
        code_line: 4,
        description: "Set curr to the head node.".to_string(),
        visual: VisualState::LinkedListCycle {
            nodes: nodes_vec.clone(),
            cycle_target_idx: cycle_target,
            slow_idx: nodes.first().map(|_| 0),
            fast_idx: None,
            visited_indices: seen.clone(),
            has_cycle: None,
        },
    });

    let mut current = nodes.first().map(|_| 0);
    while let Some(index) = current {
        if seen.contains(&index) {
            steps.push(Step {
                code_line: 6,
                description: format!(
                    "node[{}] = {} has already been visited, so the list contains a cycle.",
                    index, nodes[index]
                ),
                visual: VisualState::LinkedListCycle {
                    nodes: nodes_vec,
                    cycle_target_idx: cycle_target,
                    slow_idx: Some(index),
                    fast_idx: None,
                    visited_indices: seen.clone(),
                    has_cycle: Some(true),
                },
            });
            return steps;
        }

        seen.insert(index);
        steps.push(Step {
            code_line: 7,
            description: format!(
                "Record node[{}] = {} in seen ({} visited node{}).",
                index,
                nodes[index],
                seen.len(),
                if seen.len() == 1 { "" } else { "s" }
            ),
            visual: VisualState::LinkedListCycle {
                nodes: nodes_vec.clone(),
                cycle_target_idx: cycle_target,
                slow_idx: Some(index),
                fast_idx: None,
                visited_indices: seen.clone(),
                has_cycle: None,
            },
        });

        current = next_index(index);
        steps.push(Step {
            code_line: 8,
            description: current.map_or_else(
                || "Advance curr to None.".to_string(),
                |next| format!("Advance curr to node[{}] = {}.", next, nodes[next]),
            ),
            visual: VisualState::LinkedListCycle {
                nodes: nodes_vec.clone(),
                cycle_target_idx: cycle_target,
                slow_idx: current,
                fast_idx: None,
                visited_indices: seen.clone(),
                has_cycle: None,
            },
        });
    }

    steps.push(Step {
        code_line: 9,
        description: "curr reached None without revisiting a node. Return False.".to_string(),
        visual: VisualState::LinkedListCycle {
            nodes: nodes_vec,
            cycle_target_idx: cycle_target,
            slow_idx: None,
            fast_idx: None,
            visited_indices: seen,
            has_cycle: Some(false),
        },
    });
    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    fn final_result(steps: &[Step]) -> Option<bool> {
        match &steps.last().expect("trace has a final step").visual {
            VisualState::LinkedListCycle { has_cycle, .. } => *has_cycle,
            visual => panic!("unexpected final visual: {visual:?}"),
        }
    }

    #[test]
    fn visited_set_detects_cycle_and_acyclic_list() {
        assert_eq!(
            final_result(&generate_linked_list_cycle_steps(&[1, 2, 3, 4], 1, 1)),
            Some(true)
        );
        assert_eq!(
            final_result(&generate_linked_list_cycle_steps(&[1, 2, 3, 4], -1, 1)),
            Some(false)
        );
    }

    #[test]
    fn cycle_trace_rejects_inputs_that_are_too_large() {
        let steps =
            generate_linked_list_cycle_steps(&vec![0; LINKED_LIST_CYCLE_TRACE_LIMIT + 1], -1, 1);
        assert!(matches!(
            steps[0].visual,
            VisualState::TraceUnavailable { .. }
        ));
    }
}
