use crate::app::VisualizerApp;
use crate::model::ThemePalette;
use eframe::egui::{self, Color32, RichText, Rounding, Stroke};

impl VisualizerApp {
    #[allow(clippy::too_many_arguments)]
    pub(in crate::ui::canvas) fn render_array_1d(
        &self,
        ui: &mut egui::Ui,
        p: &ThemePalette,
        title: &str,
        elements: &[i32],
        active_idx: Option<usize>,
        secondary_idx: Option<usize>,
        pointers: &[(&'static str, usize)],
        status_message: &str,
        is_success: Option<bool>,
    ) {
        let z = self.canvas_zoom;
        ui.heading(RichText::new(title).color(p.amber).size(16.0 * z));
        ui.add_space(12.0 * z);

        ui.horizontal(|ui| {
            for (idx, &val) in elements.iter().enumerate() {
                let is_active = active_idx == Some(idx);
                let is_secondary = secondary_idx == Some(idx);
                let bg_color = if is_active {
                    p.amber
                } else if is_secondary {
                    p.cyan
                } else {
                    p.cell_bg
                };

                let text_color = if is_active || is_secondary {
                    p.sidebar_bg
                } else {
                    p.text_primary
                };

                ui.vertical(|ui| {
                    let ptr_text = pointers
                        .iter()
                        .filter(|(_, p_idx)| *p_idx == idx)
                        .map(|(name, _)| *name)
                        .collect::<Vec<_>>()
                        .join(",");

                    if !ptr_text.is_empty() {
                        ui.label(
                            RichText::new(&ptr_text)
                                .font(egui::FontId::monospace(10.0 * z))
                                .color(p.amber)
                                .strong(),
                        );
                    } else {
                        ui.label(RichText::new(" ").font(egui::FontId::monospace(10.0 * z)));
                    }

                    egui::Frame::none()
                        .fill(bg_color)
                        .rounding(Rounding::same(8.0 * z))
                        .inner_margin(egui::Margin::symmetric(10.0 * z, 8.0 * z))
                        .show(ui, |ui| {
                            ui.label(
                                RichText::new(val.to_string())
                                    .font(egui::FontId::monospace(14.0 * z))
                                    .color(text_color)
                                    .strong(),
                            );
                        });

                    ui.label(
                        RichText::new(format!("i={}", idx))
                            .font(egui::FontId::monospace(10.0 * z))
                            .color(p.text_muted),
                    );
                });
                ui.add_space(4.0 * z);
            }
        });

        if !status_message.is_empty() {
            ui.add_space(16.0 * z);
            let status_color = match is_success {
                Some(true) => p.emerald_text,
                Some(false) => p.red,
                None => p.text_primary,
            };
            ui.label(
                RichText::new(status_message)
                    .font(egui::FontId::proportional(13.0 * z))
                    .color(status_color)
                    .strong(),
            );
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(in crate::ui::canvas) fn render_contains_duplicate(
        &self,
        ui: &mut egui::Ui,
        p: &ThemePalette,
        nums: &[i32],
        active_idx: Option<usize>,
        seen_set: &std::collections::BTreeSet<i32>,
        dup_val: Option<i32>,
        has_dup: Option<bool>,
    ) {
        let z = self.canvas_zoom;
        let font_sz = 16.0 * z;
        let label_sz = (10.0 * z).max(8.0);
        let margin = (10.0 * z).max(6.0);

        let approach = self
            .current_problem
            .details()
            .approach_by_id(self.selected_approach_id);
        let title = approach.map_or_else(
            || "Contains Duplicate Detection".to_owned(),
            |approach| {
                format!(
                    "Contains Duplicate - {} ({})",
                    approach.name, approach.time_complexity
                )
            },
        );
        ui.heading(RichText::new(title).color(p.cyan).size(16.0 * z));
        ui.add_space(8.0 * z);

        ui.group(|ui| {
            ui.label(
                RichText::new(if self.selected_approach_id == 1 {
                    "SORTED NUMS ARRAY"
                } else {
                    "INPUT NUMS ARRAY"
                })
                .font(egui::FontId::monospace(11.0 * z))
                .color(p.text_muted),
            );
            ui.horizontal(|ui| {
                for (i, &val) in nums.iter().enumerate() {
                    let is_active = active_idx == Some(i);
                    let is_previous = self.selected_approach_id == 1
                        && active_idx.is_some_and(|active| active > 0 && i + 1 == active);
                    let is_dup = dup_val == Some(val) && (is_active || is_previous);
                    let fill = if is_dup {
                        p.red
                    } else if is_active {
                        p.amber
                    } else if is_previous {
                        p.pink
                    } else {
                        p.cell_bg
                    };
                    let (label_color, val_color) = if is_dup || is_active || is_previous {
                        (Color32::from_rgb(30, 35, 45), Color32::from_rgb(30, 35, 45))
                    } else {
                        (p.text_muted, Color32::WHITE)
                    };
                    let pointer = if is_active {
                        " (i)"
                    } else if is_previous {
                        " (i - 1)"
                    } else {
                        ""
                    };

                    egui::Frame::none()
                        .fill(fill)
                        .rounding(Rounding::same(8.0 * z))
                        .stroke(Stroke::new(1.0_f32, p.cell_border))
                        .inner_margin(margin)
                        .show(ui, |ui| {
                            ui.vertical(|ui| {
                                ui.label(
                                    RichText::new(format!("index={}{}", i, pointer))
                                        .font(egui::FontId::proportional(label_sz))
                                        .color(label_color),
                                );
                                ui.label(
                                    RichText::new(val.to_string())
                                        .font(egui::FontId::monospace(font_sz))
                                        .strong()
                                        .color(val_color),
                                );
                            });
                        });
                }
            });
        });

        if self.selected_approach_id == 0 {
            ui.add_space(20.0 * z);

            ui.group(|ui| {
                ui.label(
                    RichText::new("HASHSET `SEEN`")
                        .font(egui::FontId::monospace(11.0 * z))
                        .color(p.text_muted),
                );
                ui.horizontal_wrapped(|ui| {
                    if seen_set.is_empty() {
                        ui.label(RichText::new("Set is empty {}").italics().color(p.text_dim));
                    } else {
                        for &val in seen_set {
                            let is_dup = dup_val == Some(val);
                            let fill = if is_dup { p.red } else { p.cell_bg };
                            egui::Frame::none()
                                .fill(fill)
                                .rounding(Rounding::same(6.0 * z))
                                .stroke(Stroke::new(1.0_f32, p.purple))
                                .inner_margin(margin)
                                .show(ui, |ui| {
                                    ui.label(
                                        RichText::new(val.to_string())
                                            .font(egui::FontId::monospace(font_sz))
                                            .strong()
                                            .color(Color32::WHITE),
                                    );
                                });
                        }
                    }
                });
            });
        }

        if let Some(dup) = has_dup {
            ui.add_space(20.0);
            if dup {
                ui.heading(
                    RichText::new(format!(
                        "Duplicate Found! Value {} appears at least twice.",
                        dup_val.unwrap_or(0)
                    ))
                    .color(p.red)
                    .size(18.0),
                );
            } else {
                ui.heading(
                    RichText::new("All Elements Are Distinct! (Return False)")
                        .color(p.emerald_text)
                        .size(18.0),
                );
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(in crate::ui::canvas) fn render_longest_consecutive(
        &self,
        ui: &mut egui::Ui,
        p: &ThemePalette,
        nums: &[i32],
        num_set: &std::collections::BTreeSet<i32>,
        curr_num: Option<i32>,
        curr_seq: &[i32],
        max_len: usize,
        is_seq_start: Option<bool>,
    ) {
        let z = self.canvas_zoom;
        let font_title = (16.0 * z).max(10.0);
        let font_label = (11.0 * z).max(8.0);
        let font_sz = (16.0 * z).max(9.0);
        let margin = (8.0 * z).max(4.0);

        ui.heading(
            RichText::new("Longest Consecutive Sequence (HashSet O(N))")
                .color(p.cyan)
                .size(font_title),
        );
        ui.add_space(8.0 * z);

        ui.group(|ui| {
            ui.label(
                RichText::new("INPUT ARRAY (nums)")
                    .font(egui::FontId::monospace(font_label))
                    .color(p.text_muted),
            );
            ui.horizontal(|ui| {
                for &val in nums {
                    let is_curr = curr_num == Some(val);
                    let is_in_seq = curr_seq.contains(&val);
                    let fill = if is_in_seq {
                        p.emerald
                    } else if is_curr {
                        p.amber
                    } else {
                        p.cell_bg
                    };

                    egui::Frame::none()
                        .fill(fill)
                        .rounding(Rounding::same(8.0 * z))
                        .stroke(Stroke::new(1.0_f32 * z, p.cell_border))
                        .inner_margin(margin)
                        .show(ui, |ui| {
                            ui.label(
                                RichText::new(val.to_string())
                                    .font(egui::FontId::monospace(font_sz))
                                    .strong()
                                    .color(Color32::WHITE),
                            );
                        });
                }
            });
        });

        ui.add_space(16.0 * z);

        ui.group(|ui| {
            ui.label(
                RichText::new("NUMSET (HashSet of unique values)")
                    .font(egui::FontId::monospace(font_label))
                    .color(p.text_muted),
            );
            ui.horizontal_wrapped(|ui| {
                for &val in num_set {
                    let is_curr = curr_num == Some(val);
                    let is_in_seq = curr_seq.contains(&val);

                    let fill = if is_in_seq {
                        p.emerald
                    } else if is_curr {
                        if is_seq_start == Some(true) {
                            p.amber
                        } else {
                            p.text_dim
                        }
                    } else {
                        p.cell_bg
                    };

                    egui::Frame::none()
                        .fill(fill)
                        .rounding(Rounding::same(6.0 * z))
                        .stroke(Stroke::new(1.0_f32 * z, p.purple))
                        .inner_margin(margin)
                        .show(ui, |ui| {
                            ui.label(
                                RichText::new(val.to_string())
                                    .font(egui::FontId::monospace((14.0 * z).max(9.0)))
                                    .strong()
                                    .color(Color32::WHITE),
                            );
                        });
                }
            });
        });

        ui.add_space(20.0 * z);

        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.label(
                    RichText::new("CURRENT STREAK SEQUENCE")
                        .font(egui::FontId::monospace(font_label))
                        .color(p.emerald_text),
                );
                ui.horizontal(|ui| {
                    if curr_seq.is_empty() {
                        ui.label(
                            RichText::new("None (searching for sequence start...)")
                                .font(egui::FontId::monospace((14.0 * z).max(9.0)))
                                .color(p.text_dim),
                        );
                    } else {
                        for (i, &val) in curr_seq.iter().enumerate() {
                            egui::Frame::none()
                                .fill(p.emerald)
                                .rounding(Rounding::same(8.0 * z))
                                .inner_margin((10.0 * z).max(4.0))
                                .show(ui, |ui| {
                                    ui.label(
                                        RichText::new(val.to_string())
                                            .font(egui::FontId::monospace((18.0 * z).max(10.0)))
                                            .strong()
                                            .color(Color32::WHITE),
                                    );
                                });
                            if i + 1 < curr_seq.len() {
                                ui.label(
                                    RichText::new("->")
                                        .font(egui::FontId::monospace((14.0 * z).max(9.0)))
                                        .color(p.cyan),
                                );
                            }
                        }
                    }
                });
            });

            ui.add_space(20.0 * z);

            egui::Frame::none()
                .fill(p.cell_bg)
                .rounding(Rounding::same(8.0 * z))
                .stroke(Stroke::new(1.0_f32 * z, p.emerald_text))
                .inner_margin((12.0 * z).max(5.0))
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("Max Streak (longest)")
                                .font(egui::FontId::proportional((11.0 * z).max(8.0)))
                                .color(p.text_muted),
                        );
                        ui.label(
                            RichText::new(format!("{}", max_len))
                                .font(egui::FontId::monospace((22.0 * z).max(12.0)))
                                .strong()
                                .color(p.emerald_text),
                        );
                    });
                });
        });
    }
}
