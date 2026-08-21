use crate::app::{RightTab, VisualizerApp};
use crate::model::problem::approach_code_lines;
use crate::model::{ApproachMeta, Problem, Step, ThemePalette};
use crate::ui::theme_helpers::difficulty_color;
use eframe::egui::{self, Color32, Frame, RichText, Rounding};

pub fn render_right_sidebar_inspector(
    app: &mut VisualizerApp,
    ctx: &egui::Context,
    p: &ThemePalette,
) {
    if !app.show_right_sidebar {
        return;
    }

    let max_right_w = (ctx.screen_rect().width() * 0.42).clamp(280.0, 550.0);
    let default_right_w = (ctx.screen_rect().width() * 0.35).clamp(300.0, 420.0);

    egui::SidePanel::right("right_sidebar")
        .min_width(240.0)
        .max_width(max_right_w)
        .default_width(default_right_w)
        .frame(
            Frame::none()
                .stroke(egui::Stroke::NONE)
                .inner_margin(12.0)
                .fill(p.sidebar_bg),
        )
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui
                    .selectable_label(
                        app.right_tab == RightTab::CodeTrace,
                        RichText::new("Code Trace").strong(),
                    )
                    .clicked()
                {
                    app.right_tab = RightTab::CodeTrace;
                }
                if ui
                    .selectable_label(
                        app.right_tab == RightTab::ProblemDetails,
                        RichText::new("Problem Statement & Examples").strong(),
                    )
                    .clicked()
                {
                    app.right_tab = RightTab::ProblemDetails;
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .button(
                            RichText::new("Hide")
                                .font(egui::FontId::proportional(11.0))
                                .color(p.text_muted),
                        )
                        .clicked()
                    {
                        app.show_right_sidebar = false;
                    }
                });
            });

            ui.separator();
            ui.add_space(6.0);

            match app.right_tab {
                RightTab::CodeTrace => {
                    if let Some(step) = app.steps.get(app.current_step_idx) {
                        // Keep the active step visible while the details below it scroll.
                        egui::Frame::none()
                            .fill(p.sidebar_bg)
                            .inner_margin(0.0)
                            .show(ui, |ui| {
                                egui::Frame::group(ui.style())
                                    .fill(p.step_box_bg)
                                    .rounding(Rounding::same(8.0))
                                    .inner_margin(10.0)
                                    .show(ui, |ui| {
                                        ui.label(
                                            RichText::new(format!(
                                                "STEP {} / {}",
                                                app.current_step_idx + 1,
                                                app.steps.len()
                                            ))
                                            .font(egui::FontId::monospace(11.0))
                                            .color(p.cyan)
                                            .strong(),
                                        );
                                        ui.add_space(4.0);
                                        ui.label(
                                            RichText::new(&step.description)
                                                .font(egui::FontId::proportional(13.0))
                                                .color(p.text_primary),
                                        );

                                        if let Some(formula) = app
                                            .current_problem
                                            .formula_for_approach(app.selected_approach_id)
                                        {
                                            ui.add_space(6.0);
                                            egui::Frame::none()
                                                .fill(p.cell_bg)
                                                .rounding(Rounding::same(4.0))
                                                .inner_margin(egui::Margin::symmetric(6.0, 3.0))
                                                .show(ui, |ui| {
                                                    ui.horizontal(|ui| {
                                                        ui.label(
                                                            RichText::new("Invariant:")
                                                                .font(egui::FontId::monospace(10.0))
                                                                .color(p.amber)
                                                                .strong(),
                                                        );
                                                        ui.label(
                                                            RichText::new(formula)
                                                                .font(egui::FontId::monospace(11.0))
                                                                .color(p.cyan)
                                                                .strong(),
                                                        );
                                                    });
                                                });
                                        }
                                    });

                                ui.add_space(6.0);
                                render_variable_scope_chips(
                                    ui,
                                    step,
                                    app.current_problem,
                                    app.selected_approach_id,
                                    p,
                                );
                            });

                        ui.add_space(6.0);
                        ui.separator();
                        ui.add_space(6.0);

                        // Complexity and source code share the lower scroll area.
                        egui::ScrollArea::vertical()
                            .id_source("right_code_trace_lower_scroll")
                            .show(ui, |ui| {
                                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);

                                if let Some(app_meta) = app
                                    .current_problem
                                    .details()
                                    .approach_by_id(app.selected_approach_id)
                                {
                                    render_complexity_card(ui, app_meta, p);
                                }

                                let code_lines = approach_code_lines(
                                    app.current_problem,
                                    app.selected_approach_id,
                                );

                                ui.add_space(10.0);
                                ui.horizontal(|ui| {
                                    ui.label(
                                        RichText::new("Python Implementation")
                                            .strong()
                                            .color(p.text_muted),
                                    );
                                    ui.with_layout(
                                        egui::Layout::right_to_left(egui::Align::Center),
                                        |ui| {
                                            if ui
                                                .button(RichText::new("Copy Code").size(11.0))
                                                .on_hover_text(
                                                    "Copy full Python solution to clipboard",
                                                )
                                                .clicked()
                                            {
                                                let full_code = code_lines
                                                    .iter()
                                                    .map(|(_, text)| *text)
                                                    .collect::<Vec<_>>()
                                                    .join("\n");
                                                ui.output_mut(|o| o.copied_text = full_code);
                                            }
                                        },
                                    );
                                });
                                ui.add_space(6.0);

                                let should_auto_focus =
                                    app.last_focused_step_idx != Some(app.current_step_idx);

                                for (line_num, line_text) in &code_lines {
                                    let is_active = step.code_line == *line_num;
                                    let text_color = if is_active {
                                        p.text_primary
                                    } else {
                                        p.text_muted
                                    };
                                    let bg = if is_active {
                                        p.code_active_bg
                                    } else {
                                        Color32::TRANSPARENT
                                    };

                                    let frame_resp = egui::Frame::none()
                                        .fill(bg)
                                        .rounding(Rounding::same(4.0))
                                        .inner_margin(4.0)
                                        .show(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                ui.label(
                                                    RichText::new(format!("{:2} | ", line_num))
                                                        .font(egui::FontId::monospace(11.0))
                                                        .color(p.text_dim),
                                                );
                                                let mut rt = RichText::new(*line_text)
                                                    .font(egui::FontId::monospace(12.0))
                                                    .color(text_color);
                                                if is_active {
                                                    rt = rt.strong();
                                                }
                                                ui.label(rt);
                                            });
                                        });

                                    if is_active && should_auto_focus {
                                        frame_resp.response.scroll_to_me(Some(egui::Align::Center));
                                        app.last_focused_step_idx = Some(app.current_step_idx);
                                    }
                                }
                                ui.add_space(8.0);
                                ui.horizontal(|ui| {
                                    ui.label(
                                        RichText::new("⚡ Engine")
                                            .font(egui::FontId::monospace(10.0))
                                            .color(p.amber),
                                    );
                                    ui.label(
                                        RichText::new(format!(
                                            "{:.2} ms",
                                            app.step_generation_time_ms
                                        ))
                                        .font(egui::FontId::monospace(10.0))
                                        .color(p.text_dim),
                                    );
                                });
                                ui.add_space(16.0);
                            });
                    }
                }
                RightTab::ProblemDetails => {
                    let details = app.current_problem.details();
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.heading(
                            RichText::new(format!("#{} {}", details.id, details.title))
                                .color(p.cyan)
                                .strong()
                                .size(18.0),
                        );
                        ui.add_space(4.0);

                        ui.horizontal(|ui| {
                            let d_color = difficulty_color(details.difficulty, p);
                            ui.label(
                                RichText::new(details.difficulty.label())
                                    .color(d_color)
                                    .strong(),
                            );
                            ui.label(
                                RichText::new(format!("• Category: {}", details.category.name()))
                                    .color(p.text_muted),
                            );
                        });

                        ui.add_space(10.0);
                        ui.label(RichText::new("Description").strong().color(p.text_primary));
                        ui.add_space(4.0);
                        ui.label(
                            RichText::new(details.statement)
                                .font(egui::FontId::proportional(13.0))
                                .color(p.text_primary),
                        );

                        ui.add_space(14.0);
                        ui.label(RichText::new("Examples").strong().color(p.text_primary));
                        ui.add_space(4.0);

                        for (ex_idx, ex) in details.examples.iter().enumerate() {
                            egui::Frame::group(ui.style())
                                .fill(p.step_box_bg)
                                .rounding(Rounding::same(8.0))
                                .inner_margin(10.0)
                                .show(ui, |ui| {
                                    ui.label(
                                        RichText::new(format!("Example {}", ex_idx + 1))
                                            .strong()
                                            .color(p.amber),
                                    );
                                    ui.add_space(4.0);
                                    ui.label(
                                        RichText::new(format!("Input: {}", ex.input))
                                            .font(egui::FontId::monospace(12.0))
                                            .color(p.cyan),
                                    );
                                    ui.label(
                                        RichText::new(format!("Output: {}", ex.output))
                                            .font(egui::FontId::monospace(12.0))
                                            .color(p.emerald_text),
                                    );
                                    if !ex.explanation.is_empty() {
                                        ui.label(
                                            RichText::new(format!(
                                                "Explanation: {}",
                                                ex.explanation
                                            ))
                                            .font(egui::FontId::proportional(12.0))
                                            .color(p.text_muted),
                                        );
                                    }
                                });
                            ui.add_space(6.0);
                        }

                        ui.add_space(10.0);
                        ui.label(RichText::new("Constraints").strong().color(p.text_primary));
                        ui.add_space(4.0);
                        for constraint in details.constraints {
                            ui.label(
                                RichText::new(format!("• {}", constraint))
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                        }

                        ui.add_space(14.0);
                        ui.label(
                            RichText::new("Solution Approaches")
                                .strong()
                                .color(p.text_primary),
                        );
                        ui.add_space(4.0);
                        for app_meta in details.approaches {
                            let is_selected = app_meta.id == app.selected_approach_id;
                            let bg = if is_selected {
                                p.code_active_bg
                            } else {
                                p.step_box_bg
                            };
                            egui::Frame::group(ui.style())
                                .fill(bg)
                                .rounding(Rounding::same(8.0))
                                .inner_margin(10.0)
                                .show(ui, |ui| {
                                    ui.label(
                                        RichText::new(format!(
                                            "Approach {}: {}",
                                            app_meta.id + 1,
                                            app_meta.name
                                        ))
                                        .strong()
                                        .color(if is_selected { p.cyan } else { p.text_primary }),
                                    );
                                    ui.label(
                                        RichText::new(format!(
                                            "Time: {} | Space: {}",
                                            app_meta.time_complexity, app_meta.space_complexity
                                        ))
                                        .font(egui::FontId::monospace(11.0))
                                        .color(p.text_muted),
                                    );
                                    if !app_meta.description.is_empty() {
                                        ui.add_space(4.0);
                                        ui.label(
                                            RichText::new(app_meta.description)
                                                .font(egui::FontId::proportional(12.0))
                                                .color(p.text_primary),
                                        );
                                    }
                                });
                            ui.add_space(6.0);
                        }

                        ui.add_space(14.0);
                        if ui
                            .button(RichText::new("Open on LeetCode.com").strong().color(p.cyan))
                            .clicked()
                        {
                            #[cfg(not(target_arch = "wasm32"))]
                            let _ = open::that(details.leetcode_url);
                            #[cfg(target_arch = "wasm32")]
                            if let Some(win) = web_sys::window() {
                                let _ =
                                    win.open_with_url_and_target(details.leetcode_url, "_blank");
                            }
                        }
                    });
                }
            }
        });
}

