use crate::model::{Step, VisualState};
use std::collections::BTreeMap;

pub fn generate_min_window_substring_steps(s: &str, t: &str) -> Vec<Step> {
    let mut steps = Vec::new();
    let s_chars: Vec<char> = s.chars().collect();

    if t.is_empty() || s.is_empty() {
        steps.push(Step {
            code_line: 3,
            description: "Empty string provided. Minimum window = \"\".".to_string(),
            visual: VisualState::TwoPointers {
                chars: s_chars,
                left: 0,
                right: 0,
                is_valid: Some(true),
                skipped: false,
            },
        });
        return steps;
    }

    let mut t_count: BTreeMap<char, usize> = BTreeMap::new();
    for c in t.chars() {
        *t_count.entry(c).or_insert(0) += 1;
    }

    let mut window: BTreeMap<char, usize> = BTreeMap::new();
    let mut have = 0usize;
    let need = t_count.len();

    let mut res_len = usize::MAX;
    let mut res_bounds = (0, 0);

    steps.push(Step {
        code_line: 3,
        description: format!(
            "Minimum Window Substring: s=\"{}\", t=\"{}\". Need {} unique char counts.",
            s, t, need
        ),
        visual: VisualState::TwoPointers {
            chars: s_chars.clone(),
            left: 0,
            right: 0,
            is_valid: None,
            skipped: false,
        },
    });

    let mut l = 0usize;
    for r in 0..s_chars.len() {
        let c = s_chars[r];
        *window.entry(c).or_insert(0) += 1;

        if let Some(&req) = t_count.get(&c) {
            if window[&c] == req {
                have += 1;
            }
        }

        while have == need {
            let win_len = r - l + 1;
            if win_len < res_len {
                res_len = win_len;
                res_bounds = (l, r);
            }

            steps.push(Step {
                code_line: 6,
                description: format!(
                    "Valid window covering t: [{}..={}] (\"{}\"), len = {}. Shrink left...",
                    l,
                    r,
                    s_chars[l..=r].iter().collect::<String>(),
                    win_len
                ),
                visual: VisualState::TwoPointers {
                    chars: s_chars.clone(),
                    left: l,
                    right: r,
                    is_valid: Some(true),
                    skipped: false,
                },
            });

            let left_char = s_chars[l];
            if let Some(cnt) = window.get_mut(&left_char) {
                *cnt -= 1;
                if let Some(&req) = t_count.get(&left_char) {
                    if *cnt < req {
                        have -= 1;
                    }
                }
            }
            l += 1;
        }
    }

    let result_str = if res_len != usize::MAX {
        s_chars[res_bounds.0..=res_bounds.1]
            .iter()
            .collect::<String>()
    } else {
        String::new()
    };

    steps.push(Step {
        code_line: 9,
        description: if !result_str.is_empty() {
            format!(
                "Minimum Window Substring = \"{}\" (length {}).",
                result_str, res_len
            )
        } else {
            format!("No valid window substring found in s.")
        },
        visual: VisualState::TwoPointers {
            chars: s_chars,
            left: res_bounds.0,
            right: res_bounds.1,
            is_valid: Some(!result_str.is_empty()),
            skipped: false,
        },
    });

    steps
}
