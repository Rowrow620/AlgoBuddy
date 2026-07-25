use crate::model::{Step, VisualState};

pub fn generate_largest_rectangle_steps(heights: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut stack: Vec<(usize, i32)> = Vec::new(); // (index, height)
    let mut max_area = 0i32;

    let char_repr: Vec<char> = heights.iter().map(|h| h.to_string().chars().next().unwrap_or(' ')).collect();

    steps.push(Step {
        code_line: 3,
        description: format!("Largest Rectangle in Histogram for heights = {:?}. Monotonic increasing stack.", heights),
        visual: VisualState::Stack {
            chars: char_repr.clone(),
            active_idx: None,
            stack: vec![],
            is_valid: None,
        },
    });

    for (i, &h) in heights.iter().enumerate() {
        let mut start = i;

        while let Some(&(prev_idx, prev_h)) = stack.last() {
            if prev_h > h {
                stack.pop();
                let width = (i - prev_idx) as i32;
                let area = prev_h * width;
                if area > max_area { max_area = area; }
                start = prev_idx;

                let stack_chars: Vec<char> = stack.iter().map(|(_, height)| height.to_string().chars().next().unwrap_or(' ')).collect();
                steps.push(Step {
                    code_line: 6,
                    description: format!("Height {} < popped height {}. Rectangle width = {} - {} = {}. Area = {} * {} = {}. maxArea = {}.",
                        h, prev_h, i, prev_idx, width, prev_h, width, area, max_area),
                    visual: VisualState::Stack {
                        chars: char_repr.clone(),
                        active_idx: Some(i),
                        stack: stack_chars,
                        is_valid: None,
                    },
                });
            } else {
                break;
            }
        }

        stack.push((start, h));
        let stack_chars: Vec<char> = stack.iter().map(|(_, height)| height.to_string().chars().next().unwrap_or(' ')).collect();
        steps.push(Step {
            code_line: 8,
            description: format!("Push (start={}, height={}) to stack: {:?}", start, h, stack_chars),
            visual: VisualState::Stack {
                chars: char_repr.clone(),
                active_idx: Some(i),
                stack: stack_chars,
                is_valid: None,
            },
        });
    }

    let n = heights.len();
    while let Some((start_idx, h)) = stack.pop() {
        let width = (n - start_idx) as i32;
        let area = h * width;
        if area > max_area { max_area = area; }

        let stack_chars: Vec<char> = stack.iter().map(|(_, height)| height.to_string().chars().next().unwrap_or(' ')).collect();
        steps.push(Step {
            code_line: 10,
            description: format!("Cleanup stack: height {} extends to end. Width = {} - {} = {}. Area = {}. maxArea = {}.",
                h, n, start_idx, width, area, max_area),
            visual: VisualState::Stack {
                chars: char_repr.clone(),
                active_idx: None,
                stack: stack_chars,
                is_valid: None,
            },
        });
    }

    steps.push(Step {
        code_line: 11,
        description: format!("Largest Rectangle in Histogram evaluation complete! Maximum area = {}.", max_area),
        visual: VisualState::Stack {
            chars: char_repr,
            active_idx: None,
            stack: vec![],
            is_valid: Some(true),
        },
    });

    steps
}
