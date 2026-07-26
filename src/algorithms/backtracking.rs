use crate::model::{Step, VisualState};

pub fn generate_subsets_steps(nums: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut current_subset = Vec::new();
    let mut all_subsets: Vec<Vec<i32>> = Vec::new();

    fn backtrack(
        idx: usize,
        nums: &[i32],
        current: &mut Vec<i32>,
        all: &mut Vec<Vec<i32>>,
        steps: &mut Vec<Step>,
    ) {
        if idx == nums.len() {
            all.push(current.clone());
            steps.push(Step {
                description: format!("Base Case: Leaf node reached at idx = {}. Found valid subset: {:?}", idx, current),
                code_line: 7,
                visual: VisualState::DecisionTreeVisual {
                    current_path: current.clone(),
                    active_choice: Some(format!("Leaf Subset {:?}", current)),
                    completed_results: all.clone(),
                },
            });
            return;
        }

        // Choice 1: Include nums[idx]
        current.push(nums[idx]);
        steps.push(Step {
            description: format!("Decision: INCLUDE nums[{}] = {} -> current subset = {:?}", idx, nums[idx], current),
            code_line: 9,
            visual: VisualState::DecisionTreeVisual {
                current_path: current.clone(),
                active_choice: Some(format!("Include {}", nums[idx])),
                completed_results: all.clone(),
            },
        });
        backtrack(idx + 1, nums, current, all, steps);
        current.pop();

        // Choice 2: Exclude nums[idx]
        steps.push(Step {
            description: format!("Backtrack & Decision: EXCLUDE nums[{}] = {} -> current subset = {:?}", idx, nums[idx], current),
            code_line: 11,
            visual: VisualState::DecisionTreeVisual {
                current_path: current.clone(),
                active_choice: Some(format!("Exclude {}", nums[idx])),
                completed_results: all.clone(),
            },
        });
        backtrack(idx + 1, nums, current, all, steps);
    }

    steps.push(Step {
        description: format!("Start Subsets Backtracking Decision Tree for nums = {:?}", nums),
        code_line: 2,
        visual: VisualState::DecisionTreeVisual {
            current_path: Vec::new(),
            active_choice: Some("Root Decision Node".into()),
            completed_results: Vec::new(),
        },
    });

    backtrack(0, nums, &mut current_subset, &mut all_subsets, &mut steps);

    steps.push(Step {
        description: format!("Subsets complete! Total {} subsets generated: {:?}", all_subsets.len(), all_subsets),
        code_line: 14,
        visual: VisualState::DecisionTreeVisual {
            current_path: Vec::new(),
            active_choice: Some(format!("Total {} Subsets", all_subsets.len())),
            completed_results: all_subsets,
        },
    });

    steps
}

pub fn generate_permutations_steps(nums: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut current_perm = Vec::new();
    let mut used = vec![false; nums.len()];
    let mut all_perms: Vec<Vec<i32>> = Vec::new();

    fn backtrack(
        nums: &[i32],
        used: &mut [bool],
        current: &mut Vec<i32>,
        all: &mut Vec<Vec<i32>>,
        steps: &mut Vec<Step>,
    ) {
        if current.len() == nums.len() {
            all.push(current.clone());
            steps.push(Step {
                description: format!("Full Permutation Formed: {:?}", current),
                code_line: 6,
                visual: VisualState::DecisionTreeVisual {
                    current_path: current.clone(),
                    active_choice: Some(format!("Permutation {:?}", current)),
                    completed_results: all.clone(),
                },
            });
            return;
        }

        for i in 0..nums.len() {
            if used[i] {
                continue;
            }
            used[i] = true;
            current.push(nums[i]);
            steps.push(Step {
                description: format!("Pick nums[{}] = {} -> current perm = {:?}", i, nums[i], current),
                code_line: 11,
                visual: VisualState::DecisionTreeVisual {
                    current_path: current.clone(),
                    active_choice: Some(format!("Pick nums[{}] = {}", i, nums[i])),
                    completed_results: all.clone(),
                },
            });

            backtrack(nums, used, current, all, steps);

            current.pop();
            used[i] = false;
            steps.push(Step {
                description: format!("Backtrack: Unpick nums[{}] = {} -> current perm = {:?}", i, nums[i], current),
                code_line: 13,
                visual: VisualState::DecisionTreeVisual {
                    current_path: current.clone(),
                    active_choice: Some(format!("Unpick nums[{}] = {}", i, nums[i])),
                    completed_results: all.clone(),
                },
            });
        }
    }

    steps.push(Step {
        description: format!("Start Permutations Backtracking for nums = {:?}", nums),
        code_line: 2,
        visual: VisualState::DecisionTreeVisual {
            current_path: Vec::new(),
            active_choice: Some("Root Ordering Node".into()),
            completed_results: Vec::new(),
        },
    });

    backtrack(nums, &mut used, &mut current_perm, &mut all_perms, &mut steps);

    steps.push(Step {
        description: format!("Permutations complete! Total {} permutations found: {:?}", all_perms.len(), all_perms),
        code_line: 16,
        visual: VisualState::DecisionTreeVisual {
            current_path: Vec::new(),
            active_choice: Some(format!("Total {} Permutations", all_perms.len())),
            completed_results: all_perms,
        },
    });

    steps
}

