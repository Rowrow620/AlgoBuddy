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

        let matched_left = l;
        let matched_right = r;
        l += 1;
        r = r.saturating_sub(1);

        steps.push(Step {
            code_line: 11,
            description: format!(
                "Match: s[{}]='{}' == s[{}]='{}'. Moved pointers inward to l={}, r={}.",
                matched_left, chars[matched_left], matched_right, chars[matched_right], l, r
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

#[cfg(test)]
mod tests {
    use super::*;

    fn movement_positions(steps: &[Step]) -> Vec<(usize, usize)> {
        steps
            .iter()
            .filter(|step| step.code_line == 11)
            .filter_map(|step| match &step.visual {
                VisualState::TwoPointers { left, right, .. } => Some((*left, *right)),
                _ => None,
            })
            .collect()
    }

    fn final_result(steps: &[Step]) -> Option<bool> {
        match &steps
            .last()
            .expect("palindrome trace must not be empty")
            .visual
        {
            VisualState::TwoPointers { is_valid, .. } => *is_valid,
            _ => None,
        }
    }

    #[test]
    fn two_pointer_trace_snapshots_pointer_updates_and_results() {
        let racecar = generate_palindrome_two_pointers("racecar");
        assert_eq!(movement_positions(&racecar), vec![(1, 5), (2, 4), (3, 3)]);
        assert_eq!(final_result(&racecar), Some(true));

        let spaced_palindrome = generate_palindrome_two_pointers("race car");
        assert_eq!(
            movement_positions(&spaced_palindrome),
            vec![(1, 6), (2, 5), (3, 4)]
        );
        assert!(spaced_palindrome.iter().any(|step| {
            step.code_line == 8
                && matches!(
                    &step.visual,
                    VisualState::TwoPointers {
                        left: 3,
                        right: 3,
                        ..
                    }
                )
        }));
        assert_eq!(final_result(&spaced_palindrome), Some(true));

        let mismatch = generate_palindrome_two_pointers("race a car");
        assert_eq!(movement_positions(&mismatch), vec![(1, 8), (2, 7), (3, 6)]);
        assert!(mismatch.last().is_some_and(|step| step.code_line == 10));
        assert_eq!(final_result(&mismatch), Some(false));

        assert_eq!(
            crate::model::Problem::ValidPalindrome.formula(),
            Some("all previously compared alphanumeric pairs matched")
        );
    }
}
