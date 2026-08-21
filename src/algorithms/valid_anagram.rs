use crate::model::{Step, VisualState};

pub(crate) const VALID_ANAGRAM_VISUALIZATION_LIMIT: usize = 128;

pub fn generate_valid_anagram_steps(s: &str, t: &str, approach_id: usize) -> Vec<Step> {
    match approach_id {
        0 => generate_anagram_counter(s, t),
        1 => generate_anagram_sorting(s, t),
        _ => Vec::new(),
    }
}

fn validate_trace_input(s: &str, t: &str) -> Option<Vec<Step>> {
    if !s.bytes().all(|byte| byte.is_ascii_lowercase())
        || !t.bytes().all(|byte| byte.is_ascii_lowercase())
    {
        let message = "Valid Anagram traces accept lowercase English letters (a-z) only. Update both inputs to match the displayed solutions.".to_string();
        return Some(vec![Step {
            code_line: 3,
            description: message.clone(),
            visual: VisualState::TraceUnavailable { message },
        }]);
    }

    if s.len().max(t.len()) > VALID_ANAGRAM_VISUALIZATION_LIMIT {
        return Some(vec![Step::trace_unavailable(
            "Valid Anagram",
            VALID_ANAGRAM_VISUALIZATION_LIMIT,
            "each step stores character frequency vectors",
        )]);
    }

    None
}

fn generate_anagram_counter(s: &str, t: &str) -> Vec<Step> {
    if let Some(unavailable) = validate_trace_input(s, t) {
        return unavailable;
    }

    let mut steps = Vec::new();
    let mut s_counts = [0usize; 26];
    let mut t_counts = [0usize; 26];

    if s.len() != t.len() {
        steps.push(Step {
            code_line: 3,
            description: format!("Length mismatch: len(s)={} != len(t)={}. Strings cannot be anagrams. Return False.", s.len(), t.len()),
            visual: VisualState::ValidAnagram {
                s: s.to_string(),
                t: t.to_string(),
                s_counts,
                t_counts,
                strings_are_sorted: false,
                active_s_idx: None,
                active_t_idx: None,
                is_anagram: Some(false),
            },
        });
        return steps;
    }

    steps.push(Step {
        code_line: 4,
        description: "Initialized frequency counter arrays for s and t.".to_string(),
        visual: VisualState::ValidAnagram {
            s: s.to_string(),
            t: t.to_string(),
            s_counts,
            t_counts,
            strings_are_sorted: false,
            active_s_idx: None,
            active_t_idx: None,
            is_anagram: None,
        },
    });

    for (i, (byte_s, byte_t)) in s.bytes().zip(t.bytes()).enumerate() {
        let ch_s = byte_s as char;
        let ch_t = byte_t as char;
        let idx_s = (byte_s - b'a') as usize;
        s_counts[idx_s] += 1;

        steps.push(Step {
            code_line: 6,
            description: format!("Index i={}: increment count_s for '{}'.", i, ch_s),
            visual: VisualState::ValidAnagram {
                s: s.to_string(),
                t: t.to_string(),
                s_counts,
                t_counts,
                strings_are_sorted: false,
                active_s_idx: Some(i),
                active_t_idx: None,
                is_anagram: None,
            },
        });

        let idx_t = (byte_t - b'a') as usize;
        t_counts[idx_t] += 1;

        steps.push(Step {
            code_line: 7,
            description: format!("Index i={}: increment count_t for '{}'.", i, ch_t),
            visual: VisualState::ValidAnagram {
                s: s.to_string(),
                t: t.to_string(),
                s_counts,
                t_counts,
                strings_are_sorted: false,
                active_s_idx: Some(i),
                active_t_idx: Some(i),
                is_anagram: None,
            },
        });
    }

    let matches = s_counts == t_counts;
    steps.push(Step {
        code_line: 9,
        description: if matches {
            "All character frequency counts match! The strings are valid anagrams. Return True."
                .to_string()
        } else {
            "Character frequency counts do not match. The strings are not anagrams. Return False."
                .to_string()
        },
        visual: VisualState::ValidAnagram {
            s: s.to_string(),
            t: t.to_string(),
            s_counts,
            t_counts,
            strings_are_sorted: false,
            active_s_idx: None,
            active_t_idx: None,
            is_anagram: Some(matches),
        },
    });

    steps
}

