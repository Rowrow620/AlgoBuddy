use crate::model::{Step, VisualState};

pub fn generate_daily_temperatures_steps(temperatures: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();
    let n = temperatures.len();
    let mut res = vec![0i32; n];
    let mut stack: Vec<usize> = Vec::new(); // indices

    let all_chars: Vec<char> = temperatures
        .iter()
        .map(|t| t.to_string().chars().next().unwrap_or(' '))
        .collect();

    steps.push(Step {
        code_line: 3,
        description: format!(
            "Daily Temperatures for temperatures = {:?}. Monotonic decreasing stack.",
            temperatures
        ),
        visual: VisualState::Stack {
            chars: all_chars.clone(),
            active_idx: None,
            stack: vec![],
            is_valid: None,
        },
    });

    for (i, &t) in temperatures.iter().enumerate() {
        while let Some(&top_idx) = stack.last() {
            if temperatures[top_idx] < t {
                stack.pop();
                res[top_idx] = (i - top_idx) as i32;

                let stack_chars: Vec<char> = stack
                    .iter()
                    .map(|idx| temperatures[*idx].to_string().chars().next().unwrap_or(' '))
                    .collect();
                steps.push(Step {
                    code_line: 6,
                    description: format!("Warmer day found for idx {} (temp {}): t={} > {}. Days to wait = {} - {} = {}.",
                        top_idx, temperatures[top_idx], t, temperatures[top_idx], i, top_idx, res[top_idx]),
                    visual: VisualState::Stack {
                        chars: all_chars.clone(),
                        active_idx: Some(i),
                        stack: stack_chars,
                        is_valid: None,
                    },
                });
            } else {
                break;
            }
        }

        stack.push(i);
        let stack_chars: Vec<char> = stack
            .iter()
            .map(|idx| temperatures[*idx].to_string().chars().next().unwrap_or(' '))
            .collect();
        steps.push(Step {
            code_line: 8,
            description: format!("Push index {} (temp {}) to stack: {:?}", i, t, stack_chars),
            visual: VisualState::Stack {
                chars: all_chars.clone(),
                active_idx: Some(i),
                stack: stack_chars,
                is_valid: None,
            },
        });
    }

    steps.push(Step {
        code_line: 9,
        description: format!(
            "Daily Temperatures traversal complete! Output days = {:?}.",
            res
        ),
        visual: VisualState::Stack {
            chars: all_chars,
            active_idx: None,
            stack: vec![],
            is_valid: Some(true),
        },
    });

    steps
}
