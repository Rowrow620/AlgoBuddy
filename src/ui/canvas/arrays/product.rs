use crate::app::VisualizerApp;
use crate::model::{ProductPhase, ThemePalette};
use eframe::egui::{self, Color32, RichText, Rounding, Stroke};

impl VisualizerApp {
    #[allow(clippy::too_many_arguments)]
    pub(in crate::ui::canvas) fn render_product(
        &self,
        ui: &mut egui::Ui,
        p: &ThemePalette,
        nums: &[i32],
        output: &[i64],
        active_idx: Option<usize>,
        prefix_val: i64,
        suffix_val: i64,
        phase: &ProductPhase,
    ) {
        let z = self.canvas_zoom;
        let font_title = (16.0 * z).max(10.0);
        let font_num = (16.0 * z).max(9.0);
        let margin = (10.0 * z).max(4.0);

        ui.heading(
            RichText::new("1. Input Array (nums)")
                .color(p.cyan)
                .size(font_title),
        );
        ui.add_space(8.0 * z);
        ui.horizontal(|ui| {
            for (idx, &val) in nums.iter().enumerate() {
                let is_active = active_idx == Some(idx);
                let fill = if is_active { p.amber } else { p.cell_bg };
                egui::Frame::none()
                    .fill(fill)
                    .rounding(Rounding::same(8.0 * z))
                    .stroke(Stroke::new(1.0_f32 * z, p.cell_border))
                    .inner_margin(margin)
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.label(
                                RichText::new(format!("i={}", idx))
                                    .font(egui::FontId::proportional((10.0 * z).max(8.0)))
                                    .color(p.text_muted),
                            );
                            ui.label(
                                RichText::new(val.to_string())
                                    .font(egui::FontId::monospace(font_num))
                                    .strong()
                                    .color(Color32::WHITE),
                            );
                        });
                    });
            }
        });

        ui.add_space(24.0 * z);

        ui.heading(
            RichText::new("2. Running Prefix / Suffix Values")
                .color(p.purple)
                .size(font_title),
        );
        ui.add_space(8.0 * z);
        ui.horizontal(|ui| {
            egui::Frame::none()
                .fill(p.cell_bg)
                .rounding(Rounding::same(8.0 * z))
                .stroke(Stroke::new(1.0_f32 * z, p.cyan))
                .inner_margin((12.0 * z).max(5.0))
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("prefix")
                                .font(egui::FontId::monospace((12.0 * z).max(8.0)))
                                .color(p.text_muted),
                        );
                        ui.label(
                            RichText::new(prefix_val.to_string())
                                .font(egui::FontId::monospace((18.0 * z).max(10.0)))
                                .strong()
                                .color(p.cyan),
                        );
                    });
                });
            ui.add_space(16.0 * z);
            egui::Frame::none()
                .fill(p.cell_bg)
                .rounding(Rounding::same(8.0 * z))
                .stroke(Stroke::new(1.0_f32 * z, p.pink))
                .inner_margin((12.0 * z).max(5.0))
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("suffix")
                                .font(egui::FontId::monospace((12.0 * z).max(8.0)))
                                .color(p.text_muted),
                        );
                        ui.label(
                            RichText::new(suffix_val.to_string())
                                .font(egui::FontId::monospace((18.0 * z).max(10.0)))
                                .strong()
                                .color(p.pink),
                        );
                    });
                });
            ui.add_space(16.0 * z);
            let phase_label = match phase {
                ProductPhase::Init => "Initializing",
                ProductPhase::PrefixPass => "Prefix Pass (left to right)",
                ProductPhase::SuffixPass => "Suffix Pass (right to left)",
                ProductPhase::Complete => "Complete",
            };
            ui.label(
                RichText::new(format!("Phase: {}", phase_label))
                    .font(egui::FontId::proportional((14.0 * z).max(9.0)))
                    .strong()
                    .color(p.text_primary),
            );
        });

        ui.add_space(24.0 * z);

        ui.heading(
            RichText::new("3. Output Array")
                .color(p.emerald_text)
                .size(font_title),
        );
        ui.add_space(8.0 * z);
        ui.horizontal(|ui| {
            for (idx, &val) in output.iter().enumerate() {
                let is_active = active_idx == Some(idx);
                let fill = if is_active {
                    match phase {
                        ProductPhase::PrefixPass => p.cyan,
                        ProductPhase::SuffixPass => p.pink,
                        _ => p.emerald,
                    }
                } else {
                    p.cell_bg
                };
                egui::Frame::none()
                    .fill(fill)
                    .rounding(Rounding::same(8.0 * z))
                    .stroke(Stroke::new(1.0_f32 * z, p.emerald_text))
                    .inner_margin((10.0 * z).max(4.0))
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.label(
                                RichText::new(format!("o[{}]", idx))
                                    .font(egui::FontId::proportional((10.0 * z).max(8.0)))
                                    .color(p.text_muted),
                            );
                            ui.label(
                                RichText::new(val.to_string())
                                    .font(egui::FontId::monospace(font_num))
                                    .strong()
                                    .color(Color32::WHITE),
                            );
                        });
                    });
            }
        });
    }
}
