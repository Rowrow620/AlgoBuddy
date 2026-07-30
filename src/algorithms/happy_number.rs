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
            code_line: 6,
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
        code_line: 8,
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
