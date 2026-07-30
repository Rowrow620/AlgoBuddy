use crate::model::{Step, VisualState};

pub fn generate_eval_rpn_steps(tokens: &[String]) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut stack = Vec::new();

    let all_chars: Vec<char> = tokens
        .iter()
        .map(|s| s.chars().next().unwrap_or(' '))
        .collect();

    steps.push(Step {
        code_line: 3,
        description: format!("Evaluate Reverse Polish Notation for tokens: {:?}", tokens),
        visual: VisualState::Stack {
            chars: all_chars.clone(),
            active_idx: None,
            stack: vec![],
            is_valid: None,
        },
    });

    for (i, token) in tokens.iter().enumerate() {
        if let Ok(num) = token.parse::<i32>() {
            stack.push(num);
            let stack_chars: Vec<char> = stack
                .iter()
                .map(|n| n.to_string().chars().next().unwrap_or(' '))
                .collect();
            steps.push(Step {
                code_line: 6,
                description: format!(
                    "Token '{}' is integer. Push {} onto stack: {:?}.",
                    token, num, stack
                ),
                visual: VisualState::Stack {
                    chars: all_chars.clone(),
                    active_idx: Some(i),
                    stack: stack_chars,
                    is_valid: None,
                },
            });
        } else if stack.len() >= 2 {
            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();
            let res = match token.as_str() {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => a / b,
                _ => 0,
            };
            stack.push(res);

            let stack_chars: Vec<char> = stack
                .iter()
                .map(|n| n.to_string().chars().next().unwrap_or(' '))
                .collect();
            steps.push(Step {
                code_line: 9,
                description: format!(
                    "Operator '{}': pop {} and {}. Calculate {} {} {} = {}. Push result: {:?}.",
                    token, b, a, a, token, b, res, stack
                ),
                visual: VisualState::Stack {
                    chars: all_chars.clone(),
                    active_idx: Some(i),
                    stack: stack_chars,
                    is_valid: None,
                },
            });
        }
    }

    let final_res = stack.last().copied().unwrap_or(0);
    let final_chars: Vec<char> = stack
        .iter()
        .map(|n| n.to_string().chars().next().unwrap_or(' '))
        .collect();
    steps.push(Step {
        code_line: 11,
        description: format!("Evaluation complete! Final RPN result = {}.", final_res),
        visual: VisualState::Stack {
            chars: all_chars,
            active_idx: None,
            stack: final_chars,
            is_valid: Some(true),
        },
    });

    steps
}
