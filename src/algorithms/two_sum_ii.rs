use crate::model::{Step, VisualState};

pub fn generate_two_sum_ii_steps(nums: &[i32], target: i32) -> Vec<Step> {
    let mut steps = Vec::new();

    // Build a char representation for TwoPointers visual
    let char_repr: Vec<char> = nums
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",")
        .chars()
        .collect();

    let mut l = 0usize;
    let mut r = nums.len() - 1;

    steps.push(Step {
        code_line: 3,
        description: format!(
            "Two Sum II: nums = {:?}, target = {}. Initialize l=0, r={}.",
            nums, target, r
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
        let sum = nums[l] + nums[r];

        steps.push(Step {
            code_line: 5,
            description: format!(
                "l={}, r={}: nums[{}] + nums[{}] = {} + {} = {}. Target = {}.",
                l, r, l, r, nums[l], nums[r], sum, target
            ),
            visual: VisualState::TwoPointers {
                chars: char_repr.clone(),
                left: l,
                right: r,
                is_valid: None,
                skipped: false,
            },
        });

        if sum == target {
            steps.push(Step {
                code_line: 6,
                description: format!("Found! nums[{}] + nums[{}] = {} + {} = {} == target. Return [{}, {}] (1-indexed).",
                    l, r, nums[l], nums[r], sum, l + 1, r + 1),
                visual: VisualState::TwoPointers {
                    chars: char_repr.clone(),
                    left: l,
                    right: r,
                    is_valid: Some(true),
                    skipped: false,
                },
            });
            return steps;
        } else if sum < target {
            steps.push(Step {
                code_line: 7,
                description: format!(
                    "Sum {} < target {}. Move left pointer right: l = {}.",
                    sum,
                    target,
                    l + 1
                ),
                visual: VisualState::TwoPointers {
                    chars: char_repr.clone(),
                    left: l + 1,
                    right: r,
                    is_valid: None,
                    skipped: false,
                },
            });
            l += 1;
        } else {
            steps.push(Step {
                code_line: 8,
                description: format!(
                    "Sum {} > target {}. Move right pointer left: r = {}.",
                    sum,
                    target,
                    r - 1
                ),
                visual: VisualState::TwoPointers {
                    chars: char_repr.clone(),
                    left: l,
                    right: r - 1,
                    is_valid: None,
                    skipped: false,
                },
            });
            r -= 1;
        }

        // Safety: prevent infinite loops on bad input
        if steps.len() > 100 {
            break;
        }
    }

    steps.push(Step {
        code_line: 9,
        description: "No valid pair found. Return [].".to_string(),
        visual: VisualState::TwoPointers {
            chars: char_repr,
            left: l,
            right: r,
            is_valid: Some(false),
            skipped: false,
        },
    });

    steps
}
