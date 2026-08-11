use crate::app::{SettingsFocusTarget, SettingsPage, VisualizerApp};
use crate::model::{ColorblindMode, Theme, ThemePalette};
use crate::shortcuts::{key_display_label, ShortcutAction, ShortcutBindings};
use eframe::egui::{self, Color32, Frame, RichText, Stroke};

fn render_shortcut_binding_button(
    app: &mut VisualizerApp,
    ui: &mut egui::Ui,
    action: ShortcutAction,
    width: f32,
    p: &ThemePalette,
    ctx: &egui::Context,
) {
    let current_key = app.shortcut_bindings.key(action);
    let is_capturing = app.shortcut_capture == Some(action);
    let capture_error = if is_capturing {
        app.shortcut_rebind_error.as_deref()
    } else {
        None
    };
    let button_text = if is_capturing {
        "Press a key..."
    } else {
        key_display_label(current_key)
    };
    let mut help_text = if is_capturing {
        format!(
            "Listening for {}. Press a key; Escape cancels.",
            action.settings_label()
        )
    } else {
        format!(
            "Change {}. Current key: {}.",
            action.settings_label(),
            key_display_label(current_key)
        )
    };
    if let Some(error) = capture_error {
        help_text.push(' ');
        help_text.push_str(error);
    }

    let mut button =
        egui::Button::new(RichText::new(button_text).strong().color(if is_capturing {
            p.amber
        } else {
            p.text_primary
        }))
        .min_size(egui::vec2(width, 30.0));
    if is_capturing {
        button = button.fill(p.cell_bg).stroke(Stroke::new(1.0_f32, p.amber));
    }

    let response = ui
        .add_sized([width, 30.0], button)
        .on_hover_text(&help_text);
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), help_text.clone())
    });
    if let Some(error) = capture_error {
        ui.colored_label(p.red, error);
    }

    if response.clicked() {
        response.request_focus();
        if is_capturing {
            app.cancel_shortcut_capture();
        } else {
            app.begin_shortcut_capture(action, ctx);
        }
    }
}

fn render_shortcut_settings_page(
    app: &mut VisualizerApp,
    ui: &mut egui::Ui,
    p: &ThemePalette,
    ctx: &egui::Context,
) {
    let back_response = ui.button("Back to Settings");
    if app.settings_focus_target == Some(SettingsFocusTarget::ShortcutBackButton) {
        back_response.request_focus();
        app.settings_focus_target = None;
    }
    if back_response.clicked() {
        app.return_to_general_settings();
        return;
    }

    ui.add_space(8.0);
    ui.heading(
        RichText::new("Keyboard Shortcuts")
            .color(p.cyan)
            .strong()
            .size(15.0),
    );
    ui.add_space(4.0);
    ui.label(
        RichText::new(
            "Select a shortcut, then press a new key. Supported keys are letters, numbers, arrows, Space, +, and -. Press Escape to cancel.",
        )
        .color(p.text_muted),
    );

    if let Some(action) = app.shortcut_capture {
        ui.add_space(8.0);
        Frame::none()
            .fill(p.cell_bg)
            .stroke(Stroke::new(1.0_f32, p.amber))
            .inner_margin(8.0)
            .show(ui, |ui| {
                ui.label(
                    RichText::new(format!("Waiting for: {}", action.settings_label()))
                        .strong()
                        .color(p.amber),
                );
                ui.label("Press a supported key now, or press Escape to cancel.");
            });
    }

    ui.add_space(8.0);
    let defaults = ShortcutBindings::default();
    if ui
        .add_enabled(
            app.shortcut_bindings != defaults,
            egui::Button::new("Restore Default Shortcuts"),
        )
        .on_hover_text("Restore every keyboard shortcut to its original key")
        .clicked()
    {
        app.restore_default_shortcuts();
    }

    ui.add_space(8.0);
    let shortcut_interact_height = ui.spacing().interact_size.y.max(30.0);
    ui.spacing_mut().interact_size.y = shortcut_interact_height;
    if ui.available_width() >= 380.0 {
        egui::Grid::new("keyboard_shortcut_capture_grid")
            .num_columns(2)
            .spacing([18.0, 8.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label(RichText::new("Action").strong().color(p.text_primary));
                ui.label(RichText::new("Shortcut").strong().color(p.text_primary));
                ui.end_row();

                for action in ShortcutAction::ALL {
                    ui.label(action.settings_label());
                    ui.vertical(|ui| {
                        ui.set_width(132.0);
                        render_shortcut_binding_button(app, ui, action, 132.0, p, ctx);
                    });
                    ui.end_row();
                }
            });
    } else {
        for action in ShortcutAction::ALL {
            ui.group(|ui| {
                ui.set_width(ui.available_width());
                ui.label(RichText::new(action.settings_label()).strong());
                let button_width = (ui.available_width() - 8.0).max(100.0);
                ui.vertical(|ui| {
                    ui.set_width(button_width);
                    render_shortcut_binding_button(app, ui, action, button_width, p, ctx);
                });
            });
            ui.add_space(4.0);
        }
    }

    ui.add_space(8.0);
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new("Mouse Zoom").strong().color(p.text_primary));
        ui.label("Ctrl + Scroll Wheel (fixed gesture)");
    });
    #[cfg(not(target_arch = "wasm32"))]
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new("Fullscreen").strong().color(p.text_primary));
        ui.label("F11 (fixed shortcut)");
    });

    ui.add_space(12.0);
    if ui
        .button(RichText::new("Close Settings").strong())
        .clicked()
    {
        app.close_settings();
    }
}

