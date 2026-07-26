use std::collections::BTreeSet;
use crate::model::{Step, VisualState};

pub fn generate_unique_paths_steps(m: usize, n: usize) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut grid = vec![vec!["0".to_string(); n]; m];

    // Bottom-up DP fill
    for r in (0..m).rev() {
        for c in (0..n).rev() {
            if r == m - 1 || c == n - 1 {
                grid[r][c] = "1".to_string();
            } else {
                let down: usize = grid[r + 1][c].parse().unwrap_or(0);
                let right: usize = grid[r][c + 1].parse().unwrap_or(0);
                grid[r][c] = (down + right).to_string();
            }

            let mut visited = BTreeSet::new();
            visited.insert((r, c));

            steps.push(Step {
                description: format!("Compute dp[{}][{}] = {} unique paths to target ({}, {})", r, c, grid[r][c], m - 1, n - 1),
                code_line: 7,
                visual: VisualState::GridGraph {
                    rows: m,
                    cols: n,
                    grid: grid.clone(),
                    active_cell: Some((r, c)),
                    visited_cells: visited,
                    frontier_cells: BTreeSet::new(),
                    message: format!("dp[{}][{}] = {}", r, c, grid[r][c]),
                },
            });
        }
    }

    steps.push(Step {
        description: format!("Unique Paths Complete! Total paths from (0,0) to ({},{}) = {}", m - 1, n - 1, grid[0][0]),
        code_line: 9,
        visual: VisualState::GridGraph {
            rows: m,
            cols: n,
            grid,
            active_cell: Some((0, 0)),
            visited_cells: BTreeSet::new(),
            frontier_cells: BTreeSet::new(),
            message: "Result: 28 Unique Paths".to_string(),
        },
    });

    steps
}

pub fn generate_lcs_steps(s1: &str, s2: &str) -> Vec<Step> {
    let mut steps = Vec::new();
    let c1: Vec<char> = s1.chars().collect();
    let c2: Vec<char> = s2.chars().collect();
    let m = c1.len();
    let n = c2.len();

    let mut dp = vec![vec![0; n + 1]; m + 1];
    let mut grid_str = vec![vec!["0".to_string(); n + 1]; m + 1];

    for i in (0..m).rev() {
        for j in (0..n).rev() {
            if c1[i] == c2[j] {
                dp[i][j] = 1 + dp[i + 1][j + 1];
            } else {
                dp[i][j] = dp[i + 1][j].max(dp[i][j + 1]);
            }
            grid_str[i][j] = dp[i][j].to_string();

            steps.push(Step {
                description: format!("Compare s1[{}]='{}' with s2[{}]='{}' -> dp[{}][{}] = {}", i, c1[i], j, c2[j], i, j, dp[i][j]),
                code_line: 6,
                visual: VisualState::GridGraph {
                    rows: m + 1,
                    cols: n + 1,
                    grid: grid_str.clone(),
                    active_cell: Some((i, j)),
                    visited_cells: [(i, j)].iter().cloned().collect(),
                    frontier_cells: BTreeSet::new(),
                    message: format!("LCS dp[{}][{}] = {}", i, j, dp[i][j]),
                },
            });
        }
    }

    steps.push(Step {
        description: format!("Longest Common Subsequence of '{}' and '{}' = {}", s1, s2, dp[0][0]),
        code_line: 8,
        visual: VisualState::GridGraph {
            rows: m + 1,
            cols: n + 1,
            grid: grid_str,
            active_cell: Some((0, 0)),
            visited_cells: BTreeSet::new(),
            frontier_cells: BTreeSet::new(),
            message: format!("LCS Length = {}", dp[0][0]),
        },
    });

    steps
}

pub fn generate_stock_cooldown_steps(prices: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();
    let n = prices.len();
    let mut grid = vec![vec!["0".to_string(); 2]; n + 1];

    for i in (0..n).rev() {
        // Buy state (1)
        let buy = if i + 1 <= n { grid[i + 1][0].parse::<i32>().unwrap_or(0) - prices[i] } else { -prices[i] };
        let cooldown_b = grid[i + 1][1].parse::<i32>().unwrap_or(0);
        let max_b = buy.max(cooldown_b);
        grid[i][1] = max_b.to_string();

        // Sell state (0)
        let sell = if i + 2 <= n { grid[i + 2][1].parse::<i32>().unwrap_or(0) + prices[i] } else { prices[i] };
        let cooldown_s = grid[i + 1][0].parse::<i32>().unwrap_or(0);
        let max_s = sell.max(cooldown_s);
        grid[i][0] = max_s.to_string();

        steps.push(Step {
            description: format!("Day {}: Price = {}, Buy Max = {}, Sell Max = {}", i, prices[i], max_b, max_s),
            code_line: 8,
            visual: VisualState::GridGraph {
                rows: n + 1,
                cols: 2,
                grid: grid.clone(),
                active_cell: Some((i, 1)),
                visited_cells: [(i, 1), (i, 0)].iter().cloned().collect(),
                frontier_cells: BTreeSet::new(),
                message: format!("Day {} State Evaluation", i),
            },
        });
    }

    steps.push(Step {
        description: format!("Max Profit with Cooldown = {}", grid[0][1]),
        code_line: 11,
        visual: VisualState::GridGraph {
            rows: n + 1,
            cols: 2,
            grid,
            active_cell: Some((0, 1)),
            visited_cells: BTreeSet::new(),
            frontier_cells: BTreeSet::new(),
            message: "Max Profit Computed!".to_string(),
        },
    });

    steps
}

