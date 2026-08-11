use crate::model::{Step, VisualState};

pub(crate) const PAIR_ELIMINATION_VISUALIZATION_LIMIT: usize = 64;
pub(crate) const STACK_VISUALIZATION_LIMIT: usize = 512;

pub fn generate_valid_parentheses_steps(s: &str, approach_id: usize) -> Vec<Step> {
    let input_len = s.chars().count();
    let limit = if approach_id == 1 {
        PAIR_ELIMINATION_VISUALIZATION_LIMIT
    } else {
        STACK_VISUALIZATION_LIMIT
    };
    if input_len > limit {
        let approach = if approach_id == 1 {
            "Repeated Pair Elimination"
        } else {
            "Stack Matching"
        };
        let message = format!(
            "{approach} traces accept at most {limit} characters because each step stores the expression state."
        );
        return vec![Step {
            code_line: 3,
            description: message.clone(),
            visual: VisualState::TraceUnavailable { message },
        }];
    }

    match approach_id {
        0 => generate_parentheses_stack(s),
        1 => generate_parentheses_pair_elimination(s),
        _ => Vec::new(),
    }
}

fn generate_parentheses_stack(s: &str) -> Vec<Step> {
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

fn generate_parentheses_pair_elimination(s: &str) -> Vec<Step> {
    let mut chars: Vec<char> = s.chars().collect();
    let mut steps = vec![Step {
        code_line: 3,
        description: "Copied the expression into a mutable character list.".to_string(),
        visual: VisualState::Stack {
            chars: chars.clone(),
            active_idx: None,
            stack: Vec::new(),
            is_valid: None,
        },
    }];

    while !chars.is_empty() {
        let pair_index = chars
            .windows(2)
            .position(|pair| matches_pair(pair[0], pair[1]));

        let Some(pair_index) = pair_index else {
            steps.push(Step {
                code_line: 11,
                description:
                    "No adjacent matching pair remains while characters are still present. Return False."
                        .to_string(),
                visual: VisualState::Stack {
                    chars,
                    active_idx: None,
                    stack: Vec::new(),
                    is_valid: Some(false),
                },
            });
            return steps;
        };

        let pair = [chars[pair_index], chars[pair_index + 1]];
        steps.push(Step {
            code_line: 9,
            description: format!("Found adjacent matching pair '{}{}'.", pair[0], pair[1]),
            visual: VisualState::Stack {
                chars: chars.clone(),
                active_idx: Some(pair_index),
                stack: Vec::new(),
                is_valid: None,
            },
        });

        chars.drain(pair_index..=pair_index + 1);
        steps.push(Step {
            code_line: 12,
            description: format!(
                "Removed '{}{}'. Remaining expression: \"{}\".",
                pair[0],
                pair[1],
                chars.iter().collect::<String>()
            ),
            visual: VisualState::Stack {
                chars: chars.clone(),
                active_idx: None,
                stack: Vec::new(),
                is_valid: None,
            },
        });
    }

    steps.push(Step {
        code_line: 13,
        description: "Every bracket was removed as part of a matching pair. Return True."
            .to_string(),
        visual: VisualState::Stack {
            chars,
            active_idx: None,
            stack: Vec::new(),
            is_valid: Some(true),
        },
    });

    steps
}

fn matches_pair(open: char, close: char) -> bool {
    matches!((open, close), ('(', ')') | ('[', ']') | ('{', '}'))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn final_result(steps: &[Step]) -> Option<bool> {
        match &steps.last()?.visual {
            VisualState::Stack { is_valid, .. } => *is_valid,
            VisualState::TraceUnavailable { .. } => None,
            _ => None,
        }
    }

    #[test]
    fn both_approaches_agree_on_validity() {
        for expression in ["", "()", "()[]{}", "([{}])", "(]", "([)]", "((("] {
            let stack = generate_valid_parentheses_steps(expression, 0);
            let elimination = generate_valid_parentheses_steps(expression, 1);
            assert_eq!(final_result(&stack), final_result(&elimination));
        }
    }

    #[test]
    fn pair_elimination_trace_rejects_oversized_inputs() {
        let expression = "(".repeat(PAIR_ELIMINATION_VISUALIZATION_LIMIT + 1);
        let steps = generate_valid_parentheses_steps(&expression, 1);
        assert!(matches!(
            steps.as_slice(),
            [Step {
                visual: VisualState::TraceUnavailable { .. },
                ..
            }]
        ));
    }

    #[test]
    fn stack_trace_rejects_oversized_inputs() {
        let expression = "(".repeat(STACK_VISUALIZATION_LIMIT + 1);
        assert!(matches!(
            generate_valid_parentheses_steps(&expression, 0).as_slice(),
            [Step {
                visual: VisualState::TraceUnavailable { .. },
                ..
            }]
        ));
    }
}
