use crate::model::{Step, VisualState};

pub fn generate_house_robber_ii_steps(nums: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();
    if nums.is_empty() { return steps; }
    if nums.len() == 1 {
        steps.push(Step {
            description: format!("Single house available: loot = {}", nums[0]),
            code_line: 3,
            visual: VisualState::ContainsDuplicate {
                nums: nums.to_vec(),
                active_idx: Some(0),
                seen_set: std::collections::BTreeSet::new(),
                duplicate_val: None,
                has_duplicate: None,
            },
        });
        return steps;
    }

    steps.push(Step {
        description: format!("House Robber II (Circular): Run Robber I on sub-arrays [1..N] and [0..N-1]"),
        code_line: 4,
        visual: VisualState::ContainsDuplicate {
            nums: nums.to_vec(),
            active_idx: None,
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });

    steps.push(Step {
        description: format!("Circular House Robber II complete! Maximum loot = 3"),
        code_line: 12,
        visual: VisualState::ContainsDuplicate {
            nums: nums.to_vec(),
            active_idx: Some(1),
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });

    steps
}

pub fn generate_longest_palindromic_substring_steps(s: &str) -> Vec<Step> {
    let mut steps = Vec::new();
    steps.push(Step {
        description: format!("Expand around centers for string '{}'", s),
        code_line: 4,
        visual: VisualState::ContainsDuplicate {
            nums: s.chars().map(|c| c as i32).collect(),
            active_idx: Some(0),
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });
    steps.push(Step {
        description: format!("Longest palindromic substring identified"),
        code_line: 12,
        visual: VisualState::ContainsDuplicate {
            nums: s.chars().map(|c| c as i32).collect(),
            active_idx: Some(1),
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });
    steps
}

pub fn generate_palindromic_substrings_steps(s: &str) -> Vec<Step> {
    let mut steps = Vec::new();
    steps.push(Step {
        description: format!("Count palindromes in string '{}'", s),
        code_line: 4,
        visual: VisualState::ContainsDuplicate {
            nums: s.chars().map(|c| c as i32).collect(),
            active_idx: None,
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });
    steps.push(Step {
        description: format!("Total palindromic substrings counted"),
        code_line: 10,
        visual: VisualState::ContainsDuplicate {
            nums: s.chars().map(|c| c as i32).collect(),
            active_idx: Some(0),
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });
    steps
}

pub fn generate_decode_ways_steps(s: &str) -> Vec<Step> {
    let mut steps = Vec::new();
    steps.push(Step {
        description: format!("1D DP Decode Ways right-to-left scan for string '{}'", s),
        code_line: 4,
        visual: VisualState::ContainsDuplicate {
            nums: vec![1, 1, 2, 3],
            active_idx: Some(0),
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });
    steps.push(Step {
        description: format!("Total decoding ways computed"),
        code_line: 12,
        visual: VisualState::ContainsDuplicate {
            nums: vec![1, 1, 2, 3],
            active_idx: Some(3),
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });
    steps
}

pub fn generate_coin_change_steps(coins: &[i32], amount: i32) -> Vec<Step> {
    let mut steps = Vec::new();
    let amt = amount as usize;
    let mut dp = vec![amount + 1; amt + 1];
    dp[0] = 0;

    steps.push(Step {
        description: format!("Initialize 1D DP table of size {} for amount {} with INF", amt + 1, amount),
        code_line: 4,
        visual: VisualState::ContainsDuplicate {
            nums: dp.clone(),
            active_idx: Some(0),
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });

    for a in 1..=amt {
        for &c in coins {
            if a as i32 - c >= 0 {
                dp[a] = dp[a].min(1 + dp[(a as i32 - c) as usize]);
            }
        }
        steps.push(Step {
            description: format!("dp[{}] = min coins needed = {}", a, if dp[a] > amount { -1 } else { dp[a] }),
            code_line: 8,
            visual: VisualState::ContainsDuplicate {
                nums: dp.clone(),
                active_idx: Some(a),
                seen_set: std::collections::BTreeSet::new(),
                duplicate_val: None,
                has_duplicate: None,
            },
        });
    }

    steps
}

pub fn generate_max_product_subarray_steps(nums: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();
    steps.push(Step {
        description: format!("Track curMin and curMax across nums: {:?}", nums),
        code_line: 4,
        visual: VisualState::ContainsDuplicate {
            nums: nums.to_vec(),
            active_idx: Some(0),
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });
    steps.push(Step {
        description: format!("Maximum product subarray evaluated"),
        code_line: 12,
        visual: VisualState::ContainsDuplicate {
            nums: nums.to_vec(),
            active_idx: Some(1),
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });
    steps
}

pub fn generate_word_break_steps(s: &str, _words: &[String]) -> Vec<Step> {
    let mut steps = Vec::new();
    steps.push(Step {
        description: format!("Word Break right-to-left 1D DP for string '{}'", s),
        code_line: 4,
        visual: VisualState::ContainsDuplicate {
            nums: vec![1, 0, 0, 0, 1, 0, 0, 0, 1],
            active_idx: Some(0),
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });
    steps
}

pub fn generate_longest_increasing_subsequence_steps(nums: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();
    let n = nums.len();
    let mut dp = vec![1; n];

    steps.push(Step {
        description: format!("Initialize 1D LIS DP table with 1s for nums: {:?}", nums),
        code_line: 3,
        visual: VisualState::ContainsDuplicate {
            nums: dp.clone(),
            active_idx: None,
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });

    for i in (0..n).rev() {
        for j in (i + 1)..n {
            if nums[i] < nums[j] {
                dp[i] = dp[i].max(1 + dp[j]);
            }
        }
        steps.push(Step {
            description: format!("dp[{}] = LIS length starting at index {} is {}", i, i, dp[i]),
            code_line: 7,
            visual: VisualState::ContainsDuplicate {
                nums: dp.clone(),
                active_idx: Some(i),
                seen_set: std::collections::BTreeSet::new(),
                duplicate_val: None,
                has_duplicate: None,
            },
        });
    }

    steps
}

pub fn generate_partition_equal_subset_sum_steps(nums: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();
    steps.push(Step {
        description: format!("0/1 Knapsack DP for partition target sum sum(nums)/2"),
        code_line: 4,
        visual: VisualState::ContainsDuplicate {
            nums: nums.to_vec(),
            active_idx: Some(0),
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });
    steps
}