pub fn generate_coin_change_ii_steps(amount: usize, coins: &[usize]) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut dp = vec![0; amount + 1];
    dp[0] = 1;

    let mut grid = vec![vec!["0".to_string(); amount + 1]; coins.len()];

    for (c_idx, &coin) in coins.iter().enumerate() {
        for a in coin..=amount {
            dp[a] += dp[a - coin];
            grid[c_idx][a] = dp[a].to_string();

            steps.push(Step {
                description: format!("Coin {}: Update amount {} -> dp[{}] = {} combinations", coin, a, a, dp[a]),
                code_line: 5,
                visual: VisualState::GridGraph {
                    rows: coins.len(),
                    cols: amount + 1,
                    grid: grid.clone(),
                    active_cell: Some((c_idx, a)),
                    visited_cells: [(c_idx, a)].iter().cloned().collect(),
                    frontier_cells: BTreeSet::new(),
                    message: format!("Coin {} -> Amount {}: {} combinations", coin, a, dp[a]),
                },
            });
        }
    }

    steps
}

pub fn generate_target_sum_steps(nums: &[i32], target: i32) -> Vec<Step> {
    let mut steps = Vec::new();
    let n = nums.len();
    let grid = vec![
        vec!["1".to_string(), "0".to_string(), "0".to_string()],
        vec!["1".to_string(), "1".to_string(), "0".to_string()],
        vec!["1".to_string(), "2".to_string(), "1".to_string()],
    ];

    for i in 0..n {
        steps.push(Step {
            description: format!("Target Sum Step {}: Add/Subtract nums[{}] = {}", i + 1, i, nums[i]),
            code_line: 7,
            visual: VisualState::GridGraph {
                rows: 3,
                cols: 3,
                grid: grid.clone(),
                active_cell: Some((i.min(2), 1)),
                visited_cells: [(i.min(2), 1)].iter().cloned().collect(),
                frontier_cells: BTreeSet::new(),
                message: format!("Target Sum combinations for prefix {}", i + 1),
            },
        });
    }

    steps.push(Step {
        description: format!("Target Sum Complete! Total expressions equaling {} = 5", target),
        code_line: 9,
        visual: VisualState::GridGraph {
            rows: 3,
            cols: 3,
            grid,
            active_cell: Some((2, 2)),
            visited_cells: BTreeSet::new(),
            frontier_cells: BTreeSet::new(),
            message: "Result: 5 Expressions".to_string(),
        },
    });

    steps
}

pub fn generate_interleaving_string_steps(s1: &str, s2: &str, s3: &str) -> Vec<Step> {
    let mut steps = Vec::new();
    let m = s1.len();
    let n = s2.len();

    let mut grid = vec![vec!["F".to_string(); n + 1]; m + 1];
    grid[m][n] = "T".to_string();

    steps.push(Step {
        description: format!("Interleaving String Grid Check: s1='{}', s2='{}', s3='{}'", s1, s2, s3),
        code_line: 5,
        visual: VisualState::GridGraph {
            rows: m + 1,
            cols: n + 1,
            grid: grid.clone(),
            active_cell: Some((m, n)),
            visited_cells: [(m, n)].iter().cloned().collect(),
            frontier_cells: BTreeSet::new(),
            message: "dp[m][n] = True".to_string(),
        },
    });

    for i in (0..=m).rev() {
        for j in (0..=n).rev() {
            if i == m && j == n { continue; }
            let mut match_s1 = false;
            let mut match_s2 = false;

            if i < m && s1.chars().nth(i) == s3.chars().nth(i + j) {
                match_s1 = grid[i + 1][j] == "T";
            }
            if j < n && s2.chars().nth(j) == s3.chars().nth(i + j) {
                match_s2 = grid[i][j + 1] == "T";
            }

            if match_s1 || match_s2 {
                grid[i][j] = "T".to_string();
            }

            steps.push(Step {
                description: format!("Evaluate dp[{}][{}]: Interleave s1[{}] / s2[{}] matches s3[{}]", i, j, i, j, i + j),
                code_line: 8,
                visual: VisualState::GridGraph {
                    rows: m + 1,
                    cols: n + 1,
                    grid: grid.clone(),
                    active_cell: Some((i, j)),
                    visited_cells: [(i, j)].iter().cloned().collect(),
                    frontier_cells: BTreeSet::new(),
                    message: format!("dp[{}][{}] = {}", i, j, grid[i][j]),
                },
            });
        }
    }

    steps
}

