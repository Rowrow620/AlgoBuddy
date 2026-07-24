use std::collections::BTreeMap;
use crate::model::{Step, VisualState};

pub fn generate_two_sum_steps(nums: &[i32], target: i32, approach_id: usize) -> Vec<Step> {
    if approach_id == 1 {
        generate_two_sum_brute_force(nums, target)
    } else {
        generate_two_sum_hash_map(nums, target)
    }
}

fn generate_two_sum_hash_map(nums: &[i32], target: i32) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut map: BTreeMap<i32, usize> = BTreeMap::new();
    let nums_vec = nums.to_vec();

    // 1. Init map (code_line 3)
    steps.push(Step {
        code_line: 3,
        description: "Initialized empty hash map prevMap = {} to store value -> index pairs.".to_string(),
        visual: VisualState::TwoSum {
            nums: nums_vec.clone(),
            target,
            active_idx: None,
            secondary_idx: None,
            map: map.clone(),
            found_indices: None,
        },
    });

    // 2. Loop through nums (code_line 4)
    for (i, &n) in nums.iter().enumerate() {
        let diff = target - n;

        steps.push(Step {
            code_line: 5,
            description: format!("Index i={}: Current num={}, calculating complement diff = target ({}) - n ({}) = {}.", i, n, target, n, diff),
            visual: VisualState::TwoSum {
                nums: nums_vec.clone(),
                target,
                active_idx: Some(i),
                secondary_idx: None,
                map: map.clone(),
                found_indices: None,
            },
        });

        if let Some(&prev_idx) = map.get(&diff) {
            steps.push(Step {
                code_line: 7,
                description: format!("Found complement diff={} in prevMap at index {}! Solution indices: [{}, {}].", diff, prev_idx, prev_idx, i),
                visual: VisualState::TwoSum {
                    nums: nums_vec.clone(),
                    target,
                    active_idx: Some(i),
                    secondary_idx: Some(prev_idx),
                    map: map.clone(),
                    found_indices: Some((prev_idx, i)),
                },
            });
            return steps;
        } else {
            steps.push(Step {
                code_line: 6,
                description: format!("Complement diff={} not found in prevMap.", diff),
                visual: VisualState::TwoSum {
                    nums: nums_vec.clone(),
                    target,
                    active_idx: Some(i),
                    secondary_idx: None,
                    map: map.clone(),
                    found_indices: None,
                },
            });
        }

        map.insert(n, i);
        steps.push(Step {
            code_line: 8,
            description: format!("Inserted key n={} with index i={} into prevMap.", n, i),
            visual: VisualState::TwoSum {
                nums: nums_vec.clone(),
                target,
                active_idx: Some(i),
                secondary_idx: None,
                map: map.clone(),
                found_indices: None,
            },
        });
    }

    steps.push(Step {
        code_line: 9,
        description: "No two sum pair found. Returning empty array [].".to_string(),
        visual: VisualState::TwoSum {
            nums: nums_vec,
            target,
            active_idx: None,
            secondary_idx: None,
            map,
            found_indices: None,
        },
    });

    steps
}

fn generate_two_sum_brute_force(nums: &[i32], target: i32) -> Vec<Step> {
    let mut steps = Vec::new();
    let nums_vec = nums.to_vec();
    let n = nums.len();
    let empty_map = BTreeMap::new();

    steps.push(Step {
        code_line: 3,
        description: format!("Initialized Brute Force pair search. Array length n={}.", n),
        visual: VisualState::TwoSum {
            nums: nums_vec.clone(),
            target,
            active_idx: None,
            secondary_idx: None,
            map: empty_map.clone(),
            found_indices: None,
        },
    });

    for i in 0..n {
        steps.push(Step {
            code_line: 4,
            description: format!("Outer loop: i = {} (nums[i] = {}).", i, nums[i]),
            visual: VisualState::TwoSum {
                nums: nums_vec.clone(),
                target,
                active_idx: Some(i),
                secondary_idx: None,
                map: empty_map.clone(),
                found_indices: None,
            },
        });

        for j in (i + 1)..n {
            let sum = nums[i] + nums[j];
            if sum == target {
                steps.push(Step {
                    code_line: 6,
                    description: format!("Checking pair (i={}, j={}): nums[{}] ({}) + nums[{}] ({}) == {} MATCH!", i, j, i, nums[i], j, nums[j], target),
                    visual: VisualState::TwoSum {
                        nums: nums_vec.clone(),
                        target,
                        active_idx: Some(i),
                        secondary_idx: Some(j),
                        map: empty_map.clone(),
                        found_indices: Some((i, j)),
                    },
                });

                steps.push(Step {
                    code_line: 7,
                    description: format!("Returning solution indices [{}, {}].", i, j),
                    visual: VisualState::TwoSum {
                        nums: nums_vec.clone(),
                        target,
                        active_idx: Some(i),
                        secondary_idx: Some(j),
                        map: empty_map.clone(),
                        found_indices: Some((i, j)),
                    },
                });
                return steps;
            } else {
                steps.push(Step {
                    code_line: 6,
                    description: format!("Checking pair (i={}, j={}): nums[{}] ({}) + nums[{}] ({}) = {} != target ({}).", i, j, i, nums[i], j, nums[j], sum, target),
                    visual: VisualState::TwoSum {
                        nums: nums_vec.clone(),
                        target,
                        active_idx: Some(i),
                        secondary_idx: Some(j),
                        map: empty_map.clone(),
                        found_indices: None,
                    },
                });
            }
        }
    }

    steps.push(Step {
        code_line: 8,
        description: "No pair found. Returning [].".to_string(),
        visual: VisualState::TwoSum {
            nums: nums_vec,
            target,
            active_idx: None,
            secondary_idx: None,
            map: empty_map,
            found_indices: None,
        },
    });

    steps
}
