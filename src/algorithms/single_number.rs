use crate::model::{Step, VisualState};

pub fn generate_single_number_steps(nums: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();
    let num_vec = nums.to_vec();
    let mut res = 0;

    steps.push(Step {
        code_line: 3,
        description: format!(
            "Single Number using Bitwise XOR (a ^ a = 0, a ^ 0 = a). Initial res = 0."
        ),
        visual: VisualState::ContainsDuplicate {
            nums: num_vec.clone(),
            active_idx: None,
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });

    for (i, &n) in nums.iter().enumerate() {
        let prev_res = res;
        res ^= n;
        steps.push(Step {
            code_line: 6,
            description: format!(
                "XOR with nums[{}] = {}: {} ^ {} = {}.",
                i, n, prev_res, n, res
            ),
            visual: VisualState::ContainsDuplicate {
                nums: num_vec.clone(),
                active_idx: Some(i),
                seen_set: std::collections::BTreeSet::new(),
                duplicate_val: Some(res),
                has_duplicate: None,
            },
        });
    }

    steps.push(Step {
        code_line: 8,
        description: format!("Single non-duplicate element = {}.", res),
        visual: VisualState::ContainsDuplicate {
            nums: num_vec,
            active_idx: None,
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: Some(res),
            has_duplicate: Some(true),
        },
    });

    steps
}
