use crate::model::{Step, VisualState};

pub fn generate_climbing_stairs_steps(n: usize) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut dp = vec![0i32; (n + 1).max(3)];
    dp[1] = 1;
    dp[2] = 2;

    steps.push(Step {
        code_line: 3,
        description: format!("Base cases for Climbing Stairs (n={}): dp[1]=1 way, dp[2]=2 ways.", n),
        visual: VisualState::ContainsDuplicate {
            nums: dp.clone(),
            active_idx: Some(2),
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });

    for i in 3..=n {
        dp[i] = dp[i - 1] + dp[i - 2];
        steps.push(Step {
            code_line: 6,
            description: format!("Step {}: dp[{}] = dp[{}] + dp[{}] = {} + {} = {} ways.", i, i, i - 1, i - 2, dp[i - 1], dp[i - 2], dp[i]),
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
        description: format!("Total distinct ways to climb {} stairs = {}.", n, dp[n]),
        visual: VisualState::ContainsDuplicate {
            nums: dp,
            active_idx: Some(n),
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: Some(true),
        },
    });

    steps
}
