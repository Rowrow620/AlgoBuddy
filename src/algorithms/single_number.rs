use crate::model::{Step, VisualState};

pub fn generate_single_number_steps(nums: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();
    let num_vec = nums.to_vec();
    let mut res = 0;

    steps.push(Step {
        code_line: 3,
        description: "Single Number using Bitwise XOR (a ^ a = 0, a ^ 0 = a). Initial res = 0."
            .into(),
        visual: VisualState::ContainsDuplicate {
            nums: num_vec.clone(),
            active_idx: None,
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });

    for (i, &n) in nums.iter().enumerate() {
        let prev_res = res;
        res ^= n;
        steps.push(Step {
            code_line: 4,
            description: format!(
                "XOR with nums[{}] = {}: {} ^ {} = {}.",
                i, n, prev_res, n, res
            ),
            visual: VisualState::ContainsDuplicate {
                nums: num_vec.clone(),
                active_idx: Some(i),
                seen_set: std::collections::BTreeSet::new(),
                duplicate_val: Some(res),
                has_duplicate: None,
            },
        });
    }

    steps.push(Step {
        code_line: 5,
        description: format!("Single non-duplicate element = {}.", res),
        visual: VisualState::ContainsDuplicate {
            nums: num_vec,
            active_idx: None,
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: Some(res),
            has_duplicate: Some(true),
        },
    });

    steps
}

pub(crate) const SINGLE_NUMBER_NESTED_SCAN_LIMIT: usize = 40;

pub fn generate_single_number_nested_scan_steps(nums: &[i32]) -> Vec<Step> {
    if nums.len() > SINGLE_NUMBER_NESTED_SCAN_LIMIT {
        let message = format!(
            "Nested frequency-scan visualization supports up to {} values; shorten the input before building the quadratic trace.",
            SINGLE_NUMBER_NESTED_SCAN_LIMIT
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
        description:
            "Try each value as the single number, counting it with a fresh full-array scan.".into(),
        visual: VisualState::Array1D {
            title: "Single Number: Nested Frequency Scan".into(),
            elements: elements.clone(),
            active_idx: None,
            secondary_idx: None,
            pointers: Vec::new(),
            status_message: "start outer scan".into(),
            is_success: None,
        },
    }];

    for (i, &value) in nums.iter().enumerate() {
        let mut count = 0usize;
        steps.push(Step {
            code_line: 4,
            description: format!(
                "Treat nums[{i}] = {value} as the candidate and reset count to 0."
            ),
            visual: VisualState::Array1D {
                title: "Single Number: Nested Frequency Scan".into(),
                elements: elements.clone(),
                active_idx: Some(i),
                secondary_idx: None,
                pointers: Vec::new(),
                status_message: format!("candidate = {value}"),
                is_success: None,
            },
        });

        for (j, &candidate) in nums.iter().enumerate() {
            if candidate == value {
                count += 1;
            }
            steps.push(Step {
                code_line: 6,
                description: format!(
                    "Compare nums[{j}] = {candidate} with candidate {value}; running count = {count}."
                ),
                visual: VisualState::Array1D {
                    title: "Single Number: Nested Frequency Scan".into(),
                    elements: elements.clone(),
                    active_idx: Some(i),
                    secondary_idx: Some(j),
                    pointers: Vec::new(),
                    status_message: if candidate == value {
                        "match; increment count".into()
                    } else {
                        "different value".into()
                    },
                    is_success: None,
                },
            });
        }

        if count == 1 {
            steps.push(Step {
                code_line: 7,
                description: format!("{value} appears exactly once, so it is the single number."),
                visual: VisualState::Array1D {
                    title: "Single Number: Nested Frequency Scan".into(),
                    elements,
                    active_idx: Some(i),
                    secondary_idx: None,
                    pointers: Vec::new(),
                    status_message: format!("return {value}"),
                    is_success: Some(true),
                },
            });
            return steps;
        }
    }

    steps.push(Step {
        code_line: 8,
        description:
            "No value appeared exactly once; the input does not satisfy the problem contract."
                .into(),
        visual: VisualState::Array1D {
            title: "Single Number: Nested Frequency Scan".into(),
            elements,
            active_idx: None,
            secondary_idx: None,
            pointers: Vec::new(),
            status_message: "no single number".into(),
            is_success: Some(false),
        },
    });
    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nested_scan_finds_the_value_that_occurs_once() {
        let steps = generate_single_number_nested_scan_steps(&[4, 1, 2, 1, 2]);
        assert!(steps
            .last()
            .expect("trace must not be empty")
            .description
            .contains("4 appears exactly once"));
    }

    #[test]
    fn nested_scan_has_a_quadratic_trace_limit() {
        let nums = vec![0; SINGLE_NUMBER_NESTED_SCAN_LIMIT + 1];
        let steps = generate_single_number_nested_scan_steps(&nums);
        assert!(matches!(
            steps.as_slice(),
            [Step {
                visual: VisualState::TraceUnavailable { .. },
                ..
            }]
        ));
    }
}
