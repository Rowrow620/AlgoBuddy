use crate::model::{Step, VisualState};

pub fn generate_find_median_sorted_arrays_steps(nums1: &[i32], nums2: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();

    // Ensure A is smaller array
    let (a, b) = if nums1.len() <= nums2.len() {
        (nums1.to_vec(), nums2.to_vec())
    } else {
        (nums2.to_vec(), nums1.to_vec())
    };

    let total = a.len() + b.len();
    let half = (total + 1) / 2;

    steps.push(Step {
        code_line: 3,
        description: format!("Median of Two Sorted Arrays: A = {:?}, B = {:?}. Total length = {}, half partition size = {}.", a, b, total, half),
        visual: VisualState::BinarySearch {
            nums: a.clone(),
            target: 0,
            left: 0,
            right: a.len(),
            mid: None,
            found_idx: None,
        },
    });

    let mut l = 0isize;
    let mut r = a.len() as isize;

    while l <= r {
        let i = (l + (r - l) / 2) as usize;
        let j = half.saturating_sub(i);

        let a_left = if i > 0 { a[i - 1] } else { i32::MIN };
        let a_right = if i < a.len() { a[i] } else { i32::MAX };
        let b_left = if j > 0 { b[j - 1] } else { i32::MIN };
        let b_right = if j < b.len() { b[j] } else { i32::MAX };

        steps.push(Step {
            code_line: 6,
            description: format!("Partition A at idx {} (left max={}, right min={}), B at idx {} (left max={}, right min={}).",
                i, if a_left == i32::MIN { " -inf".to_string() } else { a_left.to_string() },
                if a_right == i32::MAX { "inf".to_string() } else { a_right.to_string() },
                j, if b_left == i32::MIN { " -inf".to_string() } else { b_left.to_string() },
                if b_right == i32::MAX { "inf".to_string() } else { b_right.to_string() }),
            visual: VisualState::BinarySearch {
                nums: a.clone(),
                target: 0,
                left: l as usize,
                right: r as usize,
                mid: Some(i),
                found_idx: None,
            },
        });

        if a_left <= b_right && b_left <= a_right {
            let median = if total % 2 == 1 {
                a_left.max(b_left) as f64
            } else {
                (a_left.max(b_left) + a_right.min(b_right)) as f64 / 2.0
            };

            steps.push(Step {
                code_line: 10,
                description: format!("Valid partition found! A_left ({}) <= B_right ({}) and B_left ({}) <= A_right ({}). Median = {:.2}.",
                    a_left, b_right, b_left, a_right, median),
                visual: VisualState::BinarySearch {
                    nums: a.clone(),
                    target: 0,
                    left: 0,
                    right: a.len(),
                    mid: Some(i),
                    found_idx: Some(i),
                },
            });
            return steps;
        } else if a_left > b_right {
            steps.push(Step {
                code_line: 12,
                description: format!(
                    "A_left ({}) > B_right ({}) -> Too many elements from A. Move r to {}.",
                    a_left,
                    b_right,
                    i - 1
                ),
                visual: VisualState::BinarySearch {
                    nums: a.clone(),
                    target: 0,
                    left: l as usize,
                    right: (i - 1).max(0),
                    mid: Some(i),
                    found_idx: None,
                },
            });
            r = i as isize - 1;
        } else {
            steps.push(Step {
                code_line: 14,
                description: format!(
                    "B_left ({}) > A_right ({}) -> Too few elements from A. Move l to {}.",
                    b_left,
                    a_right,
                    i + 1
                ),
                visual: VisualState::BinarySearch {
                    nums: a.clone(),
                    target: 0,
                    left: i + 1,
                    right: r as usize,
                    mid: Some(i),
                    found_idx: None,
                },
            });
            l = i as isize + 1;
        }
    }

    steps
}
