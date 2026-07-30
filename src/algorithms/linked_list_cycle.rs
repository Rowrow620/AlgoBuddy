use crate::model::{Step, VisualState};

pub fn generate_linked_list_cycle_steps(nodes: &[i32], cycle_index: i32) -> Vec<Step> {
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
            code_line: 9,
            description: "Empty list has no cycle. Return False.".to_string(),
            visual: VisualState::LinkedListCycle {
                nodes: nodes_vec,
                cycle_target_idx: None,
                slow_idx: None,
                fast_idx: None,
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
            code_line: 6,
            description: format!("Advanced slow by 1 step to index {} (val={}) and fast by 2 steps to index {} (val={}).", slow, nodes[slow], fast, nodes[fast]),
            visual: VisualState::LinkedListCycle {
                nodes: nodes_vec.clone(),
                cycle_target_idx: cycle_target,
                slow_idx: Some(slow),
                fast_idx: Some(fast),
                has_cycle: None,
            },
        });

        if slow == fast {
            steps.push(Step {
                code_line: 8,
                description: format!(
                    "Pointers met! slow == fast at index {} (val={}). Cycle detected! Return True.",
                    slow, nodes[slow]
                ),
                visual: VisualState::LinkedListCycle {
                    nodes: nodes_vec.clone(),
                    cycle_target_idx: cycle_target,
                    slow_idx: Some(slow),
                    fast_idx: Some(fast),
                    has_cycle: Some(true),
                },
            });
            return steps;
        }
    }

    steps.push(Step {
        code_line: 9,
        description: "Fast pointer reached null (end of list). No cycle exists. Return False."
            .to_string(),
        visual: VisualState::LinkedListCycle {
            nodes: nodes_vec,
            cycle_target_idx: cycle_target,
            slow_idx: Some(slow),
            fast_idx: Some(fast),
            has_cycle: Some(false),
        },
    });

    steps
}
