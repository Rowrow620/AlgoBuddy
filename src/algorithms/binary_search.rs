use crate::model::{Step, VisualState};

pub fn generate_binary_search_steps(nums: &[i32], target: i32) -> Vec<Step> {
    let mut steps = Vec::new();
    let nums_vec = nums.to_vec();

    if nums.is_empty() {
        steps.push(Step {
            code_line: 12,
            description: "Input array is empty. Return -1.".to_string(),
            visual: VisualState::BinarySearch {
                nums: nums_vec,
                target,
                left: 0,
                right: 0,
                mid: None,
                found_idx: None,
            },
        });
        return steps;
    }

    let mut l = 0;
    let mut r = nums.len() - 1;

    // 1. Pointer init (code_line 3)
    steps.push(Step {
        code_line: 3,
        description: format!("Initialized left bound l=0 (nums[0]={}) and right bound r={} (nums[{}]={}). Searching target={}.", nums[l], r, r, nums[r], target),
        visual: VisualState::BinarySearch {
            nums: nums_vec.clone(),
            target,
            left: l,
            right: r,
            mid: None,
            found_idx: None,
        },
    });

    // 2. Loop while l <= r (code_line 4)
    while l <= r {
        let m = l + (r - l) / 2;

        steps.push(Step {
            code_line: 5,
            description: format!(
                "Calculated midpoint m = {} + ({} - {}) // 2 = {}. nums[{}] = {}.",
                l, r, l, m, m, nums[m]
            ),
            visual: VisualState::BinarySearch {
                nums: nums_vec.clone(),
                target,
                left: l,
                right: r,
                mid: Some(m),
                found_idx: None,
            },
        });

        if nums[m] == target {
            steps.push(Step {
                code_line: 11,
                description: format!(
                    "Target {} MATCHED at midpoint index m={}! Return {}.",
                    target, m, m
                ),
                visual: VisualState::BinarySearch {
                    nums: nums_vec.clone(),
                    target,
                    left: l,
                    right: r,
                    mid: Some(m),
                    found_idx: Some(m),
                },
            });
            return steps;
        } else if nums[m] > target {
            steps.push(Step {
                code_line: 7,
                description: format!("nums[{}] ({}) > target ({}). Target must be in left half. Narrowing right bound r to m - 1 = {}.", m, nums[m], target, m.saturating_sub(1)),
                visual: VisualState::BinarySearch {
                    nums: nums_vec.clone(),
                    target,
                    left: l,
                    right: r,
                    mid: Some(m),
                    found_idx: None,
                },
            });
            if m == 0 {
                break;
            }
            r = m - 1;
        } else {
            steps.push(Step {
                code_line: 9,
                description: format!("nums[{}] ({}) < target ({}). Target must be in right half. Narrowing left bound l to m + 1 = {}.", m, nums[m], target, m + 1),
                visual: VisualState::BinarySearch {
                    nums: nums_vec.clone(),
                    target,
                    left: l,
                    right: r,
                    mid: Some(m),
                    found_idx: None,
                },
            });
            l = m + 1;
        }
    }

    steps.push(Step {
        code_line: 12,
        description: format!(
            "Search bounds crossed (l > r). Target {} not found in array. Return -1.",
            target
        ),
        visual: VisualState::BinarySearch {
            nums: nums_vec,
            target,
            left: l,
            right: r,
            mid: None,
            found_idx: None,
        },
    });

    steps
}
