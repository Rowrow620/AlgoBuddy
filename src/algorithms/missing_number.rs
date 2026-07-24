use crate::model::{Step, VisualState};

pub fn generate_missing_number_steps(nums: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();
    let n = nums.len();
    let expected_sum = (n * (n + 1)) / 2;
    let actual_sum: i32 = nums.iter().sum();
    let missing = expected_sum as i32 - actual_sum;

    steps.push(Step {
        code_line: 3,
        description: format!("Finding missing number in array of len {}: {:?}", n, nums),
        visual: VisualState::ContainsDuplicate {
            nums: nums.to_vec(),
            active_idx: None,
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });

    steps.push(Step {
        code_line: 5,
        description: format!("Expected sum(0..={}) = {}. Actual array sum = {}.", n, expected_sum, actual_sum),
        visual: VisualState::ContainsDuplicate {
            nums: vec![expected_sum as i32, actual_sum],
            active_idx: Some(1),
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });

    steps.push(Step {
        code_line: 7,
        description: format!("Missing number = expected_sum - actual_sum = {} - {} = {}.", expected_sum, actual_sum, missing),
        visual: VisualState::ContainsDuplicate {
            nums: vec![missing],
            active_idx: Some(0),
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: Some(missing),
            has_duplicate: Some(true),
        },
    });

    steps
}
