use crate::model::{Step, VisualState};

fn min_stack_visual(
    stack: &[i32],
    min_stack: &[i32],
    active_idx: Option<usize>,
    pointer: Option<&'static str>,
    status_message: String,
) -> VisualState {
    let pointers = active_idx
        .zip(pointer)
        .map(|(idx, label)| vec![(label, idx)])
        .unwrap_or_default();

    VisualState::Array1D {
        title: format!("Min Stack | minStack = {:?}", min_stack),
        elements: stack.to_vec(),
        active_idx,
        secondary_idx: None,
        pointers,
        status_message,
        is_success: None,
    }
}

pub fn generate_min_stack_steps(ops: &[(&str, Option<i32>)]) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut stack = Vec::new();
    let mut min_stack = Vec::new();

    steps.push(Step {
        code_line: 4,
        description: "Initialize MinStack: main stack = [], minStack = [].".to_string(),
        visual: min_stack_visual(
            &stack,
            &min_stack,
            None,
            None,
            "Both stacks are empty.".to_string(),
        ),
    });

    for (op, val) in ops {
        match *op {
            "push" => {
                let v = val.unwrap();
                stack.push(v);
                let current_min = min_stack.last().copied().map_or(v, |m: i32| m.min(v));
                min_stack.push(current_min);

                let description = format!(
                    "push({}): main stack = {:?}, minStack = {:?} (current min = {}).",
                    v, stack, min_stack, current_min
                );
                steps.push(Step {
                    code_line: 8,
                    description: description.clone(),
                    visual: min_stack_visual(
                        &stack,
                        &min_stack,
                        stack.len().checked_sub(1),
                        Some("top"),
                        description,
                    ),
                });
            }
            "pop" => {
                if !stack.is_empty() {
                    let popped = stack.pop().unwrap();
                    min_stack.pop();
                    let description = format!(
                        "pop(): popped {}. main stack = {:?}, minStack = {:?}.",
                        popped, stack, min_stack
                    );
                    steps.push(Step {
                        code_line: 10,
                        description: description.clone(),
                        visual: min_stack_visual(
                            &stack,
                            &min_stack,
                            stack.len().checked_sub(1),
                            Some("top"),
                            description,
                        ),
                    });
                }
            }
            "top" => {
                if let Some(&top_val) = stack.last() {
                    let description = format!("top(): returns {}.", top_val);
                    steps.push(Step {
                        code_line: 11,
                        description: description.clone(),
                        visual: min_stack_visual(
                            &stack,
                            &min_stack,
                            stack.len().checked_sub(1),
                            Some("top"),
                            description,
                        ),
                    });
                }
            }
            "getMin" => {
                if let Some(&min_val) = min_stack.last() {
                    let description = format!("getMin(): O(1) current minimum = {}.", min_val);
                    let min_idx = stack.iter().rposition(|&value| value == min_val);
                    steps.push(Step {
                        code_line: 12,
                        description: description.clone(),
                        visual: min_stack_visual(
                            &stack,
                            &min_stack,
                            min_idx,
                            Some("min"),
                            description,
                        ),
                    });
                }
            }
            _ => {}
        }
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_full_integer_values_and_uses_semantic_source_lines() {
        let steps = generate_min_stack_steps(&[
            ("push", Some(-12)),
            ("push", Some(30)),
            ("push", Some(-3)),
            ("getMin", None),
            ("pop", None),
            ("top", None),
            ("getMin", None),
        ]);

        assert_eq!(
            steps.iter().map(|step| step.code_line).collect::<Vec<_>>(),
            vec![4, 8, 8, 8, 12, 10, 11, 12]
        );

        for step in &steps {
            let VisualState::Array1D {
                elements,
                active_idx,
                title,
                ..
            } = &step.visual
            else {
                panic!("Min Stack must use its integer-native array view");
            };
            assert!(active_idx.is_none_or(|idx| idx < elements.len()));
            assert!(title.starts_with("Min Stack | minStack ="));
        }

        let VisualState::Array1D { elements, .. } = &steps[3].visual else {
            unreachable!();
        };
        assert_eq!(elements, &[-12, 30, -3]);
        assert!(steps[4].description.contains("current minimum = -12"));
        assert!(steps[7].description.contains("current minimum = -12"));
        assert_eq!(
            crate::model::Problem::MinStack.formula(),
            Some("min_stack.push(min(val, min_stack[-1] if min_stack else val))")
        );
    }
}
