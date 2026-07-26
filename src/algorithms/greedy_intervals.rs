use crate::model::{Step, VisualState};

pub fn generate_maximum_subarray_steps(nums: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut max_sum = nums[0];
    let mut cur_sum = 0;

    steps.push(Step {
        description: format!("Kadane's Algorithm for Maximum Subarray on {:?}", nums),
        code_line: 3,
        visual: VisualState::ContainsDuplicate {
            nums: nums.to_vec(),
            active_idx: None,
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });

    for (i, &n) in nums.iter().enumerate() {
        if cur_sum < 0 { cur_sum = 0; }
        cur_sum += n;
        if cur_sum > max_sum { max_sum = cur_sum; }

        steps.push(Step {
            description: format!("Index {}: num = {}, curSum = {}, maxSoFar = {}", i, n, cur_sum, max_sum),
            code_line: 6,
            visual: VisualState::ContainsDuplicate {
                nums: nums.to_vec(),
                active_idx: Some(i),
                seen_set: std::collections::BTreeSet::new(),
                duplicate_val: None,
                has_duplicate: None,
            },
        });
    }

    steps
}

pub fn generate_jump_game_steps(nums: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut goal = nums.len() - 1;

    steps.push(Step {
        description: format!("Jump Game: Start backwards goal at index {}", goal),
        code_line: 3,
        visual: VisualState::ContainsDuplicate {
            nums: nums.to_vec(),
            active_idx: Some(goal),
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });

    for i in (0..nums.len() - 1).rev() {
        if i + nums[i] as usize >= goal {
            goal = i;
        }
        steps.push(Step {
            description: format!("Index {}: jump = {}, reach = {}, updated goal = {}", i, nums[i], i + nums[i] as usize, goal),
            code_line: 6,
            visual: VisualState::ContainsDuplicate {
                nums: nums.to_vec(),
                active_idx: Some(i),
                seen_set: std::collections::BTreeSet::new(),
                duplicate_val: None,
                has_duplicate: None,
            },
        });
    }

    steps
}

pub fn generate_jump_game_ii_steps(nums: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut jumps = 0;
    let mut l = 0;
    let mut r = 0;

    while r < nums.len() - 1 {
        let mut farthest = 0;
        for i in l..=r {
            if i + nums[i] as usize > farthest {
                farthest = i + nums[i] as usize;
            }
        }
        l = r + 1;
        r = farthest;
        jumps += 1;

        steps.push(Step {
            description: format!("Jump {}: level window [{}, {}], farthest = {}", jumps, l, r, farthest),
            code_line: 7,
            visual: VisualState::ContainsDuplicate {
                nums: nums.to_vec(),
                active_idx: Some(r.min(nums.len() - 1)),
                seen_set: std::collections::BTreeSet::new(),
                duplicate_val: None,
                has_duplicate: None,
            },
        });
    }

    steps
}

pub fn generate_gas_station_steps(gas: &[i32], cost: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut _total_tank = 0;
    let mut curr_tank = 0;
    let mut starting_station = 0;

    for i in 0..gas.len() {
        _total_tank += gas[i] - cost[i];
        curr_tank += gas[i] - cost[i];

        if curr_tank < 0 {
            starting_station = i + 1;
            curr_tank = 0;
        }

        steps.push(Step {
            description: format!("Station {}: gas = {}, cost = {}, currTank = {}, startStation = {}", i, gas[i], cost[i], curr_tank, starting_station),
            code_line: 7,
            visual: VisualState::ContainsDuplicate {
                nums: gas.to_vec(),
                active_idx: Some(i),
                seen_set: std::collections::BTreeSet::new(),
                duplicate_val: None,
                has_duplicate: None,
            },
        });
    }

    steps
}

pub fn generate_hand_of_straights_steps(hand: &[i32], group_size: usize) -> Vec<Step> {
    let mut steps = Vec::new();
    steps.push(Step {
        description: format!("Hand of Straights: Form consecutive groups of size {} from {:?}", group_size, hand),
        code_line: 4,
        visual: VisualState::ContainsDuplicate {
            nums: hand.to_vec(),
            active_idx: None,
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });
    steps
}

pub fn generate_merge_triplets_steps() -> Vec<Step> {
    let mut steps = Vec::new();
    steps.push(Step {
        description: "Merge Triplets: Filter out invalid triplets with values > target and check coverage".into(),
        code_line: 4,
        visual: VisualState::ContainsDuplicate {
            nums: vec![1],
            active_idx: Some(0),
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });
    steps
}

pub fn generate_partition_labels_steps(s: &str) -> Vec<Step> {
    let mut steps = Vec::new();
    steps.push(Step {
        description: format!("Partition Labels for string '{}'", s),
        code_line: 4,
        visual: VisualState::ContainsDuplicate {
            nums: vec![s.len() as i32],
            active_idx: None,
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });
    steps
}

pub fn generate_valid_parenthesis_string_steps(s: &str) -> Vec<Step> {
    let mut steps = Vec::new();
    steps.push(Step {
        description: format!("Valid Parenthesis String: Track minOpen and maxOpen count range for '{}'", s),
        code_line: 4,
        visual: VisualState::ContainsDuplicate {
            nums: vec![s.len() as i32],
            active_idx: None,
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });
    steps
}

pub fn generate_insert_interval_steps() -> Vec<Step> {
    let mut steps = Vec::new();
    steps.push(Step {
        description: "Insert Interval: 3-phase scan (left non-overlapping, merge overlapping, right)".into(),
        code_line: 4,
        visual: VisualState::ContainsDuplicate {
            nums: vec![1, 3, 2, 5, 6, 9],
            active_idx: Some(2),
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });
    steps
}

pub fn generate_merge_intervals_steps() -> Vec<Step> {
    let mut steps = Vec::new();
    steps.push(Step {
        description: "Merge Intervals: Sort by start time and merge adjacent overlaps".into(),
        code_line: 4,
        visual: VisualState::ContainsDuplicate {
            nums: vec![1, 3, 2, 6, 8, 10, 15, 18],
            active_idx: None,
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });
    steps
}

pub fn generate_non_overlapping_intervals_steps() -> Vec<Step> {
    let mut steps = Vec::new();
    steps.push(Step {
        description: "Non-overlapping Intervals: Remove interval with larger end time on overlap".into(),
        code_line: 4,
        visual: VisualState::ContainsDuplicate {
            nums: vec![1, 2, 2, 3, 3, 4, 1, 3],
            active_idx: None,
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });
    steps
}

pub fn generate_meeting_rooms_ii_steps() -> Vec<Step> {
    let mut steps = Vec::new();
    steps.push(Step {
        description: "Meeting Rooms II: Sort start and end times to count max simultaneous active meetings".into(),
        code_line: 4,
        visual: VisualState::ContainsDuplicate {
            nums: vec![0, 30, 5, 10, 15, 20],
            active_idx: None,
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });
    steps
}

pub fn generate_min_interval_query_steps() -> Vec<Step> {
    let mut steps = Vec::new();
    steps.push(Step {
        description: "Minimum Interval Query: Process sorted queries with min-heap of active interval lengths".into(),
        code_line: 4,
        visual: VisualState::ContainsDuplicate {
            nums: vec![1, 4, 2, 4, 3, 6, 4, 4],
            active_idx: None,
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });
    steps
}
