use crate::app::VisualizerApp;
use crate::model::ThemePalette;
use eframe::egui::{self, RichText, Rounding, Stroke};

impl VisualizerApp {
    #[allow(clippy::too_many_arguments, clippy::needless_range_loop)]
    pub(super) fn render_sudoku(
        &self,
        ui: &mut egui::Ui,
        p: &ThemePalette,
        board: &[[char; 9]; 9],
        active_r: Option<usize>,
        active_c: Option<usize>,
        dup_pos: Option<(usize, usize)>,
        is_valid: Option<bool>,
    ) {
        let z = self.canvas_zoom;
        let font_title = (16.0 * z).max(10.0);
        let font_cell = (16.0 * z).max(9.0);
        let margin = (8.0 * z).max(3.0);

        ui.heading(
            RichText::new("9x9 Sudoku Board Validation Grid")
                .color(p.cyan)
                .size(font_title),
        );
        ui.add_space(8.0 * z);

        ui.group(|ui| {
            ui.vertical(|ui| {
                for r in 0..9 {
                    if r > 0 && r % 3 == 0 {
                        ui.add_space(4.0 * z);
                    }
                    ui.horizontal(|ui| {
                        for c in 0..9 {
                            if c > 0 && c % 3 == 0 {
                                ui.add_space(4.0 * z);
                            }

                            let val = board[r][c];
                            let is_active = active_r == Some(r) && active_c == Some(c);
                            let is_row_col = active_r == Some(r) || active_c == Some(c);
                            let is_dup = dup_pos == Some((r, c));

                            let fill = if is_dup {
                                p.red
                            } else if is_active {
                                p.amber
                            } else if is_row_col {
                                p.code_active_bg
                            } else if val != '.' {
                                p.cell_bg
                            } else {
                                p.sidebar_bg
                            };

                            let border_color = if (r / 3 * 3 + c / 3) % 2 == 0 {
                                p.purple
                            } else {
                                p.cell_border
                            };

                            egui::Frame::none()
                                .fill(fill)
                                .rounding(Rounding::same(4.0 * z))
                                .stroke(Stroke::new(1.0_f32 * z, border_color))
                                .inner_margin(margin)
                                .show(ui, |ui| {
                                    let mut text_rt = RichText::new(val.to_string())
                                        .font(egui::FontId::monospace(font_cell))
                                        .strong();
                                    if val == '.' {
                                        text_rt = text_rt.color(p.text_dim);
                                    } else {
                                        text_rt = text_rt.color(p.text_primary);
                                    }
                                    ui.label(text_rt);
                                });
                        }
                    });
                }
            });
        });

        if let Some(valid) = is_valid {
            ui.add_space(20.0 * z);
            if valid {
                ui.heading(
                    RichText::new(
                        "Valid Sudoku Board! All rows, cols & 3x3 boxes satisfy constraint.",
                    )
                    .color(p.emerald_text)
                    .size((18.0 * z).max(11.0)),
                );
            } else {
                ui.heading(
                    RichText::new("Invalid Sudoku Board! Duplicate digit detected.")
                        .color(p.red)
                        .size((18.0 * z).max(11.0)),
                );
            }
        }
    }
}
