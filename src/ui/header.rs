use crate::app::{ViewMode, VisualizerApp};
use crate::model::ThemePalette;
use crate::ui::theme_helpers::difficulty_color;
use eframe::egui::{self, Color32, Frame, RichText, Rounding, Stroke};
use web_time::Instant;

pub fn render_header_panel(app: &mut VisualizerApp, ctx: &egui::Context, p: &ThemePalette) {
    egui::TopBottomPanel::top("header_panel")
        .frame(Frame::none().inner_margin(12.0).fill(p.bg_dark))
        .show(ctx, |ui| {
            let prob = app.current_problem;
            let details = prob.details();

            ui.horizontal(|ui| {
                if !app.show_roadmap_sidebar {
                    if ui
                        .button(RichText::new("▶ Show Roadmap").strong().color(p.cyan))
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
                let fav_label = if is_fav {
                    "★ Favorited"
                } else {
                    "☆ Favorite"
                };
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
                    if !app.show_right_sidebar {
                        if ui
                            .button(RichText::new("Code & Problem ◀").strong().color(p.cyan))
                            .clicked()
                        {
                            app.show_right_sidebar = true;
                        }
                        ui.add_space(12.0);
                    }

                    if ui
                        .button(RichText::new("⚙ Settings").strong().color(p.cyan))
                        .clicked()
                    {
                        app.show_settings_modal = true;
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        ui.add_space(8.0);
                        let fs_label = if app.is_fullscreen {
                            "🗗 Windowed"
                        } else {
                            "⛶ Fullscreen"
                        };
                        if ui
                            .button(RichText::new(fs_label).strong().color(p.cyan))
                            .clicked()
                        {
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
                            RichText::new(format!(
                                "🏆 {} / 150 Solved ({:.1}%)",
                                solved_count, pct
                            ))
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
                            RichText::new(format!("🌐 LeetCode #{} ↗", details.id))
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

            // Multi-Approach Selector Row
            ui.horizontal(|ui| {
                ui.label(RichText::new("Approach:").strong().color(p.text_primary));
                for approach in details.approaches {
                    let is_sel = app.selected_approach_id == approach.id;
                    let btn_label = format!("{} ({})", approach.name, approach.time_complexity);

                    let bg_fill = if is_sel {
                        p.cell_bg
                    } else {
                        Color32::TRANSPARENT
                    };
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
                    .rounding(Rounding::same(6.0));

                    if ui.add(btn).clicked() {
                        app.selected_approach_id = approach.id;
                        app.recompute_steps();
                    }
                }
            });

            ui.add_space(6.0);

            // Playback Control Bar
            ui.horizontal(|ui| {
                let play_text = if app.is_playing { "Pause" } else { "Play" };
                if ui.button(RichText::new(play_text).strong()).clicked() {
                    if app.current_step_idx >= app.steps.len().saturating_sub(1) {
                        app.current_step_idx = 0;
                    }
                    app.is_playing = !app.is_playing;
                    app.last_step_time = Instant::now();
                }
                if ui.button("Prev").clicked() {
                    app.is_playing = false;
                    app.current_step_idx = app.current_step_idx.saturating_sub(1);
                }
                if ui.button("Next").clicked() {
                    app.is_playing = false;
                    if app.current_step_idx < app.steps.len().saturating_sub(1) {
                        app.current_step_idx += 1;
                    }
                }
                if ui.button("Reset [R]")
                    .on_hover_text("Reset timeline to step 1 (Shortcut: R)")
                    .clicked() {
                    app.is_playing = false;
                    app.current_step_idx = 0;
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
                if ui
                    .add(
                        egui::Slider::new(&mut mult, 0.25..=4.0)
                            .step_by(0.25)
                            .custom_formatter(|val, _| format!("{:.2}x", val)),
                    )
                    .changed()
                {
                    app.playback_speed_ms = (500.0 / mult).round() as u64;
                }
            });
        });
}
