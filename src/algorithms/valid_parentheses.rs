use crate::model::{Step, VisualState};

pub fn generate_valid_parentheses_steps(s: &str) -> Vec<Step> {
    let mut steps = Vec::new();
    let chars: Vec<char> = s.chars().collect();
    let mut stack: Vec<char> = Vec::new();

    // 1. Stack init (code_line 3)
    steps.push(Step {
        code_line: 3,
        description: "Initialized empty stack = [].".to_string(),
        visual: VisualState::Stack {
            chars: chars.clone(),
            active_idx: None,
            stack: stack.clone(),
            is_valid: None,
        },
    });

    // 2. Iterate characters (code_line 5)
    for (i, &c) in chars.iter().enumerate() {
        if c == ')' || c == ']' || c == '}' {
            // Closing bracket (code_line 6-7)
            let expected_open = match c {
                ')' => '(',
                ']' => '[',
                '}' => '{',
                _ => unreachable!(),
            };

            if let Some(&top) = stack.last() {
                if top == expected_open {
                    stack.pop();
                    steps.push(Step {
                        code_line: 8,
                        description: format!("Index {}: found closing bracket '{}', matching open bracket '{}' popped from stack.", i, c, top),
                        visual: VisualState::Stack {
                            chars: chars.clone(),
                            active_idx: Some(i),
                            stack: stack.clone(),
                            is_valid: None,
                        },
                    });
                } else {
                    steps.push(Step {
                        code_line: 10,
                        description: format!("Index {}: closing bracket '{}' mismatch! Expected '{}', found top of stack '{}'. Return False.", i, c, expected_open, top),
                        visual: VisualState::Stack {
                            chars: chars.clone(),
                            active_idx: Some(i),
                            stack: stack.clone(),
                            is_valid: Some(false),
                        },
                    });
                    return steps;
                }
            } else {
                steps.push(Step {
                    code_line: 10,
                    description: format!(
                        "Index {}: closing bracket '{}' found, but stack is empty! Return False.",
                        i, c
                    ),
                    visual: VisualState::Stack {
                        chars: chars.clone(),
                        active_idx: Some(i),
                        stack: stack.clone(),
                        is_valid: Some(false),
                    },
                });
                return steps;
            }
        } else {
            // Opening bracket (code_line 12)
            stack.push(c);
            steps.push(Step {
                code_line: 12,
                description: format!("Index {}: opening bracket '{}' pushed to stack.", i, c),
                visual: VisualState::Stack {
                    chars: chars.clone(),
                    active_idx: Some(i),
                    stack: stack.clone(),
                    is_valid: None,
                },
            });
        }
    }

    // 3. Final check (code_line 13)
    let is_valid = stack.is_empty();
    steps.push(Step {
        code_line: 13,
        description: if is_valid {
            "All brackets matched and stack is empty. Return True.".to_string()
        } else {
            format!(
                "String ended but stack still contains unclosed brackets {:?}. Return False.",
                stack
            )
        },
        visual: VisualState::Stack {
            chars: chars.clone(),
            active_idx: None,
            stack,
            is_valid: Some(is_valid),
        },
    });

    steps
}
