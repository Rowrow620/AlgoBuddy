use crate::model::{Step, VisualState};

pub fn generate_parentheses_combinations_steps(n: usize) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut current = String::new();
    let mut results = Vec::new();

    steps.push(Step {
        code_line: 3,
        description: format!(
            "Generate Parentheses for n={}. Backtracking constraints: open < n, close < open.",
            n
        ),
        visual: VisualState::Stack {
            chars: vec![],
            active_idx: None,
            stack: vec![],
            is_valid: None,
        },
    });

    fn backtrack(
        open: usize,
        close: usize,
        n: usize,
        current: &mut String,
        results: &mut Vec<String>,
        steps: &mut Vec<Step>,
    ) {
        if open == n && close == n {
            results.push(current.clone());
            let stack_chars: Vec<char> = current.chars().collect();
            steps.push(Step {
                code_line: 5,
                description: format!(
                    "Valid combination generated: \"{}\". Total = {}.",
                    current,
                    results.len()
                ),
                visual: VisualState::Stack {
                    chars: stack_chars.clone(),
                    active_idx: None,
                    stack: stack_chars,
                    is_valid: None,
                },
            });
            return;
        }

        if open < n {
            current.push('(');
            let stack_chars: Vec<char> = current.chars().collect();
            steps.push(Step {
                code_line: 8,
                description: format!(
                    "open ({}) < n ({}): Add '('. Current string = \"{}\".",
                    open, n, current
                ),
                visual: VisualState::Stack {
                    chars: stack_chars.clone(),
                    active_idx: Some(stack_chars.len() - 1),
                    stack: stack_chars,
                    is_valid: None,
                },
            });

            backtrack(open + 1, close, n, current, results, steps);
            let removed = current.pop().expect("the open branch pushed a character");
            let stack_chars: Vec<char> = current.chars().collect();
            steps.push(Step {
                code_line: 10,
                description: format!(
                    "Backtrack from the open branch: pop '{}'. Restored prefix = \"{}\".",
                    removed, current
                ),
                visual: VisualState::Stack {
                    chars: stack_chars.clone(),
                    active_idx: None,
                    stack: stack_chars,
                    is_valid: None,
                },
            });
        }

        if close < open {
            current.push(')');
            let stack_chars: Vec<char> = current.chars().collect();
            steps.push(Step {
                code_line: 13,
                description: format!(
                    "close ({}) < open ({}): Add ')'. Current string = \"{}\".",
                    close, open, current
                ),
                visual: VisualState::Stack {
                    chars: stack_chars.clone(),
                    active_idx: Some(stack_chars.len() - 1),
                    stack: stack_chars,
                    is_valid: None,
                },
            });

            backtrack(open, close + 1, n, current, results, steps);
            let removed = current.pop().expect("the close branch pushed a character");
            let stack_chars: Vec<char> = current.chars().collect();
            steps.push(Step {
                code_line: 15,
                description: format!(
                    "Backtrack from the close branch: pop '{}'. Restored prefix = \"{}\".",
                    removed, current
                ),
                visual: VisualState::Stack {
                    chars: stack_chars.clone(),
                    active_idx: None,
                    stack: stack_chars,
                    is_valid: None,
                },
            });
        }
    }

    backtrack(0, 0, n, &mut current, &mut results, &mut steps);

    let res_str = results.join(", ");
    steps.push(Step {
        code_line: 17,
        description: format!(
            "Backtracking search complete! All {} valid combinations for n={}: [{}].",
            results.len(),
            n,
            res_str
        ),
        visual: VisualState::Stack {
            chars: vec![],
            active_idx: None,
            stack: vec![],
            is_valid: Some(true),
        },
    });

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn active_indices_always_reference_the_current_prefix() {
        let steps = generate_parentheses_combinations_steps(3);

        for step in &steps {
            let VisualState::Stack {
                chars,
                active_idx,
                is_valid,
                ..
            } = &step.visual
            else {
                panic!("Generate Parentheses must use the stack view");
            };
            assert!(active_idx.is_none_or(|idx| idx < chars.len()));
            if step.code_line == 5 {
                assert_eq!(*is_valid, None, "a generated leaf is not a final outcome");
            }
        }

        let push_count = steps
            .iter()
            .filter(|step| matches!(step.code_line, 8 | 13))
            .count();
        let pop_count = steps
            .iter()
            .filter(|step| matches!(step.code_line, 10 | 15))
            .count();
        assert_eq!(push_count, pop_count);

        for pair in steps.windows(2) {
            let VisualState::Stack { chars: before, .. } = &pair[0].visual else {
                unreachable!();
            };
            let VisualState::Stack { chars: after, .. } = &pair[1].visual else {
                unreachable!();
            };

            match pair[1].code_line {
                8 | 13 => {
                    assert_eq!(after.len(), before.len() + 1);
                    assert!(after.starts_with(before));
                }
                10 | 15 => {
                    assert_eq!(after.len() + 1, before.len());
                    assert!(before.starts_with(after));
                    assert!(pair[1].description.starts_with("Backtrack"));
                }
                _ => assert_eq!(after, before),
            }
        }

        let final_step = steps
            .last()
            .expect("generation must have a completion step");
        assert_eq!(final_step.code_line, 17);
        assert!(final_step.description.contains("All 5 valid combinations"));
        assert!(matches!(
            final_step.visual,
            VisualState::Stack {
                active_idx: None,
                is_valid: Some(true),
                ..
            }
        ));
    }
}
