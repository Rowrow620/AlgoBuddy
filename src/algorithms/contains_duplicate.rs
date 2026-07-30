use crate::model::{Step, VisualState};
use std::collections::BTreeSet;

pub fn generate_contains_duplicate_steps(nums: &[i32], approach_id: usize) -> Vec<Step> {
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
    } else {
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
    }

    steps
}
