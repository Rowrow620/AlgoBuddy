use crate::model::{Step, VisualState};

pub fn generate_meeting_rooms_steps(intervals: &[(i32, i32)]) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut sorted = intervals.to_vec();
    sorted.sort_by_key(|i| i.0);

    steps.push(Step {
        code_line: 3,
        description: format!("Sorted meeting intervals by start time: {:?}", sorted),
        visual: VisualState::ContainsDuplicate {
            nums: sorted.iter().map(|i| i.0).collect(),
            active_idx: None,
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });

    for i in 1..sorted.len() {
        let prev = sorted[i - 1];
        let curr = sorted[i];

        steps.push(Step {
            code_line: 5,
            description: format!(
                "Checking overlap: prev meeting [{}, {}] vs curr meeting [{}, {}].",
                prev.0, prev.1, curr.0, curr.1
            ),
            visual: VisualState::ContainsDuplicate {
                nums: vec![prev.0, prev.1, curr.0, curr.1],
                active_idx: Some(2),
                seen_set: std::collections::BTreeSet::new(),
                duplicate_val: None,
                has_duplicate: None,
            },
        });

        if curr.0 < prev.1 {
            steps.push(Step {
                code_line: 7,
                description: format!("Overlap detected! curr.start ({}) < prev.end ({}). Cannot attend all meetings! Return False.", curr.0, prev.1),
                visual: VisualState::ContainsDuplicate {
                    nums: vec![prev.0, prev.1, curr.0, curr.1],
                    active_idx: Some(2),
                    seen_set: std::collections::BTreeSet::new(),
                    duplicate_val: Some(curr.0),
                    has_duplicate: Some(false),
                },
            });
            return steps;
        }
    }

    steps.push(Step {
        code_line: 9,
        description: "No meeting overlaps! Can attend all meetings. Return True.".to_string(),
        visual: VisualState::ContainsDuplicate {
            nums: sorted.iter().map(|i| i.0).collect(),
            active_idx: None,
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: Some(true),
        },
    });

    steps
}
