use crate::model::{Step, VisualState};

pub fn generate_min_cost_stairs_steps(cost: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();
    let n = cost.len();
    let mut dp = vec![0i32; n + 1];

    steps.push(Step {
        code_line: 3,
        description: format!("Min Cost Climbing Stairs for cost array: {:?}", cost),
        visual: VisualState::ContainsDuplicate {
            nums: cost.to_vec(),
            active_idx: None,
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });

    for i in 2..=n {
        dp[i] = (dp[i - 1] + cost[i - 1]).min(dp[i - 2] + cost[i - 2]);
        steps.push(Step {
            code_line: 6,
            description: format!(
                "Step {}: min(dp[{}] + cost[{}], dp[{}] + cost[{}]) = min({}, {}) = {}.",
                i,
                i - 1,
                i - 1,
                i - 2,
                i - 2,
                dp[i - 1] + cost[i - 1],
                dp[i - 2] + cost[i - 2],
                dp[i]
            ),
            visual: VisualState::ContainsDuplicate {
                nums: dp.clone(),
                active_idx: Some(i),
                seen_set: std::collections::BTreeSet::new(),
                duplicate_val: None,
                has_duplicate: None,
            },
        });
    }

    steps.push(Step {
        code_line: 8,
        description: format!("Minimum cost to reach top of the floor = {}.", dp[n]),
        visual: VisualState::ContainsDuplicate {
            nums: dp,
            active_idx: Some(n),
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: Some(true),
        },
    });

    steps
}

pub(crate) const MIN_COST_STAIRS_RECURSION_LIMIT: usize = 12;

pub fn generate_min_cost_stairs_recursive_steps(cost: &[i32]) -> Vec<Step> {
    if cost.len() > MIN_COST_STAIRS_RECURSION_LIMIT {
        let message = format!(
            "Naive-recursion visualization supports up to {} cost entries; shorten the input before building the exponential trace.",
            MIN_COST_STAIRS_RECURSION_LIMIT
        );
        return vec![Step {
            code_line: 3,
            description: message.clone(),
            visual: VisualState::TraceUnavailable { message },
        }];
    }

    let mut steps = Vec::new();
    let from_zero = min_cost_recursively(cost, 0, 0, &mut steps);
    let from_one = min_cost_recursively(cost, 1, 0, &mut steps);
    let result = from_zero.min(from_one);
    steps.push(Step {
        code_line: 6,
        description: format!(
            "Choose the cheaper starting stair: min({from_zero}, {from_one}) = {result}."
        ),
        visual: VisualState::Array1D {
            title: "Min Cost Climbing Stairs: Naive Recursion".into(),
            elements: cost.to_vec(),
            active_idx: None,
            secondary_idx: None,
            pointers: Vec::new(),
            status_message: format!("return {result}"),
            is_success: Some(true),
        },
    });
    steps
}

fn min_cost_recursively(cost: &[i32], index: usize, depth: usize, steps: &mut Vec<Step>) -> i32 {
    if index >= cost.len() {
        steps.push(Step {
            code_line: 4,
            description: format!(
                "{}Index {index} is at or above the top; this branch adds 0 cost.",
                "  ".repeat(depth)
            ),
            visual: VisualState::Array1D {
                title: "Min Cost Climbing Stairs: Naive Recursion".into(),
                elements: cost.to_vec(),
                active_idx: None,
                secondary_idx: None,
                pointers: Vec::new(),
                status_message: "base case returns 0".into(),
                is_success: None,
            },
        });
        return 0;
    }

    let one_step = min_cost_recursively(cost, index + 1, depth + 1, steps);
    let two_steps = min_cost_recursively(cost, index + 2, depth + 1, steps);
    let result = cost[index] + one_step.min(two_steps);
    steps.push(Step {
        code_line: 5,
        description: format!(
            "{}dfs({index}) = cost[{index}] ({}) + min({one_step}, {two_steps}) = {result}.",
            "  ".repeat(depth),
            cost[index]
        ),
        visual: VisualState::Array1D {
            title: "Min Cost Climbing Stairs: Naive Recursion".into(),
            elements: cost.to_vec(),
            active_idx: Some(index),
            secondary_idx: None,
            pointers: Vec::new(),
            status_message: format!("dfs({index}) = {result}"),
            is_success: None,
        },
    });
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn naive_recursion_finds_the_minimum_cost() {
        let steps = generate_min_cost_stairs_recursive_steps(&[10, 15, 20]);
        assert!(steps
            .last()
            .expect("trace must not be empty")
            .description
            .ends_with("= 15."));
    }

    #[test]
    fn naive_recursion_stops_before_an_exponential_trace() {
        let cost = vec![1; MIN_COST_STAIRS_RECURSION_LIMIT + 1];
        let steps = generate_min_cost_stairs_recursive_steps(&cost);
        assert!(matches!(
            steps.as_slice(),
            [Step {
                visual: VisualState::TraceUnavailable { .. },
                ..
            }]
        ));
    }
}
