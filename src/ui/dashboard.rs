use crate::app::{ViewMode, VisualizerApp};
use crate::model::{Category, ThemePalette};
use crate::ui::theme_helpers::difficulty_color;
use eframe::egui::{self, Frame, RichText, Rounding};

pub fn render_fullscreen_roadmap_dashboard(
    app: &mut VisualizerApp,
    ctx: &egui::Context,
    p: &ThemePalette,
) {
    let mut prob_to_select = None;
    let total_solved = app.completed_problems.len();
    let overall_pct = (total_solved as f32 / 150.0) * 100.0;

    egui::CentralPanel::default()
        .frame(Frame::none().fill(p.sidebar_bg).inner_margin(24.0))
        .show(ctx, |ui| {
            // Dashboard navigation.
            ui.horizontal(|ui| {
                if ui
                    .button(
                        RichText::new("Back to Visualizer")
                            .strong()
                            .color(p.cyan)
                            .size(14.0),
                    )
                    .clicked()
                {
                    app.view_mode = ViewMode::Visualizer;
                }
                ui.add_space(16.0);
                ui.heading(
                    RichText::new("NeetCode 150 Mastery Dashboard")
                        .color(p.amber)
                        .strong()
                        .size(20.0),
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .button(RichText::new("Reset All Progress").strong().color(p.red))
                        .clicked()
                    {
                        app.show_reset_confirm_modal = true;
                    }

                    ui.add_space(12.0);
                    egui::Frame::none()
                        .fill(p.cell_bg)
                        .rounding(Rounding::same(6.0))
                        .inner_margin(8.0)
                        .show(ui, |ui| {
                            ui.label(
                                RichText::new(format!(
                                    "Overall Progress: {} / {} Solved ({:.1}%)",
                                    total_solved,
                                    app.visible_problems().len(),
                                    overall_pct
                                ))
                                .font(egui::FontId::monospace(13.0))
                                .color(p.emerald_text)
                                .strong(),
                            );
                        });
                });
            });

            ui.add_space(16.0);
            ui.separator();
            ui.add_space(16.0);

            // Keep category progress and the problem launcher in separate columns.
            ui.columns(2, |cols| {
                cols[0].heading(
                    RichText::new("Category Completion Breakdown")
                        .color(p.cyan)
                        .size(15.0),
                );
                cols[0].add_space(10.0);

                egui::ScrollArea::vertical()
                    .id_source("cat_scroll")
                    .show(&mut cols[0], |ui| {
                        for &category in Category::all() {
                            let total_in_cat = app
                                .visible_problems()
                                .iter()
                                .filter(|p| p.category() == category)
                                .count()
                                .max(1);
                            let solved_in_cat = app
                                .visible_problems()
                                .iter()
                                .filter(|p| {
                                    p.category() == category
                                        && app.completed_problems.contains(&p.id())
                                })
                                .count();
                            let pct = (solved_in_cat as f32 / total_in_cat as f32) * 100.0;
                            let col = if solved_in_cat == total_in_cat && solved_in_cat > 0 {
                                p.emerald_text
                            } else if solved_in_cat > 0 {
                                p.amber
                            } else {
                                p.text_dim
                            };

                            egui::Frame::group(ui.style())
                                .fill(p.step_box_bg)
                                .rounding(Rounding::same(6.0))
                                .inner_margin(8.0)
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            RichText::new(category.name())
                                                .strong()
                                                .color(p.text_primary),
                                        );
                                        ui.with_layout(
                                            egui::Layout::right_to_left(egui::Align::Center),
                                            |ui| {
                                                ui.label(
                                                    RichText::new(format!(
                                                        "{}/{} ({:.0}%)",
                                                        solved_in_cat, total_in_cat, pct
                                                    ))
                                                    .font(egui::FontId::monospace(12.0))
                                                    .color(col)
                                                    .strong(),
                                                );
                                            },
                                        );
                                    });
                                    ui.add_space(4.0);
                                    let ratio = solved_in_cat as f32 / total_in_cat as f32;

                                    let progress_fill = if ratio > 0.75 {
                                        p.emerald_text
                                    } else if ratio >= 0.25 {
                                        p.cyan
                                    } else {
                                        p.amber
                                    };

                                    ui.add(
                                        egui::ProgressBar::new(ratio).fill(progress_fill).text(""),
                                    );
                                });
                            ui.add_space(6.0);
                        }
                    });

                cols[1].heading(
                    RichText::new("Implemented Problems & Checkmarks")
                        .color(p.amber)
                        .size(15.0),
                );
                cols[1].add_space(10.0);

                egui::ScrollArea::vertical()
                    .id_source("prob_scroll")
                    .show(&mut cols[1], |ui| {
                        egui::Grid::new("fullscreen_roadmap_grid")
                            .striped(true)
                            .spacing([12.0, 8.0])
                            .show(ui, |ui| {
                                for prob in app.visible_problems() {
                                    let details = prob.details();
                                    let d_color = difficulty_color(details.difficulty, p);
                                    let mut is_completed =
                                        app.completed_problems.contains(&details.id);

                                    if ui.checkbox(&mut is_completed, "").changed() {
                                        if is_completed {
                                            app.completed_problems.insert(details.id);
                                        } else {
                                            app.completed_problems.remove(&details.id);
                                        }
                                    }

                                    if ui
                                        .button(
                                            RichText::new(format!(
                                                "#{} {}",
                                                details.id, details.title
                                            ))
                                            .color(if is_completed {
                                                p.emerald_text
                                            } else {
                                                p.cyan
                                            })
                                            .strong(),
                                        )
                                        .clicked()
                                    {
                                        prob_to_select = Some(prob);
                                    }
                                    ui.label(
                                        RichText::new(details.difficulty.label())
                                            .color(d_color)
                                            .strong(),
                                    );
                                    ui.label(
                                        RichText::new(details.category.name()).color(p.text_muted),
                                    );
                                    ui.end_row();
                                }
                            });
                    });
            });
        });

    if let Some(prob) = prob_to_select {
        app.select_problem(prob);
        app.view_mode = ViewMode::Visualizer;
    }

    crate::ui::modals::render_reset_confirm_modal(app, ctx);
}
