use crate::model::{Step, VisualState};

pub fn generate_min_stack_steps(ops: &[(&str, Option<i32>)]) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut stack = Vec::new();
    let mut min_stack = Vec::new();

    steps.push(Step {
        code_line: 3,
        description: "Initialize MinStack: main stack = [], minStack = [].".to_string(),
        visual: VisualState::Stack {
            chars: vec![],
            active_idx: None,
            stack: vec![],
            is_valid: None,
        },
    });

    for (op, val) in ops {
        match *op {
            "push" => {
                let v = val.unwrap();
                stack.push(v);
                let current_min = min_stack.last().copied().map_or(v, |m: i32| m.min(v));
                min_stack.push(current_min);

                let stack_chars: Vec<char> = stack
                    .iter()
                    .map(|n| n.to_string().chars().next().unwrap_or(' '))
                    .collect();
                steps.push(Step {
                    code_line: 5,
                    description: format!(
                        "push({}): main stack = {:?}, minStack = {:?} (current min = {}).",
                        v, stack, min_stack, current_min
                    ),
                    visual: VisualState::Stack {
                        chars: vec![],
                        active_idx: Some(stack.len() - 1),
                        stack: stack_chars,
                        is_valid: None,
                    },
                });
            }
            "pop" => {
                if !stack.is_empty() {
                    let popped = stack.pop().unwrap();
                    min_stack.pop();
                    let stack_chars: Vec<char> = stack
                        .iter()
                        .map(|n| n.to_string().chars().next().unwrap_or(' '))
                        .collect();
                    steps.push(Step {
                        code_line: 7,
                        description: format!(
                            "pop(): popped {}. main stack = {:?}, minStack = {:?}.",
                            popped, stack, min_stack
                        ),
                        visual: VisualState::Stack {
                            chars: vec![],
                            active_idx: None,
                            stack: stack_chars,
                            is_valid: None,
                        },
                    });
                }
            }
            "top" => {
                if let Some(&top_val) = stack.last() {
                    let stack_chars: Vec<char> = stack
                        .iter()
                        .map(|n| n.to_string().chars().next().unwrap_or(' '))
                        .collect();
                    steps.push(Step {
                        code_line: 9,
                        description: format!("top(): returns {}.", top_val),
                        visual: VisualState::Stack {
                            chars: vec![],
                            active_idx: Some(stack.len() - 1),
                            stack: stack_chars,
                            is_valid: None,
                        },
                    });
                }
            }
            "getMin" => {
                if let Some(&min_val) = min_stack.last() {
                    let stack_chars: Vec<char> = stack
                        .iter()
                        .map(|n| n.to_string().chars().next().unwrap_or(' '))
                        .collect();
                    steps.push(Step {
                        code_line: 11,
                        description: format!("getMin(): O(1) current minimum = {}.", min_val),
                        visual: VisualState::Stack {
                            chars: vec![],
                            active_idx: None,
                            stack: stack_chars,
                            is_valid: Some(true),
                        },
                    });
                }
            }
            _ => {}
        }
    }

    steps
}
