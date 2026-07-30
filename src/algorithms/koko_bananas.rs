use crate::model::{Step, VisualState};

pub fn generate_koko_eating_bananas_steps(piles: &[i32], h: i32) -> Vec<Step> {
    let mut steps = Vec::new();
    let piles_vec = piles.to_vec();

    if piles.is_empty() {
        return steps;
    }

    let max_pile = *piles.iter().max().unwrap_or(&1);
    let mut l = 1i32;
    let mut r = max_pile;
    let mut res = max_pile;

    // Step 1: Initial state
    steps.push(Step {
        code_line: 3,
        description: format!(
            "Koko Eating Bananas: piles = {:?}, h = {} hours. Search speed k range [1..{}] (max pile).",
            piles_vec, h, max_pile
        ),
        visual: VisualState::Array1D {
            title: format!("Piles of Bananas (Target Time h = {} hours)", h),
            elements: piles_vec.clone(),
            active_idx: None,
            secondary_idx: None,
            pointers: vec![("l (min speed)", l as usize), ("r (max speed)", r as usize)],
            status_message: format!("Searching min speed k in range [{}, {}]. Target h = {} hours.", l, r, h),
            is_success: None,
        },
    });

    while l <= r {
        let k = l + (r - l) / 2;
        let mut total_hours: i64 = 0;
        for &p in piles {
            total_hours += ((p as i64) + (k as i64) - 1) / (k as i64);
        }

        let is_valid = total_hours <= (h as i64);

        steps.push(Step {
            code_line: 5,
            description: format!(
                "Testing candidate speed k = {} bananas/hr. Total hours required: {} hours (limit h = {}).",
                k, total_hours, h
            ),
            visual: VisualState::Array1D {
                title: format!("Testing Speed k = {} bananas/hr", k),
                elements: piles_vec.clone(),
                active_idx: None,
                secondary_idx: None,
                pointers: vec![
                    ("l", l as usize),
                    ("r", r as usize),
                    ("k (mid)", k as usize),
                ],
                status_message: format!(
                    "Speed k = {} -> Total Hours = {} / {} max hours allowed",
                    k, total_hours, h
                ),
                is_success: Some(is_valid),
            },
        });

        if is_valid {
            res = res.min(k);
            steps.push(Step {
                code_line: 7,
                description: format!(
                    "Total hours ({} hrs) <= h ({} hrs). Speed k = {} is FEASIBLE! Record candidate answer res = {}. Try slower speed (r = k - 1 = {}).",
                    total_hours, h, k, res, k - 1
                ),
                visual: VisualState::Array1D {
                    title: format!("Speed k = {} is Valid (hours = {} <= h = {})", k, total_hours, h),
                    elements: piles_vec.clone(),
                    active_idx: None,
                    secondary_idx: None,
                    pointers: vec![
                        ("l", l as usize),
                        ("r", (k - 1).max(0) as usize),
                    ],
                    status_message: format!("Feasible speed! Updated best answer res = {}. Narrowing right bound r to {}.", res, k - 1),
                    is_success: Some(true),
                },
            });
            r = k - 1;
        } else {
            steps.push(Step {
                code_line: 10,
                description: format!(
                    "Total hours ({} hrs) > h ({} hrs). Speed k = {} is TOO SLOW! Must eat faster. Narrowing left bound (l = k + 1 = {}).",
                    total_hours, h, k, k + 1
                ),
                visual: VisualState::Array1D {
                    title: format!("Speed k = {} is Too Slow (hours = {} > h = {})", k, total_hours, h),
                    elements: piles_vec.clone(),
                    active_idx: None,
                    secondary_idx: None,
                    pointers: vec![
                        ("l", (k + 1) as usize),
                        ("r", r as usize),
                    ],
                    status_message: format!("Too slow! Increasing min speed left bound l to {}.", k + 1),
                    is_success: Some(false),
                },
            });
            l = k + 1;
        }
    }

    steps.push(Step {
        code_line: 12,
        description: format!(
            "Binary search completed. Minimum eating speed k to finish within {} hours is k = {}.",
            h, res
        ),
        visual: VisualState::Array1D {
            title: format!("Minimum Speed Found: k = {} bananas/hr", res),
            elements: piles_vec.clone(),
            active_idx: None,
            secondary_idx: None,
            pointers: vec![("min speed k", res as usize)],
            status_message: format!(
                "Optimal minimum speed k = {} bananas/hr for piles {:?} and h = {}.",
                res, piles_vec, h
            ),
            is_success: Some(true),
        },
    });

    steps
}
