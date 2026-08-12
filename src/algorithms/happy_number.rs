use crate::model::{Step, VisualState};
use std::collections::BTreeSet;

pub fn generate_happy_number_steps(n: i32) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut seen = BTreeSet::new();
    let mut curr = n;

    steps.push(Step {
        code_line: 3,
        description: format!("Happy Number cycle detection for starting number n={}.", n),
        visual: VisualState::ContainsDuplicate {
            nums: vec![curr],
            active_idx: Some(0),
            seen_set: seen.clone(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });

    while curr != 1 && !seen.contains(&curr) {
        seen.insert(curr);

        let mut sum = 0;
        let mut temp = curr;
        while temp > 0 {
            let digit = temp % 10;
            sum += digit * digit;
            temp /= 10;
        }

        steps.push(Step {
            code_line: 5,
            description: format!(
                "Sum of squared digits of {} -> {}. Added to seen set.",
                curr, sum
            ),
            visual: VisualState::ContainsDuplicate {
                nums: vec![curr, sum],
                active_idx: Some(1),
                seen_set: seen.clone(),
                duplicate_val: None,
                has_duplicate: None,
            },
        });

        curr = sum;
    }

    let is_happy = curr == 1;
    steps.push(Step {
        code_line: if is_happy { 6 } else { 7 },
        description: if is_happy {
            format!("Reached 1! Number {} is a Happy Number! Return True.", n)
        } else {
            format!(
                "Infinite cycle detected at {}! Number {} is NOT a Happy Number. Return False.",
                curr, n
            )
        },
        visual: VisualState::ContainsDuplicate {
            nums: vec![curr],
            active_idx: Some(0),
            seen_set: seen,
            duplicate_val: if is_happy { None } else { Some(curr) },
            has_duplicate: Some(is_happy),
        },
    });

    steps
}

pub(crate) const HAPPY_NUMBER_LINEAR_SEARCH_LIMIT: usize = 128;

pub fn generate_happy_number_linear_search_steps(n: i32) -> Vec<Step> {
    generate_happy_number_linear_search_steps_with_limit(n, HAPPY_NUMBER_LINEAR_SEARCH_LIMIT)
}

fn generate_happy_number_linear_search_steps_with_limit(n: i32, limit: usize) -> Vec<Step> {
    let mut sequence = Vec::new();
    let mut current = n;

    let (is_happy, repeated_index) = loop {
        if current == 1 {
            break (true, None);
        }
        if let Some(index) = sequence.iter().position(|&value| value == current) {
            break (false, Some(index));
        }
        if sequence.len() >= limit {
            let message = format!(
                "Linear repeat-search visualization supports up to {limit} generated values; shorten the trace before continuing."
            );
            return vec![Step {
                code_line: 3,
                description: message.clone(),
                visual: VisualState::TraceUnavailable { message },
            }];
        }
        sequence.push(current);
        current = square_digit_sum(current);
    };

    let mut steps = vec![Step {
        code_line: 3,
        description: format!(
            "Start a sequence for n={n}; each repeat check scans the values already generated."
        ),
        visual: VisualState::Array1D {
            title: "Happy Number: Linear Repeat Search".into(),
            elements: Vec::new(),
            active_idx: None,
            secondary_idx: None,
            pointers: Vec::new(),
            status_message: format!("current = {n}"),
            is_success: None,
        },
    }];

    for index in 0..sequence.len() {
        let value = sequence[index];
        let next = square_digit_sum(value);
        steps.push(Step {
            code_line: 7,
            description: format!(
                "No earlier copy of {value} was found. Append it, then replace it with the squared-digit sum {next}."
            ),
            visual: VisualState::Array1D {
                title: "Happy Number: Linear Repeat Search".into(),
                elements: sequence[..=index].to_vec(),
                active_idx: Some(index),
                secondary_idx: None,
                pointers: vec![("current", index)],
                status_message: format!("{value} -> {next}"),
                is_success: None,
            },
        });
    }

    if is_happy {
        let mut completed = sequence;
        completed.push(1);
        let final_index = completed.len() - 1;
        steps.push(Step {
            code_line: 8,
            description: format!("The sequence reached 1, so {n} is a happy number."),
            visual: VisualState::Array1D {
                title: "Happy Number: Linear Repeat Search".into(),
                elements: completed,
                active_idx: Some(final_index),
                secondary_idx: None,
                pointers: Vec::new(),
                status_message: "return True".into(),
                is_success: Some(true),
            },
        });
    } else {
        let index = repeated_index.expect("an unhappy sequence must repeat");
        steps.push(Step {
            code_line: 5,
            description: format!(
                "A linear scan finds {current} already stored at sequence index {index}; the sequence cycles."
            ),
            visual: VisualState::Array1D {
                title: "Happy Number: Linear Repeat Search".into(),
                elements: sequence,
                active_idx: Some(index),
                secondary_idx: None,
                pointers: Vec::new(),
                status_message: "return False".into(),
                is_success: Some(false),
            },
        });
    }

    steps
}

fn square_digit_sum(n: i32) -> i32 {
    let mut value = n.unsigned_abs();
    let mut sum = 0u32;
    while value > 0 {
        let digit = value % 10;
        sum += digit * digit;
        value /= 10;
    }
    sum as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn final_success(steps: &[Step]) -> Option<bool> {
        match &steps.last().expect("trace must not be empty").visual {
            VisualState::Array1D { is_success, .. } => *is_success,
            VisualState::TraceUnavailable { .. } => None,
            _ => panic!("unexpected visual state"),
        }
    }

    #[test]
    fn linear_repeat_search_classifies_happy_and_unhappy_numbers() {
        assert_eq!(
            final_success(&generate_happy_number_linear_search_steps(19)),
            Some(true)
        );
        assert_eq!(
            final_success(&generate_happy_number_linear_search_steps(2)),
            Some(false)
        );
    }

    #[test]
    fn linear_repeat_search_stops_before_an_oversized_trace() {
        let steps = generate_happy_number_linear_search_steps_with_limit(19, 1);
        assert!(matches!(
            steps.as_slice(),
            [Step {
                visual: VisualState::TraceUnavailable { .. },
                ..
            }]
        ));
    }
}
