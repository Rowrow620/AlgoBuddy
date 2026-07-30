use crate::model::{Step, VisualState};
use std::collections::VecDeque;

pub fn generate_sliding_window_max_steps(nums: &[i32], k: usize) -> Vec<Step> {
    let mut steps = Vec::new();
    let n = nums.len();

    let char_repr: Vec<char> = nums
        .iter()
        .map(|n| n.to_string().chars().next().unwrap_or(' '))
        .collect();

    if n == 0 || k == 0 {
        steps.push(Step {
            code_line: 3,
            description: "Empty array or k=0. Return [].".to_string(),
            visual: VisualState::TwoPointers {
                chars: vec![],
                left: 0,
                right: 0,
                is_valid: Some(true),
                skipped: false,
            },
        });
        return steps;
    }

    let mut deque: VecDeque<usize> = VecDeque::new(); // monotonic decreasing deque of indices
    let mut res = Vec::new();

    steps.push(Step {
        code_line: 3,
        description: format!(
            "Sliding Window Maximum for nums = {:?}, window size k = {}.",
            nums, k
        ),
        visual: VisualState::TwoPointers {
            chars: char_repr.clone(),
            left: 0,
            right: k.min(n) - 1,
            is_valid: None,
            skipped: false,
        },
    });

    let mut l = 0usize;
    for r in 0..n {
        while let Some(&back_idx) = deque.back() {
            if nums[back_idx] < nums[r] {
                deque.pop_back();
            } else {
                break;
            }
        }
        deque.push_back(r);

        if l > deque[0] {
            deque.pop_front();
        }

        if r + 1 >= k {
            let max_val = nums[deque[0]];
            res.push(max_val);

            steps.push(Step {
                code_line: 6,
                description: format!("Window [{}..={}] ({:?}): Monotonic deque max = nums[{}] = {}. Output so far: {:?}.",
                    l, r, &nums[l..=r], deque[0], max_val, res),
                visual: VisualState::TwoPointers {
                    chars: char_repr.clone(),
                    left: l,
                    right: r,
                    is_valid: None,
                    skipped: false,
                },
            });
            l += 1;
        }
    }

    steps.push(Step {
        code_line: 8,
        description: format!("Sliding Window Maximum complete! Result array = {:?}.", res),
        visual: VisualState::TwoPointers {
            chars: char_repr,
            left: 0,
            right: n - 1,
            is_valid: Some(true),
            skipped: false,
        },
    });

    steps
}
