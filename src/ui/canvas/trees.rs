use crate::app::VisualizerApp;
use crate::model::{Problem, ThemePalette};
use eframe::egui::{self, Color32, RichText, Rounding, Stroke};

impl VisualizerApp {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn render_tree(
        &self,
        ui: &mut egui::Ui,
        p: &ThemePalette,
        tree_nodes: &[Option<i32>],
        active_idx: Option<usize>,
        sec_idx: Option<usize>,
        depth_val: Option<i32>,
        max_diameter: Option<i32>,
    ) {
        let z = self.canvas_zoom;
        let font_title = (16.0 * z).max(10.0);
        let font_label = (11.0 * z).max(8.0);
        let font_node = (16.0 * z).max(9.0);
        let margin = (10.0 * z).max(4.0);

        ui.heading(
            RichText::new("Binary Tree Node Graph Hierarchy")
                .color(p.cyan)
                .size(font_title),
        );
        ui.add_space(8.0 * z);

        ui.group(|ui| {
            ui.label(
                RichText::new("BINARY TREE LEVEL-ORDER NODES")
                    .font(egui::FontId::monospace(font_label))
                    .color(p.text_muted),
            );
            ui.horizontal_wrapped(|ui| {
                for (i, node_opt) in tree_nodes.iter().enumerate() {
                    let is_active = active_idx == Some(i);
                    let is_sec = sec_idx == Some(i);

                    let fill = if is_active {
                        p.cyan
                    } else if is_sec {
                        p.pink
                    } else if node_opt.is_some() {
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
                                let label = if is_active {
                                    "Active"
                                } else if is_sec {
                                    if self.current_problem == Problem::LowestCommonAncestorBst {
                                        "Target"
                                    } else {
                                        "Child"
                                    }
                                } else {
                                    ""
                                };
                                ui.label(
                                    RichText::new(format!("i={} {}", i, label))
                                        .font(egui::FontId::proportional((10.0 * z).max(8.0)))
                                        .color(Color32::WHITE),
                                );
                                let val_str = match node_opt {
                                    Some(v) => format!("[ {} ]", v),
                                    None => "null".to_string(),
                                };
                                ui.label(
                                    RichText::new(val_str)
                                        .font(egui::FontId::monospace(font_node))
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
            if let Some(d) = depth_val {
                let is_balance_difference =
                    self.current_problem == Problem::BalancedTree && self.selected_approach_id == 1;
                egui::Frame::none()
                    .fill(p.cell_bg)
                    .rounding(Rounding::same(8.0 * z))
                    .stroke(Stroke::new(1.0_f32 * z, p.cyan))
                    .inner_margin((12.0 * z).max(5.0))
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.label(
                                RichText::new(if is_balance_difference {
                                    "Current Node Height Difference"
                                } else {
                                    "Current / Max Tree Depth"
                                })
                                .font(egui::FontId::proportional((11.0 * z).max(8.0)))
                                .color(p.text_muted),
                            );
                            ui.label(
                                RichText::new(if is_balance_difference {
                                    format!("Difference: {d}")
                                } else {
                                    format!("Depth: {d}")
                                })
                                .font(egui::FontId::monospace((18.0 * z).max(10.0)))
                                .strong()
                                .color(p.cyan),
                            );
                        });
                    });
            }

            if let Some(diam) = max_diameter {
                ui.add_space(16.0 * z);
                egui::Frame::none()
                    .fill(p.cell_bg)
                    .rounding(Rounding::same(8.0 * z))
                    .stroke(Stroke::new(1.0_f32 * z, p.emerald_text))
                    .inner_margin((12.0 * z).max(5.0))
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.label(
                                RichText::new("Maximum Tree Diameter (Edges Path)")
                                    .font(egui::FontId::proportional((11.0 * z).max(8.0)))
                                    .color(p.text_muted),
                            );
                            ui.label(
                                RichText::new(format!("Diameter: {}", diam))
                                    .font(egui::FontId::monospace((18.0 * z).max(10.0)))
                                    .strong()
                                    .color(p.emerald_text),
                            );
                        });
                    });
            }
        });
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn render_tree_max_path(
        &self,
        ui: &mut egui::Ui,
        p: &ThemePalette,
        tree_nodes: &[Option<i32>],
        active_idx: Option<usize>,
        secondary_idx: Option<usize>,
        left_gain: Option<i32>,
        right_gain: Option<i32>,
        through_node_sum: Option<i32>,
        returned_gain: Option<i32>,
        max_path_sum: Option<i32>,
    ) {
        self.render_tree(ui, p, tree_nodes, active_idx, secondary_idx, None, None);

        let z = self.canvas_zoom;
        let label_size = (11.0 * z).max(8.0);
        let value_size = (16.0 * z).max(10.0);
        ui.add_space(12.0 * z);
        ui.group(|ui| {
            ui.label(
                RichText::new("POST-ORDER PATH GAIN STATE")
                    .font(egui::FontId::monospace(label_size))
                    .color(p.text_muted),
            );
            ui.add_space(6.0 * z);
            ui.horizontal_wrapped(|ui| {
                if let (Some(left), Some(right)) = (left_gain, right_gain) {
                    ui.label(
                        RichText::new(format!("Left gain: {left}   Right gain: {right}"))
                            .font(egui::FontId::monospace(value_size))
                            .color(p.cyan),
                    );
                }
                if let Some(sum) = through_node_sum {
                    ui.label(
                        RichText::new(format!("Path through node: {sum}"))
                            .font(egui::FontId::monospace(value_size))
                            .color(p.amber),
                    );
                }
                if let Some(gain) = returned_gain {
                    ui.label(
                        RichText::new(format!("Returned gain: {gain}"))
                            .font(egui::FontId::monospace(value_size))
                            .color(p.cyan),
                    );
                }
                if let Some(best) = max_path_sum {
                    ui.label(
                        RichText::new(format!("Maximum path sum: {best}"))
                            .font(egui::FontId::monospace(value_size))
                            .strong()
                            .color(p.emerald_text),
                    );
                }
            });
        });
    }

    pub(super) fn render_trie(&self, ui: &mut egui::Ui, p: &ThemePalette) {
        let z = self.canvas_zoom;
        let font_title = (16.0 * z).max(10.0);
        let font_label = (11.0 * z).max(8.0);
        let font_root = (14.0 * z).max(9.0);
        let font_word_idx = (12.0 * z).max(8.0);
        let font_char = (13.0 * z).max(8.0);
        let margin = (10.0 * z).max(4.0);

        ui.heading(
            RichText::new("Trie (Prefix Tree) Character Node Hierarchy")
                .color(p.cyan)
                .size(font_title),
        );
        ui.add_space(8.0 * z);

        ui.group(|ui| {
            ui.label(
                RichText::new("TRIE CHARACTER NODE PATHS")
                    .font(egui::FontId::monospace(font_label))
                    .color(p.text_muted),
            );
            ui.add_space(8.0 * z);

            ui.horizontal_wrapped(|ui| {
                egui::Frame::none()
                    .fill(p.cyan)
                    .rounding(Rounding::same(20.0 * z))
                    .inner_margin(egui::Margin::symmetric(14.0 * z, 10.0 * z))
                    .show(ui, |ui| {
                        ui.label(
                            RichText::new("ROOT (*)")
                                .font(egui::FontId::monospace(font_root))
                                .strong()
                                .color(Color32::WHITE),
                        );
                    });

                ui.label(
                    RichText::new(" ──► ")
                        .font(egui::FontId::monospace(16.0 * z))
                        .color(p.cyan),
                );

                let words: Vec<&str> = match self.current_problem {
                    Problem::ImplementTrie => self
                        .get_input_str(Problem::ImplementTrie, "words", "apple, app, ape")
                        .split(',')
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                        .collect(),
                    Problem::WordDictionary => self
                        .get_input_str(Problem::WordDictionary, "words", "bad, dad, mad")
                        .split(',')
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                        .collect(),
                    Problem::WordSearchII => self
                        .get_input_str(Problem::WordSearchII, "words", "oath, pea, eat, rain")
                        .split(',')
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                        .collect(),
                    _ => vec!["apple", "app", "ape"],
                };

                for (w_idx, w) in words.iter().enumerate() {
                    egui::Frame::group(ui.style())
                        .fill(p.step_box_bg)
                        .rounding(Rounding::same(12.0 * z))
                        .stroke(Stroke::new(1.5_f32 * z, p.cyan))
                        .inner_margin(margin)
                        .show(ui, |ui| {
                            ui.vertical(|ui| {
                                ui.label(
                                    RichText::new(format!("Word #{}: \"{}\"", w_idx + 1, w))
                                        .font(egui::FontId::monospace(font_word_idx))
                                        .color(p.amber)
                                        .strong(),
                                );
                                ui.add_space(4.0 * z);
                                ui.horizontal(|ui| {
                                    for (c_idx, ch) in w.chars().enumerate() {
                                        let is_last = c_idx == w.len() - 1;
                                        let bg_color = if is_last { p.emerald } else { p.cell_bg };
                                        let text_color = if is_last {
                                            Color32::WHITE
                                        } else {
                                            p.text_primary
                                        };

                                        egui::Frame::none()
                                            .fill(bg_color)
                                            .rounding(Rounding::same(14.0 * z))
                                            .stroke(Stroke::new(1.0_f32 * z, p.cell_border))
                                            .inner_margin(egui::Margin::symmetric(8.0 * z, 4.0 * z))
                                            .show(ui, |ui| {
                                                if is_last {
                                                    ui.label(
                                                        RichText::new(format!("'{}' [end]", ch))
                                                            .font(egui::FontId::monospace(
                                                                font_char,
                                                            ))
                                                            .strong()
                                                            .color(text_color),
                                                    );
                                                } else {
                                                    ui.label(
                                                        RichText::new(format!("'{}'", ch))
                                                            .font(egui::FontId::monospace(
                                                                font_char,
                                                            ))
                                                            .strong()
                                                            .color(text_color),
                                                    );
                                                }
                                            });

                                        if c_idx < w.len() - 1 {
                                            ui.label(RichText::new("►").color(p.text_dim));
                                        }
                                    }
                                });
                            });
                        });
                    ui.add_space(6.0 * z);
                }
            });
        });
    }
}
