use crate::app::{AiChatMessage, AiSender, VisualizerApp};
use crate::model::ThemePalette;
use eframe::egui::{self, Color32, FontId, Frame, Pos2, Rect, RichText, Rounding, Stroke};
use std::time::{Duration, Instant};

// Aged phosphor CRT palette — warm yellowish-green, not neon.
const CRT_BG: Color32 = Color32::from_rgb(5, 10, 3);
const PHOSPHOR_BRIGHT: Color32 = Color32::from_rgb(140, 200, 40);
const PHOSPHOR_DIM: Color32 = Color32::from_rgb(80, 130, 20);
const PHOSPHOR_FAINT: Color32 = Color32::from_rgb(45, 75, 12);
const BEZEL_BORDER: Color32 = Color32::from_rgb(30, 55, 15);
const SCANLINE_DARK: Color32 = Color32::from_rgba_premultiplied(0, 0, 0, 35);

pub fn render_ai_chat_dock(app: &mut VisualizerApp, ui: &mut egui::Ui, _p: &ThemePalette) {
    let z = app.canvas_zoom;
    let margin = (10.0 * z).clamp(6.0, 14.0);

    let mut elapsed_ms = app.terminal_boot_start.elapsed().as_millis();
    if elapsed_ms < 3800 && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
        app.terminal_boot_start = app
            .terminal_boot_start
            .checked_sub(Duration::from_secs(10))
            .unwrap_or(app.terminal_boot_start);
        elapsed_ms = app.terminal_boot_start.elapsed().as_millis();
    }
    let is_booting = elapsed_ms < 3800;
    let mut any_animating = is_booting;

    let outer_response = Frame::none()
        .fill(CRT_BG)
        .rounding(Rounding::same(4.0 * z))
        .stroke(Stroke::new(2.0_f32, BEZEL_BORDER))
        .inner_margin(margin)
        .show(ui, |ui| {
            // Window title bar
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("algobuddy@helper:~")
                        .font(FontId::monospace(10.5 * z))
                        .color(PHOSPHOR_DIM),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new("--  []  X")
                            .font(FontId::monospace(10.0 * z))
                            .color(PHOSPHOR_FAINT),
                    );
                });
            });

            ui.add_space(2.0 * z);

            let scroll_height = (130.0 * z).clamp(90.0, 210.0);
            egui::ScrollArea::vertical()
                .id_source("retro_crt_terminal_scroll")
                .max_height(scroll_height)
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    // Phase 1: Teletype OS Banner
                    if let Some((text, streaming)) =
                        typewriter_text("AlgoBuddy OS Personal Computer", 50, elapsed_ms, 12)
                    {
                        if streaming {
                            any_animating = true;
                        }
                        ui.label(
                            RichText::new(text)
                                .font(FontId::monospace(11.0 * z))
                                .color(PHOSPHOR_DIM),
                        );
                    }

                    if let Some((text, streaming)) =
                        typewriter_text("Version 0.9.1", 450, elapsed_ms, 12)
                    {
                        if streaming {
                            any_animating = true;
                        }
                        ui.label(
                            RichText::new(text)
                                .font(FontId::monospace(11.0 * z))
                                .color(PHOSPHOR_DIM),
                        );
                    }

                    if elapsed_ms >= 700 {
                        let problem_title = app.current_problem.details().title;
                        let loading_base = format!("Loading \"{}\"...", problem_title);
                        let loading_done = format!("Loading \"{}\"... DONE", problem_title);

                        let text = if elapsed_ms < 1100 {
                            let (part, streaming) =
                                typewriter_text(&loading_base, 700, elapsed_ms, 14)
                                    .unwrap_or_default();
                            if streaming {
                                any_animating = true;
                            }
                            part
                        } else {
                            loading_done
                        };
                        ui.add_space(2.0 * z);
                        ui.label(
                            RichText::new(text)
                                .font(FontId::monospace(11.0 * z))
                                .color(PHOSPHOR_FAINT),
                        );
                    }

                    // Phase 2: C:\>algobuddy.exe command typed and loading dots
                    if elapsed_ms >= 1300 {
                        let (cmd_text, is_typing_cmd) = if elapsed_ms < 1700 {
                            typewriter_text("C:\\>algobuddy.exe", 1300, elapsed_ms, 18)
                                .unwrap_or_default()
                        } else if elapsed_ms < 3200 {
                            any_animating = true;
                            let dot_count = ((elapsed_ms - 1700) / 250) % 4 + 1;
                            let dots = ".".repeat(dot_count as usize);
                            (format!("C:\\>algobuddy.exe{dots}"), true)
                        } else {
                            ("C:\\>algobuddy.exe".to_string(), false)
                        };

                        if is_typing_cmd {
                            any_animating = true;
                        }

                        ui.label(
                            RichText::new(cmd_text)
                                .font(FontId::monospace(11.5 * z))
                                .color(PHOSPHOR_DIM),
                        );
                    }

                    // Phase 3: Loaded & Helper greeting
                    if elapsed_ms >= 3200 {
                        ui.add_space(2.0 * z);

                        if let Some((text, streaming)) = typewriter_text(
                            "algobuddy.exe loaded. Need any help? ^_^",
                            3200,
                            elapsed_ms,
                            14,
                        ) {
                            if streaming {
                                any_animating = true;
                            }
                            ui.label(
                                RichText::new(text)
                                    .font(FontId::monospace(11.5 * z))
                                    .color(PHOSPHOR_BRIGHT),
                            );
                        }

                        ui.add_space(4.0 * z);

                        // Chat history with character-by-character typewriter rendering for AI
                        for msg in &app.ai_chat_history {
                            match msg.sender {
                                AiSender::User => {
                                    ui.label(
                                        RichText::new(format!("C:\\>{}", msg.text))
                                            .font(FontId::monospace(11.5 * z))
                                            .color(PHOSPHOR_BRIGHT),
                                    );
                                }
                                AiSender::AlgoBuddyAi => {
                                    let msg_elapsed = msg.created_at.elapsed().as_millis();
                                    let ms_per_char = if msg.text.starts_with("Available commands:")
                                    {
                                        7
                                    } else {
                                        14
                                    };
                                    let char_count = (msg_elapsed / ms_per_char) as usize;
                                    let total_chars = msg.text.chars().count();

                                    let display_text = if char_count < total_chars {
                                        any_animating = true;
                                        let partial: String =
                                            msg.text.chars().take(char_count).collect();
                                        partial
                                    } else {
                                        msg.text.clone()
                                    };

                                    ui.label(
                                        RichText::new(display_text)
                                            .font(FontId::monospace(11.0 * z))
                                            .color(PHOSPHOR_DIM),
                                    );
                                    ui.add_space(3.0 * z);
                                }
                            }
                        }
                    }
                });

            // Input line — only after initial boot
            if !is_booting {
                ui.add_space(4.0 * z);

                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 2.0 * z;

                    ui.label(
                        RichText::new("C:\\>")
                            .font(FontId::monospace(11.5 * z))
                            .strong()
                            .color(PHOSPHOR_BRIGHT),
                    );

                    let blink = (ui.input(|i| i.time) * 2.0).fract() < 0.5;
                    let hint_cursor = if blink { "_" } else { " " };

                    let text_edit_response = ui.add(
                        egui::TextEdit::singleline(&mut app.ai_input_text)
                            .font(FontId::monospace(11.5 * z))
                            .text_color(PHOSPHOR_BRIGHT)
                            .frame(false)
                            .margin(egui::Margin::ZERO)
                            .hint_text(RichText::new(hint_cursor).color(PHOSPHOR_BRIGHT))
                            .desired_width(ui.available_width() - (10.0 * z)),
                    );

                    let submitted = text_edit_response.lost_focus()
                        && ui.input(|i| i.key_pressed(egui::Key::Enter));

                    if submitted {
                        if !app.ai_input_text.trim().is_empty() {
                            let query = app.ai_input_text.clone();
                            app.ai_input_text.clear();
                            send_ai_message(app, &query);
                        } else {
                            // Fast-forward animation if they hit enter on an empty line
                            for msg in &mut app.ai_chat_history {
                                msg.created_at = msg
                                    .created_at
                                    .checked_sub(Duration::from_secs(60))
                                    .unwrap_or(msg.created_at);
                            }
                        }
                        text_edit_response.request_focus();
                    }
                });
            }
        });

    if any_animating {
        ui.ctx().request_repaint_after(Duration::from_millis(20));
    } else {
        ui.ctx().request_repaint_after(Duration::from_millis(400));
    }

    // CRT scanline overlay — paint dark horizontal lines across the entire terminal rect.
    let terminal_rect = outer_response.response.rect;
    draw_scanlines(ui, terminal_rect, z);

    // Vignette — darken edges to simulate CRT curvature falloff.
    draw_vignette(ui, terminal_rect, z);
}

