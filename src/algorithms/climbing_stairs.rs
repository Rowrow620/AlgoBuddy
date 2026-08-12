use crate::model::{Step, VisualState};

pub fn generate_climbing_stairs_steps(n: usize) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut dp = vec![0i32; (n + 1).max(3)];
    dp[1] = 1;
    dp[2] = 2;

    steps.push(Step {
        code_line: 3,
        description: format!(
            "Base cases for Climbing Stairs (n={}): dp[1]=1 way, dp[2]=2 ways.",
            n
        ),
        visual: VisualState::ContainsDuplicate {
            nums: dp.clone(),
            active_idx: Some(2),
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });

    for i in 3..=n {
        dp[i] = dp[i - 1] + dp[i - 2];
        steps.push(Step {
            code_line: 6,
            description: format!(
                "Step {}: dp[{}] = dp[{}] + dp[{}] = {} + {} = {} ways.",
                i,
                i,
                i - 1,
                i - 2,
                dp[i - 1],
                dp[i - 2],
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
        description: format!("Total distinct ways to climb {} stairs = {}.", n, dp[n]),
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

pub(crate) const CLIMBING_STAIRS_RECURSION_LIMIT: usize = 12;

pub fn generate_climbing_stairs_recursive_steps(n: usize) -> Vec<Step> {
    if n > CLIMBING_STAIRS_RECURSION_LIMIT {
        let message = format!(
            "Naive-recursion visualization supports n up to {}; lower n before building the exponential trace.",
            CLIMBING_STAIRS_RECURSION_LIMIT
        );
        return vec![Step {
            code_line: 3,
            description: message.clone(),
            visual: VisualState::TraceUnavailable { message },
        }];
    }

    let mut steps = Vec::new();
    let result = climb_recursively(n, 0, &mut steps);
    steps.push(Step {
        code_line: if n <= 2 { 3 } else { 4 },
        description: format!("Naive recursion finds {result} ways to climb {n} stairs."),
        visual: VisualState::Array1D {
            title: "Climbing Stairs: Naive Recursion".into(),
            elements: vec![n as i32, result],
            active_idx: Some(1),
            secondary_idx: None,
            pointers: Vec::new(),
            status_message: format!("return {result}"),
            is_success: Some(true),
        },
    });
    steps
}

fn climb_recursively(n: usize, depth: usize, steps: &mut Vec<Step>) -> i32 {
    if n <= 2 {
        steps.push(Step {
            code_line: 3,
            description: format!("{}Base case climbStairs({n}) = {n}.", "  ".repeat(depth)),
            visual: VisualState::Array1D {
                title: "Climbing Stairs: Naive Recursion".into(),
                elements: vec![n as i32],
                active_idx: Some(0),
                secondary_idx: None,
                pointers: Vec::new(),
                status_message: format!("base case returns {n}"),
                is_success: None,
            },
        });
        return n as i32;
    }

    let one_step = climb_recursively(n - 1, depth + 1, steps);
    let two_steps = climb_recursively(n - 2, depth + 1, steps);
    let result = one_step + two_steps;
    steps.push(Step {
        code_line: 4,
        description: format!(
            "{}climbStairs({n}) = {one_step} + {two_steps} = {result}.",
            "  ".repeat(depth)
        ),
        visual: VisualState::Array1D {
            title: "Climbing Stairs: Naive Recursion".into(),
            elements: vec![one_step, two_steps, result],
            active_idx: Some(2),
            secondary_idx: None,
            pointers: Vec::new(),
            status_message: format!("combine branches for n={n}"),
            is_success: None,
        },
    });
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn naive_recursion_counts_the_stair_combinations() {
        let steps = generate_climbing_stairs_recursive_steps(5);
        assert!(steps
            .last()
            .expect("trace must not be empty")
            .description
            .contains("8 ways"));
    }

    #[test]
    fn naive_recursion_stops_before_an_exponential_trace() {
        let steps = generate_climbing_stairs_recursive_steps(CLIMBING_STAIRS_RECURSION_LIMIT + 1);
        assert!(matches!(
            steps.as_slice(),
            [Step {
                visual: VisualState::TraceUnavailable { .. },
                ..
            }]
        ));
    }
}
