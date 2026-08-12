use crate::model::{Step, VisualState};
use std::collections::BTreeMap;

pub fn generate_group_anagrams_steps(strs: &[String], approach_id: usize) -> Vec<Step> {
    if strs
        .iter()
        .any(|value| !value.bytes().all(|byte| byte.is_ascii_lowercase()))
    {
        let message = "Group Anagrams traces accept lowercase English letters (a-z) only; empty strings are supported. Update every input value to match the displayed solutions."
            .to_string();
        return vec![Step {
            code_line: 5,
            description: message.clone(),
            visual: VisualState::TraceUnavailable { message },
        }];
    }

    let mut steps = Vec::new();
    let strs_vec = strs.to_vec();
    let mut groups: BTreeMap<String, Vec<String>> = BTreeMap::new();

    steps.push(Step {
        code_line: 3,
        description:
            "Initialized empty HashMap `res = defaultdict(list)` to store grouped anagrams."
                .to_string(),
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
            description: format!(
                "Processing string strs[{}] = \"{}\". Generated HashMap key signature: \"{}\".",
                i, s, key_str
            ),
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
            description: format!(
                "Appended \"{}\" to HashMap group matching key signature.",
                s
            ),
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
        description: format!(
            "Grouped all strings into {} anagram categories! Result: {:?}.",
            final_grouped.len(),
            final_grouped
        ),
        visual: VisualState::GroupAnagrams {
            input_strs: strs_vec,
            active_idx: None,
            key_fmt: String::new(),
            groups,
        },
    });

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grouped_result(strs: &[String], approach_id: usize) -> Vec<Vec<String>> {
        let steps = generate_group_anagrams_steps(strs, approach_id);
        let VisualState::GroupAnagrams { groups, .. } = &steps
            .last()
            .expect("valid input must produce a completed trace")
            .visual
        else {
            panic!("expected a group-anagrams result");
        };

        let mut result: Vec<Vec<String>> = groups.values().cloned().collect();
        for group in &mut result {
            group.sort();
        }
        result.sort();
        result
    }

    #[test]
    fn unsupported_characters_are_rejected_by_both_approaches() {
        let strs = vec!["A".to_string(), "B".to_string()];

        for approach_id in [0, 1] {
            let steps = generate_group_anagrams_steps(&strs, approach_id);
            assert_eq!(steps.len(), 1);
            assert!(steps[0].description.contains("lowercase English letters"));
            assert!(matches!(
                &steps[0].visual,
                VisualState::TraceUnavailable { message }
                    if message.contains("lowercase English letters")
            ));
        }
    }

    #[test]
    fn valid_inputs_produce_the_same_groups_for_both_approaches() {
        let strs = ["eat", "tea", "tan", "ate", "nat", "bat"]
            .map(str::to_string)
            .to_vec();
        let expected = vec![
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
            vec!["bat".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
        ];

        assert_eq!(grouped_result(&strs, 0), expected);
        assert_eq!(grouped_result(&strs, 1), expected);
    }

    #[test]
    fn empty_strings_remain_supported() {
        let strs = vec![String::new(), String::new(), "a".to_string()];

        for approach_id in [0, 1] {
            let result = grouped_result(&strs, approach_id);
            assert!(result.contains(&vec![String::new(), String::new()]));
            assert!(result.contains(&vec!["a".to_string()]));
        }
    }
}
