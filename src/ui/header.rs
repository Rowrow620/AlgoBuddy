use crate::app::{ViewMode, VisualizerApp};
use crate::model::ThemePalette;
use crate::shortcuts::ShortcutAction;
use crate::ui::theme_helpers::difficulty_color;
use eframe::egui::{self, Frame, RichText, Rounding, Stroke};

pub fn render_header_panel(app: &mut VisualizerApp, ctx: &egui::Context, p: &ThemePalette) {
    let header = egui::TopBottomPanel::top("header_panel")
        .frame(Frame::none().inner_margin(12.0).fill(p.bg_dark))
        .show(ctx, |ui| {
            let prob = app.current_problem;
            let details = prob.details();

            ui.horizontal(|ui| {
                if !app.show_roadmap_sidebar {
                    if ui
                        .button(RichText::new("Show Roadmap").strong().color(p.cyan))
                        .clicked()
                    {
                        app.show_roadmap_sidebar = true;
                    }
                    ui.add_space(8.0);
                }

                ui.heading(
                    RichText::new(format!("#{} {}", prob.id(), prob.title()))
                        .font(egui::FontId::proportional(18.0))
                        .strong()
                        .color(p.cyan),
                );

                let d_color = difficulty_color(prob.difficulty(), p);
                egui::Frame::none()
                    .fill(p.cell_bg)
                    .rounding(Rounding::same(4.0))
                    .inner_margin(4.0)
                    .show(ui, |ui| {
                        ui.label(
                            RichText::new(prob.difficulty().label())
                                .font(egui::FontId::monospace(11.0))
                                .strong()
                                .color(d_color),
                        );
                    });

                ui.label(
                    RichText::new(format!("Category: {}", prob.category().name()))
                        .font(egui::FontId::proportional(12.0))
                        .color(p.text_muted),
                );

                let is_fav = app.favorite_problems.contains(&details.id);
                let fav_label = if is_fav { "Favorited" } else { "Favorite" };
                let fav_color = if is_fav { p.amber } else { p.text_muted };
                if ui
                    .button(
                        RichText::new(fav_label)
                            .font(egui::FontId::proportional(11.0))
                            .strong()
                            .color(fav_color),
                    )
                    .clicked()
                {
                    if is_fav {
                        app.favorite_problems.remove(&details.id);
                    } else {
                        app.favorite_problems.insert(details.id);
                    }
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .button(RichText::new("Settings").strong().color(p.cyan))
                        .clicked()
                    {
                        app.open_settings();
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        ui.add_space(8.0);
                        let fs_label = if app.is_fullscreen {
                            "Windowed"
                        } else {
                            "Fullscreen"
                        };
                        let fullscreen_help = "Toggle fullscreen mode (Shortcut: F11, fixed)";
                        let fullscreen_response = ui
                            .button(RichText::new(fs_label).strong().color(p.cyan))
                            .on_hover_text(fullscreen_help);
                        fullscreen_response.widget_info(|| {
                            egui::WidgetInfo::labeled(
                                egui::WidgetType::Button,
                                ui.is_enabled(),
                                fullscreen_help,
                            )
                        });
                        if fullscreen_response.clicked() {
                            app.is_fullscreen = !app.is_fullscreen;
                            ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(
                                app.is_fullscreen,
                            ));
                        }
                    }

                    ui.add_space(8.0);
                    let solved_count = app.completed_problems.len();
                    let pct = (solved_count as f32 / 150.0) * 100.0;
                    if ui
                        .button(
                            RichText::new(format!("{} / 150 Solved ({:.1}%)", solved_count, pct))
                                .strong()
                                .color(p.amber),
                        )
                        .clicked()
                    {
                        app.view_mode = ViewMode::RoadmapDashboard;
                    }

                    ui.add_space(8.0);
                    if ui
                        .button(
                            RichText::new(format!("LeetCode #{}", details.id))
                                .strong()
                                .color(p.cyan),
                        )
                        .clicked()
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        let _ = open::that(details.leetcode_url);
                        #[cfg(target_arch = "wasm32")]
                        if let Some(win) = web_sys::window() {
                            let _ = win.open_with_url_and_target(details.leetcode_url, "_blank");
                        }
                    }
                });
            });

            ui.add_space(6.0);

            ui.horizontal_wrapped(|ui| {
                let selector_label = if details.approaches.len() > 1 {
                    "Solutions:"
                } else {
                    "Approach:"
                };
                ui.label(RichText::new(selector_label).strong().color(p.text_primary));
                for approach in details.approaches {
                    let is_sel = app.selected_approach_id == approach.id;
                    let btn_label = format!("{} ({})", approach.name, approach.time_complexity);

                    let bg_fill = if is_sel { p.cell_bg } else { p.step_box_bg };
                    let stroke = if is_sel {
                        Stroke::new(1.5_f32, p.amber)
                    } else {
                        Stroke::new(1.0_f32, p.text_dim.gamma_multiply(0.3_f32))
                    };
                    let text_color = if is_sel { p.amber } else { p.text_muted };

                    let btn = egui::Button::new(
                        RichText::new(&btn_label)
                            .font(egui::FontId::monospace(12.0))
                            .color(text_color)
                            .strong(),
                    )
                    .fill(bg_fill)
                    .stroke(stroke)
                    .rounding(Rounding::same(6.0))
                    .selected(is_sel);

                    let tooltip = if is_sel {
                        format!(
                            "Current solution. Time {}; space {}.",
                            approach.time_complexity, approach.space_complexity
                        )
                    } else {
                        format!(
                            "Use {}. Time {}; space {}. Switching restarts the trace at step 1.",
                            approach.name, approach.time_complexity, approach.space_complexity
                        )
                    };

                    let response = ui.add(btn).on_hover_text(tooltip);
                    let accessible_label = format!(
                        "{}. Time {}; space {}. Selecting a solution restarts the trace at step 1.",
                        approach.name, approach.time_complexity, approach.space_complexity
                    );
                    response.widget_info(|| {
                        egui::WidgetInfo::selected(
                            egui::WidgetType::RadioButton,
                            ui.is_enabled(),
                            is_sel,
                            &accessible_label,
                        )
                    });

                    if response.clicked() {
                        app.select_approach(approach.id);
                    }
                }
            });

            ui.add_space(6.0);

            // Timeline navigation and playback controls.
            ui.horizontal(|ui| {
                let play_text = if app.is_playing { "Pause" } else { "Play" };
                let play_tooltip = if app.is_playing {
                    app.shortcut_bindings
                        .hint(ShortcutAction::PlayPause, "Pause timeline playback")
                } else {
                    app.shortcut_bindings.hint(
                        ShortcutAction::PlayPause,
                        "Play timeline from the current step",
                    )
                };
                let play_response = ui
                    .button(RichText::new(play_text).strong())
                    .on_hover_text(&play_tooltip);
                play_response.widget_info(|| {
                    egui::WidgetInfo::labeled(
                        egui::WidgetType::Button,
                        ui.is_enabled(),
                        &play_tooltip,
                    )
                });
                if play_response.clicked() {
                    app.perform_shortcut_action(ShortcutAction::PlayPause);
                }
                let previous_tooltip = app
                    .shortcut_bindings
                    .hint(ShortcutAction::PreviousStep, "Go to the previous step");
                let previous_response = ui.button("Prev").on_hover_text(&previous_tooltip);
                previous_response.widget_info(|| {
                    egui::WidgetInfo::labeled(
                        egui::WidgetType::Button,
                        ui.is_enabled(),
                        &previous_tooltip,
                    )
                });
                if previous_response.clicked() {
                    app.perform_shortcut_action(ShortcutAction::PreviousStep);
                }
                let next_tooltip = app
                    .shortcut_bindings
                    .hint(ShortcutAction::NextStep, "Go to the next step");
                let next_response = ui.button("Next").on_hover_text(&next_tooltip);
                next_response.widget_info(|| {
                    egui::WidgetInfo::labeled(
                        egui::WidgetType::Button,
                        ui.is_enabled(),
                        &next_tooltip,
                    )
                });
                if next_response.clicked() {
                    app.perform_shortcut_action(ShortcutAction::NextStep);
                }
                let reset_tooltip = app
                    .shortcut_bindings
                    .hint(ShortcutAction::ResetTimeline, "Reset timeline to step 1");
                let reset_response = ui.button("Reset").on_hover_text(&reset_tooltip);
                reset_response.widget_info(|| {
                    egui::WidgetInfo::labeled(
                        egui::WidgetType::Button,
                        ui.is_enabled(),
                        &reset_tooltip,
                    )
                });
                if reset_response.clicked() {
                    app.perform_shortcut_action(ShortcutAction::ResetTimeline);
                }

                ui.separator();
                ui.label(
                    RichText::new(format!(
                        "Step {} / {}",
                        app.current_step_idx + 1,
                        app.steps.len()
                    ))
                    .strong(),
                );
                let max_idx = app.steps.len().saturating_sub(1);
                ui.add(egui::Slider::new(&mut app.current_step_idx, 0..=max_idx).show_value(false));

                ui.separator();
                ui.label(RichText::new("Speed:").strong().color(p.text_primary));
                let mut mult = (500.0 / app.playback_speed_ms as f32 * 100.0).round() / 100.0;

                let speed_tooltip = format!(
                    "Sets the step playback speed. Faster: {}; slower: {}.",
                    app.shortcut_bindings.key_label(ShortcutAction::SpeedUp),
                    app.shortcut_bindings.key_label(ShortcutAction::SpeedDown)
                );
                let speed_response = ui
                    .add(
                        egui::Slider::new(&mut mult, 0.25..=4.0)
                            .step_by(0.25)
                            .custom_formatter(|val, _| format!("{:.2}x", val)),
                    )
                    .on_hover_text(&speed_tooltip);
                speed_response.widget_info(|| {
                    egui::WidgetInfo::slider(ui.is_enabled(), mult as f64, &speed_tooltip)
                });

                if speed_response.changed() {
                    app.playback_speed_ms = (500.0 / mult).round() as u64;
                }
            });
        });

    if !app.show_right_sidebar {
        let restore_button_y = header.response.rect.bottom() + 12.0;
        egui::Area::new(egui::Id::new("show_code_problem_button"))
            .anchor(egui::Align2::RIGHT_TOP, [-12.0, restore_button_y])
            .order(egui::Order::Foreground)
            .movable(false)
            .show(ctx, |ui| {
                if ui
                    .button(RichText::new("Show Code/Problem").strong().color(p.cyan))
                    .on_hover_text("Show the code trace and problem statement panel")
                    .clicked()
                {
                    app.show_right_sidebar = true;
                }
            });
    }
}
