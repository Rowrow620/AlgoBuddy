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
        for approach in problem.details().approaches {
            app.selected_approach_id = approach.id;
            app.recompute_steps();

            if app.steps.is_empty() {
                failed_problems.push(format!(
                    "{:?} (approach {}): steps vector is empty",
                    problem, approach.id
                ));
                continue;
            }

            for (idx, step) in app.steps.iter().enumerate() {
                if step.description.trim().is_empty() {
                    failed_problems.push(format!(
                        "{:?} (approach {}, step {}): empty description",
                        problem, approach.id, idx
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

#[test]
fn canvas_zoom_helpers_adjust_reset_and_clamp() {
    let mut zoom = CANVAS_ZOOM_DEFAULT;

    zoom = canvas_zoom_in(zoom);
    assert!((zoom - 1.1).abs() < f32::EPSILON);

    zoom = CANVAS_ZOOM_DEFAULT;
    assert_eq!(zoom, CANVAS_ZOOM_DEFAULT);

    zoom = canvas_zoom_out(zoom);
    assert!((zoom - 0.9).abs() < f32::EPSILON);

    for _ in 0..100 {
        zoom = canvas_zoom_out(zoom);
    }
    assert_eq!(zoom, CANVAS_ZOOM_MIN);

    for _ in 0..100 {
        zoom = canvas_zoom_in(zoom);
    }
    assert_eq!(zoom, CANVAS_ZOOM_MAX);
}

#[test]
fn shortcut_actions_respect_playback_speed_bounds() {
    let mut app = VisualizerApp::default();

    app.playback_speed_ms = PLAYBACK_SPEED_MIN_MS;
    app.perform_shortcut_action(ShortcutAction::SpeedUp);
    assert_eq!(app.playback_speed_ms, PLAYBACK_SPEED_MIN_MS);

    app.playback_speed_ms = PLAYBACK_SPEED_MAX_MS;
    app.perform_shortcut_action(ShortcutAction::SpeedDown);
    assert_eq!(app.playback_speed_ms, PLAYBACK_SPEED_MAX_MS);
}

#[test]
fn pausing_at_the_final_step_does_not_restart_the_timeline() {
    let mut app = VisualizerApp::default();
    let final_step = app.steps.len().saturating_sub(1);
    app.current_step_idx = final_step;
    app.is_playing = true;

    app.perform_shortcut_action(ShortcutAction::PlayPause);
    assert!(!app.is_playing);
    assert_eq!(app.current_step_idx, final_step);

    app.perform_shortcut_action(ShortcutAction::PlayPause);
    assert!(app.is_playing);
    assert_eq!(app.current_step_idx, 0);
}

#[test]
fn shortcut_capture_rebinds_and_clears_its_transient_state() {
    let mut app = VisualizerApp::default();
    app.shortcut_capture = Some(ShortcutAction::PlayPause);

    app.apply_shortcut_capture_key(egui::Key::P, egui::Modifiers::NONE, false);

    assert_eq!(
        app.shortcut_bindings.action_for_key(egui::Key::P),
        Some(ShortcutAction::PlayPause)
    );
    assert_eq!(app.shortcut_bindings.action_for_key(egui::Key::Space), None);
    assert_eq!(app.shortcut_capture, None);
    assert_eq!(app.shortcut_rebind_error, None);
}

#[test]
fn shortcut_capture_keeps_listening_after_conflicts_and_unsupported_keys() {
    let mut app = VisualizerApp::default();
    app.shortcut_capture = Some(ShortcutAction::PlayPause);

    app.apply_shortcut_capture_key(egui::Key::ArrowRight, egui::Modifiers::NONE, false);
    assert_eq!(app.shortcut_capture, Some(ShortcutAction::PlayPause));
    assert!(app
        .shortcut_rebind_error
        .as_deref()
        .is_some_and(|error| error.contains("Next Step")));
    assert_eq!(
        app.shortcut_bindings.key(ShortcutAction::PlayPause),
        egui::Key::Space
    );

    app.apply_shortcut_capture_key(egui::Key::F5, egui::Modifiers::NONE, false);
    assert_eq!(app.shortcut_capture, Some(ShortcutAction::PlayPause));
    assert!(app
        .shortcut_rebind_error
        .as_deref()
        .is_some_and(|error| error.contains("cannot be used")));
}

#[test]
fn shortcut_capture_cancels_with_escape_and_rejects_modified_keys() {
    let modified_inputs = [
        egui::Modifiers {
            alt: true,
            ..egui::Modifiers::NONE
        },
        egui::Modifiers {
            ctrl: true,
            ..egui::Modifiers::NONE
        },
        egui::Modifiers {
            command: true,
            ..egui::Modifiers::NONE
        },
        egui::Modifiers {
            mac_cmd: true,
            ..egui::Modifiers::NONE
        },
    ];

    for modifiers in modified_inputs {
        let mut app = VisualizerApp::default();
        app.shortcut_capture = Some(ShortcutAction::PlayPause);
        app.apply_shortcut_capture_key(egui::Key::P, modifiers, false);
        assert_eq!(app.shortcut_capture, Some(ShortcutAction::PlayPause));
        assert!(app.shortcut_rebind_error.is_some());
        assert_eq!(
            app.shortcut_bindings.key(ShortcutAction::PlayPause),
            egui::Key::Space
        );
    }

    let mut app = VisualizerApp::default();
    app.shortcut_capture = Some(ShortcutAction::PlayPause);
    app.shortcut_rebind_error = Some("test".to_owned());

    app.apply_shortcut_capture_key(egui::Key::Escape, egui::Modifiers::NONE, false);
    assert_eq!(app.shortcut_capture, None);
    assert_eq!(app.shortcut_rebind_error, None);
    assert_eq!(
        app.shortcut_bindings.key(ShortcutAction::PlayPause),
        egui::Key::Space
    );
}

#[test]
fn shortcut_capture_ignores_repeats_and_accepts_shifted_keys() {
    let mut app = VisualizerApp::default();
    app.shortcut_capture = Some(ShortcutAction::PlayPause);
    app.apply_shortcut_capture_key(egui::Key::P, egui::Modifiers::NONE, true);
    assert_eq!(app.shortcut_capture, Some(ShortcutAction::PlayPause));

    app.apply_shortcut_capture_key(
        egui::Key::P,
        egui::Modifiers {
            shift: true,
            ..egui::Modifiers::NONE
        },
        false,
    );
    assert_eq!(app.shortcut_capture, None);
    assert_eq!(
        app.shortcut_bindings.key(ShortcutAction::PlayPause),
        egui::Key::P
    );

    app.shortcut_capture = Some(ShortcutAction::ZoomIn);
    app.apply_shortcut_capture_key(
        egui::Key::Equals,
        egui::Modifiers {
            shift: true,
            ..egui::Modifiers::NONE
        },
        false,
    );
    assert_eq!(app.shortcut_capture, None);
    assert_eq!(
        app.shortcut_bindings.key(ShortcutAction::ZoomIn),
        egui::Key::Plus
    );
}

#[test]
fn capture_processing_consumes_key_and_text_events() {
    let mut app = VisualizerApp::default();
    app.shortcut_capture = Some(ShortcutAction::PlayPause);
    let ctx = egui::Context::default();
    let mut remaining_keyboard_events = usize::MAX;

    let _ = ctx.run(
        egui::RawInput {
            events: vec![
                egui::Event::Key {
                    key: egui::Key::P,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::NONE,
                },
                egui::Event::Text("p".to_owned()),
            ],
            ..egui::RawInput::default()
        },
        |ctx| {
            app.process_shortcut_capture_input(ctx);
            remaining_keyboard_events = ctx.input(|input| {
                input
                    .events
                    .iter()
                    .filter(|event| matches!(event, egui::Event::Key { .. } | egui::Event::Text(_)))
                    .count()
            });
        },
    );

    assert_eq!(remaining_keyboard_events, 0);
    assert_eq!(
        app.shortcut_bindings.key(ShortcutAction::PlayPause),
        egui::Key::P
    );
}

#[test]
fn settings_navigation_and_default_restore_clear_capture_state() {
    let mut app = VisualizerApp::default();
    app.show_settings_modal = true;
    app.open_shortcut_settings();
    assert_eq!(app.settings_page, SettingsPage::KeyboardShortcuts);
    assert_eq!(
        app.settings_focus_target,
        Some(SettingsFocusTarget::ShortcutBackButton)
    );
    app.shortcut_capture = Some(ShortcutAction::PlayPause);
    app.shortcut_rebind_error = Some("test".to_owned());

    app.restore_default_shortcuts();
    assert_eq!(app.shortcut_capture, None);
    assert_eq!(app.shortcut_rebind_error, None);

    app.return_to_general_settings();
    assert_eq!(app.settings_page, SettingsPage::General);
    assert_eq!(
        app.settings_focus_target,
        Some(SettingsFocusTarget::KeyboardMenuButton)
    );

    app.close_settings();
    assert!(!app.show_settings_modal);
    assert_eq!(app.settings_page, SettingsPage::General);
    assert_eq!(app.settings_focus_target, None);
}
