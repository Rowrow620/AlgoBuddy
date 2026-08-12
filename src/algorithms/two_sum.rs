use crate::model::{Step, VisualState};
use std::collections::BTreeMap;

pub(crate) const BRUTE_FORCE_VISUALIZATION_LIMIT: usize = 40;
pub(crate) const HASH_MAP_VISUALIZATION_LIMIT: usize = 128;

pub fn generate_two_sum_steps(nums: &[i32], target: i32, approach_id: usize) -> Vec<Step> {
    match approach_id {
        0 => generate_two_sum_hash_map(nums, target),
        1 => generate_two_sum_brute_force(nums, target),
        _ => Vec::new(),
    }
}

fn generate_two_sum_hash_map(nums: &[i32], target: i32) -> Vec<Step> {
    if nums.len() > HASH_MAP_VISUALIZATION_LIMIT {
        return vec![Step {
            code_line: 3,
            description: format!(
                "Hash Map visualization supports up to {} values; shorten the input to build the detailed trace.",
                HASH_MAP_VISUALIZATION_LIMIT
            ),
            visual: VisualState::TraceUnavailable {
                message: format!(
                    "Detailed Hash Map traces accept at most {} values because each step stores an array and map snapshot.",
                    HASH_MAP_VISUALIZATION_LIMIT
                ),
            },
        }];
    }

    let mut steps = Vec::new();
    let mut map: BTreeMap<i32, usize> = BTreeMap::new();
    let nums_vec = nums.to_vec();

    // 1. Init map (code_line 3)
    steps.push(Step {
        code_line: 3,
        description: "Initialized empty hash map prevMap = {} to store value -> index pairs."
            .to_string(),
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
        let diff = i64::from(target) - i64::from(n);

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

        let previous_index = i32::try_from(diff)
            .ok()
            .and_then(|complement| map.get(&complement).copied());
        if let Some(prev_idx) = previous_index {
            steps.push(Step {
                code_line: 7,
                description: format!(
                    "Found complement diff={} in prevMap at index {}! Solution indices: [{}, {}].",
                    diff, prev_idx, prev_idx, i
                ),
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
    if nums.len() > BRUTE_FORCE_VISUALIZATION_LIMIT {
        return vec![Step {
            code_line: 3,
            description: format!(
                "Brute Force visualization supports up to {} values; shorten the input to build this quadratic trace.",
                BRUTE_FORCE_VISUALIZATION_LIMIT
            ),
            visual: VisualState::TraceUnavailable {
                message: format!(
                    "Brute Force traces accept at most {} values because every pair becomes a timeline step.",
                    BRUTE_FORCE_VISUALIZATION_LIMIT
                ),
            },
        }];
    }

    let nums_vec = nums.to_vec();
    let empty_map = BTreeMap::new();
    let mut steps = Vec::new();
    let n = nums.len();

    steps.push(Step {
        code_line: 3,
        description: format!("Start the outer loop over all {} values.", n),
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
            code_line: 3,
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
            let sum = i64::from(nums[i]) + i64::from(nums[j]);

            steps.push(Step {
                code_line: 4,
                description: format!(
                    "Inner loop: j = {} (nums[j] = {}). Compare this pair with i = {}.",
                    j, nums[j], i
                ),
                visual: VisualState::TwoSum {
                    nums: nums_vec.clone(),
                    target,
                    active_idx: Some(i),
                    secondary_idx: Some(j),
                    map: empty_map.clone(),
                    found_indices: None,
                },
            });

            if sum == i64::from(target) {
                steps.push(Step {
                    code_line: 5,
                    description: format!(
                        "Pair (i={}, j={}) matches: {} + {} == {}.",
                        i, j, nums[i], nums[j], target
                    ),
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
                    code_line: 6,
                    description: format!("Return solution indices [{}, {}].", i, j),
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
                    code_line: 5,
                    description: format!(
                        "Pair (i={}, j={}) does not match: {} + {} = {}, not {}.",
                        i, j, nums[i], nums[j], sum, target
                    ),
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
        code_line: 7,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn result(nums: &[i32], target: i32, approach_id: usize) -> Option<(usize, usize)> {
        let steps = generate_two_sum_steps(nums, target, approach_id);
        match &steps.last().expect("trace must have a result").visual {
            VisualState::TwoSum { found_indices, .. } => *found_indices,
            _ => panic!("expected two-sum state"),
        }
    }

    #[test]
    fn hash_map_and_brute_force_find_valid_pairs() {
        for (nums, target) in [(vec![2, 7, 11, 15], 9), (vec![3, 3], 6)] {
            for approach_id in [0, 1] {
                let (left, right) =
                    result(&nums, target, approach_id).expect("both approaches must find a pair");
                assert_ne!(left, right);
                assert_eq!(nums[left] + nums[right], target);
            }
        }
    }

    #[test]
    fn brute_force_trace_has_a_visualization_limit() {
        let nums = vec![0; BRUTE_FORCE_VISUALIZATION_LIMIT + 1];
        let steps = generate_two_sum_steps(&nums, 0, 1);

        assert_eq!(steps.len(), 1);
        assert!(steps[0].description.contains("supports up to"));
        assert!(matches!(
            &steps[0].visual,
            VisualState::TraceUnavailable { .. }
        ));
    }

    #[test]
    fn hash_map_trace_has_a_visualization_limit() {
        let nums = vec![0; HASH_MAP_VISUALIZATION_LIMIT + 1];
        let steps = generate_two_sum_steps(&nums, 1, 0);

        assert_eq!(steps.len(), 1);
        assert!(steps[0].description.contains("supports up to"));
        assert!(matches!(
            &steps[0].visual,
            VisualState::TraceUnavailable { .. }
        ));
    }

    #[test]
    fn over_limit_hash_map_input_with_a_valid_pair_is_explicitly_unavailable() {
        let nums = vec![0; HASH_MAP_VISUALIZATION_LIMIT + 1];
        assert_eq!(nums[0] + nums[1], 0, "fixture must contain a valid pair");

        let steps = generate_two_sum_steps(&nums, 0, 0);

        assert_eq!(steps.len(), 1);
        assert!(matches!(
            &steps[0].visual,
            VisualState::TraceUnavailable { message }
                if message.contains("at most") && message.contains("snapshot")
        ));
    }

    #[test]
    fn arithmetic_does_not_wrap_for_extreme_values() {
        let nums = [i32::MAX, i32::MAX];

        assert_eq!(result(&nums, -2, 0), None);
        assert_eq!(result(&nums, -2, 1), None);
    }
}