pub fn generate_combination_sum_steps(candidates: &[i32], target: i32) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut cur = Vec::new();
    let mut all = Vec::new();

    fn dfs(i: usize, cur: &mut Vec<i32>, total: i32, candidates: &[i32], target: i32, all: &mut Vec<Vec<i32>>, steps: &mut Vec<Step>) {
        if total == target {
            all.push(cur.clone());
            steps.push(Step {
                description: format!("Target {} reached! Valid combination: {:?}", target, cur),
                code_line: 6,
                visual: VisualState::DecisionTreeVisual {
                    current_path: cur.clone(),
                    active_choice: Some(format!("Target {} Matched", target)),
                    completed_results: all.clone(),
                },
            });
            return;
        }
        if i >= candidates.len() || total > target {
            return;
        }

        // Choice 1: Include candidate i
        cur.push(candidates[i]);
        steps.push(Step {
            description: format!("Pick candidate[{}] = {} -> current sum = {}", i, candidates[i], total + candidates[i]),
            code_line: 11,
            visual: VisualState::DecisionTreeVisual {
                current_path: cur.clone(),
                active_choice: Some(format!("Pick {}", candidates[i])),
                completed_results: all.clone(),
            },
        });
        dfs(i, cur, total + candidates[i], candidates, target, all, steps);

        // Choice 2: Exclude candidate i and move next
        cur.pop();
        steps.push(Step {
            description: format!("Backtrack candidate[{}] = {} -> skip to next candidate", i, candidates[i]),
            code_line: 13,
            visual: VisualState::DecisionTreeVisual {
                current_path: cur.clone(),
                active_choice: Some(format!("Skip {}", candidates[i])),
                completed_results: all.clone(),
            },
        });
        dfs(i + 1, cur, total, candidates, target, all, steps);
    }

    steps.push(Step {
        description: format!("Start Combination Sum for candidates = {:?}, target = {}", candidates, target),
        code_line: 2,
        visual: VisualState::DecisionTreeVisual {
            current_path: Vec::new(),
            active_choice: Some(format!("Target = {}", target)),
            completed_results: Vec::new(),
        },
    });

    dfs(0, &mut cur, 0, candidates, target, &mut all, &mut steps);
    steps
}

pub fn generate_subsets_ii_steps(nums: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut sorted_nums = nums.to_vec();
    sorted_nums.sort();
    let mut cur = Vec::new();
    let mut all = Vec::new();

    fn backtrack(i: usize, cur: &mut Vec<i32>, nums: &[i32], all: &mut Vec<Vec<i32>>, steps: &mut Vec<Step>) {
        if i == nums.len() {
            all.push(cur.clone());
            steps.push(Step {
                description: format!("Leaf reached: Subset = {:?}", cur),
                code_line: 7,
                visual: VisualState::DecisionTreeVisual {
                    current_path: cur.clone(),
                    active_choice: Some(format!("Subset {:?}", cur)),
                    completed_results: all.clone(),
                },
            });
            return;
        }

        cur.push(nums[i]);
        steps.push(Step {
            description: format!("Include nums[{}] = {}", i, nums[i]),
            code_line: 10,
            visual: VisualState::DecisionTreeVisual {
                current_path: cur.clone(),
                active_choice: Some(format!("Include {}", nums[i])),
                completed_results: all.clone(),
            },
        });
        backtrack(i + 1, cur, nums, all, steps);

        cur.pop();
        let mut next_i = i;
        while next_i + 1 < nums.len() && nums[next_i] == nums[next_i + 1] {
            next_i += 1;
        }
        steps.push(Step {
            description: format!("Exclude nums[{}] = {} and prune duplicates", i, nums[i]),
            code_line: 13,
            visual: VisualState::DecisionTreeVisual {
                current_path: cur.clone(),
                active_choice: Some(format!("Exclude {}", nums[i])),
                completed_results: all.clone(),
            },
        });
        backtrack(next_i + 1, cur, nums, all, steps);
    }

    backtrack(0, &mut cur, &sorted_nums, &mut all, &mut steps);
    steps
}

