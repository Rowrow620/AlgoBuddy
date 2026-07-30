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
