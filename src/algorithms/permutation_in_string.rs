use crate::model::{Step, VisualState};

pub fn generate_permutation_in_string_steps(s1: &str, s2: &str) -> Vec<Step> {
    let mut steps = Vec::new();
    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();

    if s1_chars.len() > s2_chars.len() {
        steps.push(Step {
            code_line: 3,
            description: format!(
                "s1 (\"{}\") is longer than s2 (\"{}\"). Permutation impossible. Return False.",
                s1, s2
            ),
            visual: VisualState::TwoPointers {
                chars: s2_chars,
                left: 0,
                right: 0,
                is_valid: Some(false),
                skipped: false,
            },
        });
        return steps;
    }

    let mut s1_count = [0usize; 26];
    let mut s2_count = [0usize; 26];

    for i in 0..s1_chars.len() {
        s1_count[(s1_chars[i] as u8 - b'a') as usize] += 1;
        s2_count[(s2_chars[i] as u8 - b'a') as usize] += 1;
    }

    steps.push(Step {
        code_line: 3,
        description: format!(
            "Permutation in String: s1=\"{}\", s2=\"{}\". Initial window len = {}.",
            s1,
            s2,
            s1_chars.len()
        ),
        visual: VisualState::TwoPointers {
            chars: s2_chars.clone(),
            left: 0,
            right: s1_chars.len() - 1,
            is_valid: None,
            skipped: false,
        },
    });

    let mut matches = 0usize;
    for i in 0..26 {
        if s1_count[i] == s2_count[i] {
            matches += 1;
        }
    }

    let mut l = 0usize;
    for r in s1_chars.len()..s2_chars.len() {
        if matches == 26 {
            steps.push(Step {
                code_line: 6,
                description: format!("Match found! Window [{}..={}] (\"{}\") is a permutation of s1 (\"{}\"). Return True.",
                    l, r - 1, s2_chars[l..r].iter().collect::<String>(), s1),
                visual: VisualState::TwoPointers {
                    chars: s2_chars.clone(),
                    left: l,
                    right: r - 1,
                    is_valid: Some(true),
                    skipped: false,
                },
            });
            return steps;
        }

        let r_idx = (s2_chars[r] as u8 - b'a') as usize;
        s2_count[r_idx] += 1;
        if s1_count[r_idx] == s2_count[r_idx] {
            matches += 1;
        } else if s1_count[r_idx] + 1 == s2_count[r_idx] {
            matches -= 1;
        }

        let l_idx = (s2_chars[l] as u8 - b'a') as usize;
        s2_count[l_idx] -= 1;
        if s1_count[l_idx] == s2_count[l_idx] {
            matches += 1;
        } else if s1_count[l_idx] == s2_count[l_idx] + 1 {
            matches -= 1;
        }

        l += 1;

        steps.push(Step {
            code_line: 8,
            description: format!(
                "Slide window to [{}..={}]: \"{}\". Matches = {} / 26.",
                l,
                r,
                s2_chars[l..=r].iter().collect::<String>(),
                matches
            ),
            visual: VisualState::TwoPointers {
                chars: s2_chars.clone(),
                left: l,
                right: r,
                is_valid: None,
                skipped: false,
            },
        });
    }

    let is_match = matches == 26;
    steps.push(Step {
        code_line: 9,
        description: if is_match {
            format!("Permutation of s1 found at end of s2! Return True.")
        } else {
            format!("No permutation of s1 found in s2. Return False.")
        },
        visual: VisualState::TwoPointers {
            chars: s2_chars,
            left: l,
            right: s1_chars.len().saturating_sub(1),
            is_valid: Some(is_match),
            skipped: false,
        },
    });

    steps
}
