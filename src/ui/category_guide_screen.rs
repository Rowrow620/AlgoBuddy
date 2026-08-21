use crate::app::VisualizerApp;
use crate::model::{get_category_guide, Category, ThemePalette};
use eframe::egui::{self, Frame, RichText, Rounding, Stroke};

pub fn render_fullscreen_category_masterclass(
    app: &mut VisualizerApp,
    ctx: &egui::Context,
    p: &ThemePalette,
    cat: Category,
) {
    let guide = get_category_guide(cat);

    egui::CentralPanel::default()
        .frame(Frame::none().fill(p.bg_dark).inner_margin(24.0))
        .show(ctx, |ui| {
            // Header Navigation & Category Switcher.
            ui.horizontal(|ui| {
                if ui
                    .button(
                        RichText::new("Back to Visualizer")
                            .strong()
                            .color(p.cyan)
                            .size(13.0),
                    )
                    .clicked()
                {
                    app.return_to_visualizer();
                }

                ui.add_space(8.0);

                if ui
                    .button(
                        RichText::new("Back to Dashboard")
                            .strong()
                            .color(p.cyan)
                            .size(13.0),
                    )
                    .clicked()
                {
                    app.open_dashboard();
                }

                ui.add_space(16.0);

                ui.heading(
                    RichText::new("Category Guide")
                        .color(p.amber)
                        .strong()
                        .size(18.0),
                );

                ui.add_space(16.0);

                let mut current_cat = cat;
                egui::ComboBox::from_id_source("category_masterclass_selector")
                    .selected_text(RichText::new(cat.name()).strong().color(p.cyan))
                    .show_ui(ui, |ui| {
                        for &c in Category::all() {
                            if ui.selectable_value(&mut current_cat, c, c.name()).clicked() {
                                app.open_category_masterclass(c);
                            }
                        }
                    });
            });

            ui.add_space(6.0);
            ui.separator();
            ui.add_space(8.0);

            // Main Content Area.
            egui::ScrollArea::vertical().show(ui, |ui| {
                render_category_guide_overview_tab(ui, p, &guide);
            });
        });
}

fn render_category_guide_overview_tab(
    ui: &mut egui::Ui,
    p: &ThemePalette,
    guide: &crate::model::CategoryGuideData,
) {
    ui.add_space(4.0);
    ui.label(
        RichText::new(guide.summary)
            .color(p.text_primary)
            .size(14.0),
    );
    ui.add_space(12.0);

    egui::Frame::none()
        .fill(p.sidebar_bg)
        .rounding(Rounding::same(8.0))
        .stroke(Stroke::new(1.0_f32, p.cell_border))
        .inner_margin(14.0)
        .show(ui, |ui| {
            ui.heading(RichText::new("How It Works").color(p.cyan).size(16.0));
            ui.add_space(6.0);
            ui.label(
                RichText::new(guide.how_it_works)
                    .color(p.text_primary)
                    .size(13.5),
            );
        });

    ui.add_space(12.0);

    egui::Frame::none()
        .fill(p.sidebar_bg)
        .rounding(Rounding::same(8.0))
        .stroke(Stroke::new(1.0_f32, p.cell_border))
        .inner_margin(14.0)
        .show(ui, |ui| {
            ui.heading(
                RichText::new("Common Patterns & Key Techniques")
                    .color(p.cyan)
                    .size(16.0),
            );
            ui.add_space(6.0);
            for pattern in guide.key_patterns {
                ui.label(
                    RichText::new(format!("• {pattern}"))
                        .color(p.text_primary)
                        .size(13.5),
                );
                ui.add_space(3.0);
            }
        });

    ui.add_space(12.0);

    egui::Frame::none()
        .fill(p.sidebar_bg)
        .rounding(Rounding::same(8.0))
        .stroke(Stroke::new(1.0_f32, p.cell_border))
        .inner_margin(14.0)
        .show(ui, |ui| {
            ui.heading(
                RichText::new("Standard Operations Complexity Cheat Sheet")
                    .color(p.cyan)
                    .size(16.0),
            );
            ui.add_space(8.0);

            egui::Grid::new("fullscreen_complexity_table_grid")
                .striped(true)
                .min_col_width(220.0)
                .show(ui, |ui| {
                    ui.label(
                        RichText::new("Operation")
                            .strong()
                            .color(p.text_muted)
                            .size(13.0),
                    );
                    ui.label(
                        RichText::new("Time Complexity")
                            .strong()
                            .color(p.cyan)
                            .size(13.0),
                    );
                    ui.label(
                        RichText::new("Space Complexity")
                            .strong()
                            .color(p.purple)
                            .size(13.0),
                    );
                    ui.end_row();

                    for (op, time, space) in guide.complexity_table {
                        ui.label(RichText::new(*op).color(p.text_primary).size(13.0));
                        ui.label(RichText::new(*time).color(p.cyan).strong().size(13.0));
                        ui.label(RichText::new(*space).color(p.purple).strong().size(13.0));
                        ui.end_row();
                    }
                });
        });

    ui.add_space(12.0);

    egui::Frame::none()
        .fill(p.sidebar_bg)
        .rounding(Rounding::same(8.0))
        .stroke(Stroke::new(1.0_f32, p.cell_border))
        .inner_margin(14.0)
        .show(ui, |ui| {
            ui.heading(
                RichText::new("Pro Tips & Interview Insights")
                    .color(p.cyan)
                    .size(16.0),
            );
            ui.add_space(6.0);
            for tip in guide.pro_tips {
                ui.label(RichText::new(format!("• {tip}")).color(p.amber).size(13.5));
                ui.add_space(3.0);
            }
        });
}
