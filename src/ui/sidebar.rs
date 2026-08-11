use crate::app::VisualizerApp;
use crate::model::{Category, Difficulty, Problem, ThemePalette};
use crate::ui::theme_helpers::difficulty_color;
use eframe::egui::{self, Frame, RichText};

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

            ui.add_space(6.0);
            ui.separator();
            ui.add_space(6.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                // Favorites are rendered separately from roadmap categories.
                let fav_problems: Vec<Problem> = app
                    .visible_problems()
                    .into_iter()
                    .filter(|p| app.favorite_problems.contains(&p.id()))
                    .filter(|p| {
                        if let Some(diff) = app.selected_difficulty {
                            p.difficulty() == diff
                        } else {
                            true
                        }
                    })
                    .filter(|p| {
                        if app.search_query.trim().is_empty() {
                            true
                        } else {
                            let q = app.search_query.to_lowercase();
                            p.title().to_lowercase().contains(&q) || p.id().to_string().contains(&q)
                        }
                    })
                    .collect();

                let has_active_filter =
                    !app.search_query.trim().is_empty() || app.selected_difficulty.is_some();

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
                                let is_selected = app.current_problem == prob;
                                let diff_color = difficulty_color(prob.difficulty(), p);

                                ui.horizontal(|ui| {
                                    let favorite_rt = RichText::new("[x]")
                                        .font(egui::FontId::proportional(12.0))
                                        .color(p.amber)
                                        .strong();
                                    if ui
                                        .button(favorite_rt)
                                        .on_hover_text("Remove from Favorites")
                                        .clicked()
                                    {
                                        app.favorite_problems.remove(&prob.id());
                                    }

                                    let title_text = format!("#{} {}", prob.id(), prob.title());
                                    let btn_rt = RichText::new(&title_text)
                                        .font(egui::FontId::proportional(12.0));
                                    let btn_text = if is_selected {
                                        btn_rt.color(egui::Color32::WHITE).strong()
                                    } else {
                                        btn_rt.color(p.text_primary)
                                    };

                                    let title_max_w = (ui.available_width() - 58.0).max(40.0);
                                    ui.scope(|ui| {
                                        ui.set_max_width(title_max_w);
                                        ui.style_mut().wrap_mode =
                                            Some(egui::TextWrapMode::Truncate);
                                        if ui
                                            .selectable_label(is_selected, btn_text)
                                            .on_hover_text(&title_text)
                                            .clicked()
                                        {
                                            app.select_problem(prob);
                                        }
                                    });

                                    ui.with_layout(
                                        egui::Layout::right_to_left(egui::Align::Center),
                                        |ui| {
                                            ui.label(
                                                RichText::new(prob.difficulty().label())
                                                    .font(egui::FontId::monospace(10.0))
                                                    .color(diff_color),
                                            );
                                        },
                                    );
                                });
                            }
                        }
                    });
                    ui.add_space(8.0);
                }

                for &category in Category::all() {
                    let problems_in_cat: Vec<Problem> = app
                        .visible_problems()
                        .into_iter()
                        .filter(|p| p.category() == category)
                        .filter(|p| {
                            if let Some(diff) = app.selected_difficulty {
                                p.difficulty() == diff
                            } else {
                                true
                            }
                        })
                        .filter(|p| {
                            if app.search_query.trim().is_empty() {
                                true
                            } else {
                                let q = app.search_query.to_lowercase();
                                p.title().to_lowercase().contains(&q)
                                    || p.id().to_string().contains(&q)
                            }
                        })
                        .collect();

                    let total_in_cat = app
                        .visible_problems()
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
                                let is_selected = app.current_problem == prob;
                                let diff_color = difficulty_color(prob.difficulty(), p);
                                let is_fav = app.favorite_problems.contains(&prob.id());

                                ui.horizontal(|ui| {
                                    let (favorite_text, favorite_color) = if is_fav {
                                        ("[x]", p.amber)
                                    } else {
                                        ("[ ]", p.text_muted)
                                    };
                                    let favorite_rt = RichText::new(favorite_text)
                                        .font(egui::FontId::proportional(12.0))
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
                                    let btn_rt = RichText::new(&title_text)
                                        .font(egui::FontId::proportional(12.0));
                                    let btn_text = if is_selected {
                                        btn_rt.color(egui::Color32::WHITE).strong()
                                    } else {
                                        btn_rt.color(p.text_primary)
                                    };

                                    let title_max_w = (ui.available_width() - 58.0).max(40.0);
                                    ui.scope(|ui| {
                                        ui.set_max_width(title_max_w);
                                        ui.style_mut().wrap_mode =
                                            Some(egui::TextWrapMode::Truncate);
                                        if ui
                                            .selectable_label(is_selected, btn_text)
                                            .on_hover_text(&title_text)
                                            .clicked()
                                        {
                                            app.select_problem(prob);
                                        }
                                    });

                                    ui.with_layout(
                                        egui::Layout::right_to_left(egui::Align::Center),
                                        |ui| {
                                            ui.label(
                                                RichText::new(prob.difficulty().label())
                                                    .font(egui::FontId::monospace(10.0))
                                                    .color(diff_color),
                                            );
                                        },
                                    );
                                });
                            }
                        }
                    });
                }
            });
        });
}
