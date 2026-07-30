use crate::model::{Step, VisualState};

pub fn generate_trapping_rain_steps(height: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();
    let n = height.len();

    let char_repr: Vec<char> = height
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",")
        .chars()
        .collect();

    if n < 3 {
        steps.push(Step {
            code_line: 3,
            description: format!(
                "Height array {:?} has fewer than 3 bars. No water can be trapped. Return 0.",
                height
            ),
            visual: VisualState::TwoPointers {
                chars: char_repr,
                left: 0,
                right: 0,
                is_valid: Some(true),
                skipped: false,
            },
        });
        return steps;
    }

    let mut l = 0usize;
    let mut r = n - 1;
    let mut left_max = height[l];
    let mut right_max = height[r];
    let mut total_water = 0i32;

    steps.push(Step {
        code_line: 3,
        description: format!(
            "Trapping Rain Water: height = {:?}. l=0 (leftMax={}), r={} (rightMax={}).",
            height, left_max, r, right_max
        ),
        visual: VisualState::TwoPointers {
            chars: char_repr.clone(),
            left: 0,
            right: char_repr.len().saturating_sub(1),
            is_valid: None,
            skipped: false,
        },
    });

    while l < r {
        if left_max < right_max {
            l += 1;
            left_max = left_max.max(height[l]);
            let water = left_max - height[l];
            total_water += water;

            steps.push(Step {
                code_line: 6,
                description: format!("leftMax ({}) < rightMax ({}). Move l to {}. leftMax = max({}, {}) = {}. Water at l = {} - {} = {}. Total = {}.",
                    left_max.min(right_max + 1), right_max, l, left_max, height[l], left_max, left_max, height[l], water, total_water),
                visual: VisualState::TwoPointers {
                    chars: char_repr.clone(),
                    left: l,
                    right: r,
                    is_valid: None,
                    skipped: false,
                },
            });
        } else {
            r -= 1;
            right_max = right_max.max(height[r]);
            let water = right_max - height[r];
            total_water += water;

            steps.push(Step {
                code_line: 8,
                description: format!("leftMax ({}) >= rightMax ({}). Move r to {}. rightMax = max({}, {}) = {}. Water at r = {} - {} = {}. Total = {}.",
                    left_max, right_max.min(left_max + 1), r, right_max, height[r], right_max, right_max, height[r], water, total_water),
                visual: VisualState::TwoPointers {
                    chars: char_repr.clone(),
                    left: l,
                    right: r,
                    is_valid: None,
                    skipped: false,
                },
            });
        }

        if steps.len() > 150 {
            break;
        }
    }

    steps.push(Step {
        code_line: 9,
        description: format!(
            "Pointers crossed. Total trapped water = {} units.",
            total_water
        ),
        visual: VisualState::TwoPointers {
            chars: char_repr,
            left: l,
            right: r,
            is_valid: Some(true),
            skipped: false,
        },
    });

    steps
}
