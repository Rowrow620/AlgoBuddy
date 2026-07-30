use crate::model::{Step, VisualState};

pub fn generate_search_rotated_array_steps(nums: &[i32], target: i32) -> Vec<Step> {
    let mut steps = Vec::new();
    let n = nums.len();

    steps.push(Step {
        code_line: 3,
        description: format!(
            "Search in Rotated Sorted Array: nums = {:?}, target = {}.",
            nums, target
        ),
        visual: VisualState::BinarySearch {
            nums: nums.to_vec(),
            target,
            left: 0,
            right: n.saturating_sub(1),
            mid: None,
            found_idx: None,
        },
    });

    let mut l = 0isize;
    let mut r = (n as isize) - 1;

    while l <= r {
        let mid = l + (r - l) / 2;
        let mid_idx = mid as usize;
        let mid_val = nums[mid_idx];

        steps.push(Step {
            code_line: 5,
            description: format!(
                "l={}, r={}: mid_idx={}, nums[mid]={}. Target = {}.",
                l, r, mid_idx, mid_val, target
            ),
            visual: VisualState::BinarySearch {
                nums: nums.to_vec(),
                target,
                left: l as usize,
                right: r as usize,
                mid: Some(mid_idx),
                found_idx: None,
            },
        });

        if mid_val == target {
            steps.push(Step {
                code_line: 6,
                description: format!(
                    "Found target {} at index {}! Return {}.",
                    target, mid_idx, mid_idx
                ),
                visual: VisualState::BinarySearch {
                    nums: nums.to_vec(),
                    target,
                    left: l as usize,
                    right: r as usize,
                    mid: Some(mid_idx),
                    found_idx: Some(mid_idx),
                },
            });
            return steps;
        }

        // Left half is sorted
        if nums[l as usize] <= mid_val {
            if nums[l as usize] <= target && target < mid_val {
                steps.push(Step {
                    code_line: 8,
                    description: format!("Left half sorted (nums[{}]={} <= {}). Target in left range [{}..{}). Move r to {}.",
                        l, nums[l as usize], mid_val, nums[l as usize], mid_val, mid - 1),
                    visual: VisualState::BinarySearch {
                        nums: nums.to_vec(),
                        target,
                        left: l as usize,
                        right: (mid - 1).max(0) as usize,
                        mid: Some(mid_idx),
                        found_idx: None,
                    },
                });
                r = mid - 1;
            } else {
                steps.push(Step {
                    code_line: 9,
                    description: format!("Left half sorted, but target not in left range. Search right half: move l to {}.", mid + 1),
                    visual: VisualState::BinarySearch {
                        nums: nums.to_vec(),
                        target,
                        left: (mid + 1) as usize,
                        right: r as usize,
                        mid: Some(mid_idx),
                        found_idx: None,
                    },
                });
                l = mid + 1;
            }
        } else {
            // Right half is sorted
            if mid_val < target && target <= nums[r as usize] {
                steps.push(Step {
                    code_line: 11,
                    description: format!(
                        "Right half sorted (mid {} < target <= nums[{}]={}). Move l to {}.",
                        mid_val,
                        r,
                        nums[r as usize],
                        mid + 1
                    ),
                    visual: VisualState::BinarySearch {
                        nums: nums.to_vec(),
                        target,
                        left: (mid + 1) as usize,
                        right: r as usize,
                        mid: Some(mid_idx),
                        found_idx: None,
                    },
                });
                l = mid + 1;
            } else {
                steps.push(Step {
                    code_line: 12,
                    description: format!(
                        "Right half sorted, but target not in right range. Move r to {}.",
                        mid - 1
                    ),
                    visual: VisualState::BinarySearch {
                        nums: nums.to_vec(),
                        target,
                        left: l as usize,
                        right: (mid - 1).max(0) as usize,
                        mid: Some(mid_idx),
                        found_idx: None,
                    },
                });
                r = mid - 1;
            }
        }
    }

    steps.push(Step {
        code_line: 14,
        description: format!(
            "Target {} not found in rotated sorted array. Return -1.",
            target
        ),
        visual: VisualState::BinarySearch {
            nums: nums.to_vec(),
            target,
            left: 0,
            right: 0,
            mid: None,
            found_idx: None,
        },
    });

    steps
}
