use crate::model::{Step, VisualState};

pub fn generate_time_key_value_store_steps() -> Vec<Step> {
    let mut steps = Vec::new();

    // Demonstrate TimeMap with key "foo" and operations set("foo", "bar", 1), set("foo", "bar2", 4), get("foo", 3), get("foo", 5)
    let timestamps = vec![(1, "bar"), (4, "bar2")];
    let query_key = "foo";

    steps.push(Step {
        code_line: 2,
        description: format!("TimeMap initialized. Key '{}' has timestamps: {:?}", query_key, timestamps),
        visual: VisualState::BinarySearch {
            nums: vec![1, 4],
            target: 3,
            left: 0,
            right: 1,
            mid: None,
            found_idx: None,
        },
    });

    // Query get("foo", 3)
    let target_t = 3;
    steps.push(Step {
        code_line: 6,
        description: format!("get('foo', timestamp={}): Binary search timestamps [1, 4] for largest time <= 3.", target_t),
        visual: VisualState::BinarySearch {
            nums: vec![1, 4],
            target: target_t,
            left: 0,
            right: 1,
            mid: Some(0),
            found_idx: None,
        },
    });

    steps.push(Step {
        code_line: 8,
        description: format!("mid=0 (time=1 <= 3): Save candidate 'bar', move left pointer right."),
        visual: VisualState::BinarySearch {
            nums: vec![1, 4],
            target: target_t,
            left: 1,
            right: 1,
            mid: Some(0),
            found_idx: Some(0),
        },
    });

    steps.push(Step {
        code_line: 11,
        description: format!("get('foo', 3) -> Returns candidate 'bar' (timestamp 1)."),
        visual: VisualState::BinarySearch {
            nums: vec![1, 4],
            target: target_t,
            left: 0,
            right: 1,
            mid: None,
            found_idx: Some(0),
        },
    });

    // Query get("foo", 5)
    steps.push(Step {
        code_line: 6,
        description: format!("get('foo', timestamp=5): Binary search timestamps [1, 4] for largest time <= 5."),
        visual: VisualState::BinarySearch {
            nums: vec![1, 4],
            target: 5,
            left: 0,
            right: 1,
            mid: Some(1),
            found_idx: None,
        },
    });

    steps.push(Step {
        code_line: 8,
        description: format!("mid=1 (time=4 <= 5): Save candidate 'bar2'. Found latest timestamp!"),
        visual: VisualState::BinarySearch {
            nums: vec![1, 4],
            target: 5,
            left: 1,
            right: 1,
            mid: Some(1),
            found_idx: Some(1),
        },
    });

    steps.push(Step {
        code_line: 11,
        description: format!("get('foo', 5) -> Returns candidate 'bar2' (timestamp 4)."),
        visual: VisualState::BinarySearch {
            nums: vec![1, 4],
            target: 5,
            left: 0,
            right: 1,
            mid: None,
            found_idx: Some(1),
        },
    });

    steps
}
