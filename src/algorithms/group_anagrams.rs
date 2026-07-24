use std::collections::BTreeMap;
use crate::model::{Step, VisualState};

pub fn generate_group_anagrams_steps(strs: &[String], approach_id: usize) -> Vec<Step> {
    let mut steps = Vec::new();
    let strs_vec = strs.to_vec();
    let mut groups: BTreeMap<String, Vec<String>> = BTreeMap::new();

    steps.push(Step {
        code_line: 3,
        description: "Initialized empty HashMap `res = defaultdict(list)` to store grouped anagrams.".to_string(),
        visual: VisualState::GroupAnagrams {
            input_strs: strs_vec.clone(),
            active_idx: None,
            key_fmt: String::new(),
            groups: groups.clone(),
        },
    });

    for (i, s) in strs.iter().enumerate() {
        let key_str = if approach_id == 0 {
            // Frequency Tuple
            let mut counts = [0usize; 26];
            for ch in s.chars() {
                if ch.is_ascii_lowercase() {
                    let idx = (ch as u8 - b'a') as usize;
                    counts[idx] += 1;
                }
            }
            format!("{:?}", counts)
        } else {
            // Sorted String
            let mut chars: Vec<char> = s.chars().collect();
            chars.sort();
            chars.into_iter().collect::<String>()
        };

        steps.push(Step {
            code_line: 5,
            description: format!("Processing string strs[{}] = \"{}\". Generated HashMap key signature: \"{}\".", i, s, key_str),
            visual: VisualState::GroupAnagrams {
                input_strs: strs_vec.clone(),
                active_idx: Some(i),
                key_fmt: key_str.clone(),
                groups: groups.clone(),
            },
        });

        groups.entry(key_str.clone()).or_default().push(s.clone());

        steps.push(Step {
            code_line: 8,
            description: format!("Appended \"{}\" to HashMap group matching key signature.", s),
            visual: VisualState::GroupAnagrams {
                input_strs: strs_vec.clone(),
                active_idx: Some(i),
                key_fmt: key_str,
                groups: groups.clone(),
            },
        });
    }

    let final_grouped: Vec<Vec<String>> = groups.values().cloned().collect();
    steps.push(Step {
        code_line: 9,
        description: format!("Grouped all strings into {} anagram categories! Result: {:?}.", final_grouped.len(), final_grouped),
        visual: VisualState::GroupAnagrams {
            input_strs: strs_vec,
            active_idx: None,
            key_fmt: String::new(),
            groups,
        },
    });

    steps
}
