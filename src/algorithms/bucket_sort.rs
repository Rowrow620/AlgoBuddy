use std::collections::BTreeMap;
use crate::model::{Step, VisualState};

pub fn generate_bucket_sort_steps(nums: &[i32], k: usize) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut count_map: BTreeMap<i32, usize> = BTreeMap::new();
    let mut buckets: Vec<Vec<i32>> = vec![Vec::new(); nums.len() + 1];
    let mut result: Vec<i32> = Vec::new();

    // Step 0: Init
    steps.push(Step {
        code_line: 4,
        description: format!(
            "Initialized empty frequency map count = {{}} and bucket array freq of length {}.",
            nums.len() + 1
        ),
        visual: VisualState::TopK {
            nums: nums.to_vec(),
            active_nums_idx: None,
            count_map: count_map.clone(),
            buckets: buckets.clone(),
            active_bucket_idx: None,
            result: result.clone(),
        },
    });

    // Step 1: Count Frequencies
    for (idx, &num) in nums.iter().enumerate() {
        let count_val = {
            let c = count_map.entry(num).or_insert(0);
            *c += 1;
            *c
        };

        steps.push(Step {
            code_line: 7,
            description: format!(
                "Read nums[{}] = {}. Updated count_map: count[{}] = {}.",
                idx, num, num, count_val
            ),
            visual: VisualState::TopK {
                nums: nums.to_vec(),
                active_nums_idx: Some(idx),
                count_map: count_map.clone(),
                buckets: buckets.clone(),
                active_bucket_idx: None,
                result: result.clone(),
            },
        });
    }

    // Step 2: Populate Buckets
    for (&num, &cnt) in count_map.iter() {
        buckets[cnt].push(num);
        steps.push(Step {
            code_line: 9,
            description: format!(
                "Placed number {} (frequency {}) into bucket freq[{}].",
                num, cnt, cnt
            ),
            visual: VisualState::TopK {
                nums: nums.to_vec(),
                active_nums_idx: None,
                count_map: count_map.clone(),
                buckets: buckets.clone(),
                active_bucket_idx: Some(cnt),
                result: result.clone(),
            },
        });
    }

    // Step 3: Collect Top K (Reverse Scan)
    for i in (1..buckets.len()).rev() {
        steps.push(Step {
            code_line: 12,
            description: format!(
                "Checking bucket index freq[{}] (frequency = {}). Contains: {:?}.",
                i, i, buckets[i]
            ),
            visual: VisualState::TopK {
                nums: nums.to_vec(),
                active_nums_idx: None,
                count_map: count_map.clone(),
                buckets: buckets.clone(),
                active_bucket_idx: Some(i),
                result: result.clone(),
            },
        });

        let current_bucket = buckets[i].clone();
        for &num in &current_bucket {
            result.push(num);
            let is_complete = result.len() == k;

            steps.push(Step {
                code_line: 14,
                description: format!(
                    "Collected {} from bucket freq[{}] into result list. Progress: {}/{}.",
                    num, i, result.len(), k
                ),
                visual: VisualState::TopK {
                    nums: nums.to_vec(),
                    active_nums_idx: None,
                    count_map: count_map.clone(),
                    buckets: buckets.clone(),
                    active_bucket_idx: Some(i),
                    result: result.clone(),
                },
            });

            if is_complete {
                steps.push(Step {
                    code_line: 16,
                    description: format!(
                        "Complete! Gathered top {} frequent elements: {:?}.",
                        k, result
                    ),
                    visual: VisualState::TopK {
                        nums: nums.to_vec(),
                        active_nums_idx: None,
                        count_map: count_map.clone(),
                        buckets: buckets.clone(),
                        active_bucket_idx: None,
                        result: result.clone(),
                    },
                });
                return steps;
            }
        }
    }

    steps
}
