use crate::model::{Step, VisualState};

pub fn generate_parentheses_combinations_steps(n: usize) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut current = String::new();
    let mut results = Vec::new();

    steps.push(Step {
        code_line: 3,
        description: format!("Generate Parentheses for n={}. Backtracking constraints: open < n, close < open.", n),
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
                description: format!("Valid combination generated: \"{}\". Total = {}.", current, results.len()),
                visual: VisualState::Stack {
                    chars: stack_chars.clone(),
                    active_idx: None,
                    stack: stack_chars,
                    is_valid: Some(true),
                },
            });
            return;
        }

        if open < n {
            current.push('(');
            let stack_chars: Vec<char> = current.chars().collect();
            steps.push(Step {
                code_line: 7,
                description: format!("open ({}) < n ({}): Add '('. Current string = \"{}\".", open, n, current),
                visual: VisualState::Stack {
                    chars: vec![],
                    active_idx: Some(stack_chars.len() - 1),
                    stack: stack_chars,
                    is_valid: None,
                },
            });

            backtrack(open + 1, close, n, current, results, steps);
            current.pop();
        }

        if close < open {
            current.push(')');
            let stack_chars: Vec<char> = current.chars().collect();
            steps.push(Step {
                code_line: 9,
                description: format!("close ({}) < open ({}): Add ')'. Current string = \"{}\".", close, open, current),
                visual: VisualState::Stack {
                    chars: vec![],
                    active_idx: Some(stack_chars.len() - 1),
                    stack: stack_chars,
                    is_valid: None,
                },
            });

            backtrack(open, close + 1, n, current, results, steps);
            current.pop();
        }
    }

    backtrack(0, 0, n, &mut current, &mut results, &mut steps);

    let res_str = results.join(", ");
    steps.push(Step {
        code_line: 10,
        description: format!("Backtracking search complete! All {} valid combinations for n={}: [{}].", results.len(), n, res_str),
        visual: VisualState::Stack {
            chars: vec![],
            active_idx: None,
            stack: vec![],
            is_valid: Some(true),
        },
    });

    steps
}
