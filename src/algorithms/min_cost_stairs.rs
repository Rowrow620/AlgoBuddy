use crate::model::{Step, VisualState};

pub fn generate_min_cost_stairs_steps(cost: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();
    let n = cost.len();
    let mut dp = vec![0i32; n + 1];

    steps.push(Step {
        code_line: 3,
        description: format!("Min Cost Climbing Stairs for cost array: {:?}", cost),
        visual: VisualState::ContainsDuplicate {
            nums: cost.to_vec(),
            active_idx: None,
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });

    for i in 2..=n {
        dp[i] = (dp[i - 1] + cost[i - 1]).min(dp[i - 2] + cost[i - 2]);
        steps.push(Step {
            code_line: 6,
            description: format!(
                "Step {}: min(dp[{}] + cost[{}], dp[{}] + cost[{}]) = min({}, {}) = {}.",
                i,
                i - 1,
                i - 1,
                i - 2,
                i - 2,
                dp[i - 1] + cost[i - 1],
                dp[i - 2] + cost[i - 2],
                dp[i]
            ),
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
        description: format!("Minimum cost to reach top of the floor = {}.", dp[n]),
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
