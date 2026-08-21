use crate::model::{Step, VisualState};

pub fn generate_find_min_rotated_steps(nums: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();
    if nums.is_empty() {
        steps.push(Step {
            code_line: 3,
            description: "Input array is empty. Cannot find minimum.".to_string(),
            visual: VisualState::BinarySearch {
                nums: Vec::new(),
                target: 0,
                left: 0,
                right: 0,
                mid: None,
                found_idx: None,
            },
        });
        return steps;
    }

    let n = nums.len();

    steps.push(Step {
        code_line: 3,
        description: format!("Find Minimum in Rotated Sorted Array: nums = {:?}", nums),
        visual: VisualState::BinarySearch {
            nums: nums.to_vec(),
            target: nums[0],
            left: 0,
            right: n.saturating_sub(1),
            mid: None,
            found_idx: None,
        },
    });

    let mut l = 0isize;
    let mut r = (n as isize) - 1;

    while l < r {
        let mid = l + (r - l) / 2;
        let mid_idx = mid as usize;
        let mid_val = nums[mid_idx];
        let r_val = nums[r as usize];

        steps.push(Step {
            code_line: 5,
            description: format!(
                "l={}, r={}: mid_idx={}, nums[mid]={}, nums[r]={}.",
                l, r, mid_idx, mid_val, r_val
            ),
            visual: VisualState::BinarySearch {
                nums: nums.to_vec(),
                target: r_val,
                left: l as usize,
                right: r as usize,
                mid: Some(mid_idx),
                found_idx: None,
            },
        });

        if mid_val > r_val {
            steps.push(Step {
                code_line: 7,
                description: format!(
                    "nums[mid] ({}) > nums[r] ({}) -> Minimum is in right half. Move l to {}.",
                    mid_val,
                    r_val,
                    mid + 1
                ),
                visual: VisualState::BinarySearch {
                    nums: nums.to_vec(),
                    target: r_val,
                    left: (mid + 1) as usize,
                    right: r as usize,
                    mid: Some(mid_idx),
                    found_idx: None,
                },
            });
            l = mid + 1;
        } else {
            steps.push(Step {
                code_line: 9,
                description: format!(
                    "nums[mid] ({}) <= nums[r] ({}) -> Minimum is mid or to left. Move r to {}.",
                    mid_val, r_val, mid
                ),
                visual: VisualState::BinarySearch {
                    nums: nums.to_vec(),
                    target: r_val,
                    left: l as usize,
                    right: mid_idx,
                    mid: Some(mid_idx),
                    found_idx: None,
                },
            });
            r = mid;
        }
    }

    let min_idx = l as usize;
    steps.push(Step {
        code_line: 10,
        description: format!(
            "Minimum found at index {} with value {}!",
            min_idx, nums[min_idx]
        ),
        visual: VisualState::BinarySearch {
            nums: nums.to_vec(),
            target: nums[min_idx],
            left: min_idx,
            right: min_idx,
            mid: Some(min_idx),
            found_idx: Some(min_idx),
        },
    });

    steps
}
