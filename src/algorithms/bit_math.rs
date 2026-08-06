use crate::model::{Step, VisualState};

pub fn generate_number_1_bits_steps(n: u32) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut val = n;
    let mut count = 0;

    steps.push(Step {
        description: format!(
            "Initialize Hamming Weight calculation for n = {} (binary: {:032b})",
            n, n
        ),
        code_line: 3,
        visual: VisualState::ContainsDuplicate {
            nums: vec![val as i32, count],
            active_idx: Some(0),
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });

    while val != 0 {
        val &= val - 1;
        count += 1;
        steps.push(Step {
            description: format!("Clear lowest set bit using n &= (n - 1) -> new n = {} (binary: {:032b}), count = {}", val, val, count),
            code_line: 6,
            visual: VisualState::ContainsDuplicate {
                nums: vec![val as i32, count],
                active_idx: Some(1),
                seen_set: std::collections::BTreeSet::new(),
                duplicate_val: None,
                has_duplicate: None,
            },
        });
    }

    steps
}

pub fn generate_counting_bits_array_steps(n: usize) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut dp = vec![0i32; n + 1];

    steps.push(Step {
        code_line: 3,
        description: format!("Counting bits for range 0..={} using DP offsets.", n),
        visual: VisualState::ContainsDuplicate {
            nums: dp.clone(),
            active_idx: None,
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });

    let mut offset = 1;
    for i in 1..=n {
        if offset * 2 == i {
            offset = i;
        }
        dp[i] = 1 + dp[i - offset];
        steps.push(Step {
            code_line: 6,
            description: format!(
                "i={} (offset={}): dp[{}] = 1 + dp[{}] = 1 + {} = {}.",
                i,
                offset,
                i,
                i - offset,
                dp[i - offset],
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
        description: format!("Counting bits array for 0..={}: {:?}.", n, dp),
        visual: VisualState::ContainsDuplicate {
            nums: dp,
            active_idx: None,
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: Some(true),
        },
    });

    steps
}

pub fn generate_sum_two_integers_steps(a: i32, b: i32) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut cur_a = a;
    let mut cur_b = b;

    steps.push(Step {
        description: format!("Bitwise addition for a = {}, b = {} without + / -", a, b),
        code_line: 3,
        visual: VisualState::ContainsDuplicate {
            nums: vec![cur_a, cur_b],
            active_idx: None,
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });

    while cur_b != 0 {
        let carry = (cur_a & cur_b) << 1;
        cur_a ^= cur_b;
        cur_b = carry;

        steps.push(Step {
            description: format!(
                "Sum without carry a = a ^ b = {}, Carry b = (a & b) << 1 = {}",
                cur_a, cur_b
            ),
            code_line: 6,
            visual: VisualState::ContainsDuplicate {
                nums: vec![cur_a, cur_b],
                active_idx: Some(0),
                seen_set: std::collections::BTreeSet::new(),
                duplicate_val: None,
                has_duplicate: None,
            },
        });
    }

    steps
}

pub fn generate_reverse_integer_steps(x: i32) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut val = x;
    let mut res = 0i32;

    steps.push(Step {
        description: format!("Reverse digits for 32-bit signed integer x = {}", x),
        code_line: 3,
        visual: VisualState::ContainsDuplicate {
            nums: vec![val, res],
            active_idx: None,
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });

    while val != 0 {
        let pop = val % 10;
        val /= 10;
        if res > i32::MAX / 10 || (res == i32::MAX / 10 && pop > 7) {
            break;
        }
        if res < i32::MIN / 10 || (res == i32::MIN / 10 && pop < -8) {
            break;
        }
        res = res * 10 + pop;

        steps.push(Step {
            description: format!("Pop digit = {}, new reversed result = {}", pop, res),
            code_line: 7,
            visual: VisualState::ContainsDuplicate {
                nums: vec![val, res],
                active_idx: Some(1),
                seen_set: std::collections::BTreeSet::new(),
                duplicate_val: None,
                has_duplicate: None,
            },
        });
    }

    steps
}

pub fn generate_rotate_image_steps(matrix: &[Vec<i32>]) -> Vec<Step> {
    let mut steps = Vec::new();
    if matrix.is_empty() || matrix[0].is_empty() {
        return steps;
    }
    let mut mat = matrix.to_vec();
    let rows = mat.len();
    let cols = mat[0].len();
    let mut grid: Vec<Vec<String>> = mat
        .iter()
        .map(|row| row.iter().map(|v| v.to_string()).collect())
        .collect();

    steps.push(Step {
        description: format!(
            "Rotate {}x{} Image 90 degrees clockwise in-place",
            rows, cols
        ),
        code_line: 3,
        visual: VisualState::GridGraph {
            rows,
            cols,
            grid: grid.clone(),
            active_cell: None,
            visited_cells: std::collections::BTreeSet::new(),
            frontier_cells: std::collections::BTreeSet::new(),
            message: "Initial Matrix".into(),
        },
    });

    // Step 1: Transpose
    for i in 0..rows {
        for j in i + 1..cols {
            let tmp = mat[i][j];
            mat[i][j] = mat[j][i];
            mat[j][i] = tmp;
            grid[i][j] = mat[i][j].to_string();
            grid[j][i] = mat[j][i].to_string();

            steps.push(Step {
                description: format!(
                    "1. Transpose: Swap element ({}, {}) [{}] with ({}, {}) [{}]",
                    i, j, mat[j][i], j, i, mat[i][j]
                ),
                code_line: 7,
                visual: VisualState::GridGraph {
                    rows,
                    cols,
                    grid: grid.clone(),
                    active_cell: Some((i, j)),
                    visited_cells: std::collections::BTreeSet::new(),
                    frontier_cells: std::collections::BTreeSet::new(),
                    message: format!("Transposing element ({}, {})", i, j),
                },
            });
        }
    }

    // Step 2: Reverse rows
    for (r, row) in mat.iter_mut().enumerate() {
        row.reverse();
        grid[r] = row.iter().map(|v| v.to_string()).collect();
        steps.push(Step {
            description: format!("2. Reverse row {}: Reversed row elements -> {:?}", r, row),
            code_line: 12,
            visual: VisualState::GridGraph {
                rows,
                cols,
                grid: grid.clone(),
                active_cell: Some((r, cols / 2)),
                visited_cells: std::collections::BTreeSet::new(),
                frontier_cells: std::collections::BTreeSet::new(),
                message: format!("Reversed Row {}", r),
            },
        });
    }

    steps.push(Step {
        description: "90 degrees clockwise rotation complete!".into(),
        code_line: 14,
        visual: VisualState::GridGraph {
            rows,
            cols,
            grid,
            active_cell: None,
            visited_cells: std::collections::BTreeSet::new(),
            frontier_cells: std::collections::BTreeSet::new(),
            message: "Rotation Complete".into(),
        },
    });

    steps
}

pub fn generate_spiral_matrix_steps(matrix: &[Vec<i32>]) -> Vec<Step> {
    let mut steps = Vec::new();
    if matrix.is_empty() || matrix[0].is_empty() {
        return steps;
    }

    let rows = matrix.len();
    let cols = matrix[0].len();
    let grid: Vec<Vec<String>> = matrix
        .iter()
        .map(|row| row.iter().map(|v| v.to_string()).collect())
        .collect();
    let mut visited = std::collections::BTreeSet::new();

    let mut res = Vec::new();
    let mut top = 0;
    let mut bottom = rows as i32 - 1;
    let mut left = 0;
    let mut right = cols as i32 - 1;

    steps.push(Step {
        description: format!(
            "Initialize Spiral Matrix Traversal for {}x{} grid",
            rows, cols
        ),
        code_line: 3,
        visual: VisualState::GridGraph {
            rows,
            cols,
            grid: grid.clone(),
            active_cell: None,
            visited_cells: visited.clone(),
            frontier_cells: std::collections::BTreeSet::new(),
            message: "Starting Spiral Traversal".into(),
        },
    });

    while top <= bottom && left <= right {
        // Traverse Right
        for i in left..=right {
            let val = matrix[top as usize][i as usize];
            res.push(val);
            visited.insert((top as usize, i as usize));
            steps.push(Step {
                description: format!(
                    "Traverse Right along top row {}: Add element {} at ({}, {})",
                    top, val, top, i
                ),
                code_line: 5,
                visual: VisualState::GridGraph {
                    rows,
                    cols,
                    grid: grid.clone(),
                    active_cell: Some((top as usize, i as usize)),
                    visited_cells: visited.clone(),
                    frontier_cells: std::collections::BTreeSet::new(),
                    message: format!("Traversing Right: Output = {:?}", res),
                },
            });
        }
        top += 1;

        // Traverse Down
        for i in top..=bottom {
            let val = matrix[i as usize][right as usize];
            res.push(val);
            visited.insert((i as usize, right as usize));
            steps.push(Step {
                description: format!(
                    "Traverse Down along right column {}: Add element {} at ({}, {})",
                    right, val, i, right
                ),
                code_line: 8,
                visual: VisualState::GridGraph {
                    rows,
                    cols,
                    grid: grid.clone(),
                    active_cell: Some((i as usize, right as usize)),
                    visited_cells: visited.clone(),
                    frontier_cells: std::collections::BTreeSet::new(),
                    message: format!("Traversing Down: Output = {:?}", res),
                },
            });
        }
        right -= 1;

        if !(top <= bottom && left <= right) {
            break;
        }

        // Traverse Left
        for i in (left..=right).rev() {
            let val = matrix[bottom as usize][i as usize];
            res.push(val);
            visited.insert((bottom as usize, i as usize));
            steps.push(Step {
                description: format!(
                    "Traverse Left along bottom row {}: Add element {} at ({}, {})",
                    bottom, val, bottom, i
                ),
                code_line: 12,
                visual: VisualState::GridGraph {
                    rows,
                    cols,
                    grid: grid.clone(),
                    active_cell: Some((bottom as usize, i as usize)),
                    visited_cells: visited.clone(),
                    frontier_cells: std::collections::BTreeSet::new(),
                    message: format!("Traversing Left: Output = {:?}", res),
                },
            });
        }
        bottom -= 1;

        // Traverse Up
        for i in (top..=bottom).rev() {
            let val = matrix[i as usize][left as usize];
            res.push(val);
            visited.insert((i as usize, left as usize));
            steps.push(Step {
                description: format!(
                    "Traverse Up along left column {}: Add element {} at ({}, {})",
                    left, val, i, left
                ),
                code_line: 15,
                visual: VisualState::GridGraph {
                    rows,
                    cols,
                    grid: grid.clone(),
                    active_cell: Some((i as usize, left as usize)),
                    visited_cells: visited.clone(),
                    frontier_cells: std::collections::BTreeSet::new(),
                    message: format!("Traversing Up: Output = {:?}", res),
                },
            });
        }
        left += 1;
    }

    steps.push(Step {
        description: format!(
            "Spiral matrix traversal complete! Output sequence: {:?}",
            res
        ),
        code_line: 17,
        visual: VisualState::GridGraph {
            rows,
            cols,
            grid,
            active_cell: None,
            visited_cells: visited,
            frontier_cells: std::collections::BTreeSet::new(),
            message: format!("Spiral Traversal Complete: {:?}", res),
        },
    });

    steps
}

pub fn generate_set_matrix_zeroes_steps(matrix: &[Vec<i32>]) -> Vec<Step> {
    let mut steps = Vec::new();
    if matrix.is_empty() || matrix[0].is_empty() {
        return steps;
    }

    let mut mat = matrix.to_vec();
    let rows = mat.len();
    let cols = mat[0].len();
    let mut grid: Vec<Vec<String>> = mat
        .iter()
        .map(|row| row.iter().map(|v| v.to_string()).collect())
        .collect();
    let mut zero_rows = std::collections::BTreeSet::new();
    let mut zero_cols = std::collections::BTreeSet::new();

    steps.push(Step {
        description: format!(
            "Set Matrix Zeroes: Scan {}x{} matrix to identify rows & cols containing 0",
            rows, cols
        ),
        code_line: 3,
        visual: VisualState::GridGraph {
            rows,
            cols,
            grid: grid.clone(),
            active_cell: None,
            visited_cells: std::collections::BTreeSet::new(),
            frontier_cells: std::collections::BTreeSet::new(),
            message: "Scanning Matrix for Zeroes".into(),
        },
    });

    // Step 1: Scan for 0s
    for r in 0..rows {
        for c in 0..cols {
            if mat[r][c] == 0 {
                zero_rows.insert(r);
                zero_cols.insert(c);
                steps.push(Step {
                    description: format!(
                        "Found 0 at cell ({}, {}): Mark row {} and column {} to be zeroed",
                        r, c, r, c
                    ),
                    code_line: 6,
                    visual: VisualState::GridGraph {
                        rows,
                        cols,
                        grid: grid.clone(),
                        active_cell: Some((r, c)),
                        visited_cells: std::collections::BTreeSet::new(),
                        frontier_cells: std::collections::BTreeSet::new(),
                        message: format!("Marked row {} & col {} for zeroing", r, c),
                    },
                });
            }
        }
    }

    // Step 2: Zero out marked rows and cols
    for r in 0..rows {
        for c in 0..cols {
            if zero_rows.contains(&r) || zero_cols.contains(&c) {
                mat[r][c] = 0;
                grid[r][c] = "0".to_string();
            }
        }
    }

    steps.push(Step {
        description: format!(
            "Zeroed out marked rows {:?} and columns {:?}: Matrix update complete!",
            zero_rows, zero_cols
        ),
        code_line: 10,
        visual: VisualState::GridGraph {
            rows,
            cols,
            grid,
            active_cell: None,
            visited_cells: std::collections::BTreeSet::new(),
            frontier_cells: std::collections::BTreeSet::new(),
            message: "Matrix Zeroes Update Complete".into(),
        },
    });

    steps
}

pub fn generate_pow_xn_steps(x: f64, n: i32) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut cur_x = x;
    let mut cur_n = (n as i64).abs();
    let mut res = 1.0f64;

    steps.push(Step {
        description: format!(
            "Binary exponentiation: compute pow({:.2}, {}) using Divide & Conquer",
            x, n
        ),
        code_line: 3,
        visual: VisualState::ContainsDuplicate {
            nums: vec![x as i32, n, 1],
            active_idx: Some(0),
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });

    while cur_n > 0 {
        if cur_n % 2 == 1 {
            res *= cur_x;
            steps.push(Step {
                description: format!(
                    "N = {} is odd: Multiply result by x ({:.4}) -> new result = {:.4}",
                    cur_n, cur_x, res
                ),
                code_line: 5,
                visual: VisualState::ContainsDuplicate {
                    nums: vec![cur_x as i32, cur_n as i32, res as i32],
                    active_idx: Some(2),
                    seen_set: std::collections::BTreeSet::new(),
                    duplicate_val: Some(res as i32),
                    has_duplicate: None,
                },
            });
        }

        cur_x *= cur_x;
        cur_n /= 2;

        if cur_n > 0 {
            steps.push(Step {
                description: format!(
                    "Square base x -> {:.4}, Halve exponent N -> {}",
                    cur_x, cur_n
                ),
                code_line: 7,
                visual: VisualState::ContainsDuplicate {
                    nums: vec![cur_x as i32, cur_n as i32, res as i32],
                    active_idx: Some(0),
                    seen_set: std::collections::BTreeSet::new(),
                    duplicate_val: None,
                    has_duplicate: None,
                },
            });
        }
    }

    let final_res = if n < 0 { 1.0 / res } else { res };
    steps.push(Step {
        description: format!("Pow({:.2}, {}) complete! Result = {:.4}", x, n, final_res),
        code_line: 9,
        visual: VisualState::ContainsDuplicate {
            nums: vec![x as i32, n, final_res as i32],
            active_idx: None,
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: Some(final_res as i32),
            has_duplicate: Some(true),
        },
    });

    steps
}

pub fn generate_multiply_strings_steps(num1: &str, num2: &str) -> Vec<Step> {
    let mut steps = Vec::new();
    if num1 == "0" || num2 == "0" {
        steps.push(Step {
            description: format!("Multiplication of '{}' * '{}' = '0'", num1, num2),
            code_line: 3,
            visual: VisualState::ContainsDuplicate {
                nums: vec![0],
                active_idx: None,
                seen_set: std::collections::BTreeSet::new(),
                duplicate_val: None,
                has_duplicate: Some(true),
            },
        });
        return steps;
    }

    let n1 = num1.as_bytes();
    let n2 = num2.as_bytes();
    let mut pos = vec![0i32; n1.len() + n2.len()];

    steps.push(Step {
        description: format!(
            "Positional digit multiplication for '{}' * '{}': initialize result array of len {}",
            num1,
            num2,
            pos.len()
        ),
        code_line: 4,
        visual: VisualState::ContainsDuplicate {
            nums: pos.clone(),
            active_idx: None,
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });

    for i in (0..n1.len()).rev() {
        for j in (0..n2.len()).rev() {
            let mul = ((n1[i] - b'0') * (n2[j] - b'0')) as i32;
            let p1 = i + j;
            let p2 = i + j + 1;
            let sum = mul + pos[p2];

            pos[p2] = sum % 10;
            pos[p1] += sum / 10;

            steps.push(Step {
                description: format!("Multiply digit '{}' at [{}] by '{}' at [{}] = {}. Add to pos[{}]: new digit = {}, carry to pos[{}] = {}", 
                    n1[i] as char, i, n2[j] as char, j, mul, p2, pos[p2], p1, pos[p1]),
                code_line: 7,
                visual: VisualState::ContainsDuplicate {
                    nums: pos.clone(),
                    active_idx: Some(p2),
                    seen_set: std::collections::BTreeSet::new(),
                    duplicate_val: Some(pos[p2]),
                    has_duplicate: None,
                },
            });
        }
    }

    let result_str: String = pos
        .iter()
        .skip_while(|&&x| x == 0)
        .map(|x| x.to_string())
        .collect();

    steps.push(Step {
        description: format!(
            "Multiplication complete! '{}' * '{}' = '{}'",
            num1, num2, result_str
        ),
        code_line: 11,
        visual: VisualState::ContainsDuplicate {
            nums: pos,
            active_idx: None,
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: Some(true),
        },
    });

    steps
}

pub fn generate_detect_squares_steps() -> Vec<Step> {
    let mut steps = Vec::new();

    steps.push(Step {
        description: "Detect Squares: Initialize Frequency Map for 2D Cartesian Points".into(),
        code_line: 2,
        visual: VisualState::ContainsDuplicate {
            nums: vec![],
            active_idx: None,
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });

    // Add point 1
    steps.push(Step {
        description: "add([3, 10]): Record point (3, 10) in frequency map".into(),
        code_line: 4,
        visual: VisualState::ContainsDuplicate {
            nums: vec![3, 10],
            active_idx: Some(0),
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });

    // Add point 2
    steps.push(Step {
        description: "add([11, 2]): Record point (11, 2) in frequency map".into(),
        code_line: 4,
        visual: VisualState::ContainsDuplicate {
            nums: vec![3, 10, 11, 2],
            active_idx: Some(2),
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });

    // Add point 3
    steps.push(Step {
        description: "add([3, 2]): Record point (3, 2) in frequency map".into(),
        code_line: 4,
        visual: VisualState::ContainsDuplicate {
            nums: vec![3, 10, 11, 2, 3, 2],
            active_idx: Some(4),
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });

    // Count query
    steps.push(Step {
        description: "count([11, 10]): Query squares formed with corner (11, 10). Check diagonal (3, 2) and sides (3, 10), (11, 2)".into(),
        code_line: 7,
        visual: VisualState::ContainsDuplicate {
            nums: vec![11, 10, 3, 2, 3, 10, 11, 2],
            active_idx: Some(0),
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: Some(1),
            has_duplicate: None,
        },
    });

    steps.push(Step {
        description: "Formed 1 valid axis-aligned square with side length 8! Total count = 1"
            .into(),
        code_line: 10,
        visual: VisualState::ContainsDuplicate {
            nums: vec![1],
            active_idx: None,
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: Some(1),
            has_duplicate: Some(true),
        },
    });

    steps
}
