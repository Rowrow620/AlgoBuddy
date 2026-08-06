use crate::app::VisualizerApp;
use crate::model::{Problem, ProductPhase, ThemePalette};
use eframe::egui::{self, Color32, RichText, Rounding, Stroke};

impl VisualizerApp {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn render_array_1d(
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
    pub(super) fn render_contains_duplicate(
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

        ui.heading(
            RichText::new("Contains Duplicate Detection (HashSet O(N))")
                .color(p.cyan)
                .size(16.0 * z),
        );
        ui.add_space(8.0 * z);

        ui.group(|ui| {
            ui.label(
                RichText::new("INPUT NUMS ARRAY")
                    .font(egui::FontId::monospace(11.0 * z))
                    .color(p.text_muted),
            );
            ui.horizontal(|ui| {
                for (i, &val) in nums.iter().enumerate() {
                    let is_active = active_idx == Some(i);
                    let is_dup = dup_val == Some(val) && is_active;
                    let fill = if is_dup {
                        p.red
                    } else if is_active {
                        p.amber
                    } else {
                        p.cell_bg
                    };
                    let (label_color, val_color) = if is_dup || is_active {
                        (Color32::from_rgb(30, 35, 45), Color32::from_rgb(30, 35, 45))
                    } else {
                        (p.text_muted, Color32::WHITE)
                    };

                    egui::Frame::none()
                        .fill(fill)
                        .rounding(Rounding::same(8.0 * z))
                        .stroke(Stroke::new(1.0_f32, p.cell_border))
                        .inner_margin(margin)
                        .show(ui, |ui| {
                            ui.vertical(|ui| {
                                ui.label(
                                    RichText::new(format!("i={}", i))
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
    pub(super) fn render_longest_consecutive(
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

    #[allow(clippy::too_many_arguments)]
    pub(super) fn render_stock(
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
    pub(super) fn render_binary_search(
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

    #[allow(clippy::too_many_arguments)]
    pub(super) fn render_two_sum(
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
    pub(super) fn render_valid_anagram(
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

    #[allow(clippy::too_many_arguments)]
    pub(super) fn render_two_pointers(
        &self,
        ui: &mut egui::Ui,
        p: &ThemePalette,
        chars: &[char],
        left: usize,
        right: usize,
        is_valid: Option<bool>,
        skipped: bool,
    ) {
        let z = self.canvas_zoom;
        let font_title = (16.0 * z).max(10.0);
        let font_char = (16.0 * z).max(9.0);
        let margin = (8.0 * z).max(4.0);

        ui.heading(
            RichText::new("Two Pointers Convergence")
                .color(p.cyan)
                .size(font_title),
        );
        ui.add_space(8.0 * z);

        ui.horizontal_wrapped(|ui| {
            for (i, &c) in chars.iter().enumerate() {
                let is_left = i == left;
                let is_right = i == right;

                let fill = if is_left && is_right {
                    p.purple
                } else if is_left {
                    p.cyan
                } else if is_right {
                    p.pink
                } else if skipped && (i < left || i > right) {
                    p.text_dim
                } else {
                    p.cell_bg
                };

                egui::Frame::none()
                    .fill(fill)
                    .rounding(Rounding::same(6.0 * z))
                    .stroke(Stroke::new(1.0_f32 * z, p.cell_border))
                    .inner_margin(margin)
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            let ptr_label = if is_left && is_right {
                                "L & R"
                            } else if is_left {
                                "L ->"
                            } else if is_right {
                                "<- R"
                            } else {
                                " "
                            };
                            ui.label(
                                RichText::new(ptr_label)
                                    .font(egui::FontId::monospace((10.0 * z).max(8.0)))
                                    .strong()
                                    .color(Color32::WHITE),
                            );
                            ui.label(
                                RichText::new(c.to_string())
                                    .font(egui::FontId::monospace(font_char))
                                    .strong()
                                    .color(Color32::WHITE),
                            );
                        });
                    });
            }
        });

        if let Some(valid) = is_valid {
            ui.add_space(20.0 * z);
            let heading_sz = (18.0 * z).max(11.0);
            match self.current_problem {
                Problem::ValidPalindrome => {
                    if valid {
                        ui.heading(
                            RichText::new("Valid Palindrome!")
                                .color(p.emerald_text)
                                .size(heading_sz),
                        );
                    } else {
                        ui.heading(
                            RichText::new("Invalid Palindrome Mismatch")
                                .color(p.red)
                                .size(heading_sz),
                        );
                    }
                }
                Problem::TwoSumII => {
                    if valid {
                        ui.heading(
                            RichText::new("Target Sum Pair Found!")
                                .color(p.emerald_text)
                                .size(heading_sz),
                        );
                    } else {
                        ui.heading(
                            RichText::new("No Pair Sum Equals Target")
                                .color(p.red)
                                .size(heading_sz),
                        );
                    }
                }
                Problem::ThreeSum => {
                    if valid {
                        ui.heading(
                            RichText::new("3Sum Triplets Search Complete!")
                                .color(p.emerald_text)
                                .size(heading_sz),
                        );
                    } else {
                        ui.heading(
                            RichText::new("No Triplets Sum to 0")
                                .color(p.red)
                                .size(heading_sz),
                        );
                    }
                }
                Problem::ContainerWater => {
                    ui.heading(
                        RichText::new("Maximum Water Container Area Computed!")
                            .color(p.emerald_text)
                            .size(heading_sz),
                    );
                }
                Problem::TrappingRain => {
                    ui.heading(
                        RichText::new("Trapped Rain Water Traversal Complete!")
                            .color(p.emerald_text)
                            .size(heading_sz),
                    );
                }
                Problem::LongestSubstring => {
                    ui.heading(
                        RichText::new("Longest Substring Without Repeating Characters Found!")
                            .color(p.emerald_text)
                            .size(heading_sz),
                    );
                }
                Problem::CharacterReplacement => {
                    ui.heading(
                        RichText::new("Longest Repeating Character Replacement Window Found!")
                            .color(p.emerald_text)
                            .size(heading_sz),
                    );
                }
                Problem::PermutationInString => {
                    if valid {
                        ui.heading(
                            RichText::new("Permutation of s1 Found in s2!")
                                .color(p.emerald_text)
                                .size(heading_sz),
                        );
                    } else {
                        ui.heading(
                            RichText::new("No Permutation of s1 Found in s2")
                                .color(p.red)
                                .size(heading_sz),
                        );
                    }
                }
                Problem::MinWindowSubstring => {
                    if valid {
                        ui.heading(
                            RichText::new("Minimum Window Substring Found!")
                                .color(p.emerald_text)
                                .size(heading_sz),
                        );
                    } else {
                        ui.heading(
                            RichText::new("No Valid Window Substring Found")
                                .color(p.red)
                                .size(heading_sz),
                        );
                    }
                }
                Problem::SlidingWindowMax => {
                    ui.heading(
                        RichText::new("Sliding Window Maximum Evaluation Complete!")
                            .color(p.emerald_text)
                            .size(heading_sz),
                    );
                }
                _ => {}
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn render_product(
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
