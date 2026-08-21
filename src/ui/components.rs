use crate::app::VisualizerApp;
use crate::model::{Difficulty, Problem, ThemePalette};
use crate::ui::theme_helpers::difficulty_color;
use eframe::egui::{self, RichText};

/// Renders the difficulty filter bar ("Diff: All | Easy | Med | Hard").
pub fn render_difficulty_filter_bar(ui: &mut egui::Ui, app: &mut VisualizerApp, p: &ThemePalette) {
    ui.horizontal(|ui| {
        ui.label(
            RichText::new("Diff:")
                .font(egui::FontId::proportional(11.0))
                .color(p.text_muted),
        );

        if ui
            .selectable_label(app.selected_difficulty.is_none(), "All")
            .clicked()
        {
            app.selected_difficulty = None;
        }
        if ui
            .selectable_label(
                app.selected_difficulty == Some(Difficulty::Easy),
                RichText::new("Easy").color(difficulty_color(Difficulty::Easy, p)),
            )
            .clicked()
        {
            app.selected_difficulty = Some(Difficulty::Easy);
        }
        if ui
            .selectable_label(
                app.selected_difficulty == Some(Difficulty::Medium),
                RichText::new("Med").color(difficulty_color(Difficulty::Medium, p)),
            )
            .clicked()
        {
            app.selected_difficulty = Some(Difficulty::Medium);
        }
        if ui
            .selectable_label(
                app.selected_difficulty == Some(Difficulty::Hard),
                RichText::new("Hard").color(difficulty_color(Difficulty::Hard, p)),
            )
            .clicked()
        {
            app.selected_difficulty = Some(Difficulty::Hard);
        }
    });
}

/// Renders a single problem row in the sidebar (favorite star, truncated title, difficulty badge).
pub fn render_sidebar_problem_row(
    ui: &mut egui::Ui,
    app: &mut VisualizerApp,
    prob: Problem,
    p: &ThemePalette,
) {
    let is_selected = app.current_problem == prob;
    let diff_color = difficulty_color(prob.difficulty(), p);
    let is_fav = app.favorite_problems.contains(&prob.id());

    ui.horizontal(|ui| {
        let (favorite_text, favorite_color) = if is_fav {
            ("★", p.amber)
        } else {
            ("☆", p.text_muted)
        };
        let favorite_rt = RichText::new(favorite_text)
            .font(egui::FontId::proportional(13.0))
            .color(favorite_color)
            .strong();
        if ui
            .button(favorite_rt)
            .on_hover_text(if is_fav {
                "Remove from Favorites"
            } else {
                "Add to Favorites"
            })
            .clicked()
        {
            if is_fav {
                app.favorite_problems.remove(&prob.id());
            } else {
                app.favorite_problems.insert(prob.id());
            }
        }

        let title_text = format!("#{} {}", prob.id(), prob.title());
        let btn_rt = RichText::new(&title_text).font(egui::FontId::proportional(12.0));
        let btn_text = if is_selected {
            btn_rt.color(egui::Color32::WHITE).strong()
        } else {
            btn_rt.color(p.text_primary)
        };

        let title_max_w = (ui.available_width() - 58.0).max(40.0);
        ui.scope(|ui| {
            ui.set_max_width(title_max_w);
            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);
            if ui
                .selectable_label(is_selected, btn_text)
                .on_hover_text(&title_text)
                .clicked()
            {
                app.select_problem(prob);
            }
        });

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                RichText::new(prob.difficulty().label())
                    .font(egui::FontId::monospace(10.0))
                    .color(diff_color),
            );
        });
    });
}
