use crate::model::{Step, VisualState};

pub fn generate_search_2d_matrix_steps(matrix: &[Vec<i32>], target: i32) -> Vec<Step> {
    let mut steps = Vec::new();
    let rows = matrix.len();
    if rows == 0 || matrix[0].is_empty() { return steps; }
    let cols = matrix[0].len();

    let flat: Vec<i32> = matrix.iter().flatten().copied().collect();

    steps.push(Step {
        code_line: 3,
        description: format!("Binary Search on {}x{} matrix for target {}. Virtual 1D array length = {}.", rows, cols, target, flat.len()),
        visual: VisualState::BinarySearch {
            nums: flat.clone(),
            target,
            left: 0,
            right: flat.len().saturating_sub(1),
            mid: None,
            found_idx: None,
        },
    });

    let mut l = 0isize;
    let mut r = (rows * cols - 1) as isize;

    while l <= r {
        let mid = l + (r - l) / 2;
        let mid_idx = mid as usize;
        let row = mid_idx / cols;
        let col = mid_idx % cols;
        let val = matrix[row][col];

        steps.push(Step {
            code_line: 5,
            description: format!("l={}, r={}: mid index {} maps to matrix[{}][{}] = {}. Target = {}.", l, r, mid_idx, row, col, val, target),
            visual: VisualState::BinarySearch {
                nums: flat.clone(),
                target,
                left: l as usize,
                right: r as usize,
                mid: Some(mid_idx),
                found_idx: None,
            },
        });

        if val == target {
            steps.push(Step {
                code_line: 6,
                description: format!("Found target {} at matrix[{}][{}] (flat index {})! Return True.", target, row, col, mid_idx),
                visual: VisualState::BinarySearch {
                    nums: flat,
                    target,
                    left: l as usize,
                    right: r as usize,
                    mid: Some(mid_idx),
                    found_idx: Some(mid_idx),
                },
            });
            return steps;
        } else if val < target {
            steps.push(Step {
                code_line: 7,
                description: format!("matrix[{}][{}] = {} < target {}. Search right half: l = {}.", row, col, val, target, mid + 1),
                visual: VisualState::BinarySearch {
                    nums: flat.clone(),
                    target,
                    left: (mid + 1) as usize,
                    right: r as usize,
                    mid: Some(mid_idx),
                    found_idx: None,
                },
            });
            l = mid + 1;
        } else {
            steps.push(Step {
                code_line: 8,
                description: format!("matrix[{}][{}] = {} > target {}. Search left half: r = {}.", row, col, val, target, mid - 1),
                visual: VisualState::BinarySearch {
                    nums: flat.clone(),
                    target,
                    left: l as usize,
                    right: (mid - 1).max(0) as usize,
                    mid: Some(mid_idx),
                    found_idx: None,
                },
            });
            r = mid - 1;
        }
    }

    steps.push(Step {
        code_line: 9,
        description: format!("Target {} not found in matrix. Return False.", target),
        visual: VisualState::BinarySearch {
            nums: flat,
            target,
            left: 0,
            right: 0,
            mid: None,
            found_idx: None,
        },
    });

    steps
}