pub fn render_settings_modal(app: &mut VisualizerApp, ctx: &egui::Context) {
    if !app.show_settings_modal {
        return;
    }

    let p = app.current_palette();
    let mut is_open = true;
    let available_rect = ctx.available_rect();
    let window_max_width = (available_rect.width() - 24.0).max(1.0);
    let window_max_height = (available_rect.height() - 24.0).max(1.0);
    let title = match app.settings_page {
        SettingsPage::General => "AlgoBuddy UI Settings & Accessibility",
        SettingsPage::KeyboardShortcuts => "AlgoBuddy Keyboard Shortcut Settings",
    };

    egui::Window::new(title)
        .id(egui::Id::new("settings_modal"))
        .open(&mut is_open)
        .resizable(true)
        .collapsible(false)
        .default_width(460.0_f32.min(window_max_width))
        .min_width(320.0_f32.min(window_max_width))
        .max_width(window_max_width)
        .max_height(window_max_height)
        .vscroll(true)
        .frame(
            Frame::window(&ctx.style())
                .fill(p.sidebar_bg)
                .stroke(Stroke::new(1.0_f32, p.cell_border)),
        )
        .show(ctx, |ui| {
            if app.settings_page == SettingsPage::KeyboardShortcuts {
                render_shortcut_settings_page(app, ui, &p, ctx);
                return;
            }

            ui.heading(
                RichText::new("UI Theme Selection")
                    .color(p.cyan)
                    .strong()
                    .size(15.0),
            );
            ui.add_space(6.0);

            ui.horizontal_wrapped(|ui| {
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

            let shortcut_menu_help =
                "Keyboard Shortcuts. Open settings to review or change key bindings.";
            let shortcut_menu_response = ui
                .add_sized(
                    [ui.available_width(), 34.0],
                    egui::Button::new(RichText::new("Keyboard Shortcuts").strong().color(p.cyan)),
                )
                .on_hover_text(shortcut_menu_help);
            shortcut_menu_response.widget_info(|| {
                egui::WidgetInfo::labeled(
                    egui::WidgetType::Button,
                    ui.is_enabled(),
                    shortcut_menu_help,
                )
            });
            if app.settings_focus_target == Some(SettingsFocusTarget::KeyboardMenuButton) {
                shortcut_menu_response.request_focus();
                app.settings_focus_target = None;
            }
            if shortcut_menu_response.clicked() {
                app.open_shortcut_settings();
            }

            ui.add_space(14.0);
            ui.separator();
            ui.add_space(8.0);

            ui.heading(
                RichText::new("Current Settings")
                    .color(p.amber)
                    .strong()
                    .size(15.0),
            );
            ui.add_space(6.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new("Active Theme:").color(p.text_muted));
                ui.label(
                    RichText::new(app.theme.label())
                        .color(p.text_primary)
                        .strong(),
                );
            });
            ui.horizontal_wrapped(|ui| {
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
                app.close_settings();
            }
        });

    if !is_open {
        app.close_settings();
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