fn generate_anagram_sorting(s: &str, t: &str) -> Vec<Step> {
    if let Some(unavailable) = validate_trace_input(s, t) {
        return unavailable;
    }

    let mut steps = Vec::new();
    let empty_counts = [0usize; 26];

    if s.len() != t.len() {
        steps.push(Step {
            code_line: 4,
            description: format!(
                "Length mismatch: len(s)={} != len(t)={}. Return False.",
                s.len(),
                t.len()
            ),
            visual: VisualState::ValidAnagram {
                s: s.to_string(),
                t: t.to_string(),
                s_counts: empty_counts,
                t_counts: empty_counts,
                strings_are_sorted: false,
                active_s_idx: None,
                active_t_idx: None,
                is_anagram: Some(false),
            },
        });
        return steps;
    }

    let mut s_sorted: Vec<char> = s.chars().collect();
    let mut t_sorted: Vec<char> = t.chars().collect();

    steps.push(Step {
        code_line: 5,
        description: format!(
            "Original strings: s=\"{}\", t=\"{}\". Sorting characters...",
            s, t
        ),
        visual: VisualState::ValidAnagram {
            s: s.to_string(),
            t: t.to_string(),
            s_counts: empty_counts,
            t_counts: empty_counts,
            strings_are_sorted: false,
            active_s_idx: None,
            active_t_idx: None,
            is_anagram: None,
        },
    });

    s_sorted.sort_unstable();
    t_sorted.sort_unstable();
    let s_str: String = s_sorted.into_iter().collect();
    let t_str: String = t_sorted.into_iter().collect();

    let is_match = s_str == t_str;

    steps.push(Step {
        code_line: 6,
        description: format!(
            "Sorted strings: s_sorted=\"{}\", t_sorted=\"{}\". Comparing sorted strings...",
            s_str, t_str
        ),
        visual: VisualState::ValidAnagram {
            s: s_str.clone(),
            t: t_str.clone(),
            s_counts: empty_counts,
            t_counts: empty_counts,
            strings_are_sorted: true,
            active_s_idx: None,
            active_t_idx: None,
            is_anagram: None,
        },
    });

    steps.push(Step {
        code_line: 7,
        description: if is_match {
            format!(
                "Sorted strings \"{}\" and \"{}\" are identical! Return True.",
                s_str, t_str
            )
        } else {
            format!(
                "Sorted strings \"{}\" and \"{}\" do not match. Return False.",
                s_str, t_str
            )
        },
        visual: VisualState::ValidAnagram {
            s: s_str,
            t: t_str,
            s_counts: empty_counts,
            t_counts: empty_counts,
            strings_are_sorted: true,
            active_s_idx: None,
            active_t_idx: None,
            is_anagram: Some(is_match),
        },
    });

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    fn result(s: &str, t: &str, approach_id: usize) -> Option<bool> {
        let steps = generate_valid_anagram_steps(s, t, approach_id);
        match &steps.last().expect("trace must have a result").visual {
            VisualState::ValidAnagram { is_anagram, .. } => *is_anagram,
            _ => panic!("expected valid-anagram state"),
        }
    }

    #[test]
    fn counting_and_sorting_agree() {
        for (s, t, expected) in [
            ("anagram", "nagaram", true),
            ("rat", "car", false),
            ("a", "ab", false),
        ] {
            assert_eq!(result(s, t, 0), Some(expected));
            assert_eq!(result(s, t, 1), Some(expected));
        }
    }

    #[test]
    fn sorting_exposes_the_result_only_on_the_return_line() {
        let steps = generate_valid_anagram_steps("anagram", "nagaram", 1);
        let sorted_step = steps
            .iter()
            .find(|step| step.code_line == 6)
            .expect("sorting trace must assign sorted_t");

        match &sorted_step.visual {
            VisualState::ValidAnagram {
                strings_are_sorted,
                is_anagram,
                ..
            } => {
                assert!(*strings_are_sorted);
                assert_eq!(*is_anagram, None);
                let variables = sorted_step.visual.variables(1);
                assert!(variables.iter().any(|(name, _)| *name == "sorted_s"));
                assert!(variables.iter().any(|(name, _)| *name == "sorted_t"));
            }
            _ => panic!("expected valid-anagram state"),
        }

        assert_eq!(steps.last().expect("trace must return").code_line, 7);
        assert_eq!(result("anagram", "nagaram", 1), Some(true));
    }

    #[test]
    fn oversized_inputs_return_an_explicit_trace_status_for_each_approach() {
        let oversized = "a".repeat(VALID_ANAGRAM_VISUALIZATION_LIMIT + 1);

        for approach_id in [0, 1] {
            let steps = generate_valid_anagram_steps(&oversized, &oversized, approach_id);
            assert_eq!(steps.len(), 1);
            assert!(steps[0].description.contains("supports up to"));
            assert!(matches!(
                &steps[0].visual,
                VisualState::TraceUnavailable { .. }
            ));
        }
    }

    #[test]
    fn lowercase_ascii_validation_is_consistent_across_approaches() {
        for (s, t) in [("A", "a"), ("café", "café"), ("abc", "abC")] {
            for approach_id in [0, 1] {
                let steps = generate_valid_anagram_steps(s, t, approach_id);
                assert_eq!(steps.len(), 1);
                assert!(steps[0].description.contains("lowercase English"));
                assert!(matches!(
                    &steps[0].visual,
                    VisualState::TraceUnavailable { .. }
                ));
            }
        }
    }

    #[test]
    fn inputs_at_the_visualization_limit_still_generate_results() {
        let at_limit = "a".repeat(VALID_ANAGRAM_VISUALIZATION_LIMIT);

        assert_eq!(result(&at_limit, &at_limit, 0), Some(true));
        assert_eq!(result(&at_limit, &at_limit, 1), Some(true));
    }
}
