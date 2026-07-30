use crate::model::{Step, VisualState};

pub fn generate_car_fleet_steps(target: i32, position: &[i32], speed: &[i32]) -> Vec<Step> {
    let mut steps = Vec::new();
    let n = position.len();

    if n == 0 {
        steps.push(Step {
            code_line: 3,
            description: "No cars. Total fleets = 0.".to_string(),
            visual: VisualState::Stack {
                chars: vec![],
                active_idx: None,
                stack: vec![],
                is_valid: Some(true),
            },
        });
        return steps;
    }

    let mut cars: Vec<(i32, f64)> = position
        .iter()
        .zip(speed.iter())
        .map(|(&pos, &spd)| (pos, (target - pos) as f64 / spd as f64))
        .collect();

    cars.sort_by_key(|c| c.0); // sort by position ascending

    let char_repr: Vec<char> = cars
        .iter()
        .map(|c| c.0.to_string().chars().next().unwrap_or(' '))
        .collect();

    steps.push(Step {
        code_line: 3,
        description: format!(
            "Car Fleet: target = {}. Sorted cars (pos, time_to_target): {:?}",
            target, cars
        ),
        visual: VisualState::Stack {
            chars: char_repr.clone(),
            active_idx: None,
            stack: vec![],
            is_valid: None,
        },
    });

    let mut stack: Vec<f64> = Vec::new();

    for (i, &(pos, time)) in cars.iter().enumerate().rev() {
        stack.push(time);

        let stack_chars: Vec<char> = stack
            .iter()
            .map(|t| format!("{:.1}", t).chars().next().unwrap_or(' '))
            .collect();
        steps.push(Step {
            code_line: 6,
            description: format!(
                "Examining car at pos {} (takes {:.2} hrs to reach target). Push time to stack.",
                pos, time
            ),
            visual: VisualState::Stack {
                chars: char_repr.clone(),
                active_idx: Some(i),
                stack: stack_chars,
                is_valid: None,
            },
        });

        if stack.len() >= 2 {
            let last = stack[stack.len() - 1];
            let prev = stack[stack.len() - 2];
            if last <= prev {
                stack.pop(); // caught up to previous fleet!
                let fleet_chars: Vec<char> = stack
                    .iter()
                    .map(|t| format!("{:.1}", t).chars().next().unwrap_or(' '))
                    .collect();
                steps.push(Step {
                    code_line: 8,
                    description: format!("Car at pos {} ({:.2} hrs) catches up to car ahead ({:.2} hrs). Merges into fleet!", pos, last, prev),
                    visual: VisualState::Stack {
                        chars: char_repr.clone(),
                        active_idx: Some(i),
                        stack: fleet_chars,
                        is_valid: None,
                    },
                });
            }
        }
    }

    let fleet_count = stack.len();
    steps.push(Step {
        code_line: 9,
        description: format!(
            "Car Fleet evaluation complete! Total car fleets reaching target = {}.",
            fleet_count
        ),
        visual: VisualState::Stack {
            chars: char_repr,
            active_idx: None,
            stack: vec![],
            is_valid: Some(true),
        },
    });

    steps
}
