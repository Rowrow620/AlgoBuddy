use crate::app::VisualizerApp;
use crate::model::{EncodeDecodePhase, Problem, ThemePalette};
use eframe::egui::{self, Color32, RichText, Rounding, Stroke};

impl VisualizerApp {
    pub(super) fn render_heap_visualizer(
        &self,
        ui: &mut egui::Ui,
        p: &ThemePalette,
        heap: &[i32],
        active_idx: Option<usize>,
        swapped: Option<(usize, usize)>,
        label: &str,
    ) {
        let z = self.canvas_zoom;
        let is_plain_collection = self.selected_approach_id == 1
            && matches!(
                self.current_problem,
                Problem::KthLargestStream | Problem::LastStone
            );
        ui.heading(
            RichText::new(if is_plain_collection {
                format!("Array / Multiset View: {label}")
            } else {
                format!("Dual Tree & Array Heap View: {label}")
            })
            .color(p.amber)
            .size(16.0 * z),
        );
        ui.add_space(12.0 * z);

        ui.group(|ui| {
            ui.label(
                RichText::new(if is_plain_collection {
                    "COLLECTION VALUES IN CURRENT ORDER"
                } else {
                    "UNDERLYING HEAP ARRAY [Index: 2*i + 1, 2*i + 2]"
                })
                .font(egui::FontId::monospace(11.0 * z))
                .color(p.cyan),
            );
            ui.add_space(6.0 * z);
            ui.horizontal(|ui| {
                if heap.is_empty() {
                    ui.label(
                        RichText::new(if is_plain_collection {
                            "(Collection is empty)"
                        } else {
                            "(Heap is Empty)"
                        })
                        .italics()
                        .color(p.text_dim),
                    );
                }
                for (i, &val) in heap.iter().enumerate() {
                    let is_act = active_idx == Some(i);
                    let is_swp = swapped.is_some_and(|(a, b)| a == i || b == i);
                    let bg = if is_swp {
                        p.red
                    } else if is_act {
                        p.amber
                    } else {
                        p.cell_bg
                    };

                    egui::Frame::none()
                        .fill(bg)
                        .rounding(Rounding::same(6.0 * z))
                        .stroke(Stroke::new(1.0_f32 * z, p.purple))
                        .inner_margin((10.0 * z).max(6.0))
                        .show(ui, |ui| {
                            ui.vertical(|ui| {
                                ui.label(
                                    RichText::new(format!("i={}", i))
                                        .font(egui::FontId::monospace((10.0 * z).max(7.0)))
                                        .color(p.text_muted),
                                );
                                ui.label(
                                    RichText::new(val.to_string())
                                        .font(egui::FontId::monospace((16.0 * z).max(10.0)))
                                        .strong()
                                        .color(Color32::WHITE),
                                );
                            });
                        });
                }
            });
        });

        if is_plain_collection {
            return;
        }

        ui.add_space(16.0 * z);

        ui.group(|ui| {
            ui.label(
                RichText::new("BINARY TREE STRUCTURAL VIEW")
                    .font(egui::FontId::monospace(11.0 * z))
                    .color(p.emerald_text),
            );
            ui.add_space(6.0 * z);

            ui.horizontal(|ui| {
                for (i, &val) in heap.iter().enumerate() {
                    let is_act = active_idx == Some(i);
                    let bg = if is_act { p.amber } else { p.emerald };

                    egui::Frame::none()
                        .fill(bg)
                        .rounding(Rounding::same(20.0 * z))
                        .inner_margin((10.0 * z).max(6.0))
                        .show(ui, |ui| {
                            ui.label(
                                RichText::new(val.to_string())
                                    .font(egui::FontId::monospace((14.0 * z).max(9.0)))
                                    .strong()
                                    .color(Color32::WHITE),
                            );
                        });
                    if i + 1 < heap.len() {
                        ui.label(RichText::new("•").color(p.text_dim));
                    }
                }
            });
        });
    }