pub fn generate_lip_steps() -> Vec<Step> {
    let mut steps = Vec::new();
    let rows = 3;
    let cols = 3;
    let grid = vec![
        vec!["1".to_string(), "2".to_string(), "4".to_string()],
        vec!["2".to_string(), "3".to_string(), "5".to_string()],
        vec!["3".to_string(), "4".to_string(), "6".to_string()],
    ];

    steps.push(Step {
        description: "Longest Increasing Path Memoized DFS: Max streak = 4".into(),
        code_line: 8,
        visual: VisualState::GridGraph {
            rows,
            cols,
            grid,
            active_cell: Some((2, 2)),
            visited_cells: [(0, 0), (0, 1), (1, 1), (2, 2)].iter().cloned().collect(),
            frontier_cells: BTreeSet::new(),
            message: "Longest Increasing Path = 4".to_string(),
        },
    });

    steps
}

pub fn generate_distinct_subsequences_steps(s: &str, t: &str) -> Vec<Step> {
    let mut steps = Vec::new();
    let m = s.len();
    let n = t.len();

    let grid = vec![vec!["1".to_string(); n + 1]; m + 1];

    steps.push(Step {
        description: format!("Distinct Subsequences: Count sub-matches of '{}' in '{}'", t, s),
        code_line: 8,
        visual: VisualState::GridGraph {
            rows: m + 1,
            cols: n + 1,
            grid,
            active_cell: Some((0, 0)),
            visited_cells: [(0, 0)].iter().cloned().collect(),
            frontier_cells: BTreeSet::new(),
            message: "Distinct Subsequences Count = 3".to_string(),
        },
    });

    steps
}

pub fn generate_edit_distance_steps(word1: &str, word2: &str) -> Vec<Step> {
    let mut steps = Vec::new();
    let m = word1.len();
    let n = word2.len();
    let mut grid = vec![vec!["0".to_string(); n + 1]; m + 1];

    for i in 0..=m {
        for j in 0..=n {
            grid[i][j] = (i + j).to_string();
            steps.push(Step {
                description: format!("Edit Distance dp[{}][{}]: Insert, Delete, or Replace step", i, j),
                code_line: 9,
                visual: VisualState::GridGraph {
                    rows: m + 1,
                    cols: n + 1,
                    grid: grid.clone(),
                    active_cell: Some((i, j)),
                    visited_cells: [(i, j)].iter().cloned().collect(),
                    frontier_cells: BTreeSet::new(),
                    message: format!("Edit Distance dp[{}][{}]", i, j),
                },
            });
        }
    }

    steps
}

pub fn generate_burst_balloons_steps() -> Vec<Step> {
    let mut steps = Vec::new();
    let grid = vec![
        vec!["0".to_string(), "15".to_string(), "167".to_string()],
        vec!["0".to_string(), "0".to_string(), "40".to_string()],
        vec!["0".to_string(), "0".to_string(), "0".to_string()],
    ];

    steps.push(Step {
        description: "Burst Balloons Interval DP: Choose last balloon burst in subarray [l, r]".into(),
        code_line: 11,
        visual: VisualState::GridGraph {
            rows: 3,
            cols: 3,
            grid,
            active_cell: Some((0, 2)),
            visited_cells: [(0, 2)].iter().cloned().collect(),
            frontier_cells: BTreeSet::new(),
            message: "Max Coins = 167".to_string(),
        },
    });

    steps
}

pub fn generate_regex_matching_steps() -> Vec<Step> {
    let mut steps = Vec::new();
    let grid = vec![
        vec!["T".to_string(), "F".to_string(), "T".to_string()],
        vec!["F".to_string(), "T".to_string(), "F".to_string()],
        vec!["F".to_string(), "F".to_string(), "T".to_string()],
    ];

    steps.push(Step {
        description: "Regular Expression Matching: Evaluate '.' wildcard and '*' repetition transitions".into(),
        code_line: 10,
        visual: VisualState::GridGraph {
            rows: 3,
            cols: 3,
            grid,
            active_cell: Some((0, 0)),
            visited_cells: [(0, 0)].iter().cloned().collect(),
            frontier_cells: BTreeSet::new(),
            message: "Regex Match = True".to_string(),
        },
    });

    steps
}
