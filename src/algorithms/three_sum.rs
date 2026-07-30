use crate::model::{Step, VisualState};

pub fn generate_three_sum_steps(nums: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut sorted = nums.to_vec();
    sorted.sort();

    let char_repr: Vec<char> = sorted
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",")
        .chars()
        .collect();

    steps.push(Step {
        code_line: 3,
        description: format!("3Sum: Sort the array first. sorted = {:?}", sorted),
        visual: VisualState::TwoPointers {
            chars: char_repr.clone(),
            left: 0,
            right: char_repr.len().saturating_sub(1),
            is_valid: None,
            skipped: false,
        },
    });

    let n = sorted.len();
    let mut results: Vec<(i32, i32, i32)> = Vec::new();

    for i in 0..n {
        if i > 0 && sorted[i] == sorted[i - 1] {
            steps.push(Step {
                code_line: 5,
                description: format!(
                    "i={}: nums[{}]={} == nums[{}]={}. Skip duplicate anchor.",
                    i,
                    i,
                    sorted[i],
                    i - 1,
                    sorted[i - 1]
                ),
                visual: VisualState::TwoPointers {
                    chars: char_repr.clone(),
                    left: i,
                    right: n.saturating_sub(1),
                    is_valid: None,
                    skipped: true,
                },
            });
            continue;
        }

        let mut l = i + 1;
        let mut r = n - 1;

        steps.push(Step {
            code_line: 6,
            description: format!(
                "i={}: anchor = nums[{}] = {}. Two pointers: l={}, r={}.",
                i, i, sorted[i], l, r
            ),
            visual: VisualState::TwoPointers {
                chars: char_repr.clone(),
                left: l,
                right: r,
                is_valid: None,
                skipped: false,
            },
        });

        while l < r {
            let three_sum = sorted[i] + sorted[l] + sorted[r];

            if three_sum == 0 {
                results.push((sorted[i], sorted[l], sorted[r]));
                steps.push(Step {
                    code_line: 8,
                    description: format!(
                        "Found triplet! [{}, {}, {}] = 0. Move both pointers.",
                        sorted[i], sorted[l], sorted[r]
                    ),
                    visual: VisualState::TwoPointers {
                        chars: char_repr.clone(),
                        left: l,
                        right: r,
                        is_valid: Some(true),
                        skipped: false,
                    },
                });
                l += 1;
                while l < r && sorted[l] == sorted[l - 1] {
                    l += 1;
                }
            } else if three_sum < 0 {
                steps.push(Step {
                    code_line: 9,
                    description: format!(
                        "Sum = {} + {} + {} = {} < 0. Increment l to {}.",
                        sorted[i],
                        sorted[l],
                        sorted[r],
                        three_sum,
                        l + 1
                    ),
                    visual: VisualState::TwoPointers {
                        chars: char_repr.clone(),
                        left: l + 1,
                        right: r,
                        is_valid: None,
                        skipped: false,
                    },
                });
                l += 1;
            } else {
                steps.push(Step {
                    code_line: 10,
                    description: format!(
                        "Sum = {} + {} + {} = {} > 0. Decrement r to {}.",
                        sorted[i],
                        sorted[l],
                        sorted[r],
                        three_sum,
                        r - 1
                    ),
                    visual: VisualState::TwoPointers {
                        chars: char_repr.clone(),
                        left: l,
                        right: r - 1,
                        is_valid: None,
                        skipped: false,
                    },
                });
                r -= 1;
            }

            if steps.len() > 150 {
                break;
            }
        }

        if steps.len() > 150 {
            break;
        }
    }

    let result_str = results
        .iter()
        .map(|(a, b, c)| format!("[{}, {}, {}]", a, b, c))
        .collect::<Vec<_>>()
        .join(", ");

    steps.push(Step {
        code_line: 11,
        description: format!(
            "All triplets found: [{}]. Total = {}.",
            result_str,
            results.len()
        ),
        visual: VisualState::TwoPointers {
            chars: char_repr,
            left: 0,
            right: 0,
            is_valid: Some(!results.is_empty()),
            skipped: false,
        },
    });

    steps
}
