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
                code_line: 5,
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
        code_line: 6,
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

pub(crate) const MEETING_ROOMS_ALL_PAIRS_LIMIT: usize = 40;

pub fn generate_meeting_rooms_all_pairs_steps(intervals: &[(i32, i32)]) -> Vec<Step> {
    if intervals.len() > MEETING_ROOMS_ALL_PAIRS_LIMIT {
        let message = format!(
            "All-pairs overlap visualization supports up to {} intervals; shorten the input before building the quadratic trace.",
            MEETING_ROOMS_ALL_PAIRS_LIMIT
        );
        return vec![Step {
            code_line: 3,
            description: message.clone(),
            visual: VisualState::TraceUnavailable { message },
        }];
    }

    let elements: Vec<i32> = intervals
        .iter()
        .flat_map(|&(start, end)| [start, end])
        .collect();
    let mut steps = vec![Step {
        code_line: 3,
        description: "Compare every distinct pair of meetings without sorting them first.".into(),
        visual: VisualState::Array1D {
            title: "Meeting Rooms: All-Pairs Overlap".into(),
            elements: elements.clone(),
            active_idx: None,
            secondary_idx: None,
            pointers: Vec::new(),
            status_message: "each adjacent cell pair is [start, end]".into(),
            is_success: None,
        },
    }];

    for i in 0..intervals.len() {
        for j in i + 1..intervals.len() {
            let first = intervals[i];
            let second = intervals[j];
            let overlaps = first.0.max(second.0) < first.1.min(second.1);
            steps.push(Step {
                code_line: 6,
                description: format!(
                    "Compare pair {i}, {j}: [{}, {}] and [{}, {}] {}.",
                    first.0,
                    first.1,
                    second.0,
                    second.1,
                    if overlaps {
                        "overlap"
                    } else {
                        "do not overlap"
                    }
                ),
                visual: VisualState::Array1D {
                    title: "Meeting Rooms: All-Pairs Overlap".into(),
                    elements: elements.clone(),
                    active_idx: Some(i * 2),
                    secondary_idx: Some(j * 2),
                    pointers: Vec::new(),
                    status_message: format!(
                        "max({}, {}) < min({}, {}) is {overlaps}",
                        first.0, second.0, first.1, second.1
                    ),
                    is_success: None,
                },
            });
            if overlaps {
                steps.push(Step {
                    code_line: 7,
                    description: "An overlapping pair exists, so not all meetings can be attended."
                        .into(),
                    visual: VisualState::Array1D {
                        title: "Meeting Rooms: All-Pairs Overlap".into(),
                        elements,
                        active_idx: Some(i * 2),
                        secondary_idx: Some(j * 2),
                        pointers: Vec::new(),
                        status_message: "return False".into(),
                        is_success: Some(false),
                    },
                });
                return steps;
            }
        }
    }

    steps.push(Step {
        code_line: 8,
        description: "Every pair is disjoint or only touches at an endpoint; return True.".into(),
        visual: VisualState::Array1D {
            title: "Meeting Rooms: All-Pairs Overlap".into(),
            elements,
            active_idx: None,
            secondary_idx: None,
            pointers: Vec::new(),
            status_message: "return True".into(),
            is_success: Some(true),
        },
    });
    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    fn final_success(steps: &[Step]) -> Option<bool> {
        match &steps.last().expect("trace must not be empty").visual {
            VisualState::Array1D { is_success, .. } => *is_success,
            VisualState::TraceUnavailable { .. } => None,
            _ => panic!("unexpected visual state"),
        }
    }

    #[test]
    fn all_pairs_detects_overlap_and_allows_touching_meetings() {
        assert_eq!(
            final_success(&generate_meeting_rooms_all_pairs_steps(&[
                (0, 30),
                (5, 10),
                (15, 20)
            ])),
            Some(false)
        );
        assert_eq!(
            final_success(&generate_meeting_rooms_all_pairs_steps(&[(0, 5), (5, 10)])),
            Some(true)
        );
    }

    #[test]
    fn all_pairs_stops_before_an_oversized_trace() {
        let intervals = vec![(0, 1); MEETING_ROOMS_ALL_PAIRS_LIMIT + 1];
        let steps = generate_meeting_rooms_all_pairs_steps(&intervals);
        assert!(matches!(
            steps.as_slice(),
            [Step {
                visual: VisualState::TraceUnavailable { .. },
                ..
            }]
        ));
    }
}