    pub(super) fn render_group_anagrams(
        &self,
        ui: &mut egui::Ui,
        p: &ThemePalette,
        input_strs: &[String],
        active_idx: Option<usize>,
        key_fmt: &str,
        groups: &std::collections::BTreeMap<String, Vec<String>>,
    ) {
        let z = self.canvas_zoom;
        let font_title = (16.0 * z).max(10.0);
        let font_label = (11.0 * z).max(8.0);
        let font_sz = (14.0 * z).max(9.0);
        let font_small = (10.0 * z).max(8.0);
        let margin = (10.0 * z).max(4.0);

        ui.heading(
            RichText::new("Group Anagrams (HashMap Buckets)")
                .color(p.cyan)
                .size(font_title),
        );
        ui.add_space(8.0 * z);

        ui.group(|ui| {
            ui.label(
                RichText::new("INPUT STRINGS ARRAY")
                    .font(egui::FontId::monospace(font_label))
                    .color(p.text_muted),
            );
            ui.horizontal(|ui| {
                for (i, s) in input_strs.iter().enumerate() {
                    let is_active = active_idx == Some(i);
                    let fill = if is_active { p.amber } else { p.cell_bg };

                    egui::Frame::none()
                        .fill(fill)
                        .rounding(Rounding::same(8.0 * z))
                        .stroke(Stroke::new(1.0_f32 * z, p.cell_border))
                        .inner_margin(margin)
                        .show(ui, |ui| {
                            ui.label(
                                RichText::new(format!("\"{}\"", s))
                                    .font(egui::FontId::monospace(font_sz))
                                    .strong()
                                    .color(Color32::WHITE),
                            );
                        });
                }
            });
        });

        if !key_fmt.is_empty() {
            ui.add_space(16.0 * z);
            egui::Frame::none()
                .fill(p.cell_bg)
                .rounding(Rounding::same(8.0 * z))
                .stroke(Stroke::new(1.0_f32 * z, p.cyan))
                .inner_margin(margin)
                .show(ui, |ui| {
                    ui.label(
                        RichText::new(format!("Computed Anagram Key Signature: {}", key_fmt))
                            .font(egui::FontId::monospace((13.0 * z).max(9.0)))
                            .strong()
                            .color(p.cyan),
                    );
                });
        }

        ui.add_space(20.0 * z);

        ui.group(|ui| {
            ui.label(
                RichText::new("HASHMAP GROUPS {signature -> list of words}")
                    .font(egui::FontId::monospace(font_label))
                    .color(p.emerald_text),
            );
            // Bucket cards contain a full-width separator, so stacking them keeps
            // later groups from being laid out beyond the horizontal clip rect.
            ui.vertical(|ui| {
                if groups.is_empty() {
                    ui.label(
                        RichText::new("No groups formed yet...")
                            .italics()
                            .color(p.text_dim),
                    );
                } else {
                    for (key, items) in groups {
                        egui::Frame::none()
                            .fill(p.sidebar_bg)
                            .rounding(Rounding::same(10.0 * z))
                            .stroke(Stroke::new(1.0_f32 * z, p.emerald))
                            .inner_margin((12.0 * z).max(6.0))
                            .show(ui, |ui| {
                                ui.vertical(|ui| {
                                    ui.label(
                                        RichText::new(format!("Key: {}", key))
                                            .font(egui::FontId::monospace(font_small))
                                            .color(p.text_muted),
                                    );
                                    ui.separator();
                                    ui.horizontal(|ui| {
                                        for word in items {
                                            egui::Frame::none()
                                                .fill(p.emerald)
                                                .rounding(Rounding::same(6.0 * z))
                                                .inner_margin((6.0 * z).max(3.0))
                                                .show(ui, |ui| {
                                                    ui.label(
                                                        RichText::new(format!("\"{}\"", word))
                                                            .font(egui::FontId::monospace(font_sz))
                                                            .strong()
                                                            .color(Color32::WHITE),
                                                    );
                                                });
                                        }
                                    });
                                });
                            });
                        ui.add_space(8.0 * z);
                    }
                }
            });
        });
    }