fn typewriter_text(
    full: &str,
    start_ms: u128,
    elapsed_ms: u128,
    ms_per_char: u128,
) -> Option<(String, bool)> {
    if elapsed_ms < start_ms {
        return None;
    }
    let chars_to_show = ((elapsed_ms - start_ms) / ms_per_char) as usize;
    let total = full.chars().count();
    if chars_to_show < total {
        let partial: String = full.chars().take(chars_to_show).collect();
        Some((partial, true))
    } else {
        Some((full.to_string(), false))
    }
}

fn draw_scanlines(ui: &mut egui::Ui, rect: Rect, z: f32) {
    let painter = ui.painter();
    let line_spacing = (3.0 * z).max(2.0);
    let mut y = rect.min.y;
    while y < rect.max.y {
        painter.rect_filled(
            Rect::from_min_size(Pos2::new(rect.min.x, y), egui::vec2(rect.width(), 1.0)),
            Rounding::ZERO,
            SCANLINE_DARK,
        );
        y += line_spacing;
    }
}

fn draw_vignette(ui: &mut egui::Ui, rect: Rect, z: f32) {
    let painter = ui.painter();
    let edge = (18.0 * z).clamp(8.0, 30.0);
    let shadow = Color32::from_rgba_premultiplied(0, 0, 0, 50);

    // Top edge
    painter.rect_filled(
        Rect::from_min_size(rect.min, egui::vec2(rect.width(), edge)),
        Rounding::ZERO,
        shadow,
    );
    // Bottom edge
    painter.rect_filled(
        Rect::from_min_size(
            Pos2::new(rect.min.x, rect.max.y - edge),
            egui::vec2(rect.width(), edge),
        ),
        Rounding::ZERO,
        shadow,
    );
    // Left edge
    painter.rect_filled(
        Rect::from_min_size(rect.min, egui::vec2(edge, rect.height())),
        Rounding::ZERO,
        shadow,
    );
    // Right edge
    painter.rect_filled(
        Rect::from_min_size(
            Pos2::new(rect.max.x - edge, rect.min.y),
            egui::vec2(edge, rect.height()),
        ),
        Rounding::ZERO,
        shadow,
    );
}

fn send_ai_message(app: &mut VisualizerApp, query: &str) {
    let q = query.trim();
    if q.is_empty() {
        return;
    }

    app.ai_chat_history.push(AiChatMessage {
        sender: AiSender::User,
        text: q.to_string(),
        created_at: Instant::now(),
    });

    let response = crate::terminal::generate_offline_ai_response(app, q);
    if response.is_empty() {
        app.ai_chat_history.clear();
    } else {
        app.ai_chat_history.push(AiChatMessage {
            sender: AiSender::AlgoBuddyAi,
            text: response,
            created_at: Instant::now(),
        });
    }
}
