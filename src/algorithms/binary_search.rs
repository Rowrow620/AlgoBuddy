use crate::model::{Step, VisualState};

pub(crate) const LINEAR_SCAN_VISUALIZATION_LIMIT: usize = 128;
pub(crate) const BINARY_SEARCH_VISUALIZATION_LIMIT: usize = 10_000;

pub fn generate_binary_search_steps(nums: &[i32], target: i32, approach_id: usize) -> Vec<Step> {
    if nums.windows(2).any(|pair| pair[0] >= pair[1]) {
        let message = "Binary Search examples require nums to be sorted in strictly increasing order so every approach uses the problem's input contract."
            .to_string();
        return vec![Step {
            code_line: 3,
            description: message.clone(),
            visual: VisualState::TraceUnavailable { message },
        }];
    }

    let limit = if approach_id == 1 {
        LINEAR_SCAN_VISUALIZATION_LIMIT
    } else {
        BINARY_SEARCH_VISUALIZATION_LIMIT
    };
    if nums.len() > limit {
        let approach = if approach_id == 1 {
            "Linear Scan"
        } else {
            "Binary Search"
        };
        let message = format!(
            "{approach} traces accept at most {limit} values because each step stores the sorted array state."
        );
        return vec![Step {
            code_line: 3,
            description: message.clone(),
            visual: VisualState::TraceUnavailable { message },
        }];
    }

    match approach_id {
        0 => generate_binary_search(nums, target),
        1 => generate_linear_scan(nums, target),
        _ => Vec::new(),
    }
}

fn generate_binary_search(nums: &[i32], target: i32) -> Vec<Step> {
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
            if m == 0 {
                l = 1;
                r = 0;
                steps.push(Step {
                    code_line: 7,
                    description: format!(
                        "nums[0] ({}) > target ({}). Moving r before index 0 exhausts the search range.",
                        nums[m], target
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
                break;
            }
            steps.push(Step {
                code_line: 7,
                description: format!("nums[{}] ({}) > target ({}). Target must be in left half. Narrowing right bound r to m - 1 = {}.", m, nums[m], target, m - 1),
                visual: VisualState::BinarySearch {
                    nums: nums_vec.clone(),
                    target,
                    left: l,
                    right: r,
                    mid: Some(m),
                    found_idx: None,
                },
            });
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

fn generate_linear_scan(nums: &[i32], target: i32) -> Vec<Step> {
    let nums_vec = nums.to_vec();
    if nums.is_empty() {
        return vec![Step {
            code_line: 6,
            description: "The array is empty. Return -1.".to_string(),
            visual: VisualState::BinarySearch {
                nums: nums_vec,
                target,
                left: 0,
                right: 0,
                mid: None,
                found_idx: None,
            },
        }];
    }

    let last_index = nums.len() - 1;
    let mut steps = vec![Step {
        code_line: 3,
        description: format!(
            "Started a left-to-right scan of {} values for target {}.",
            nums.len(),
            target
        ),
        visual: VisualState::BinarySearch {
            nums: nums_vec.clone(),
            target,
            left: 0,
            right: last_index,
            mid: None,
            found_idx: None,
        },
    }];

    for (index, &value) in nums.iter().enumerate() {
        steps.push(Step {
            code_line: 4,
            description: format!(
                "Compared nums[{}] = {} with target {}.",
                index, value, target
            ),
            visual: VisualState::BinarySearch {
                nums: nums_vec.clone(),
                target,
                left: index,
                right: last_index,
                mid: Some(index),
                found_idx: None,
            },
        });

        if value == target {
            steps.push(Step {
                code_line: 5,
                description: format!(
                    "Found target {} at index {}. Return {}.",
                    target, index, index
                ),
                visual: VisualState::BinarySearch {
                    nums: nums_vec,
                    target,
                    left: index,
                    right: last_index,
                    mid: Some(index),
                    found_idx: Some(index),
                },
            });
            return steps;
        }
    }

    steps.push(Step {
        code_line: 6,
        description: format!(
            "Scanned every value without finding target {}. Return -1.",
            target
        ),
        visual: VisualState::BinarySearch {
            nums: nums_vec,
            target,
            left: nums.len(),
            right: last_index,
            mid: None,
            found_idx: None,
        },
    });

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    fn result_index(steps: &[Step]) -> Option<i32> {
        match &steps.last()?.visual {
            VisualState::BinarySearch { found_idx, .. } => {
                Some(found_idx.map_or(-1, |index| index as i32))
            }
            VisualState::TraceUnavailable { .. } => None,
            _ => None,
        }
    }

    #[test]
    fn both_approaches_find_the_same_index() {
        let nums = [-4, -1, 0, 3, 9, 12];
        for target in [-4, 3, 12, 7] {
            let binary = generate_binary_search_steps(&nums, target, 0);
            let linear = generate_binary_search_steps(&nums, target, 1);
            assert_eq!(result_index(&binary), result_index(&linear));
        }
    }

    #[test]
    fn linear_trace_rejects_oversized_inputs() {
        let nums = (0..=LINEAR_SCAN_VISUALIZATION_LIMIT)
            .map(|value| value as i32)
            .collect::<Vec<_>>();
        let steps = generate_binary_search_steps(&nums, 0, 1);
        assert!(matches!(
            steps.as_slice(),
            [Step {
                visual: VisualState::TraceUnavailable { .. },
                ..
            }]
        ));
    }

    #[test]
    fn both_approaches_reject_values_outside_the_sorted_unique_contract() {
        for nums in [vec![2, 1, 3], vec![1, 2, 2, 3]] {
            for approach_id in [0, 1] {
                let steps = generate_binary_search_steps(&nums, 2, approach_id);
                assert!(matches!(
                    steps.as_slice(),
                    [Step {
                        visual: VisualState::TraceUnavailable { .. },
                        ..
                    }]
                ));
            }
        }
    }

    #[test]
    fn binary_search_trace_rejects_oversized_inputs() {
        let nums = (0..=BINARY_SEARCH_VISUALIZATION_LIMIT)
            .map(|value| value as i32)
            .collect::<Vec<_>>();
        assert!(matches!(
            generate_binary_search_steps(&nums, -1, 0).as_slice(),
            [Step {
                visual: VisualState::TraceUnavailable { .. },
                ..
            }]
        ));
    }

    #[test]
    fn binary_search_trace_marks_the_range_exhausted_before_index_zero() {
        let steps = generate_binary_search_steps(&[1, 2], 0, 0);
        assert!(matches!(
            &steps.last().expect("trace must not be empty").visual,
            VisualState::BinarySearch {
                left: 1,
                right: 0,
                found_idx: None,
                ..
            }
        ));
    }
}