    pub(super) fn render_stack(
        &self,
        ui: &mut egui::Ui,
        p: &ThemePalette,
        chars: &[char],
        active_idx: Option<usize>,
        stack: &[char],
        is_valid: Option<bool>,
    ) {
        let z = self.canvas_zoom;
        let font_title = (16.0 * z).max(10.0);
        let font_char = (16.0 * z).max(9.0);
        let margin = (8.0 * z).max(4.0);
        let is_generation = self.current_problem == Problem::GenerateParentheses;
        let is_pair_reduction =
            self.current_problem == Problem::ValidParentheses && self.selected_approach_id == 1;

        ui.heading(
            RichText::new(if is_generation {
                "Generate Parentheses Backtracking"
            } else if is_pair_reduction {
                "Repeated Adjacent-Pair Removal"
            } else {
                "Vertical Stack Push / Pop Trace"
            })
            .color(p.cyan)
            .size(font_title),
        );
        ui.add_space(8.0 * z);

        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.label(
                    RichText::new(if is_generation {
                        "CURRENT PREFIX"
                    } else {
                        "EXPRESSION"
                    })
                    .font(egui::FontId::monospace((11.0 * z).max(8.0)))
                    .color(p.text_muted),
                );
                ui.horizontal(|ui| {
                    for (i, &c) in chars.iter().enumerate() {
                        let is_active = active_idx == Some(i)
                            || (is_pair_reduction
                                && active_idx.is_some_and(|pair_start| i == pair_start + 1));
                        let fill = if is_active { p.amber } else { p.cell_bg };
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

            if !is_pair_reduction {
                ui.add_space(30.0 * z);

                ui.group(|ui| {
                    ui.label(
                        RichText::new(if is_generation {
                            "BACKTRACK STACK (Top on right/bottom)"
                        } else {
                            "STACK (Top on right/bottom)"
                        })
                        .font(egui::FontId::monospace((11.0 * z).max(8.0)))
                        .color(p.text_muted),
                    );
                    ui.vertical(|ui| {
                        if stack.is_empty() {
                            ui.label(
                                RichText::new(if is_generation {
                                    "Current prefix is empty"
                                } else {
                                    "Stack is Empty []"
                                })
                                .italics()
                                .color(p.text_dim),
                            );
                        } else {
                            for (idx, &c) in stack.iter().rev().enumerate() {
                                let is_top = idx == 0;
                                let fill = if is_top { p.purple } else { p.cell_bg };
                                egui::Frame::none()
                                    .fill(fill)
                                    .rounding(Rounding::same(6.0 * z))
                                    .stroke(Stroke::new(1.0_f32 * z, p.cyan))
                                    .inner_margin(margin)
                                    .show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            if is_top {
                                                ui.label(
                                                    RichText::new("TOP ->")
                                                        .font(egui::FontId::monospace(
                                                            (10.0 * z).max(8.0),
                                                        ))
                                                        .color(p.amber),
                                                );
                                            }
                                            ui.label(
                                                RichText::new(c.to_string())
                                                    .font(egui::FontId::monospace(font_char))
                                                    .strong()
                                                    .color(Color32::WHITE),
                                            );
                                        });
                                    });
                            }
                        }
                    });
                });
            }
        });

        if let Some(valid) = is_valid {
            ui.add_space(20.0 * z);
            if is_generation && valid {
                ui.heading(
                    RichText::new("Parentheses Generation Complete!")
                        .color(p.emerald_text)
                        .size((18.0 * z).max(11.0)),
                );
            } else if valid {
                ui.heading(
                    RichText::new("Valid Parentheses Expression!")
                        .color(p.emerald_text)
                        .size((18.0 * z).max(11.0)),
                );
            } else {
                ui.heading(
                    RichText::new("Invalid Parentheses Expression")
                        .color(p.red)
                        .size((18.0 * z).max(11.0)),
                );
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn render_topk(
        &self,
        ui: &mut egui::Ui,
        p: &ThemePalette,
        nums: &[i32],
        active_nums_idx: Option<usize>,
        count_map: &std::collections::BTreeMap<i32, usize>,
        buckets: &[Vec<i32>],
        active_bucket_idx: Option<usize>,
        result: &[i32],
    ) {
        let z = self.canvas_zoom;
        let font_title = (16.0 * z).max(10.0);
        let font_num = (16.0 * z).max(9.0);
        let margin = (10.0 * z).max(4.0);

        ui.heading(
            RichText::new("1. Input Array & Frequency Map")
                .color(p.cyan)
                .size(font_title),
        );
        ui.add_space(8.0 * z);
        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.label(
                    RichText::new("NUMS ARRAY")
                        .font(egui::FontId::monospace((11.0 * z).max(8.0)))
                        .color(p.text_muted),
                );
                ui.horizontal(|ui| {
                    for (idx, &val) in nums.iter().enumerate() {
                        let fill = if active_nums_idx == Some(idx) {
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
                                        .font(egui::FontId::monospace(font_num))
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
                    RichText::new("COUNT MAP {num: frequency}")
                        .font(egui::FontId::monospace((11.0 * z).max(8.0)))
                        .color(p.text_muted),
                );
                ui.horizontal(|ui| {
                    if count_map.is_empty() {
                        ui.label(RichText::new("Empty {}").italics().color(p.text_dim));
                    } else {
                        for (&num, &cnt) in count_map.iter() {
                            egui::Frame::none()
                                .fill(p.cell_bg)
                                .rounding(Rounding::same(8.0 * z))
                                .stroke(Stroke::new(1.0_f32 * z, p.purple))
                                .inner_margin((8.0 * z).max(4.0))
                                .show(ui, |ui| {
                                    ui.vertical(|ui| {
                                        ui.label(
                                            RichText::new(format!("num: {}", num))
                                                .font(egui::FontId::proportional(
                                                    (12.0 * z).max(8.0),
                                                ))
                                                .color(p.text_primary),
                                        );
                                        ui.label(
                                            RichText::new(format!("{}", cnt))
                                                .font(egui::FontId::monospace(font_num))
                                                .strong()
                                                .color(p.purple),
                                        );
                                    });
                                });
                        }
                    }
                });
            });
        });

        ui.add_space(24.0 * z);

        ui.heading(
            RichText::new("2. Frequency Buckets (Index = Count)")
                .color(p.purple)
                .size(font_title),
        );
        ui.add_space(8.0 * z);
        ui.horizontal(|ui| {
            for (idx, items) in buckets.iter().enumerate() {
                let is_active = active_bucket_idx == Some(idx);
                let fill = if is_active { p.pink } else { p.sidebar_bg };
                egui::Frame::none()
                    .fill(fill)
                    .rounding(Rounding::same(10.0 * z))
                    .stroke(Stroke::new(
                        1.0_f32 * z,
                        if is_active {
                            Color32::WHITE
                        } else {
                            p.cell_border
                        },
                    ))
                    .inner_margin((12.0 * z).max(5.0))
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.label(
                                RichText::new(format!("freq[{}]", idx))
                                    .font(egui::FontId::monospace((12.0 * z).max(8.0)))
                                    .strong()
                                    .color(p.text_muted),
                            );
                            ui.separator();
                            if items.is_empty() {
                                ui.label(RichText::new("—").color(p.text_dim));
                            } else {
                                for &item in items {
                                    egui::Frame::none()
                                        .fill(p.cyan)
                                        .rounding(Rounding::same(6.0 * z))
                                        .inner_margin((6.0 * z).max(3.0))
                                        .show(ui, |ui| {
                                            ui.label(
                                                RichText::new(item.to_string())
                                                    .font(egui::FontId::monospace(
                                                        (14.0 * z).max(9.0),
                                                    ))
                                                    .strong()
                                                    .color(Color32::BLACK),
                                            );
                                        });
                                }
                            }
                        });
                    });
            }
        });

        ui.add_space(24.0 * z);

        ui.heading(
            RichText::new(format!(
                "3. Result Collector (Target k = {})",
                self.get_input_int(Problem::TopKFrequent, "k", 2)
            ))
            .color(p.emerald_text)
            .size(font_title),
        );
        ui.add_space(8.0 * z);
        ui.horizontal(|ui| {
            if result.is_empty() {
                ui.label(
                    RichText::new("Result array is empty...")
                        .italics()
                        .color(p.text_dim),
                );
            } else {
                for &val in result {
                    egui::Frame::none()
                        .fill(p.emerald)
                        .rounding(Rounding::same(10.0 * z))
                        .inner_margin((12.0 * z).max(5.0))
                        .show(ui, |ui| {
                            ui.label(
                                RichText::new(val.to_string())
                                    .font(egui::FontId::monospace((18.0 * z).max(10.0)))
                                    .strong()
                                    .color(Color32::WHITE),
                            );
                        });
                }
            }
        });
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn render_encode_decode(
        &self,
        ui: &mut egui::Ui,
        p: &ThemePalette,
        input_strs: &[String],
        encoded_so_far: &str,
        decoded_so_far: &[String],
        pointer: usize,
        active_str_idx: Option<usize>,
        phase: &EncodeDecodePhase,
    ) {
        let z = self.canvas_zoom;
        let font_title = (16.0 * z).max(10.0);
        let font_sz = (14.0 * z).max(9.0);
        let margin = (10.0 * z).max(4.0);

        ui.heading(
            RichText::new("1. Input Strings")
                .color(p.cyan)
                .size(font_title),
        );
        ui.add_space(8.0 * z);
        ui.horizontal(|ui| {
            for (idx, s) in input_strs.iter().enumerate() {
                let is_active = active_str_idx == Some(idx);
                let fill = if is_active { p.amber } else { p.cell_bg };
                egui::Frame::none()
                    .fill(fill)
                    .rounding(Rounding::same(8.0 * z))
                    .stroke(Stroke::new(1.0_f32 * z, p.cell_border))
                    .inner_margin(margin)
                    .show(ui, |ui| {
                        ui.label(
                            RichText::new(format!("\"{}\"", s))
                                .font(egui::FontId::monospace(font_sz))
                                .strong()
                                .color(Color32::WHITE),
                        );
                    });
            }
        });

        ui.add_space(24.0 * z);

        ui.heading(
            RichText::new("2. Encoded String")
                .color(p.purple)
                .size(font_title),
        );
        ui.add_space(8.0 * z);
        if encoded_so_far.is_empty() {
            ui.label(RichText::new("\"\" (empty)").italics().color(p.text_dim));
        } else {
            ui.horizontal_wrapped(|ui| {
                for (i, ch) in encoded_so_far.chars().enumerate() {
                    let is_ptr = *phase == EncodeDecodePhase::Decoding && i == pointer;
                    let fill = if is_ptr {
                        p.pink
                    } else if ch == '#' {
                        p.purple
                    } else {
                        p.cell_bg
                    };
                    egui::Frame::none()
                        .fill(fill)
                        .rounding(Rounding::same(4.0 * z))
                        .inner_margin((6.0 * z).max(3.0))
                        .show(ui, |ui| {
                            ui.label(
                                RichText::new(ch.to_string())
                                    .font(egui::FontId::monospace(font_sz))
                                    .strong()
                                    .color(Color32::WHITE),
                            );
                        });
                }
            });
        }

        ui.add_space(24.0 * z);

        ui.heading(
            RichText::new("3. Decoded Strings")
                .color(p.emerald_text)
                .size(font_title),
        );
        ui.add_space(8.0 * z);
        if decoded_so_far.is_empty() {
            ui.label(
                RichText::new("Decoded list is empty...")
                    .italics()
                    .color(p.text_dim),
            );
        } else {
            ui.horizontal(|ui| {
                for s in decoded_so_far {
                    egui::Frame::none()
                        .fill(p.emerald)
                        .rounding(Rounding::same(10.0 * z))
                        .inner_margin((12.0 * z).max(5.0))
                        .show(ui, |ui| {
                            ui.label(
                                RichText::new(format!("\"{}\"", s))
                                    .font(egui::FontId::monospace(font_sz))
                                    .strong()
                                    .color(Color32::WHITE),
                            );
                        });
                }
            });
        }
    }
}
