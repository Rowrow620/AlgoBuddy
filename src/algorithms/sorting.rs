use crate::model::{Step, VisualState};
use std::collections::BTreeMap;

pub fn generate_sorting_steps(nums: &[i32], k: usize) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut count_map: BTreeMap<i32, usize> = BTreeMap::new();
    let buckets: Vec<Vec<i32>> = vec![Vec::new(); nums.len() + 1];

    steps.push(Step {
        code_line: 3,
        description: "Initialized frequency count map.".to_string(),
        visual: VisualState::TopK {
            nums: nums.to_vec(),
            active_nums_idx: None,
            count_map: count_map.clone(),
            buckets: buckets.clone(),
            active_bucket_idx: None,
            result: vec![],
        },
    });

    for (idx, &num) in nums.iter().enumerate() {
        let count_val = {
            let c = count_map.entry(num).or_insert(0);
            *c += 1;
            *c
        };

        steps.push(Step {
            code_line: 6,
            description: format!("Counted num {} -> count = {}.", num, count_val),
            visual: VisualState::TopK {
                nums: nums.to_vec(),
                active_nums_idx: Some(idx),
                count_map: count_map.clone(),
                buckets: buckets.clone(),
                active_bucket_idx: None,
                result: vec![],
            },
        });
    }

    let mut pairs: Vec<(usize, i32)> = count_map.iter().map(|(&n, &c)| (c, n)).collect();
    steps.push(Step {
        code_line: 8,
        description: format!("Converted map into pair array arr = {:?}.", pairs),
        visual: VisualState::TopK {
            nums: nums.to_vec(),
            active_nums_idx: None,
            count_map: count_map.clone(),
            buckets: buckets.clone(),
            active_bucket_idx: None,
            result: vec![],
        },
    });

    pairs.sort_by_key(|&(c, _)| c);
    steps.push(Step {
        code_line: 10,
        description: format!("Sorted arr by frequency ascending: {:?}.", pairs),
        visual: VisualState::TopK {
            nums: nums.to_vec(),
            active_nums_idx: None,
            count_map: count_map.clone(),
            buckets: buckets.clone(),
            active_bucket_idx: None,
            result: vec![],
        },
    });

    let mut result = Vec::new();
    while result.len() < k && !pairs.is_empty() {
        if let Some((_, num)) = pairs.pop() {
            result.push(num);
            steps.push(Step {
                code_line: 14,
                description: format!(
                    "Popped highest element {} from sorted array end. Progress {}/{}.",
                    num,
                    result.len(),
                    k
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
        }
    }

    steps.push(Step {
        code_line: 16,
        description: format!("Sorting completed! Top {} elements: {:?}.", k, result),
        visual: VisualState::TopK {
            nums: nums.to_vec(),
            active_nums_idx: None,
            count_map: count_map.clone(),
            buckets: buckets.clone(),
            active_bucket_idx: None,
            result: result.clone(),
        },
    });

    steps
}