fn render_complexity_card(ui: &mut egui::Ui, app_meta: &ApproachMeta, p: &ThemePalette) {
    egui::Frame::group(ui.style())
        .fill(p.step_box_bg)
        .rounding(Rounding::same(8.0))
        .inner_margin(10.0)
        .show(ui, |ui| {
            ui.label(
                RichText::new("Algorithm Complexity Card")
                    .strong()
                    .color(p.cyan)
                    .size(12.0),
            );
            ui.add_space(6.0);

            ui.horizontal_wrapped(|ui| {
                let tc_color = if app_meta.time_complexity.contains("O(1)")
                    || app_meta.time_complexity.contains("O(log")
                    || app_meta.time_complexity == "O(N)"
                {
                    p.emerald_text
                } else if app_meta.time_complexity.contains("O(N log N)")
                    || app_meta.time_complexity.contains("O(N * K)")
                {
                    p.amber
                } else {
                    p.red
                };

                egui::Frame::none()
                    .fill(tc_color.gamma_multiply(0.15))
                    .rounding(Rounding::same(4.0))
                    .inner_margin(egui::Margin::symmetric(8.0, 4.0))
                    .show(ui, |ui| {
                        ui.label(
                            RichText::new(format!("Time: {}", app_meta.time_complexity))
                                .font(egui::FontId::monospace(12.0))
                                .color(tc_color)
                                .strong(),
                        );
                    });

                ui.add_space(4.0);

                let sc_color = if app_meta.space_complexity.contains("O(1)") {
                    p.emerald_text
                } else if app_meta.space_complexity.contains("O(N)")
                    || app_meta.space_complexity.contains("O(H)")
                {
                    p.cyan
                } else {
                    p.amber
                };

                egui::Frame::none()
                    .fill(sc_color.gamma_multiply(0.15))
                    .rounding(Rounding::same(4.0))
                    .inner_margin(egui::Margin::symmetric(8.0, 4.0))
                    .show(ui, |ui| {
                        ui.label(
                            RichText::new(format!("Space: {}", app_meta.space_complexity))
                                .font(egui::FontId::monospace(12.0))
                                .color(sc_color)
                                .strong(),
                        );
                    });
            });

            if !app_meta.rationale.is_empty() {
                ui.add_space(8.0);
                ui.label(
                    RichText::new(app_meta.rationale)
                        .font(egui::FontId::proportional(12.0))
                        .color(p.text_primary),
                );
            }
        });
}

