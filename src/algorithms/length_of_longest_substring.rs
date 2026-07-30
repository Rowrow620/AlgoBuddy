use crate::model::{Step, VisualState};
use std::collections::BTreeSet;

pub fn generate_longest_substring_steps(s: &str) -> Vec<Step> {
    let mut steps = Vec::new();
    let chars: Vec<char> = s.chars().collect();
    let mut char_set = BTreeSet::new();

    let mut l = 0usize;
    let mut max_len = 0usize;

    steps.push(Step {
        code_line: 3,
        description: format!("Longest Substring Without Repeating Characters for s = \"{}\". Initialize l=0, maxLen=0.", s),
        visual: VisualState::TwoPointers {
            chars: chars.clone(),
            left: 0,
            right: 0,
            is_valid: None,
            skipped: false,
        },
    });

    for r in 0..chars.len() {
        let c = chars[r];

        while char_set.contains(&c) {
            char_set.remove(&chars[l]);
            steps.push(Step {
                code_line: 6,
                description: format!("Duplicate char '{}' detected at r={}. Remove s[{}]='{}' from window. Increment l to {}.", c, r, l, chars[l], l + 1),
                visual: VisualState::TwoPointers {
                    chars: chars.clone(),
                    left: l + 1,
                    right: r,
                    is_valid: None,
                    skipped: true,
                },
            });
            l += 1;
        }

        char_set.insert(c);
        let curr_len = r - l + 1;
        if curr_len > max_len {
            max_len = curr_len;
        }

        steps.push(Step {
            code_line: 8,
            description: format!(
                "Added '{}' at r={}. Valid window [{}..={}] (\"{}\"), len = {}. maxLen = {}.",
                c,
                r,
                l,
                r,
                chars[l..=r].iter().collect::<String>(),
                curr_len,
                max_len
            ),
            visual: VisualState::TwoPointers {
                chars: chars.clone(),
                left: l,
                right: r,
                is_valid: None,
                skipped: false,
            },
        });
    }

    steps.push(Step {
        code_line: 9,
        description: format!(
            "Sliding window traversal complete. Maximum substring length = {}.",
            max_len
        ),
        visual: VisualState::TwoPointers {
            chars: chars.clone(),
            left: l,
            right: chars.len().saturating_sub(1),
            is_valid: Some(true),
            skipped: false,
        },
    });

    steps
}
