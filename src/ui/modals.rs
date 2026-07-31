use crate::app::VisualizerApp;
use crate::model::{ColorblindMode, Theme};
use eframe::egui::{self, Color32, Frame, RichText, Stroke};

pub fn render_settings_modal(app: &mut VisualizerApp, ctx: &egui::Context) {
    if !app.show_settings_modal {
        return;
    }

    let p = app.current_palette();
    let mut is_open = true;

    egui::Window::new("⚙ AlgoBuddy UI Settings & Accessibility")
        .open(&mut is_open)
        .resizable(false)
        .collapsible(false)
        .default_width(380.0)
        .frame(
            Frame::window(&ctx.style())
                .fill(p.sidebar_bg)
                .stroke(Stroke::new(1.0_f32, p.cell_border)),
        )
        .show(ctx, |ui| {
            ui.heading(
                RichText::new("UI Theme Selection")
                    .color(p.cyan)
                    .strong()
                    .size(15.0),
            );
            ui.add_space(6.0);

            ui.horizontal(|ui| {
                if ui
                    .selectable_label(app.theme == Theme::DarkVSCode, "VS Code Dark")
                    .clicked()
                {
                    app.theme = Theme::DarkVSCode;
                }
                if ui
                    .selectable_label(app.theme == Theme::DarkCyber, "Cyber Navy")
                    .clicked()
                {
                    app.theme = Theme::DarkCyber;
                }
                if ui
                    .selectable_label(app.theme == Theme::LightClean, "Clean Light")
                    .clicked()
                {
                    app.theme = Theme::LightClean;
                }
            });

            ui.add_space(14.0);
            ui.separator();
            ui.add_space(8.0);

            ui.heading(
                RichText::new("Colorblindness & Accessibility Filter")
                    .color(p.cyan)
                    .strong()
                    .size(15.0),
            );
            ui.add_space(6.0);

            if ui
                .selectable_label(
                    app.colorblind_mode == ColorblindMode::Off,
                    "Off (Standard Red / Emerald Green)",
                )
                .clicked()
            {
                app.colorblind_mode = ColorblindMode::Off;
            }
            if ui
                .selectable_label(
                    app.colorblind_mode == ColorblindMode::RedGreenSafe,
                    "Protan / Deuteran (Cobalt Blue / Safety Orange)",
                )
                .clicked()
            {
                app.colorblind_mode = ColorblindMode::RedGreenSafe;
            }
            if ui
                .selectable_label(
                    app.colorblind_mode == ColorblindMode::HighContrast,
                    "High Contrast B&W",
                )
                .clicked()
            {
                app.colorblind_mode = ColorblindMode::HighContrast;
            }
            ui.add_space(14.0);
            ui.separator();
            ui.add_space(8.0);

            ui.heading(
                RichText::new("Keyboard Shortcuts")
                    .color(p.cyan)
                    .strong()
                    .size(15.0),
            );
            ui.add_space(6.0);
            egui::Grid::new("keyboard_shortcuts_grid")
                .num_columns(2)
                .spacing([16.0, 6.0])
                .show(ui, |ui| {
                    ui.label(RichText::new("Spacebar").strong().color(p.text_primary));
                    ui.label("Play / Pause Timeline Animation");
                    ui.end_row();

                    ui.label(RichText::new("← / → Arrows").strong().color(p.text_primary));
                    ui.label("Previous / Next Step");
                    ui.end_row();

                    ui.label(RichText::new("R Key").strong().color(p.text_primary));
                    ui.label("Reset Timeline to Step 1");
                    ui.end_row();

                    ui.label(RichText::new("↑ / ↓ Arrows").strong().color(p.text_primary));
                    ui.label("Speed Up / Slow Down Playback");
                    ui.end_row();
                });

            ui.add_space(14.0);
            ui.separator();
            ui.add_space(8.0);

            ui.heading(
                RichText::new("Developer & Release Mode")
                    .color(p.amber)
                    .strong()
                    .size(15.0),
            );
            ui.add_space(6.0);
            ui.checkbox(
                &mut app.show_unaudited,
                "Show Unaudited / Experimental Problems (Dev Mode)",
            );
            ui.add_space(4.0);
            if !app.show_unaudited {
                ui.label(
                    RichText::new("Public Release Mode: Only showing 100% audited problems.")
                        .italics()
                        .font(egui::FontId::proportional(11.0))
                        .color(p.emerald_text),
                );
            } else {
                ui.label(
                    RichText::new("Dev Testing Mode: Showing all 134 problem visualizers.")
                        .italics()
                        .font(egui::FontId::proportional(11.0))
                        .color(p.amber),
                );
            }

            ui.horizontal(|ui| {
                ui.label(RichText::new("Active Theme:").color(p.text_muted));
                ui.label(
                    RichText::new(app.theme.label())
                        .color(p.text_primary)
                        .strong(),
                );
            });
            ui.horizontal(|ui| {
                ui.label(RichText::new("Accessibility:").color(p.text_muted));
                ui.label(
                    RichText::new(app.colorblind_mode.label())
                        .color(p.emerald_text)
                        .strong(),
                );
            });

            ui.add_space(12.0);
            if ui
                .button(RichText::new("Close Settings").strong())
                .clicked()
            {
                app.show_settings_modal = false;
            }
        });

    if !is_open {
        app.show_settings_modal = false;
    }
}

pub fn render_reset_confirm_modal(app: &mut VisualizerApp, ctx: &egui::Context) {
    if !app.show_reset_confirm_modal {
        return;
    }

    egui::Window::new("Confirm Reset")
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.label("Are you sure you want to reset all completed problem checkmarks?");
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                if ui.button("Cancel").clicked() {
                    app.show_reset_confirm_modal = false;
                }

                let dark_red = Color32::from_rgb(210, 40, 65);
                let confirm_btn = egui::Button::new(
                    RichText::new("Confirm Reset")
                        .color(Color32::WHITE)
                        .strong(),
                )
                .fill(dark_red);

                if ui.add(confirm_btn).clicked() {
                    app.completed_problems.clear();
                    app.show_reset_confirm_modal = false;
                }
            });
        });
}
