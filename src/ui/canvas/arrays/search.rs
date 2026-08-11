use crate::app::VisualizerApp;
use crate::model::ThemePalette;
use eframe::egui::{self, Color32, RichText, Rounding, Stroke};

impl VisualizerApp {
    #[allow(clippy::too_many_arguments)]
    pub(in crate::ui::canvas) fn render_stock(
        &self,
        ui: &mut egui::Ui,
        p: &ThemePalette,
        prices: &[i32],
        left_buy: usize,
        right_sell: usize,
        current_profit: i32,
        max_profit: i32,
    ) {
        let z = self.canvas_zoom;
        let font_title = (16.0 * z).max(10.0);
        let font_label = (11.0 * z).max(8.0);
        let font_price = (16.0 * z).max(9.0);
        let margin = (10.0 * z).max(4.0);

        ui.heading(
            RichText::new("Sliding Window / Buy & Sell Stock Trace")
                .color(p.cyan)
                .size(font_title),
        );
        ui.add_space(8.0 * z);

        ui.group(|ui| {
            ui.label(
                RichText::new("STOCK PRICES ARRAY (Days 0..N-1)")
                    .font(egui::FontId::monospace(font_label))
                    .color(p.text_muted),
            );
            ui.horizontal(|ui| {
                for (i, &price) in prices.iter().enumerate() {
                    let is_buy = i == left_buy;
                    let is_sell = i == right_sell;

                    let fill = if is_buy && is_sell {
                        p.purple
                    } else if is_buy {
                        p.cyan
                    } else if is_sell {
                        p.pink
                    } else {
                        p.cell_bg
                    };

                    egui::Frame::none()
                        .fill(fill)
                        .rounding(Rounding::same(8.0 * z))
                        .stroke(Stroke::new(1.0_f32 * z, p.cell_border))
                        .inner_margin(margin)
                        .show(ui, |ui| {
                            ui.vertical(|ui| {
                                let label = if is_buy && is_sell {
                                    "Buy & Sell"
                                } else if is_buy {
                                    "Buy (l)"
                                } else if is_sell {
                                    "Sell (r)"
                                } else {
                                    ""
                                };
                                ui.label(
                                    RichText::new(format!("day {} {}", i, label))
                                        .font(egui::FontId::proportional((10.0 * z).max(8.0)))
                                        .color(p.text_muted),
                                );
                                ui.label(
                                    RichText::new(format!("${}", price))
                                        .font(egui::FontId::monospace(font_price))
                                        .strong()
                                        .color(Color32::WHITE),
                                );
                            });
                        });
                }
            });
        });

        ui.add_space(20.0 * z);

        ui.horizontal(|ui| {
            egui::Frame::none()
                .fill(p.cell_bg)
                .rounding(Rounding::same(8.0 * z))
                .stroke(Stroke::new(1.0_f32 * z, p.pink))
                .inner_margin((12.0 * z).max(5.0))
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("Current Profit (prices[r] - prices[l])")
                                .font(egui::FontId::proportional((11.0 * z).max(8.0)))
                                .color(p.text_muted),
                        );
                        ui.label(
                            RichText::new(format!("${}", current_profit))
                                .font(egui::FontId::monospace((18.0 * z).max(10.0)))
                                .strong()
                                .color(p.pink),
                        );
                    });
                });

            ui.add_space(16.0 * z);

            egui::Frame::none()
                .fill(p.cell_bg)
                .rounding(Rounding::same(8.0 * z))
                .stroke(Stroke::new(1.0_f32 * z, p.emerald_text))
                .inner_margin((12.0 * z).max(5.0))
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("Maximum Achieved Profit (maxP)")
                                .font(egui::FontId::proportional((11.0 * z).max(8.0)))
                                .color(p.text_muted),
                        );
                        ui.label(
                            RichText::new(format!("${}", max_profit))
                                .font(egui::FontId::monospace((18.0 * z).max(10.0)))
                                .strong()
                                .color(p.emerald_text),
                        );
                    });
                });
        });
    }

    #[allow(clippy::too_many_arguments)]
    pub(in crate::ui::canvas) fn render_binary_search(
        &self,
        ui: &mut egui::Ui,
        p: &ThemePalette,
        nums: &[i32],
        target: i32,
        left: usize,
        right: usize,
        mid: Option<usize>,
        found_idx: Option<usize>,
    ) {
        let z = self.canvas_zoom;
        let margin = (12.0 * z).max(4.0);
        let font_sz = (18.0 * z).max(9.0);
        let font_title = (18.0 * z).max(10.0);

        ui.heading(
            RichText::new(format!(
                "Binary Search bounds (l={}, r={}) | Target = {}",
                left, right, target
            ))
            .color(p.cyan)
            .size(font_title),
        );
        ui.add_space(10.0 * z);

        ui.group(|ui| {
            ui.label(
                RichText::new("SORTED ARRAY")
                    .font(egui::FontId::monospace((11.0 * z).max(8.0)))
                    .color(p.text_muted),
            );
            ui.horizontal(|ui| {
                for (i, &num) in nums.iter().enumerate() {
                    let is_found = found_idx == Some(i);
                    let is_mid = mid == Some(i);
                    let in_range = i >= left && i <= right;

                    let fill = if is_found {
                        p.emerald
                    } else if is_mid {
                        p.amber
                    } else if in_range {
                        p.cell_bg
                    } else {
                        p.text_dim
                    };

                    egui::Frame::none()
                        .fill(fill)
                        .rounding(Rounding::same(8.0 * z))
                        .stroke(Stroke::new(1.0_f32 * z, p.cell_border))
                        .inner_margin(margin)
                        .show(ui, |ui| {
                            ui.vertical(|ui| {
                                let mut ptr_label = String::new();
                                if i == left {
                                    ptr_label.push_str("L ");
                                }
                                if is_mid {
                                    ptr_label.push_str("MID ");
                                }
                                if i == right {
                                    ptr_label.push('R');
                                }

                                ui.label(
                                    RichText::new(format!("i={} {}", i, ptr_label))
                                        .font(egui::FontId::proportional((11.0 * z).max(8.0)))
                                        .color(Color32::WHITE),
                                );
                                ui.label(
                                    RichText::new(num.to_string())
                                        .font(egui::FontId::monospace(font_sz))
                                        .strong()
                                        .color(Color32::WHITE),
                                );
                            });
                        });
                }
            });
        });

        if let Some(f) = found_idx {
            ui.add_space(20.0 * z);
            ui.heading(
                RichText::new(format!("Target {} Found at Index {}!", target, f))
                    .color(p.emerald_text)
                    .size((18.0 * z).max(11.0)),
            );
        }
    }
}
