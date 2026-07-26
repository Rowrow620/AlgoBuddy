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