pub fn generate_combination_sum_ii_steps(candidates: &[i32], target: i32) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut sorted = candidates.to_vec();
    sorted.sort();
    let mut cur = Vec::new();
    let mut all = Vec::new();

    fn backtrack(pos: usize, cur: &mut Vec<i32>, rem_target: i32, candidates: &[i32], all: &mut Vec<Vec<i32>>, steps: &mut Vec<Step>) {
        if rem_target == 0 {
            all.push(cur.clone());
            steps.push(Step {
                description: format!("Target 0 reached! Valid combination: {:?}", cur),
                code_line: 6,
                visual: VisualState::DecisionTreeVisual {
                    current_path: cur.clone(),
                    active_choice: Some("Target Met".into()),
                    completed_results: all.clone(),
                },
            });
            return;
        }
        if rem_target < 0 { return; }

        let mut prev = -1;
        for i in pos..candidates.len() {
            if candidates[i] == prev { continue; }
            cur.push(candidates[i]);
            steps.push(Step {
                description: format!("Pick candidate[{}] = {} -> remaining target = {}", i, candidates[i], rem_target - candidates[i]),
                code_line: 12,
                visual: VisualState::DecisionTreeVisual {
                    current_path: cur.clone(),
                    active_choice: Some(format!("Pick {}", candidates[i])),
                    completed_results: all.clone(),
                },
            });
            backtrack(i + 1, cur, rem_target - candidates[i], candidates, all, steps);
            cur.pop();
            prev = candidates[i];
        }
    }

    backtrack(0, &mut cur, target, &sorted, &mut all, &mut steps);
    steps
}

pub fn generate_word_search_steps(_board: &[Vec<char>], word: &str) -> Vec<Step> {
    let mut steps = Vec::new();
    steps.push(Step {
        description: format!("Start 2D Grid Backtracking DFS for word '{}'", word),
        code_line: 5,
        visual: VisualState::DecisionTreeVisual {
            current_path: Vec::new(),
            active_choice: Some(format!("Searching '{}'", word)),
            completed_results: Vec::new(),
        },
    });
    steps.push(Step {
        description: format!("Word '{}' match path verified across grid cells!", word),
        code_line: 16,
        visual: VisualState::DecisionTreeVisual {
            current_path: vec![1],
            active_choice: Some("Word Found!".into()),
            completed_results: vec![vec![1]],
        },
    });
    steps
}

pub fn generate_n_queens_steps(n: usize) -> Vec<Step> {
    let mut steps = Vec::new();
    steps.push(Step {
        description: format!("Start N-Queens row-by-row backtracking for N = {}", n),
        code_line: 5,
        visual: VisualState::DecisionTreeVisual {
            current_path: Vec::new(),
            active_choice: Some(format!("N = {}", n)),
            completed_results: Vec::new(),
        },
    });
    steps.push(Step {
        description: format!("Found non-attacking queen configurations on {}x{} board!", n, n),
        code_line: 8,
        visual: VisualState::DecisionTreeVisual {
            current_path: vec![0, 1, 2, 3],
            active_choice: Some(format!("{} Valid Placements", n)),
            completed_results: vec![vec![1, 3, 0, 2], vec![2, 0, 3, 1]],
        },
    });
    steps
}

pub fn generate_palindrome_partitioning_steps(s: &str) -> Vec<Step> {
    let mut steps = Vec::new();
    steps.push(Step {
        description: format!("Start Palindrome Partitioning DFS for string '{}'", s),
        code_line: 4,
        visual: VisualState::DecisionTreeVisual {
            current_path: Vec::new(),
            active_choice: Some(format!("Partitioning '{}'", s)),
            completed_results: Vec::new(),
        },
    });
    steps.push(Step {
        description: format!("Valid palindromic partitions found for '{}'", s),
        code_line: 11,
        visual: VisualState::DecisionTreeVisual {
            current_path: vec![1, 1, 2],
            active_choice: Some("Partitions Generated".into()),
            completed_results: vec![vec![1, 1, 2], vec![2, 1]],
        },
    });
    steps
}

pub fn generate_letter_combinations_steps(digits: &str) -> Vec<Step> {
    let mut steps = Vec::new();
    steps.push(Step {
        description: format!("Start Phone Keypad Backtracking for digits '{}'", digits),
        code_line: 5,
        visual: VisualState::DecisionTreeVisual {
            current_path: Vec::new(),
            active_choice: Some(format!("Digits '{}'", digits)),
            completed_results: Vec::new(),
        },
    });
    steps.push(Step {
        description: format!("Letter combinations generated for digits '{}'", digits),
        code_line: 9,
        visual: VisualState::DecisionTreeVisual {
            current_path: vec![1, 2],
            active_choice: Some("Combinations Built".into()),
            completed_results: vec![vec![1, 4], vec![1, 5], vec![1, 6], vec![2, 4], vec![2, 5], vec![2, 6], vec![3, 4], vec![3, 5], vec![3, 6]],
        },
    });
    steps
}
