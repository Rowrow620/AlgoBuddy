use crate::app::VisualizerApp;
use crate::model::{Category, Difficulty, Problem, ThemePalette};
use crate::ui::components;
use eframe::egui::{self, Frame, RichText};

fn sort_problems_by_difficulty(problems: &mut [Problem]) {
    problems.sort_by_key(|problem| match problem.difficulty() {
        Difficulty::Easy => 0,
        Difficulty::Medium => 1,
        Difficulty::Hard => 2,
    });
}

pub fn render_roadmap_sidebar(app: &mut VisualizerApp, ctx: &egui::Context, p: &ThemePalette) {
    if !app.show_roadmap_sidebar {
        return;
    }

    let max_left_w = (ctx.screen_rect().width() * 0.30).clamp(240.0, 400.0);
    let default_left_w = (ctx.screen_rect().width() * 0.22).clamp(250.0, 320.0);

    egui::SidePanel::left("roadmap_sidebar")
        .min_width(200.0)
        .max_width(max_left_w)
        .default_width(default_left_w)
        .frame(
            Frame::none()
                .stroke(egui::Stroke::NONE)
                .inner_margin(12.0)
                .fill(p.sidebar_bg),
        )
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading(
                    RichText::new("NeetCode Roadmap")
                        .color(p.cyan)
                        .strong()
                        .size(18.0),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .button(
                            RichText::new("Hide")
                                .font(egui::FontId::proportional(11.0))
                                .color(p.text_muted),
                        )
                        .clicked()
                    {
                        app.show_roadmap_sidebar = false;
                    }
                });
            });

            ui.add_space(8.0);

            // Search and difficulty filters.
            ui.horizontal(|ui| {
                ui.label(RichText::new("Search:").font(egui::FontId::proportional(12.0)));
                ui.add(
                    egui::TextEdit::singleline(&mut app.search_query)
                        .hint_text("Search problem...")
                        .desired_width(180.0),
                );
                if !app.search_query.is_empty()
                    && ui.small_button("x").on_hover_text("Clear search").clicked()
                {
                    app.search_query.clear();
                }
            });

            ui.add_space(4.0);
            components::render_difficulty_filter_bar(ui, app, p);

            ui.add_space(6.0);
            ui.separator();
            ui.add_space(6.0);

            let has_active_filter =
                !app.search_query.trim().is_empty() || app.selected_difficulty.is_some();
            let visible_probs = app.visible_problems();

            egui::ScrollArea::vertical().show(ui, |ui| {
                // Favorites are rendered separately from roadmap categories.
                let mut fav_problems: Vec<Problem> = visible_probs
                    .iter()
                    .copied()
                    .filter(|p| app.favorite_problems.contains(&p.id()))
                    .collect();
                sort_problems_by_difficulty(&mut fav_problems);

                if !fav_problems.is_empty()
                    || (has_active_filter && !app.favorite_problems.is_empty())
                {
                    let header_text = format!("Favorites ({})", fav_problems.len());
                    let is_active_cat = fav_problems.contains(&app.current_problem);
                    let header_color = if is_active_cat {
                        p.cyan
                    } else {
                        p.text_primary
                    };

                    egui::CollapsingHeader::new(
                        RichText::new(header_text).color(header_color).strong(),
                    )
                    .default_open(true)
                    .show(ui, |ui| {
                        if fav_problems.is_empty() {
                            ui.label(
                                RichText::new("  (Filtered Out)")
                                    .italics()
                                    .font(egui::FontId::proportional(11.0))
                                    .color(p.text_dim),
                            );
                        } else {
                            for prob in fav_problems {
                                components::render_sidebar_problem_row(ui, app, prob, p);
                            }
                        }
                    });
                    ui.add_space(8.0);
                }

                for &category in Category::all() {
                    let mut problems_in_cat: Vec<Problem> = visible_probs
                        .iter()
                        .copied()
                        .filter(|p| p.category() == category)
                        .collect();
                    sort_problems_by_difficulty(&mut problems_in_cat);
                    let total_in_cat = Problem::all()
                        .iter()
                        .filter(|p| p.category() == category)
                        .count();
                    let header_text = format!("{} ({})", category.name(), problems_in_cat.len());

                    let is_active_cat = problems_in_cat.contains(&app.current_problem);
                    let header_color = if is_active_cat {
                        p.cyan
                    } else {
                        p.text_primary
                    };

                    egui::CollapsingHeader::new(
                        RichText::new(header_text).color(header_color).strong(),
                    )
                    .default_open(is_active_cat || has_active_filter)
                    .show(ui, |ui| {
                        if problems_in_cat.is_empty() {
                            if total_in_cat == 0 {
                                ui.label(
                                    RichText::new("  (Coming Soon)")
                                        .italics()
                                        .font(egui::FontId::proportional(11.0))
                                        .color(p.text_dim),
                                );
                            } else {
                                ui.label(
                                    RichText::new("  (Filtered Out)")
                                        .italics()
                                        .font(egui::FontId::proportional(11.0))
                                        .color(p.text_dim),
                                );
                            }
                        } else {
                            for prob in problems_in_cat {
                                components::render_sidebar_problem_row(ui, app, prob, p);
                            }
                        }
                    });
                }
            });
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn difficulty_sort_is_stable_and_easy_to_hard() {
        let mut problems = vec![
            Problem::KClosestPoints,
            Problem::TaskScheduler,
            Problem::FindMedianDataStream,
            Problem::KthLargestArray,
            Problem::DesignTwitter,
            Problem::KthLargestStream,
            Problem::LastStone,
        ];

        sort_problems_by_difficulty(&mut problems);

        assert_eq!(
            problems,
            vec![
                Problem::KthLargestStream,
                Problem::LastStone,
                Problem::KClosestPoints,
                Problem::TaskScheduler,
                Problem::KthLargestArray,
                Problem::DesignTwitter,
                Problem::FindMedianDataStream,
            ]
        );
    }
}
