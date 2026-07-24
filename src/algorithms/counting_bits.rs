use crate::model::{Step, VisualState};

pub fn generate_counting_bits_array_steps(n: usize) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut dp = vec![0i32; n + 1];

    steps.push(Step {
        code_line: 3,
        description: format!("Counting bits for range 0..={} using DP offsets.", n),
        visual: VisualState::ContainsDuplicate {
            nums: dp.clone(),
            active_idx: None,
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });

    let mut offset = 1;
    for i in 1..=n {
        if offset * 2 == i {
            offset = i;
        }
        dp[i] = 1 + dp[i - offset];
        steps.push(Step {
            code_line: 6,
            description: format!("i={} (offset={}): dp[{}] = 1 + dp[{}] = 1 + {} = {}.", i, offset, i, i - offset, dp[i - offset], dp[i]),
            visual: VisualState::ContainsDuplicate {
                nums: dp.clone(),
                active_idx: Some(i),
                seen_set: std::collections::BTreeSet::new(),
                duplicate_val: None,
                has_duplicate: None,
            },
        });
    }

    steps.push(Step {
        code_line: 8,
        description: format!("Counting bits array for 0..={}: {:?}.", n, dp),
        visual: VisualState::ContainsDuplicate {
            nums: dp,
            active_idx: None,
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: Some(true),
        },
    });

    steps
}
