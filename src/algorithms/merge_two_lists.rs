use crate::model::{MergeListPhase, Step, VisualState};

const MERGE_TWO_LISTS_TRACE_LIMIT: usize = 128;

pub fn generate_merge_two_lists_steps(
    list1: &[i32],
    list2: &[i32],
    approach_id: usize,
) -> Vec<Step> {
    if list1.len().saturating_add(list2.len()) > MERGE_TWO_LISTS_TRACE_LIMIT {
        return vec![Step {
            code_line: 3,
            description: format!(
                "Merge Two Lists visualization supports up to {} total nodes; shorten the inputs to build the detailed trace.",
                MERGE_TWO_LISTS_TRACE_LIMIT
            ),
            visual: VisualState::TraceUnavailable {
                message: format!(
                    "Detailed merge traces accept at most {} total nodes because every step stores both lists and the output prefix.",
                    MERGE_TWO_LISTS_TRACE_LIMIT
                ),
            },
        }];
    }

    match approach_id {
        0 => generate_two_pointer_steps(list1, list2),
        1 => generate_collect_sort_steps(list1, list2),
        _ => Vec::new(),
    }
}

fn generate_two_pointer_steps(list1: &[i32], list2: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();
    let l1_vec = list1.to_vec();
    let l2_vec = list2.to_vec();
    let mut merged = Vec::new();

    let mut i = 0;
    let mut j = 0;

    // 1. Init dummy & tail (code_line 3-4)
    steps.push(Step {
        code_line: 3,
        description: "Initialized dummy head node and tail pointer.".to_string(),
        visual: VisualState::MergeLinkedLists {
            list1: l1_vec.clone(),
            list2: l2_vec.clone(),
            p1_idx: if !l1_vec.is_empty() { Some(0) } else { None },
            p2_idx: if !l2_vec.is_empty() { Some(0) } else { None },
            merged_so_far: merged.clone(),
            phase: MergeListPhase::PointerMerge,
        },
    });

    // 2. Compare while i < len1 && j < len2 (code_line 5)
    while i < list1.len() && j < list2.len() {
        let val1 = list1[i];
        let val2 = list2[j];

        if val1 < val2 {
            merged.push(val1);
            steps.push(Step {
                code_line: 5,
                description: format!("list1.val ({}) < list2.val ({}). Attached node val={} to tail. Advanced list1 pointer.", val1, val2, val1),
                visual: VisualState::MergeLinkedLists {
                    list1: l1_vec.clone(),
                    list2: l2_vec.clone(),
                    p1_idx: Some(i),
                    p2_idx: Some(j),
                    merged_so_far: merged.clone(),
                    phase: MergeListPhase::PointerMerge,
                },
            });
            i += 1;
        } else {
            merged.push(val2);
            steps.push(Step {
                code_line: 6,
                description: format!("list2.val ({}) <= list1.val ({}). Attached node val={} to tail. Advanced list2 pointer.", val2, val1, val2),
                visual: VisualState::MergeLinkedLists {
                    list1: l1_vec.clone(),
                    list2: l2_vec.clone(),
                    p1_idx: Some(i),
                    p2_idx: Some(j),
                    merged_so_far: merged.clone(),
                    phase: MergeListPhase::PointerMerge,
                },
            });
            j += 1;
        }
    }

    // 3. Attach remaining nodes (code_line 11)
    while i < list1.len() {
        merged.push(list1[i]);
        steps.push(Step {
            code_line: 8,
            description: format!(
                "Attached remaining node val={} from list1 to tail.",
                list1[i]
            ),
            visual: VisualState::MergeLinkedLists {
                list1: l1_vec.clone(),
                list2: l2_vec.clone(),
                p1_idx: Some(i),
                p2_idx: None,
                merged_so_far: merged.clone(),
                phase: MergeListPhase::PointerMerge,
            },
        });
        i += 1;
    }

    while j < list2.len() {
        merged.push(list2[j]);
        steps.push(Step {
            code_line: 8,
            description: format!(
                "Attached remaining node val={} from list2 to tail.",
                list2[j]
            ),
            visual: VisualState::MergeLinkedLists {
                list1: l1_vec.clone(),
                list2: l2_vec.clone(),
                p1_idx: None,
                p2_idx: Some(j),
                merged_so_far: merged.clone(),
                phase: MergeListPhase::PointerMerge,
            },
        });
        j += 1;
    }

    steps.push(Step {
        code_line: 9,
        description: format!(
            "Merged lists complete. Returned merged sorted head. Output: {:?}.",
            merged
        ),
        visual: VisualState::MergeLinkedLists {
            list1: l1_vec,
            list2: l2_vec,
            p1_idx: None,
            p2_idx: None,
            merged_so_far: merged,
            phase: MergeListPhase::Complete,
        },
    });

    steps
}

