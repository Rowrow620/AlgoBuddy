use crate::app::VisualizerApp;
use crate::model::ThemePalette;
use eframe::egui::{self, Color32, RichText, Rounding, Stroke};

impl VisualizerApp {
    #[allow(clippy::too_many_arguments)]
    pub(in crate::ui::canvas) fn render_two_sum(
        &self,
        ui: &mut egui::Ui,
        p: &ThemePalette,
        nums: &[i32],
        target: i32,
        active_idx: Option<usize>,
        secondary_idx: Option<usize>,
        map: &std::collections::BTreeMap<i32, usize>,
        found: Option<(usize, usize)>,
    ) {
        let z = self.canvas_zoom;
        let margin = (12.0 * z).max(4.0);
        let font_sz = (18.0 * z).max(9.0);
        let font_title = (18.0 * z).max(10.0);

        ui.heading(
            RichText::new(format!("Target Sum: {}", target))
                .color(p.cyan)
                .size(font_title),
        );
        ui.add_space(10.0 * z);

        ui.group(|ui| {
            ui.label(
                RichText::new("NUMS ARRAY")
                    .font(egui::FontId::monospace((11.0 * z).max(8.0)))
                    .color(p.text_muted),
            );
            ui.add_space(4.0 * z);
            ui.horizontal(|ui| {
                for (i, &num) in nums.iter().enumerate() {
                    let is_found = found.is_some_and(|(a, b)| a == i || b == i);
                    let is_primary = active_idx == Some(i);
                    let is_sec = secondary_idx == Some(i);

                    let fill = if is_found {
                        p.emerald
                    } else if is_primary {
                        p.amber
                    } else if is_sec {
                        p.pink
                    } else {
                        p.cell_bg
                    };

                    let (label_color, val_color) = if is_found || is_primary || is_sec {
                        (Color32::from_rgb(30, 35, 45), Color32::from_rgb(30, 35, 45))
                    } else {
                        (p.text_muted, Color32::WHITE)
                    };

                    egui::Frame::none()
                        .fill(fill)
                        .rounding(Rounding::same(8.0 * z))
                        .stroke(Stroke::new(1.0_f32 * z, p.cell_border))
                        .inner_margin(margin)
                        .show(ui, |ui| {
                            ui.vertical(|ui| {
                                let sec_name = if self.selected_approach_id == 0 {
                                    "prevMap[diff]"
                                } else {
                                    "j"
                                };
                                let label = if is_primary {
                                    "i"
                                } else if is_sec {
                                    sec_name
                                } else {
                                    ""
                                };
                                let header = if label.is_empty() {
                                    format!("i={}", i)
                                } else {
                                    format!("i={} ({})", i, label)
                                };
                                ui.label(
                                    RichText::new(header)
                                        .font(egui::FontId::proportional((11.0 * z).max(8.0)))
                                        .color(label_color),
                                );
                                ui.label(
                                    RichText::new(num.to_string())
                                        .font(egui::FontId::monospace(font_sz))
                                        .strong()
                                        .color(val_color),
                                );
                            });
                        });
                }
            });
        });

        ui.add_space(20.0 * z);

        if self.selected_approach_id == 0 {
            ui.group(|ui| {
                ui.label(
                    RichText::new("PREVMAP {value -> index}")
                        .font(egui::FontId::monospace((11.0 * z).max(8.0)))
                        .color(p.text_muted),
                );
                ui.add_space(4.0 * z);
                ui.horizontal(|ui| {
                    if map.is_empty() {
                        ui.label(RichText::new("Empty {}").italics().color(p.text_dim));
                    } else {
                        for (&val, &idx) in map {
                            egui::Frame::none()
                                .fill(p.cell_bg)
                                .rounding(Rounding::same(8.0 * z))
                                .stroke(Stroke::new(1.0_f32 * z, p.purple))
                                .inner_margin(margin)
                                .show(ui, |ui| {
                                    ui.vertical(|ui| {
                                        ui.label(
                                            RichText::new(format!("val={}", val))
                                                .font(egui::FontId::monospace((14.0 * z).max(9.0)))
                                                .strong()
                                                .color(p.cyan),
                                        );
                                        ui.label(
                                            RichText::new(format!("idx={}", idx))
                                                .font(egui::FontId::monospace((12.0 * z).max(8.0)))
                                                .color(p.text_muted),
                                        );
                                    });
                                });
                        }
                    }
                });
            });
        }

        if let Some((a, b)) = found {
            ui.add_space(20.0 * z);
            ui.heading(
                RichText::new(format!("Result Pair Found! Indices: [{}, {}]", a, b))
                    .color(p.emerald_text)
                    .size((18.0 * z).max(11.0)),
            );
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(in crate::ui::canvas) fn render_valid_anagram(
        &self,
        ui: &mut egui::Ui,
        p: &ThemePalette,
        s: &str,
        t: &str,
        s_counts: &[usize; 26],
        t_counts: &[usize; 26],
        active_s: Option<usize>,
        active_t: Option<usize>,
        is_anagram: Option<bool>,
    ) {
        let z = self.canvas_zoom;
        let font_title = (16.0 * z).max(10.0);
        let font_label = (12.0 * z).max(8.0);
        let font_char = (16.0 * z).max(9.0);
        let margin = (8.0 * z).max(4.0);

        ui.heading(
            RichText::new("Character Comparison")
                .color(p.cyan)
                .size(font_title),
        );
        ui.add_space(8.0 * z);

        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.label(
                    RichText::new(format!("STRING s: \"{}\"", s))
                        .font(egui::FontId::monospace(font_label))
                        .color(p.text_muted),
                );
                ui.horizontal(|ui| {
                    for (i, c) in s.chars().enumerate() {
                        let fill = if active_s == Some(i) {
                            p.amber
                        } else {
                            p.cell_bg
                        };
                        egui::Frame::none()
                            .fill(fill)
                            .rounding(Rounding::same(6.0 * z))
                            .inner_margin(margin)
                            .show(ui, |ui| {
                                ui.label(
                                    RichText::new(c.to_string())
                                        .font(egui::FontId::monospace(font_char))
                                        .strong()
                                        .color(Color32::WHITE),
                                );
                            });
                    }
                });
            });

            ui.add_space(20.0 * z);

            ui.group(|ui| {
                ui.label(
                    RichText::new(format!("STRING t: \"{}\"", t))
                        .font(egui::FontId::monospace(font_label))
                        .color(p.text_muted),
                );
                ui.horizontal(|ui| {
                    for (i, c) in t.chars().enumerate() {
                        let fill = if active_t == Some(i) {
                            p.pink
                        } else {
                            p.cell_bg
                        };
                        egui::Frame::none()
                            .fill(fill)
                            .rounding(Rounding::same(6.0 * z))
                            .inner_margin(margin)
                            .show(ui, |ui| {
                                ui.label(
                                    RichText::new(c.to_string())
                                        .font(egui::FontId::monospace(font_char))
                                        .strong()
                                        .color(Color32::WHITE),
                                );
                            });
                    }
                });
            });
        });

        ui.add_space(20.0 * z);

        if self.selected_approach_id == 0 {
            ui.group(|ui| {
                ui.label(
                    RichText::new("ALPHABET FREQUENCY LOG")
                        .font(egui::FontId::monospace((11.0 * z).max(8.0)))
                        .color(p.text_muted),
                );
                ui.horizontal_wrapped(|ui| {
                    for i in 0..26 {
                        let ch = (b'a' + i as u8) as char;
                        if s_counts[i] > 0 || t_counts[i] > 0 {
                            let match_color = if s_counts[i] == t_counts[i] {
                                p.emerald_text
                            } else {
                                p.red
                            };
                            egui::Frame::none()
                                .fill(p.cell_bg)
                                .rounding(Rounding::same(6.0 * z))
                                .stroke(Stroke::new(1.0_f32 * z, match_color))
                                .inner_margin((6.0 * z).max(3.0))
                                .show(ui, |ui| {
                                    ui.vertical(|ui| {
                                        ui.label(
                                            RichText::new(ch.to_string())
                                                .font(egui::FontId::monospace((14.0 * z).max(9.0)))
                                                .strong()
                                                .color(p.cyan),
                                        );
                                        ui.label(
                                            RichText::new(format!("s:{}", s_counts[i]))
                                                .font(egui::FontId::monospace((11.0 * z).max(8.0)))
                                                .color(p.text_muted),
                                        );
                                        ui.label(
                                            RichText::new(format!("t:{}", t_counts[i]))
                                                .font(egui::FontId::monospace((11.0 * z).max(8.0)))
                                                .color(p.text_muted),
                                        );
                                    });
                                });
                        }
                    }
                });
            });
        }

        if let Some(res) = is_anagram {
            ui.add_space(20.0 * z);
            if res {
                ui.heading(
                    RichText::new("Valid Anagram!")
                        .color(p.emerald_text)
                        .size((18.0 * z).max(11.0)),
                );
            } else {
                ui.heading(
                    RichText::new("Not an Anagram")
                        .color(p.red)
                        .size((18.0 * z).max(11.0)),
                );
            }
        }
    }
}
