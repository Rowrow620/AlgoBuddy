use std::collections::BTreeSet;
use crate::model::{Step, VisualState};

pub fn generate_longest_consecutive_steps(nums: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();
    let num_vec = nums.to_vec();
    let num_set: BTreeSet<i32> = nums.iter().copied().collect();

    // 1. Init numSet (code_line 3)
    steps.push(Step {
        code_line: 3,
        description: format!("Created numSet with {} unique elements from input array.", num_set.len()),
        visual: VisualState::LongestConsecutive {
            nums: num_vec.clone(),
            num_set: num_set.clone(),
            current_num: None,
            current_seq: Vec::new(),
            max_length: 0,
            is_seq_start: None,
        },
    });

    let mut longest = 0;

    for &n in &num_set {
        let is_start = !num_set.contains(&(n - 1));

        steps.push(Step {
            code_line: 7,
            description: if is_start {
                format!("Checking n={}: (n - 1) = {} is NOT in numSet. Element {} IS the start of a sequence!", n, n - 1, n)
            } else {
                format!("Checking n={}: (n - 1) = {} IS in numSet. Skip {} (not sequence start).", n, n - 1, n)
            },
            visual: VisualState::LongestConsecutive {
                nums: num_vec.clone(),
                num_set: num_set.clone(),
                current_num: Some(n),
                current_seq: if is_start { vec![n] } else { Vec::new() },
                max_length: longest,
                is_seq_start: Some(is_start),
            },
        });

        if is_start {
            let mut curr_seq = vec![n];
            let mut length = 1;

            while num_set.contains(&(n + length)) {
                let next_val = n + length;
                curr_seq.push(next_val);
                length += 1;

                steps.push(Step {
                    code_line: 9,
                    description: format!("Expanded sequence from start n={}: found next consecutive element {} in numSet. Current streak = {}.", n, next_val, length),
                    visual: VisualState::LongestConsecutive {
                        nums: num_vec.clone(),
                        num_set: num_set.clone(),
                        current_num: Some(n),
                        current_seq: curr_seq.clone(),
                        max_length: longest,
                        is_seq_start: Some(true),
                    },
                });
            }

            let prev_longest = longest;
            longest = longest.max(length as usize);

            steps.push(Step {
                code_line: 11,
                description: format!("Sequence ending at {}. Total streak length = {}. Updated max longest: {} -> {}.", n + (length - 1), length, prev_longest, longest),
                visual: VisualState::LongestConsecutive {
                    nums: num_vec.clone(),
                    num_set: num_set.clone(),
                    current_num: Some(n),
                    current_seq: curr_seq,
                    max_length: longest,
                    is_seq_start: Some(true),
                },
            });
        }
    }

    steps.push(Step {
        code_line: 12,
        description: format!("Completed inspection of all unique set elements. Longest consecutive sequence length = {}.", longest),
        visual: VisualState::LongestConsecutive {
            nums: num_vec,
            num_set,
            current_num: None,
            current_seq: Vec::new(),
            max_length: longest,
            is_seq_start: None,
        },
    });

    steps
}
