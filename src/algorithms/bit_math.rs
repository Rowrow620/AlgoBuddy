use crate::model::{Step, VisualState};

pub fn generate_number_1_bits_steps(n: u32) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut val = n;
    let mut count = 0;

    steps.push(Step {
        description: format!("Initialize Hamming Weight calculation for n = {} (binary: {:032b})", n, n),
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
            description: format!("Sum without carry a = a ^ b = {}, Carry b = (a & b) << 1 = {}", cur_a, cur_b),
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
        if res > i32::MAX / 10 || (res == i32::MAX / 10 && pop > 7) { break; }
        if res < i32::MIN / 10 || (res == i32::MIN / 10 && pop < -8) { break; }
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
    let mut mat = matrix.to_vec();
    let n = mat.len();

    steps.push(Step {
        description: format!("Rotate {}x{} Image 90 degrees clockwise in-place", n, n),
        code_line: 3,
        visual: VisualState::ContainsDuplicate {
            nums: mat.iter().flatten().copied().collect(),
            active_idx: None,
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });

    // Step 1: Transpose
    for i in 0..n {
        for j in i + 1..n {
            let tmp = mat[i][j];
            mat[i][j] = mat[j][i];
            mat[j][i] = tmp;
        }
    }
    steps.push(Step {
        description: "1. Transpose matrix across primary diagonal".into(),
        code_line: 7,
        visual: VisualState::ContainsDuplicate {
            nums: mat.iter().flatten().copied().collect(),
            active_idx: Some(0),
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });

    // Step 2: Reverse rows
    for row in mat.iter_mut() {
        row.reverse();
    }
    steps.push(Step {
        description: "2. Reverse elements of each row -> 90 degrees clockwise rotation complete!".into(),
        code_line: 12,
        visual: VisualState::ContainsDuplicate {
            nums: mat.iter().flatten().copied().collect(),
            active_idx: Some(0),
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });

    steps
}

pub fn generate_spiral_matrix_steps(matrix: &[Vec<i32>]) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut res = Vec::new();
    if matrix.is_empty() { return steps; }

    let mut top = 0;
    let mut bottom = matrix.len() as i32 - 1;
    let mut left = 0;
    let mut right = matrix[0].len() as i32 - 1;

    while top <= bottom && left <= right {
        for i in left..=right { res.push(matrix[top as usize][i as usize]); }
        top += 1;

        for i in top..=bottom { res.push(matrix[i as usize][right as usize]); }
        right -= 1;

        if !(top <= bottom && left <= right) { break; }

        for i in (left..=right).rev() { res.push(matrix[bottom as usize][i as usize]); }
        bottom -= 1;

        for i in (top..=bottom).rev() { res.push(matrix[i as usize][left as usize]); }
        left += 1;
    }

    steps.push(Step {
        description: format!("Spiral matrix traversal complete! Output: {:?}", res),
        code_line: 15,
        visual: VisualState::ContainsDuplicate {
            nums: res,
            active_idx: None,
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });

    steps
}

pub fn generate_set_matrix_zeroes_steps(matrix: &[Vec<i32>]) -> Vec<Step> {
    let mut steps = Vec::new();
    let mat = matrix.to_vec();

    steps.push(Step {
        description: format!("Set Matrix Zeroes: Scan zeroes and update row/col flags in-place"),
        code_line: 4,
        visual: VisualState::ContainsDuplicate {
            nums: mat.iter().flatten().copied().collect(),
            active_idx: None,
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });

    steps
}

pub fn generate_pow_xn_steps(x: f64, n: i32) -> Vec<Step> {
    let mut steps = Vec::new();
    steps.push(Step {
        description: format!("Binary exponentiation pow({:.2}, {}) using divide & conquer", x, n),
        code_line: 4,
        visual: VisualState::ContainsDuplicate {
            nums: vec![x as i32, n],
            active_idx: None,
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });
    steps
}

pub fn generate_multiply_strings_steps(num1: &str, num2: &str) -> Vec<Step> {
    let mut steps = Vec::new();
    steps.push(Step {
        description: format!("Positional digit multiplication for '{}' * '{}'", num1, num2),
        code_line: 5,
        visual: VisualState::ContainsDuplicate {
            nums: vec![num1.len() as i32, num2.len() as i32],
            active_idx: None,
            seen_set: std::collections::BTreeSet::new(),
            duplicate_val: None,
            has_duplicate: None,
        },
    });
    steps
}

pub fn generate_detect_squares_steps() -> Vec<Step> {
    let mut steps = Vec::new();
    steps.push(Step {
        description: "Detect Squares: Store point frequencies and count diagonal square matches".into(),
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
