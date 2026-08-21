use crate::model::{Step, VisualState};

pub fn generate_missing_number_steps(nums: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();
    let n = nums.len() as i64;
    let expected_sum = (n * (n + 1)) / 2;
    let actual_sum: i64 = nums.iter().map(|&x| x as i64).sum();
    let missing = (expected_sum - actual_sum) as i32;

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
        description: format!(
            "Expected sum(0..={}) = {}. Actual array sum = {}.",
            n, expected_sum, actual_sum
        ),
        visual: VisualState::ContainsDuplicate {
            nums: vec![
                expected_sum.min(i32::MAX as i64) as i32,
                actual_sum.min(i32::MAX as i64) as i32,
            ],
            active_idx: Some(1),
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });

    steps.push(Step {
        code_line: 7,
        description: format!(
            "Missing number = expected_sum - actual_sum = {} - {} = {}.",
            expected_sum, actual_sum, missing
        ),
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

pub(crate) const MISSING_NUMBER_MEMBERSHIP_SCAN_LIMIT: usize = 40;

pub fn generate_missing_number_membership_scan_steps(nums: &[i32]) -> Vec<Step> {
    if nums.len() > MISSING_NUMBER_MEMBERSHIP_SCAN_LIMIT {
        let message = format!(
            "Candidate-membership visualization supports up to {} values; shorten the input before building the quadratic trace.",
            MISSING_NUMBER_MEMBERSHIP_SCAN_LIMIT
        );
        return vec![Step {
            code_line: 3,
            description: message.clone(),
            visual: VisualState::TraceUnavailable { message },
        }];
    }

    let elements = nums.to_vec();
    let mut steps = vec![Step {
        code_line: 3,
        description: format!(
            "Try every candidate from 0 through {}; scan the whole array for each one.",
            nums.len()
        ),
        visual: VisualState::Array1D {
            title: "Missing Number: Candidate Membership Scan".into(),
            elements: elements.clone(),
            active_idx: None,
            secondary_idx: None,
            pointers: Vec::new(),
            status_message: "begin candidate scan".into(),
            is_success: None,
        },
    }];

    for candidate in 0..=nums.len() {
        let mut found = false;
        steps.push(Step {
            code_line: 4,
            description: format!("Set found = False for candidate {candidate}."),
            visual: VisualState::Array1D {
                title: "Missing Number: Candidate Membership Scan".into(),
                elements: elements.clone(),
                active_idx: None,
                secondary_idx: None,
                pointers: Vec::new(),
                status_message: "found = False".into(),
                is_success: None,
            },
        });

        for (index, &value) in nums.iter().enumerate() {
            found = value == candidate as i32;
            steps.push(Step {
                code_line: 6,
                description: format!(
                    "Compare candidate {candidate} with nums[{index}] = {value}: {}.",
                    if found { "found" } else { "not equal" }
                ),
                visual: VisualState::Array1D {
                    title: "Missing Number: Candidate Membership Scan".into(),
                    elements: elements.clone(),
                    active_idx: Some(index),
                    secondary_idx: None,
                    pointers: Vec::new(),
                    status_message: format!("found = {found}"),
                    is_success: None,
                },
            });
            if found {
                break;
            }
        }

        if !found {
            steps.push(Step {
                code_line: 7,
                description: format!(
                    "Candidate {candidate} was absent, so it is the missing number."
                ),
                visual: VisualState::Array1D {
                    title: "Missing Number: Candidate Membership Scan".into(),
                    elements,
                    active_idx: None,
                    secondary_idx: None,
                    pointers: Vec::new(),
                    status_message: format!("return {candidate}"),
                    is_success: Some(true),
                },
            });
            return steps;
        }
    }

    unreachable!("one value from 0..=len must be absent for a valid input")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn candidate_membership_scan_finds_the_gap() {
        let steps = generate_missing_number_membership_scan_steps(&[3, 0, 1]);
        assert!(steps
            .last()
            .expect("trace must not be empty")
            .description
            .contains("Candidate 2 was absent"));
    }

    #[test]
    fn candidate_membership_scan_has_a_quadratic_trace_limit() {
        let nums = vec![0; MISSING_NUMBER_MEMBERSHIP_SCAN_LIMIT + 1];
        let steps = generate_missing_number_membership_scan_steps(&nums);
        assert!(matches!(
            steps.as_slice(),
            [Step {
                visual: VisualState::TraceUnavailable { .. },
                ..
            }]
        ));
    }
}
