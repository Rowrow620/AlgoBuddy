use crate::app::VisualizerApp;
use crate::model::{MergeListPhase, Problem, ThemePalette};
use eframe::egui::{self, Color32, RichText, Rounding, Stroke};

impl VisualizerApp {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn render_merge_lists(
        &self,
        ui: &mut egui::Ui,
        p: &ThemePalette,
        list1: &[i32],
        list2: &[i32],
        p1_idx: Option<usize>,
        p2_idx: Option<usize>,
        merged: &[i32],
        phase: MergeListPhase,
    ) {
        let z = self.canvas_zoom;
        let font_title = (16.0 * z).max(10.0);
        let font_label = (11.0 * z).max(8.0);
        let font_node = (14.0 * z).max(9.0);
        let margin = (8.0 * z).max(4.0);

        let approach_name = self
            .current_problem
            .details()
            .approach_by_id(self.selected_approach_id)
            .map_or("Merge Two Sorted Linked Lists", |approach| approach.name);
        ui.heading(RichText::new(approach_name).color(p.cyan).size(font_title));
        ui.add_space(8.0 * z);

        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.label(
                    RichText::new("LIST 1")
                        .font(egui::FontId::monospace(font_label))
                        .color(p.text_muted),
                );
                ui.horizontal(|ui| {
                    for (i, &val) in list1.iter().enumerate() {
                        let fill = if p1_idx == Some(i) { p.cyan } else { p.cell_bg };
                        egui::Frame::none()
                            .fill(fill)
                            .rounding(Rounding::same(8.0 * z))
                            .stroke(Stroke::new(1.0_f32 * z, p.cell_border))
                            .inner_margin(margin)
                            .show(ui, |ui| {
                                ui.label(
                                    RichText::new(format!("( {} ) ->", val))
                                        .font(egui::FontId::monospace(font_node))
                                        .strong()
                                        .color(Color32::WHITE),
                                );
                            });
                    }
                    ui.label(
                        RichText::new("None")
                            .font(egui::FontId::monospace((12.0 * z).max(8.0)))
                            .color(p.text_dim),
                    );
                });
            });

            ui.add_space(20.0 * z);

            ui.group(|ui| {
                ui.label(
                    RichText::new("LIST 2")
                        .font(egui::FontId::monospace(font_label))
                        .color(p.text_muted),
                );
                ui.horizontal(|ui| {
                    for (i, &val) in list2.iter().enumerate() {
                        let fill = if p2_idx == Some(i) { p.pink } else { p.cell_bg };
                        egui::Frame::none()
                            .fill(fill)
                            .rounding(Rounding::same(8.0 * z))
                            .stroke(Stroke::new(1.0_f32 * z, p.cell_border))
                            .inner_margin(margin)
                            .show(ui, |ui| {
                                ui.label(
                                    RichText::new(format!("( {} ) ->", val))
                                        .font(egui::FontId::monospace(font_node))
                                        .strong()
                                        .color(Color32::WHITE),
                                );
                            });
                    }
                    ui.label(
                        RichText::new("None")
                            .font(egui::FontId::monospace((12.0 * z).max(8.0)))
                            .color(p.text_dim),
                    );
                });
            });
        });

        ui.add_space(20.0 * z);

        ui.group(|ui| {
            ui.label(
                RichText::new(match phase {
                    MergeListPhase::Collecting => "COLLECTED VALUES (UNSORTED)",
                    MergeListPhase::SortedValues => "SORTED VALUES",
                    MergeListPhase::Rebuilding => "REBUILT LINKED LIST (IN PROGRESS)",
                    MergeListPhase::Complete if self.selected_approach_id == 1 => {
                        "REBUILT SORTED LINKED LIST"
                    }
                    _ => "MERGED SORTED LIST (TAIL ATTACHMENTS)",
                })
                .font(egui::FontId::monospace(font_label))
                .color(p.emerald_text),
            );
            ui.horizontal(|ui| {
                if merged.is_empty() {
                    ui.label(
                        RichText::new("Dummy Head -> None")
                            .font(egui::FontId::monospace((14.0 * z).max(9.0)))
                            .color(p.text_dim),
                    );
                } else {
                    for &val in merged {
                        egui::Frame::none()
                            .fill(p.emerald)
                            .rounding(Rounding::same(8.0 * z))
                            .inner_margin((10.0 * z).max(4.0))
                            .show(ui, |ui| {
                                ui.label(
                                    RichText::new(
                                        if matches!(
                                            phase,
                                            MergeListPhase::Collecting
                                                | MergeListPhase::SortedValues
                                        ) {
                                            format!("[ {} ]", val)
                                        } else {
                                            format!("( {} ) ->", val)
                                        },
                                    )
                                    .font(egui::FontId::monospace((16.0 * z).max(10.0)))
                                    .strong()
                                    .color(Color32::WHITE),
                                );
                            });
                    }
                    ui.label(
                        RichText::new("None")
                            .font(egui::FontId::monospace((14.0 * z).max(9.0)))
                            .color(p.text_dim),
                    );
                }
            });
        });
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn render_list_cycle(
        &self,
        ui: &mut egui::Ui,
        p: &ThemePalette,
        nodes: &[i32],
        cycle_target: Option<usize>,
        slow: Option<usize>,
        fast: Option<usize>,
        visited: &std::collections::BTreeSet<usize>,
        has_cycle: Option<bool>,
    ) {
        let z = self.canvas_zoom;
        let font_title = (16.0 * z).max(10.0);
        let font_label = (11.0 * z).max(8.0);
        let font_node = (16.0 * z).max(9.0);
        let margin = (10.0 * z).max(4.0);

        let uses_visited_set =
            self.current_problem == Problem::LinkedListCycle && self.selected_approach_id == 1;
        let approach_name = self
            .current_problem
            .details()
            .approach_by_id(self.selected_approach_id)
            .map_or("Linked List Cycle Detection", |approach| approach.name);
        ui.heading(RichText::new(approach_name).color(p.cyan).size(font_title));
        ui.add_space(8.0 * z);

        ui.group(|ui| {
            ui.label(
                RichText::new("LINKED LIST NODES")
                    .font(egui::FontId::monospace(font_label))
                    .color(p.text_muted),
            );
            ui.horizontal(|ui| {
                for (i, &val) in nodes.iter().enumerate() {
                    let is_slow = slow == Some(i);
                    let is_fast = fast == Some(i);
                    let is_cycle_target = cycle_target == Some(i);
                    let is_visited = visited.contains(&i);

                    let fill = if is_slow && is_fast {
                        p.purple
                    } else if is_slow {
                        p.cyan
                    } else if is_fast {
                        p.pink
                    } else if is_cycle_target {
                        p.amber
                    } else if is_visited {
                        p.emerald
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
                                let mut label = String::new();
                                if uses_visited_set && is_slow {
                                    label.push_str("current");
                                } else if is_slow && is_fast {
                                    label.push_str("S & F");
                                } else if is_slow {
                                    label.push_str("slow");
                                } else if is_fast {
                                    label.push_str("fast");
                                } else if uses_visited_set && is_visited {
                                    label.push_str("seen");
                                }

                                ui.label(
                                    RichText::new(format!("idx {} {}", i, label))
                                        .font(egui::FontId::proportional((10.0 * z).max(8.0)))
                                        .color(Color32::WHITE),
                                );
                                ui.label(
                                    RichText::new(format!("( {} ) ->", val))
                                        .font(egui::FontId::monospace(font_node))
                                        .strong()
                                        .color(Color32::WHITE),
                                );
                            });
                        });
                }

                if let Some(target) = cycle_target {
                    ui.label(
                        RichText::new(format!("↺ [Cycle -> node idx {}]", target))
                            .font(egui::FontId::monospace((14.0 * z).max(9.0)))
                            .strong()
                            .color(p.amber),
                    );
                } else {
                    ui.label(
                        RichText::new("None")
                            .font(egui::FontId::monospace((14.0 * z).max(9.0)))
                            .color(p.text_dim),
                    );
                }
            });
        });

        if uses_visited_set {
            ui.add_space(12.0 * z);
            ui.group(|ui| {
                ui.label(
                    RichText::new(format!("VISITED NODE SET: {:?}", visited))
                        .font(egui::FontId::monospace(font_label))
                        .color(p.emerald_text),
                );
            });
        }

        if let Some(cycle) = has_cycle {
            ui.add_space(20.0 * z);
            if cycle && uses_visited_set {
                ui.heading(
                    RichText::new("Cycle Detected: Current Node Was Already Visited")
                        .color(p.emerald_text)
                        .size((18.0 * z).max(11.0)),
                );
            } else if cycle {
                ui.heading(
                    RichText::new("Cycle Detected! Slow & Fast Pointers Met.")
                        .color(p.emerald_text)
                        .size((18.0 * z).max(11.0)),
                );
            } else if uses_visited_set {
                ui.heading(
                    RichText::new("No Cycle Exists (Traversal Reached the End)")
                        .color(p.red)
                        .size((18.0 * z).max(11.0)),
                );
            } else {
                ui.heading(
                    RichText::new("No Cycle Exists (Fast Pointer Reached End)")
                        .color(p.red)
                        .size((18.0 * z).max(11.0)),
                );
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn render_linked_list(
        &self,
        ui: &mut egui::Ui,
        p: &ThemePalette,
        nodes: &[i32],
        prev_idx: Option<usize>,
        curr_idx: Option<usize>,
        next_idx: Option<usize>,
        reversed_so_far: &[i32],
    ) {
        let z = self.canvas_zoom;
        let font_title = (16.0 * z).max(10.0);
        let font_label = (11.0 * z).max(8.0);
        let font_node = (16.0 * z).max(9.0);
        let margin = (10.0 * z).max(4.0);

        let recursive =
            self.current_problem == Problem::ReverseLinkedList && self.selected_approach_id == 1;
        let approach_name = self
            .current_problem
            .details()
            .approach_by_id(self.selected_approach_id)
            .map_or("Singly-Linked List Pointer Reversal", |approach| {
                approach.name
            });
        ui.heading(RichText::new(approach_name).color(p.cyan).size(font_title));
        ui.add_space(8.0 * z);

        ui.group(|ui| {
            ui.label(
                RichText::new("ORIGINAL LINKED LIST NODES")
                    .font(egui::FontId::monospace(font_label))
                    .color(p.text_muted),
            );
            ui.horizontal(|ui| {
                for (i, &val) in nodes.iter().enumerate() {
                    let is_prev = prev_idx == Some(i);
                    let is_curr = curr_idx == Some(i);
                    let is_nxt = next_idx == Some(i);

                    let fill = if is_curr {
                        p.cyan
                    } else if is_prev {
                        p.purple
                    } else if is_nxt {
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
                                let mut label = String::new();
                                if recursive {
                                    if is_prev {
                                        label.push_str("linked child ");
                                    }
                                    if is_curr {
                                        label.push_str("frame ");
                                    }
                                    if is_nxt {
                                        label.push_str("recurse ");
                                    }
                                } else {
                                    if is_prev {
                                        label.push_str("prev ");
                                    }
                                    if is_curr {
                                        label.push_str("curr ");
                                    }
                                    if is_nxt {
                                        label.push_str("nxt ");
                                    }
                                }

                                ui.label(
                                    RichText::new(format!("idx {} {}", i, label))
                                        .font(egui::FontId::proportional((10.0 * z).max(8.0)))
                                        .color(Color32::WHITE),
                                );
                                ui.label(
                                    RichText::new(format!("( {} ) ->", val))
                                        .font(egui::FontId::monospace(font_node))
                                        .strong()
                                        .color(Color32::WHITE),
                                );
                            });
                        });
                }
                ui.label(
                    RichText::new("None")
                        .font(egui::FontId::monospace((14.0 * z).max(9.0)))
                        .color(p.text_dim),
                );
            });
        });

        ui.add_space(20.0 * z);

        ui.group(|ui| {
            ui.label(
                RichText::new("REVERSED LINKED LIST (Constructed from head)")
                    .font(egui::FontId::monospace(font_label))
                    .color(p.emerald_text),
            );
            ui.horizontal(|ui| {
                if reversed_so_far.is_empty() {
                    ui.label(
                        RichText::new("None")
                            .font(egui::FontId::monospace((14.0 * z).max(9.0)))
                            .color(p.text_dim),
                    );
                } else {
                    for (i, &val) in reversed_so_far.iter().enumerate() {
                        let fill = if i == 0 { p.emerald } else { p.cell_bg };
                        egui::Frame::none()
                            .fill(fill)
                            .rounding(Rounding::same(8.0 * z))
                            .stroke(Stroke::new(1.0_f32 * z, p.emerald_text))
                            .inner_margin(margin)
                            .show(ui, |ui| {
                                ui.label(
                                    RichText::new(format!("( {} ) ->", val))
                                        .font(egui::FontId::monospace(font_node))
                                        .strong()
                                        .color(Color32::WHITE),
                                );
                            });
                    }
                    ui.label(
                        RichText::new("None")
                            .font(egui::FontId::monospace((14.0 * z).max(9.0)))
                            .color(p.text_dim),
                    );
                }
            });
        });
    }
}
