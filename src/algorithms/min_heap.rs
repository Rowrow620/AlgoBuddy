use crate::model::{Step, VisualState};
use std::collections::BTreeMap;

pub fn generate_min_heap_steps(nums: &[i32], k: usize) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut count_map: BTreeMap<i32, usize> = BTreeMap::new();
    let mut result: Vec<i32> = Vec::new();
    let buckets: Vec<Vec<i32>> = vec![Vec::new(); nums.len() + 1];

    steps.push(Step {
        code_line: 3,
        description: "Initialized count map and min-heap priority queue.".to_string(),
        visual: VisualState::TopK {
            nums: nums.to_vec(),
            active_nums_idx: None,
            count_map: count_map.clone(),
            buckets: buckets.clone(),
            active_bucket_idx: None,
            result: result.clone(),
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
                result: result.clone(),
            },
        });
    }

    let mut heap: Vec<(usize, i32)> = Vec::new();

    for (&num, &cnt) in count_map.iter() {
        heap.push((cnt, num));
        heap.sort_by_key(|&(c, _)| c);

        if heap.len() > k {
            let popped = heap.remove(0);
            steps.push(Step {
                code_line: 10,
                description: format!(
                    "Pushed {} (count {}) to Min-Heap. Exceeded k={}, popped min element {}.",
                    num, cnt, k, popped.1
                ),
                visual: VisualState::TopK {
                    nums: nums.to_vec(),
                    active_nums_idx: None,
                    count_map: count_map.clone(),
                    buckets: buckets.clone(),
                    active_bucket_idx: None,
                    result: heap.iter().map(|&(_, n)| n).collect(),
                },
            });
        } else {
            steps.push(Step {
                code_line: 8,
                description: format!(
                    "Pushed {} (count {}) into Min-Heap (heap size {}/{}).",
                    num,
                    cnt,
                    heap.len(),
                    k
                ),
                visual: VisualState::TopK {
                    nums: nums.to_vec(),
                    active_nums_idx: None,
                    count_map: count_map.clone(),
                    buckets: buckets.clone(),
                    active_bucket_idx: None,
                    result: heap.iter().map(|&(_, n)| n).collect(),
                },
            });
        }
    }

    result = heap.iter().map(|&(_, n)| n).collect();

    steps.push(Step {
        code_line: 14,
        description: format!(
            "Min-Heap completed! Top {} elements in heap: {:?}.",
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

    steps
}