fn render_variable_scope_chips(
    ui: &mut egui::Ui,
    step: &Step,
    problem: Problem,
    approach_id: usize,
    p: &ThemePalette,
) {
    let mut vars = step.visual.variables(approach_id);
    if problem == Problem::BalancedTree && approach_id == 1 {
        for (name, _) in &mut vars {
            if *name == "depth" {
                *name = "height_difference";
            }
        }
    }
    if vars.is_empty() {
        return;
    }

    egui::Frame::group(ui.style())
        .fill(p.step_box_bg)
        .rounding(Rounding::same(8.0))
        .inner_margin(8.0)
        .show(ui, |ui| {
            ui.spacing_mut().scroll.bar_width = 3.0;
            ui.spacing_mut().scroll.handle_min_length = 16.0;

            egui::ScrollArea::horizontal()
                .id_source("scope_chips_horizontal_scroll")
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Scope:").strong().color(p.amber).size(12.0));
                        ui.add_space(4.0);

                        for (name, val) in vars {
                            let chip_resp = egui::Frame::none()
                                .fill(p.cell_bg)
                                .rounding(Rounding::same(4.0))
                                .inner_margin(egui::Margin::symmetric(6.0, 3.0))
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            RichText::new(format!("{}:", name))
                                                .font(egui::FontId::monospace(11.0))
                                                .color(p.cyan)
                                                .strong(),
                                        );
                                        ui.label(
                                            RichText::new(&val)
                                                .font(egui::FontId::monospace(11.0))
                                                .color(p.emerald_text)
                                                .strong(),
                                        );
                                    });
                                });

                            chip_resp
                                .response
                                .on_hover_text(format!("{}: {}", name, val));
                            ui.add_space(4.0);
                        }
                    });
                });
        });
}
