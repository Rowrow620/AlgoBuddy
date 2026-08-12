use crate::model::{Step, VisualState};

pub fn generate_plus_one_steps(digits: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut res = digits.to_vec();

    steps.push(Step {
        code_line: 3,
        description: format!("Plus One calculation for digits: {:?}", digits),
        visual: VisualState::ContainsDuplicate {
            nums: res.clone(),
            active_idx: None,
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });

    let n = res.len();
    for i in (0..n).rev() {
        if res[i] < 9 {
            res[i] += 1;
            steps.push(Step {
                code_line: 5,
                description: format!(
                    "Digit at idx {} is {} (<9). Increment to {} and return.",
                    i, digits[i], res[i]
                ),
                visual: VisualState::ContainsDuplicate {
                    nums: res.clone(),
                    active_idx: Some(i),
                    seen_set: std::collections::BTreeSet::new(),
                    duplicate_val: None,
                    has_duplicate: Some(true),
                },
            });
            return steps;
        }

        res[i] = 0;
        steps.push(Step {
            code_line: 7,
            description: format!("Digit at idx {} is 9. Carry over: set digit to 0.", i),
            visual: VisualState::ContainsDuplicate {
                nums: res.clone(),
                active_idx: Some(i),
                seen_set: std::collections::BTreeSet::new(),
                duplicate_val: None,
                has_duplicate: None,
            },
        });
    }

    res.insert(0, 1);
    steps.push(Step {
        code_line: 9,
        description: format!("All digits were 9! Insert 1 at head: {:?}", res),
        visual: VisualState::ContainsDuplicate {
            nums: res,
            active_idx: Some(0),
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: Some(true),
        },
    });

    steps
}

pub(crate) const PLUS_ONE_INTEGER_DIGIT_LIMIT: usize = 38;

pub fn generate_plus_one_integer_conversion_steps(digits: &[i32]) -> Vec<Step> {
    if digits.len() > PLUS_ONE_INTEGER_DIGIT_LIMIT {
        let message = format!(
            "Integer-conversion visualization supports up to {} digits; use the carry approach for larger inputs.",
            PLUS_ONE_INTEGER_DIGIT_LIMIT
        );
        return vec![Step {
            code_line: 3,
            description: message.clone(),
            visual: VisualState::TraceUnavailable { message },
        }];
    }

    let Some(number) = digits.iter().try_fold(0u128, |value, &digit| {
        let digit = u128::try_from(digit).ok().filter(|digit| *digit <= 9)?;
        value.checked_mul(10)?.checked_add(digit)
    }) else {
        let message = "Integer-conversion visualization requires decimal digits and a value that fits in 128 bits."
            .to_string();
        return vec![Step {
            code_line: 3,
            description: message.clone(),
            visual: VisualState::TraceUnavailable { message },
        }];
    };

    let Some(incremented) = number.checked_add(1) else {
        let message = "Adding one would exceed the integer-conversion visualizer's numeric range."
            .to_string();
        return vec![Step {
            code_line: 4,
            description: message.clone(),
            visual: VisualState::TraceUnavailable { message },
        }];
    };
    let result: Vec<i32> = incremented
        .to_string()
        .bytes()
        .map(|byte| i32::from(byte - b'0'))
        .collect();

    vec![
        Step {
            code_line: 3,
            description: format!(
                "Join {:?} and convert the decimal text to {number}.",
                digits
            ),
            visual: VisualState::Array1D {
                title: "Plus One: Integer Conversion".into(),
                elements: digits.to_vec(),
                active_idx: None,
                secondary_idx: None,
                pointers: Vec::new(),
                status_message: format!("number = {number}"),
                is_success: None,
            },
        },
        Step {
            code_line: 4,
            description: format!("Add one to the converted integer: {number} + 1 = {incremented}."),
            visual: VisualState::Array1D {
                title: "Plus One: Integer Conversion".into(),
                elements: digits.to_vec(),
                active_idx: None,
                secondary_idx: None,
                pointers: Vec::new(),
                status_message: format!("incremented = {incremented}"),
                is_success: None,
            },
        },
        Step {
            code_line: 5,
            description: format!("Split {incremented} back into digits: {:?}.", result),
            visual: VisualState::Array1D {
                title: "Plus One: Integer Conversion".into(),
                elements: result,
                active_idx: None,
                secondary_idx: None,
                pointers: Vec::new(),
                status_message: "return converted digits".into(),
                is_success: Some(true),
            },
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_conversion_handles_carry_across_all_digits() {
        let steps = generate_plus_one_integer_conversion_steps(&[9, 9]);
        assert!(matches!(
            &steps.last().expect("trace must not be empty").visual,
            VisualState::Array1D {
                elements,
                is_success: Some(true),
                ..
            } if elements == &[1, 0, 0]
        ));
    }

    #[test]
    fn integer_conversion_rejects_more_digits_than_the_trace_can_hold() {
        let digits = vec![1; PLUS_ONE_INTEGER_DIGIT_LIMIT + 1];
        let steps = generate_plus_one_integer_conversion_steps(&digits);
        assert!(matches!(
            steps.as_slice(),
            [Step {
                visual: VisualState::TraceUnavailable { .. },
                ..
            }]
        ));
    }
}
