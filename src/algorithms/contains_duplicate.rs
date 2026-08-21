use crate::model::{Step, VisualState};
use std::collections::BTreeSet;

pub(crate) const CONTAINS_DUPLICATE_VISUALIZATION_LIMIT: usize = 128;

pub fn generate_contains_duplicate_steps(nums: &[i32], approach_id: usize) -> Vec<Step> {
    if nums.len() > CONTAINS_DUPLICATE_VISUALIZATION_LIMIT {
        return vec![Step::trace_unavailable(
            "Contains Duplicate",
            CONTAINS_DUPLICATE_VISUALIZATION_LIMIT,
            "each step stores the array state",
        )];
    }

    let mut steps = Vec::new();
    let num_vec = nums.to_vec();

    if approach_id == 0 {
        // Hash Set Approach
        let mut seen = BTreeSet::new();

        steps.push(Step {
            code_line: 3,
            description: "Initialized empty HashSet `seen = set()`.".to_string(),
            visual: VisualState::ContainsDuplicate {
                nums: num_vec.clone(),
                active_idx: None,
                seen_set: seen.clone(),
                duplicate_val: None,
                has_duplicate: None,
            },
        });

        for (i, &num) in nums.iter().enumerate() {
            steps.push(Step {
                code_line: 5,
                description: format!(
                    "Checking element nums[{}] = {}. Is {} in seen set?",
                    i, num, num
                ),
                visual: VisualState::ContainsDuplicate {
                    nums: num_vec.clone(),
                    active_idx: Some(i),
                    seen_set: seen.clone(),
                    duplicate_val: None,
                    has_duplicate: None,
                },
            });

            if seen.contains(&num) {
                steps.push(Step {
                    code_line: 6,
                    description: format!(
                        "Duplicate found! Element {} is already in `seen` set. Return True.",
                        num
                    ),
                    visual: VisualState::ContainsDuplicate {
                        nums: num_vec.clone(),
                        active_idx: Some(i),
                        seen_set: seen,
                        duplicate_val: Some(num),
                        has_duplicate: Some(true),
                    },
                });
                return steps;
            }

            seen.insert(num);
            steps.push(Step {
                code_line: 7,
                description: format!("Inserted {} into `seen` set.", num),
                visual: VisualState::ContainsDuplicate {
                    nums: num_vec.clone(),
                    active_idx: Some(i),
                    seen_set: seen.clone(),
                    duplicate_val: None,
                    has_duplicate: None,
                },
            });
        }

        steps.push(Step {
            code_line: 8,
            description: "Scanned entire array. All elements are distinct. Return False."
                .to_string(),
            visual: VisualState::ContainsDuplicate {
                nums: num_vec,
                active_idx: None,
                seen_set: seen,
                duplicate_val: None,
                has_duplicate: Some(false),
            },
        });
    } else if approach_id == 1 {
        // Sorting Approach
        let mut sorted = nums.to_vec();
        sorted.sort();

        steps.push(Step {
            code_line: 3,
            description: format!("Sorted array: {:?}", sorted),
            visual: VisualState::ContainsDuplicate {
                nums: sorted.clone(),
                active_idx: None,
                seen_set: BTreeSet::new(),
                duplicate_val: None,
                has_duplicate: None,
            },
        });

        for i in 1..sorted.len() {
            let prev = sorted[i - 1];
            let curr = sorted[i];

            steps.push(Step {
                code_line: 5,
                description: format!(
                    "Comparing adjacent elements: sorted[{}] = {} vs sorted[{}] = {}.",
                    i - 1,
                    prev,
                    i,
                    curr
                ),
                visual: VisualState::ContainsDuplicate {
                    nums: sorted.clone(),
                    active_idx: Some(i),
                    seen_set: BTreeSet::new(),
                    duplicate_val: None,
                    has_duplicate: None,
                },
            });

            if prev == curr {
                steps.push(Step {
                    code_line: 6,
                    description: format!(
                        "Duplicate adjacent values equal! {} == {}. Return True.",
                        prev, curr
                    ),
                    visual: VisualState::ContainsDuplicate {
                        nums: sorted,
                        active_idx: Some(i),
                        seen_set: BTreeSet::new(),
                        duplicate_val: Some(curr),
                        has_duplicate: Some(true),
                    },
                });
                return steps;
            }
        }

        steps.push(Step {
            code_line: 7,
            description: "No adjacent elements match in sorted array. Return False.".to_string(),
            visual: VisualState::ContainsDuplicate {
                nums: sorted,
                active_idx: None,
                seen_set: BTreeSet::new(),
                duplicate_val: None,
                has_duplicate: Some(false),
            },
        });
    } else {
        return Vec::new();
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    fn result(nums: &[i32], approach_id: usize) -> Option<bool> {
        let steps = generate_contains_duplicate_steps(nums, approach_id);
        match &steps.last().expect("trace must have a result").visual {
            VisualState::ContainsDuplicate { has_duplicate, .. } => *has_duplicate,
            _ => panic!("expected contains-duplicate state"),
        }
    }

    #[test]
    fn hash_set_and_sorting_agree() {
        for (nums, expected) in [
            (vec![1, 2, 3, 1], true),
            (vec![1, 2, 3, 4], false),
            (vec![-1, -1], true),
        ] {
            assert_eq!(result(&nums, 0), Some(expected));
            assert_eq!(result(&nums, 1), Some(expected));
        }
    }

    #[test]
    fn oversized_inputs_return_an_explicit_trace_status() {
        let nums = vec![1; CONTAINS_DUPLICATE_VISUALIZATION_LIMIT + 1];
        let steps = generate_contains_duplicate_steps(&nums, 0);

        assert!(matches!(
            &steps[0].visual,
            VisualState::TraceUnavailable { .. }
        ));
    }
}
