use crate::model::{Step, VisualState};

pub fn generate_house_robber_steps(nums: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();
    let n = nums.len();

    if n == 0 {
        steps.push(Step {
            code_line: 3,
            description: "No houses available to rob. Return 0.".to_string(),
            visual: VisualState::ContainsDuplicate {
                nums: vec![],
                active_idx: None,
                seen_set: std::collections::BTreeSet::new(),
                duplicate_val: None,
                has_duplicate: Some(true),
            },
        });
        return steps;
    }

    let mut rob1 = 0i32;
    let mut rob2 = 0i32;
    let mut dp = vec![0i32; n];

    steps.push(Step {
        code_line: 3,
        description: format!("House Robber DP for house values: {:?}. Initialize rob1=0, rob2=0.", nums),
        visual: VisualState::ContainsDuplicate {
            nums: nums.to_vec(),
            active_idx: None,
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });

    for (i, &n_val) in nums.iter().enumerate() {
        let temp = (rob1 + n_val).max(rob2);
        rob1 = rob2;
        rob2 = temp;
        dp[i] = temp;

        steps.push(Step {
            code_line: 6,
            description: format!("House {}: val={}. Option A (rob house): rob1 + val = {} + {} = {}. Option B (skip house): rob2 = {}. Max loot = {}.",
                i, n_val, rob1, n_val, rob1 + n_val, rob2, temp),
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
        description: format!("Dynamic Programming traversal complete! Maximum loot = {}.", rob2),
        visual: VisualState::ContainsDuplicate {
            nums: dp,
            active_idx: Some(n - 1),
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: Some(rob2),
            has_duplicate: Some(true),
        },
    });

    steps
}
