use crate::model::{ProductPhase, Step, VisualState};

pub fn generate_product_steps(nums: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();
    let n = nums.len();
    let nums_vec = nums.to_vec();
    let mut output = vec![1i64; n];
    let mut prefix_val = 1i64;
    let mut suffix_val = 1i64;

    // 1. Init step (code_line 4)
    steps.push(Step {
        code_line: 4,
        description: format!("Initialized output array of length {} with 1s.", n),
        visual: VisualState::Product {
            nums: nums_vec.clone(),
            output: output.clone(),
            active_idx: None,
            prefix_val,
            suffix_val,
            phase: ProductPhase::Init,
        },
    });

    // 2. Prefix init step (code_line 6)
    steps.push(Step {
        code_line: 6,
        description: "Initialized prefix product variable prefix = 1.".to_string(),
        visual: VisualState::Product {
            nums: nums_vec.clone(),
            output: output.clone(),
            active_idx: None,
            prefix_val,
            suffix_val,
            phase: ProductPhase::PrefixPass,
        },
    });

    // 3. Prefix pass (code_lines 8-9)
    for i in 0..n {
        output[i] = prefix_val;
        steps.push(Step {
            code_line: 8,
            description: format!("Set output[{}] = prefix ({}).", i, prefix_val),
            visual: VisualState::Product {
                nums: nums_vec.clone(),
                output: output.clone(),
                active_idx: Some(i),
                prefix_val,
                suffix_val,
                phase: ProductPhase::PrefixPass,
            },
        });

        prefix_val *= nums[i] as i64;
        steps.push(Step {
            code_line: 9,
            description: format!(
                "Updated prefix product: prefix *= nums[{}] ({}) resulting in {}.",
                i, nums[i], prefix_val
            ),
            visual: VisualState::Product {
                nums: nums_vec.clone(),
                output: output.clone(),
                active_idx: Some(i),
                prefix_val,
                suffix_val,
                phase: ProductPhase::PrefixPass,
            },
        });
    }

    // 4. Suffix init step (code_line 11)
    steps.push(Step {
        code_line: 11,
        description: "Initialized suffix product variable suffix = 1.".to_string(),
        visual: VisualState::Product {
            nums: nums_vec.clone(),
            output: output.clone(),
            active_idx: None,
            prefix_val,
            suffix_val: 1,
            phase: ProductPhase::SuffixPass,
        },
    });
    suffix_val = 1;

    // 5. Suffix pass (code_lines 13-14)
    for i in (0..n).rev() {
        output[i] *= suffix_val;
        steps.push(Step {
            code_line: 13,
            description: format!(
                "Updated output[{}]: output[{}] *= suffix ({}) resulting in {}.",
                i, i, suffix_val, output[i]
            ),
            visual: VisualState::Product {
                nums: nums_vec.clone(),
                output: output.clone(),
                active_idx: Some(i),
                prefix_val,
                suffix_val,
                phase: ProductPhase::SuffixPass,
            },
        });

        suffix_val *= nums[i] as i64;
        steps.push(Step {
            code_line: 14,
            description: format!(
                "Updated suffix product: suffix *= nums[{}] ({}) resulting in {}.",
                i, nums[i], suffix_val
            ),
            visual: VisualState::Product {
                nums: nums_vec.clone(),
                output: output.clone(),
                active_idx: Some(i),
                prefix_val,
                suffix_val,
                phase: ProductPhase::SuffixPass,
            },
        });
    }

    // 6. Complete step (code_line 16)
    steps.push(Step {
        code_line: 16,
        description: format!(
            "Completed products of array except self calculation. Final output: {:?}.",
            output
        ),
        visual: VisualState::Product {
            nums: nums_vec,
            output,
            active_idx: None,
            prefix_val,
            suffix_val,
            phase: ProductPhase::Complete,
        },
    });

    steps
}
