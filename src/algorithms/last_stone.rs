use crate::model::{Step, VisualState};

pub fn generate_last_stone_steps(stones: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut s = stones.to_vec();

    steps.push(Step {
        code_line: 3,
        description: format!("Initial stone weights array: {:?}", s),
        visual: VisualState::ContainsDuplicate {
            nums: s.clone(),
            active_idx: None,
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });

    while s.len() > 1 {
        s.sort();
        let y = s.pop().unwrap();
        let x = s.pop().unwrap();

        steps.push(Step {
            code_line: 6,
            description: format!("Smashed heaviest stones y={} and x={}.", y, x),
            visual: VisualState::ContainsDuplicate {
                nums: s.clone(),
                active_idx: None,
                seen_set: std::collections::BTreeSet::new(),
                duplicate_val: None,
                has_duplicate: None,
            },
        });

        if y != x {
            let diff = y - x;
            s.push(diff);
            steps.push(Step {
                code_line: 8,
                description: format!("Stones destroyed! Remaining stone weight = {}.", diff),
                visual: VisualState::ContainsDuplicate {
                    nums: s.clone(),
                    active_idx: Some(s.len() - 1),
                    seen_set: std::collections::BTreeSet::new(),
                    duplicate_val: None,
                    has_duplicate: None,
                },
            });
        } else {
            steps.push(Step {
                code_line: 10,
                description: "Both stones completely destroyed!".to_string(),
                visual: VisualState::ContainsDuplicate {
                    nums: s.clone(),
                    active_idx: None,
                    seen_set: std::collections::BTreeSet::new(),
                    duplicate_val: None,
                    has_duplicate: None,
                },
            });
        }
    }

    let last = if s.is_empty() { 0 } else { s[0] };
    steps.push(Step {
        code_line: 12,
        description: format!("Final remaining stone weight = {}.", last),
        visual: VisualState::ContainsDuplicate {
            nums: vec![last],
            active_idx: Some(0),
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: Some(last),
            has_duplicate: Some(true),
        },
    });

    steps
}