fn generate_collect_sort_steps(list1: &[i32], list2: &[i32]) -> Vec<Step> {
    let l1_vec = list1.to_vec();
    let l2_vec = list2.to_vec();
    let mut values = Vec::with_capacity(list1.len().saturating_add(list2.len()));
    let mut steps = vec![Step {
        code_line: 3,
        description: "Initialize an empty array for values from both lists.".to_string(),
        visual: VisualState::MergeLinkedLists {
            list1: l1_vec.clone(),
            list2: l2_vec.clone(),
            p1_idx: list1.first().map(|_| 0),
            p2_idx: list2.first().map(|_| 0),
            merged_so_far: Vec::new(),
            phase: MergeListPhase::Collecting,
        },
    }];

    for (index, &value) in list1.iter().enumerate() {
        values.push(value);
        steps.push(Step {
            code_line: 4,
            description: format!("Collect list1 node[{}] = {}.", index, value),
            visual: VisualState::MergeLinkedLists {
                list1: l1_vec.clone(),
                list2: l2_vec.clone(),
                p1_idx: Some(index),
                p2_idx: None,
                merged_so_far: values.clone(),
                phase: MergeListPhase::Collecting,
            },
        });
    }
    for (index, &value) in list2.iter().enumerate() {
        values.push(value);
        steps.push(Step {
            code_line: 5,
            description: format!("Collect list2 node[{}] = {}.", index, value),
            visual: VisualState::MergeLinkedLists {
                list1: l1_vec.clone(),
                list2: l2_vec.clone(),
                p1_idx: None,
                p2_idx: Some(index),
                merged_so_far: values.clone(),
                phase: MergeListPhase::Collecting,
            },
        });
    }

    values.sort_unstable();
    steps.push(Step {
        code_line: 6,
        description: format!("Sort all collected values: {:?}.", values),
        visual: VisualState::MergeLinkedLists {
            list1: l1_vec.clone(),
            list2: l2_vec.clone(),
            p1_idx: None,
            p2_idx: None,
            merged_so_far: values.clone(),
            phase: MergeListPhase::SortedValues,
        },
    });

    let mut rebuilt = Vec::with_capacity(values.len());
    for &value in &values {
        rebuilt.push(value);
        steps.push(Step {
            code_line: 8,
            description: format!(
                "Append a new node with value {} to the rebuilt list.",
                value
            ),
            visual: VisualState::MergeLinkedLists {
                list1: l1_vec.clone(),
                list2: l2_vec.clone(),
                p1_idx: None,
                p2_idx: None,
                merged_so_far: rebuilt.clone(),
                phase: MergeListPhase::Rebuilding,
            },
        });
    }

    steps.push(Step {
        code_line: 9,
        description: format!("Rebuilt sorted list complete: {:?}.", rebuilt),
        visual: VisualState::MergeLinkedLists {
            list1: l1_vec,
            list2: l2_vec,
            p1_idx: None,
            p2_idx: None,
            merged_so_far: rebuilt,
            phase: MergeListPhase::Complete,
        },
    });
    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    fn final_values(steps: &[Step]) -> Vec<i32> {
        match &steps.last().expect("trace has a final step").visual {
            VisualState::MergeLinkedLists { merged_so_far, .. } => merged_so_far.clone(),
            visual => panic!("unexpected final visual: {visual:?}"),
        }
    }

    #[test]
    fn collect_sort_matches_two_pointer_merge() {
        let first = [1, 2, 4];
        let second = [1, 3, 5];
        let expected = vec![1, 1, 2, 3, 4, 5];
        assert_eq!(
            final_values(&generate_merge_two_lists_steps(&first, &second, 0)),
            expected
        );
        assert_eq!(
            final_values(&generate_merge_two_lists_steps(&first, &second, 1)),
            expected
        );
    }

    #[test]
    fn merge_trace_rejects_too_many_total_nodes() {
        let steps = generate_merge_two_lists_steps(&vec![0; MERGE_TWO_LISTS_TRACE_LIMIT], &[1], 1);
        assert!(matches!(
            steps[0].visual,
            VisualState::TraceUnavailable { .. }
        ));
    }
}
