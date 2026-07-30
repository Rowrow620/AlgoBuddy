use crate::model::{Step, VisualState};
use std::collections::BTreeMap;

pub fn generate_character_replacement_steps(s: &str, k: usize) -> Vec<Step> {
    let mut steps = Vec::new();
    let chars: Vec<char> = s.chars().collect();
    let mut counts: BTreeMap<char, usize> = BTreeMap::new();

    let mut l = 0usize;
    let mut max_freq = 0usize;
    let mut max_len = 0usize;

    steps.push(Step {
        code_line: 3,
        description: format!(
            "Character Replacement: s = \"{}\", k = {}. Initialize l=0, maxFreq=0.",
            s, k
        ),
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
        *counts.entry(c).or_insert(0) += 1;
        max_freq = max_freq.max(counts[&c]);

        while (r - l + 1) - max_freq > k {
            let left_char = chars[l];
            if let Some(cnt) = counts.get_mut(&left_char) {
                *cnt -= 1;
            }
            steps.push(Step {
                code_line: 6,
                description: format!("Window len ({}) - maxFreq ({}) > k ({}). Shrink window left: remove s[{}]='{}'. l = {}.",
                    r - l + 1, max_freq, k, l, left_char, l + 1),
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

        let curr_len = r - l + 1;
        if curr_len > max_len {
            max_len = curr_len;
        }

        steps.push(Step {
            code_line: 8,
            description: format!(
                "Valid window [{}..={}] (\"{}\"). maxFreq = {}, replacements = {}. maxLen = {}.",
                l,
                r,
                chars[l..=r].iter().collect::<String>(),
                max_freq,
                (r - l + 1) - max_freq,
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
            "Sliding window traversal complete. Maximum repeating substring length = {}.",
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
