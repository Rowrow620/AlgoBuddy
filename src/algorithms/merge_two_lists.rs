use crate::model::{Step, VisualState};

pub fn generate_merge_two_lists_steps(list1: &[i32], list2: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();
    let l1_vec = list1.to_vec();
    let l2_vec = list2.to_vec();
    let mut merged = Vec::new();

    let mut i = 0;
    let mut j = 0;

    // 1. Init dummy & tail (code_line 3-4)
    steps.push(Step {
        code_line: 4,
        description: "Initialized dummy head node and tail pointer.".to_string(),
        visual: VisualState::MergeLinkedLists {
            list1: l1_vec.clone(),
            list2: l2_vec.clone(),
            p1_idx: if !l1_vec.is_empty() { Some(0) } else { None },
            p2_idx: if !l2_vec.is_empty() { Some(0) } else { None },
            merged_so_far: merged.clone(),
        },
    });

    // 2. Compare while i < len1 && j < len2 (code_line 5)
    while i < list1.len() && j < list2.len() {
        let val1 = list1[i];
        let val2 = list2[j];

        if val1 < val2 {
            merged.push(val1);
            steps.push(Step {
                code_line: 7,
                description: format!("list1.val ({}) < list2.val ({}). Attached node val={} to tail. Advanced list1 pointer.", val1, val2, val1),
                visual: VisualState::MergeLinkedLists {
                    list1: l1_vec.clone(),
                    list2: l2_vec.clone(),
                    p1_idx: Some(i),
                    p2_idx: Some(j),
                    merged_so_far: merged.clone(),
                },
            });
            i += 1;
        } else {
            merged.push(val2);
            steps.push(Step {
                code_line: 9,
                description: format!("list2.val ({}) <= list1.val ({}). Attached node val={} to tail. Advanced list2 pointer.", val2, val1, val2),
                visual: VisualState::MergeLinkedLists {
                    list1: l1_vec.clone(),
                    list2: l2_vec.clone(),
                    p1_idx: Some(i),
                    p2_idx: Some(j),
                    merged_so_far: merged.clone(),
                },
            });
            j += 1;
        }
    }

    // 3. Attach remaining nodes (code_line 11)
    while i < list1.len() {
        merged.push(list1[i]);
        steps.push(Step {
            code_line: 11,
            description: format!("Attached remaining node val={} from list1 to tail.", list1[i]),
            visual: VisualState::MergeLinkedLists {
                list1: l1_vec.clone(),
                list2: l2_vec.clone(),
                p1_idx: Some(i),
                p2_idx: None,
                merged_so_far: merged.clone(),
            },
        });
        i += 1;
    }

    while j < list2.len() {
        merged.push(list2[j]);
        steps.push(Step {
            code_line: 11,
            description: format!("Attached remaining node val={} from list2 to tail.", list2[j]),
            visual: VisualState::MergeLinkedLists {
                list1: l1_vec.clone(),
                list2: l2_vec.clone(),
                p1_idx: None,
                p2_idx: Some(j),
                merged_so_far: merged.clone(),
            },
        });
        j += 1;
    }

    steps.push(Step {
        code_line: 12,
        description: format!("Merged lists complete. Returned merged sorted head. Output: {:?}.", merged),
        visual: VisualState::MergeLinkedLists {
            list1: l1_vec,
            list2: l2_vec,
            p1_idx: None,
            p2_idx: None,
            merged_so_far: merged,
        },
    });

    steps
}
