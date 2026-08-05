use super::*;

#[test]
fn test_all_problems_recompute_steps() {
    let mut app = VisualizerApp::default();
    let all_problems = Problem::all();

    // Verify the number of implemented problems dynamically
    assert_eq!(
        all_problems.len(),
        150,
        "Expected 150 problems in Problem::all()!"
    );

    let mut failed_problems = Vec::new();

    for &problem in all_problems {
        app.current_problem = problem;
        // Test primary approach (0) and secondary approach (1)
        for approach_id in 0..=1 {
            app.selected_approach_id = approach_id;
            app.recompute_steps();

            if app.steps.is_empty() {
                failed_problems.push(format!(
                    "{:?} (approach {}): steps vector is empty",
                    problem, approach_id
                ));
                continue;
            }

            for (idx, step) in app.steps.iter().enumerate() {
                if step.description.trim().is_empty() {
                    failed_problems.push(format!(
                        "{:?} (approach {}, step {}): empty description",
                        problem, approach_id, idx
                    ));
                }
            }
        }
    }

    if !failed_problems.is_empty() {
        panic!(
            "Problem step generation failed for {} problem/approach combinations:\n{}",
            failed_problems.len(),
            failed_problems.join("\n")
        );
    }
}

#[test]
fn test_two_sum_logic_correctness() {
    let mut app = VisualizerApp::default();
    app.current_problem = Problem::TwoSum;
    app.set_input_str(Problem::TwoSum, "nums", "2, 7, 11, 15");
    app.set_input_int(Problem::TwoSum, "target", 9);
    app.selected_approach_id = 0; // Hash map
    app.recompute_steps();

    let last_step = app.steps.last().expect("Steps should not be empty");
    if let VisualState::TwoSum { found_indices, .. } = &last_step.visual {
        assert_eq!(
            *found_indices,
            Some((0, 1)),
            "Two Sum failed to find solution pair (0, 1)"
        );
    } else {
        panic!("Expected VisualState::TwoSum");
    }
}

#[test]
fn test_contains_duplicate_logic_correctness() {
    let mut app = VisualizerApp::default();
    app.current_problem = Problem::ContainsDuplicate;
    app.set_input_str(Problem::ContainsDuplicate, "nums", "1, 2, 3, 1");
    app.recompute_steps();

    let last_step = app.steps.last().expect("Steps should not be empty");
    if let VisualState::ContainsDuplicate {
        has_duplicate,
        duplicate_val,
        ..
    } = &last_step.visual
    {
        assert_eq!(
            *has_duplicate,
            Some(true),
            "Contains Duplicate should return true for [1, 2, 3, 1]"
        );
        assert_eq!(*duplicate_val, Some(1), "Duplicate value should be 1");
    } else {
        panic!("Expected VisualState::ContainsDuplicate");
    }
}

#[test]
fn test_valid_anagram_logic_correctness() {
    let mut app = VisualizerApp::default();
    app.current_problem = Problem::ValidAnagram;
    app.set_input_str(Problem::ValidAnagram, "s", "anagram");
    app.set_input_str(Problem::ValidAnagram, "t", "nagaram");
    app.recompute_steps();

    let last_step = app.steps.last().expect("Steps should not be empty");
    assert!(
        last_step.description.contains("VALID")
            || last_step.description.contains("true")
            || last_step.description.contains("match"),
        "Valid Anagram description should indicate a valid anagram match"
    );
}

#[test]
fn test_valid_parentheses_logic_correctness() {
    let mut app = VisualizerApp::default();
    app.current_problem = Problem::ValidParentheses;
    app.set_input_str(Problem::ValidParentheses, "s", "()[]{}");
    app.recompute_steps();

    let last_step = app.steps.last().expect("Steps should not be empty");

    if let VisualState::Stack { is_valid, .. } = &last_step.visual {
        assert_eq!(
            *is_valid,
            Some(true),
            "Valid Parentheses should return true for balanced brackets"
        );
    } else {
        panic!("Expected VisualState::Stack");
    }
}

#[test]
fn test_boundary_and_edge_case_safety() {
    let mut app = VisualizerApp::default();

    // Edge Case 1: Empty input string fallback for Contains Duplicate
    app.current_problem = Problem::ContainsDuplicate;
    app.set_input_str(Problem::ContainsDuplicate, "nums", "");
    app.recompute_steps();
    assert!(
        !app.steps.is_empty(),
        "Contains Duplicate failed on empty input string"
    );

    // Edge Case 2: Target not found for Two Sum
    app.current_problem = Problem::TwoSum;
    app.set_input_str(Problem::TwoSum, "nums", "1, 2, 3");
    app.set_input_int(Problem::TwoSum, "target", 999);
    app.recompute_steps();
    assert!(
        !app.steps.is_empty(),
        "Two Sum failed on non-existent target"
    );

    // Edge Case 3: Empty string input for Valid Anagram
    app.current_problem = Problem::ValidAnagram;
    app.set_input_str(Problem::ValidAnagram, "s", "");
    app.set_input_str(Problem::ValidAnagram, "t", "");
    app.recompute_steps();
    assert!(
        !app.steps.is_empty(),
        "Valid Anagram failed on empty string input"
    );

    // Edge Case 4: Length/Content mismatch for Valid Anagram
    app.set_input_str(Problem::ValidAnagram, "s", "rat");
    app.set_input_str(Problem::ValidAnagram, "t", "car");
    app.recompute_steps();
    assert!(
        !app.steps.is_empty(),
        "Valid Anagram failed on mismatch input"
    );
}

#[test]
fn test_best_time_stock_logic_correctness() {
    let mut app = VisualizerApp::default();
    app.current_problem = Problem::BestTimeStock;
    app.set_input_str(Problem::BestTimeStock, "prices", "7, 1, 5, 3, 6, 4");
    app.recompute_steps();

    let last_step = app.steps.last().expect("Steps should not be empty");

    if let VisualState::BestTimeStock { max_profit, .. } = &last_step.visual {
        assert_eq!(
            *max_profit, 5,
            "Maximum profit should be 5 for prices [7, 1, 5, 3, 6, 4]"
        );
    } else {
        panic!("Expected VisualState::BestTimeStock");
    }
}
