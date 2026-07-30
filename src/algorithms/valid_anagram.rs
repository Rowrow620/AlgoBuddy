use crate::model::{Step, VisualState};

pub fn generate_valid_anagram_steps(s: &str, t: &str, approach_id: usize) -> Vec<Step> {
    if approach_id == 1 {
        generate_anagram_sorting(s, t)
    } else {
        generate_anagram_counter(s, t)
    }
}

fn generate_anagram_counter(s: &str, t: &str) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut s_counts = [0usize; 26];
    let mut t_counts = [0usize; 26];

    if s.len() != t.len() {
        steps.push(Step {
            code_line: 4,
            description: format!("Length mismatch: len(s)={} != len(t)={}. Strings cannot be anagrams. Return False.", s.len(), t.len()),
            visual: VisualState::ValidAnagram {
                s: s.to_string(),
                t: t.to_string(),
                s_counts,
                t_counts,
                active_s_idx: None,
                active_t_idx: None,
                is_anagram: Some(false),
            },
        });
        return steps;
    }

    steps.push(Step {
        code_line: 5,
        description: "Initialized frequency counter arrays for s and t.".to_string(),
        visual: VisualState::ValidAnagram {
            s: s.to_string(),
            t: t.to_string(),
            s_counts,
            t_counts,
            active_s_idx: None,
            active_t_idx: None,
            is_anagram: None,
        },
    });

    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();

    for i in 0..s.len() {
        let ch_s = s_chars[i];
        let ch_t = t_chars[i];

        if ch_s.is_ascii_lowercase() {
            let idx_s = (ch_s as u8 - b'a') as usize;
            s_counts[idx_s] += 1;
        }

        steps.push(Step {
            code_line: 6,
            description: format!(
                "Index i={}: countS['{}'] = 1 + countS.get('{}', 0). Updated countS.",
                i, ch_s, ch_s
            ),
            visual: VisualState::ValidAnagram {
                s: s.to_string(),
                t: t.to_string(),
                s_counts,
                t_counts,
                active_s_idx: Some(i),
                active_t_idx: None,
                is_anagram: None,
            },
        });

        if ch_t.is_ascii_lowercase() {
            let idx_t = (ch_t as u8 - b'a') as usize;
            t_counts[idx_t] += 1;
        }

        steps.push(Step {
            code_line: 7,
            description: format!(
                "Index i={}: countT['{}'] = 1 + countT.get('{}', 0). Updated countT.",
                i, ch_t, ch_t
            ),
            visual: VisualState::ValidAnagram {
                s: s.to_string(),
                t: t.to_string(),
                s_counts,
                t_counts,
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
            active_s_idx: None,
            active_t_idx: None,
            is_anagram: Some(matches),
        },
    });

    steps
}

fn generate_anagram_sorting(s: &str, t: &str) -> Vec<Step> {
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
        code_line: 5,
        description: format!(
            "Sorted strings: s_sorted=\"{}\", t_sorted=\"{}\". Comparing sorted strings...",
            s_str, t_str
        ),
        visual: VisualState::ValidAnagram {
            s: s_str.clone(),
            t: t_str.clone(),
            s_counts: empty_counts,
            t_counts: empty_counts,
            active_s_idx: None,
            active_t_idx: None,
            is_anagram: Some(is_match),
        },
    });

    steps.push(Step {
        code_line: 5,
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
            active_s_idx: None,
            active_t_idx: None,
            is_anagram: Some(is_match),
        },
    });

    steps
}
