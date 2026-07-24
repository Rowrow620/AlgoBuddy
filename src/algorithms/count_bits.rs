use crate::model::{Step, VisualState};

pub fn generate_count_bits_steps(mut n: u32) -> Vec<Step> {
    let mut steps = Vec::new();
    let original = n;
    let mut count = 0;

    steps.push(Step {
        code_line: 3,
        description: format!("Counting set bits (1s) for n = {} (binary: {:032b}).", original, original),
        visual: VisualState::ContainsDuplicate {
            nums: vec![original as i32],
            active_idx: Some(0),
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });

    while n > 0 {
        n &= n - 1; // Brian Kernighan's Algorithm clears lowest set bit
        count += 1;
        steps.push(Step {
            code_line: 6,
            description: format!("Cleared lowest set bit using n & (n - 1). Remaining n = {} (binary: {:032b}). Set count = {}.", n, n, count),
            visual: VisualState::ContainsDuplicate {
                nums: vec![n as i32],
                active_idx: Some(0),
                seen_set: std::collections::BTreeSet::new(),
                duplicate_val: Some(count as i32),
                has_duplicate: None,
            },
        });
    }

    steps.push(Step {
        code_line: 8,
        description: format!("Total set bits (Hamming weight) of {} = {}.", original, count),
        visual: VisualState::ContainsDuplicate {
            nums: vec![count as i32],
            active_idx: Some(0),
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: Some(count as i32),
            has_duplicate: Some(true),
        },
    });

    steps
}
