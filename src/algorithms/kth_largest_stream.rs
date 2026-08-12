use crate::model::{Step, VisualState};

const KTH_LARGEST_STREAM_TRACE_LIMIT: usize = 128;

pub fn generate_kth_largest_stream_steps(
    k: usize,
    nums: &[i32],
    val: i32,
    approach_id: usize,
) -> Vec<Step> {
    if nums.len().saturating_add(1) > KTH_LARGEST_STREAM_TRACE_LIMIT {
        return vec![Step {
            code_line: 3,
            description: format!(
                "Kth Largest stream visualization supports up to {} values; shorten the stream to build the detailed trace.",
                KTH_LARGEST_STREAM_TRACE_LIMIT
            ),
            visual: VisualState::TraceUnavailable {
                message: format!(
                    "Detailed stream traces accept at most {} values because each update stores the full collection state.",
                    KTH_LARGEST_STREAM_TRACE_LIMIT
                ),
            },
        }];
    }
    match approach_id {
        0 => generate_min_heap_steps(k, nums, val),
        1 => generate_append_sort_steps(k, nums, val),
        _ => Vec::new(),
    }
}

fn generate_min_heap_steps(k: usize, nums: &[i32], val: i32) -> Vec<Step> {
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
        description: format!(
            "Added stream element {}. Updated min-heap of size {}: {:?}.",
            val, k, heap
        ),
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

fn generate_append_sort_steps(k: usize, nums: &[i32], val: i32) -> Vec<Step> {
    let mut values = nums.to_vec();
    let mut steps = vec![Step {
        code_line: 3,
        description: format!(
            "Store the complete initial stream: {:?} (k = {}).",
            values, k
        ),
        visual: VisualState::HeapVisual {
            heap_elements: values.clone(),
            active_idx: None,
            swapped_pair: None,
            heap_type_label: "Unsorted full stream".to_string(),
        },
    }];

    values.push(val);
    steps.push(Step {
        code_line: 5,
        description: format!("Append the new stream value {}.", val),
        visual: VisualState::HeapVisual {
            heap_elements: values.clone(),
            active_idx: values.len().checked_sub(1),
            swapped_pair: None,
            heap_type_label: "Full stream after append".to_string(),
        },
    });

    values.sort_unstable_by(|left, right| right.cmp(left));
    steps.push(Step {
        code_line: 6,
        description: format!("Sort the entire stream in descending order: {:?}.", values),
        visual: VisualState::HeapVisual {
            heap_elements: values.clone(),
            active_idx: None,
            swapped_pair: None,
            heap_type_label: "Sorted full stream".to_string(),
        },
    });

    let result_index = k.saturating_sub(1).min(values.len().saturating_sub(1));
    let kth = values.get(result_index).copied().unwrap_or(0);
    steps.push(Step {
        code_line: 7,
        description: format!(
            "Index {} contains the {}-th largest value, {}.",
            result_index, k, kth
        ),
        visual: VisualState::HeapVisual {
            heap_elements: values,
            active_idx: Some(result_index),
            swapped_pair: None,
            heap_type_label: format!("{}-th largest = {}", k, kth),
        },
    });
    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    fn final_value(steps: &[Step]) -> i32 {
        match &steps.last().expect("trace has a final step").visual {
            VisualState::HeapVisual {
                heap_elements,
                active_idx: Some(index),
                ..
            } => heap_elements[*index],
            VisualState::ContainsDuplicate {
                duplicate_val: Some(value),
                ..
            } => *value,
            visual => panic!("unexpected final visual: {visual:?}"),
        }
    }

    #[test]
    fn append_sort_finds_the_same_kth_largest_value() {
        let nums = [4, 5, 8, 2];
        assert_eq!(
            final_value(&generate_kth_largest_stream_steps(3, &nums, 3, 0)),
            4
        );
        assert_eq!(
            final_value(&generate_kth_largest_stream_steps(3, &nums, 3, 1)),
            4
        );
    }

    #[test]
    fn stream_trace_has_a_snapshot_limit() {
        let steps =
            generate_kth_largest_stream_steps(1, &vec![0; KTH_LARGEST_STREAM_TRACE_LIMIT], 1, 1);
        assert!(matches!(
            steps[0].visual,
            VisualState::TraceUnavailable { .. }
        ));
    }
}
