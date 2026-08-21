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
        current_profit: i64,
        max_profit: i64,
    ) {
        let z = self.canvas_zoom;
        let font_title = (16.0 * z).max(10.0);
        let font_label = (11.0 * z).max(8.0);

        let approach_name = self
            .current_problem
            .details()
            .approach_by_id(self.selected_approach_id)
            .map_or("Buy & Sell Stock", |approach| approach.name);
        ui.heading(
            RichText::new(format!("{approach_name} Trace"))
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

                    let label = if is_buy && is_sell {
                        "Buy & Sell"
                    } else if is_buy {
                        if self.selected_approach_id == 1 {
                            "Buy (i)"
                        } else {
                            "Buy (l)"
                        }
                    } else if is_sell {
                        if self.selected_approach_id == 1 {
                            "Sell (j)"
                        } else {
                            "Sell (r)"
                        }
                    } else {
                        ""
                    };

                    Self::render_canvas_cell(
                        ui,
                        &format!("${price}"),
                        Some(&format!("day {i} {label}")),
                        None,
                        fill,
                        p.cell_border,
                        Color32::WHITE,
                        z,
                    );
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
                            RichText::new(if self.selected_approach_id == 1 {
                                "Current Profit (prices[j] - prices[i])"
                            } else {
                                "Current Profit (prices[r] - prices[l])"
                            })
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
        let font_title = (18.0 * z).max(10.0);

        let approach_name = self
            .current_problem
            .details()
            .approach_by_id(self.selected_approach_id)
            .map_or("Search", |approach| approach.name);
        let search_status = if self.selected_approach_id == 0 {
            format!("bounds (l={left}, r={right})")
        } else {
            mid.map_or_else(
                || {
                    if left < nums.len() {
                        "ready to scan".to_owned()
                    } else {
                        "scan complete".to_owned()
                    }
                },
                |index| format!("checking index {index}"),
            )
        };
        ui.heading(
            RichText::new(format!(
                "{approach_name} {search_status} | Target = {target}"
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

                    let mut ptr_label = String::new();
                    if self.selected_approach_id == 0 && i == left {
                        ptr_label.push_str("L ");
                    }
                    if is_mid {
                        ptr_label.push_str(if self.selected_approach_id == 1 {
                            "i "
                        } else {
                            "MID "
                        });
                    }
                    if self.selected_approach_id == 0 && i == right {
                        ptr_label.push('R');
                    }

                    Self::render_canvas_cell(
                        ui,
                        &num.to_string(),
                        Some(&format!("i={i} {ptr_label}")),
                        None,
                        fill,
                        p.cell_border,
                        Color32::WHITE,
                        z,
                    );
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
