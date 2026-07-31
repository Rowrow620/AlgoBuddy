use crate::model::{Step, VisualState};

pub fn generate_valid_palindrome_steps(s: &str, approach_id: usize) -> Vec<Step> {
    if approach_id == 1 {
        generate_palindrome_reverse(s)
    } else {
        generate_palindrome_two_pointers(s)
    }
}

fn generate_palindrome_two_pointers(s: &str) -> Vec<Step> {
    let mut steps = Vec::new();
    let chars: Vec<char> = s.chars().collect();

    if chars.is_empty() {
        steps.push(Step {
            code_line: 12,
            description: "String is empty, returning True.".to_string(),
            visual: VisualState::TwoPointers {
                chars: chars.clone(),
                left: 0,
                right: 0,
                is_valid: Some(true),
                skipped: false,
            },
        });
        return steps;
    }

    let mut l = 0;
    let mut r = chars.len() - 1;

    steps.push(Step {
        code_line: 3,
        description: format!(
            "Initialized left pointer l=0 ('{}') and right pointer r={} ('{}').",
            chars[l], r, chars[r]
        ),
        visual: VisualState::TwoPointers {
            chars: chars.clone(),
            left: l,
            right: r,
            is_valid: None,
            skipped: false,
        },
    });

    while l < r {
        let mut skipped_left = false;
        while l < r && !chars[l].is_alphanumeric() {
            skipped_left = true;
            l += 1;
            steps.push(Step {
                code_line: 6,
                description: format!(
                    "Skipped non-alphanumeric character at left. Moved l to index {}.",
                    l
                ),
                visual: VisualState::TwoPointers {
                    chars: chars.clone(),
                    left: l,
                    right: r,
                    is_valid: None,
                    skipped: true,
                },
            });
        }

        let mut skipped_right = false;
        while r > l && !chars[r].is_alphanumeric() {
            skipped_right = true;
            r -= 1;
            steps.push(Step {
                code_line: 8,
                description: format!(
                    "Skipped non-alphanumeric character at right. Moved r to index {}.",
                    r
                ),
                visual: VisualState::TwoPointers {
                    chars: chars.clone(),
                    left: l,
                    right: r,
                    is_valid: None,
                    skipped: true,
                },
            });
        }

        if l >= r {
            break;
        }

        let char_l = chars[l].to_ascii_lowercase();
        let char_r = chars[r].to_ascii_lowercase();

        if char_l != char_r {
            steps.push(Step {
                code_line: 10,
                description: format!(
                    "Mismatch detected: s[{}]='{}' != s[{}]='{}'. Return False.",
                    l, chars[l], r, chars[r]
                ),
                visual: VisualState::TwoPointers {
                    chars: chars.clone(),
                    left: l,
                    right: r,
                    is_valid: Some(false),
                    skipped: skipped_left || skipped_right,
                },
            });
            return steps;
        }

        steps.push(Step {
            code_line: 9,
            description: format!(
                "Match: s[{}]='{}' == s[{}]='{}'. Moving pointers inward.",
                l, chars[l], r, chars[r]
            ),
            visual: VisualState::TwoPointers {
                chars: chars.clone(),
                left: l,
                right: r,
                is_valid: None,
                skipped: false,
            },
        });

        l += 1;
        r = r.saturating_sub(1);
    }

    steps.push(Step {
        code_line: 12,
        description: "Pointers met/crossed. All alphanumeric characters matched! Return True."
            .to_string(),
        visual: VisualState::TwoPointers {
            chars: chars.clone(),
            left: l,
            right: r,
            is_valid: Some(true),
            skipped: false,
        },
    });

    steps
}

fn generate_palindrome_reverse(s: &str) -> Vec<Step> {
    let mut steps = Vec::new();
    let original_chars: Vec<char> = s.chars().collect();

    steps.push(Step {
        code_line: 3,
        description: "Initialized newStr = \"\" for collecting lowercase alphanumeric characters."
            .to_string(),
        visual: VisualState::TwoPointers {
            chars: original_chars.clone(),
            left: 0,
            right: 0,
            is_valid: None,
            skipped: false,
        },
    });

    let filtered_chars: Vec<char> = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    steps.push(Step {
        code_line: 6,
        description: format!(
            "Filtered non-alphanumeric characters. Filtered newStr: \"{}\".",
            filtered_chars.iter().collect::<String>()
        ),
        visual: VisualState::TwoPointers {
            chars: filtered_chars.clone(),
            left: 0,
            right: filtered_chars.len().saturating_sub(1),
            is_valid: None,
            skipped: false,
        },
    });

    let reversed_chars: Vec<char> = filtered_chars.iter().copied().rev().collect();
    let is_equal = filtered_chars == reversed_chars;

    steps.push(Step {
        code_line: 7,
        description: format!(
            "Comparing filtered string \"{}\" to reversed string \"{}\". Match = {}.",
            filtered_chars.iter().collect::<String>(),
            reversed_chars.iter().collect::<String>(),
            is_equal
        ),
        visual: VisualState::TwoPointers {
            chars: filtered_chars,
            left: 0,
            right: reversed_chars.len().saturating_sub(1),
            is_valid: Some(is_equal),
            skipped: false,
        },
    });

    steps
}
