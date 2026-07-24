use crate::model::{Step, VisualState};

pub fn generate_reverse_bits_steps(n: u32) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut res = 0u32;

    steps.push(Step {
        code_line: 3,
        description: format!("Reversing 32 bits of n = {} ({:032b}).", n, n),
        visual: VisualState::ContainsDuplicate {
            nums: vec![n as i32],
            active_idx: Some(0),
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });

    for i in 0..32 {
        let bit = (n >> i) & 1;
        res |= bit << (31 - i);
    }

    steps.push(Step {
        code_line: 7,
        description: format!("Reversed 32-bit integer = {} ({:032b}).", res, res),
        visual: VisualState::ContainsDuplicate {
            nums: vec![res as i32],
            active_idx: Some(0),
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: Some(res as i32),
            has_duplicate: Some(true),
        },
    });

    steps
}
