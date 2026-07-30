use crate::model::{Step, VisualState};

pub fn generate_container_water_steps(height: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();

    let char_repr: Vec<char> = height
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",")
        .chars()
        .collect();

    let mut l = 0usize;
    let mut r = height.len() - 1;
    let mut max_area = 0i32;

    steps.push(Step {
        code_line: 3,
        description: format!(
            "Container With Most Water: height = {:?}. l=0, r={}.",
            height, r
        ),
        visual: VisualState::TwoPointers {
            chars: char_repr.clone(),
            left: 0,
            right: char_repr.len().saturating_sub(1),
            is_valid: None,
            skipped: false,
        },
    });

    while l < r {
        let width = (r - l) as i32;
        let h = height[l].min(height[r]);
        let area = width * h;
        if area > max_area {
            max_area = area;
        }

        steps.push(Step {
            code_line: 5,
            description: format!(
                "l={}, r={}: min(h[{}]={}, h[{}]={}) * width {} = {} * {} = {}. maxArea = {}.",
                l, r, l, height[l], r, height[r], width, h, width, area, max_area
            ),
            visual: VisualState::TwoPointers {
                chars: char_repr.clone(),
                left: l,
                right: r,
                is_valid: None,
                skipped: false,
            },
        });

        if height[l] < height[r] {
            steps.push(Step {
                code_line: 7,
                description: format!(
                    "height[{}]={} < height[{}]={}. Move left pointer right: l={}.",
                    l,
                    height[l],
                    r,
                    height[r],
                    l + 1
                ),
                visual: VisualState::TwoPointers {
                    chars: char_repr.clone(),
                    left: l + 1,
                    right: r,
                    is_valid: None,
                    skipped: false,
                },
            });
            l += 1;
        } else {
            steps.push(Step {
                code_line: 8,
                description: format!(
                    "height[{}]={} >= height[{}]={}. Move right pointer left: r={}.",
                    l,
                    height[l],
                    r,
                    height[r],
                    r - 1
                ),
                visual: VisualState::TwoPointers {
                    chars: char_repr.clone(),
                    left: l,
                    right: r - 1,
                    is_valid: None,
                    skipped: false,
                },
            });
            r -= 1;
        }

        if steps.len() > 150 {
            break;
        }
    }

    steps.push(Step {
        code_line: 9,
        description: format!("Pointers crossed. Maximum water area = {}.", max_area),
        visual: VisualState::TwoPointers {
            chars: char_repr,
            left: l,
            right: r,
            is_valid: Some(true),
            skipped: false,
        },
    });

    steps
}
