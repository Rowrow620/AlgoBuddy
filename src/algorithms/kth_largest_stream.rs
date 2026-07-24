use crate::model::{Step, VisualState};

pub fn generate_kth_largest_stream_steps(k: usize, nums: &[i32], val: i32) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut heap = nums.to_vec();
    heap.sort();
    if heap.len() > k {
        heap = heap[heap.len() - k..].to_vec();
    }

    steps.push(Step {
        code_line: 3,
        description: format!("Initialized Min-Heap of size k={}: {:?}", k, heap),
        visual: VisualState::ContainsDuplicate {
            nums: heap.clone(),
            active_idx: None,
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });

    heap.push(val);
    heap.sort();
    if heap.len() > k {
        heap.remove(0);
    }

    steps.push(Step {
        code_line: 6,
        description: format!("Added stream element {}. Updated min-heap of size {}: {:?}.", val, k, heap),
        visual: VisualState::ContainsDuplicate {
            nums: heap.clone(),
            active_idx: Some(0),
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });

    let kth = heap[0];
    steps.push(Step {
        code_line: 8,
        description: format!("{}-th largest element in stream = {}.", k, kth),
        visual: VisualState::ContainsDuplicate {
            nums: heap,
            active_idx: Some(0),
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: Some(kth),
            has_duplicate: Some(true),
        },
    });

    steps
}
