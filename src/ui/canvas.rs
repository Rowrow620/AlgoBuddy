use crate::app::VisualizerApp;
use crate::model::{EncodeDecodePhase, Problem, ProductPhase, ThemePalette, VisualState};
use eframe::egui::{self, Color32, Frame, RichText, Rounding, Stroke};

impl VisualizerApp {
    pub(crate) fn render_central_canvas(&mut self, ctx: &egui::Context, p: &ThemePalette) {
        egui::CentralPanel::default()
            .frame(
                Frame::none()
                    .stroke(egui::Stroke::NONE)
                    .inner_margin(16.0)
                    .fill(p.bg_dark),
            )
            .show(ctx, |ui| {
                // Ctrl + Mouse Wheel Zoom Listener
                if ui.rect_contains_pointer(ui.max_rect()) {
                    let scroll_delta = ctx.input(|i| i.raw_scroll_delta.y);
                    let ctrl_down = ctx.input(|i| i.modifiers.ctrl);
                    if ctrl_down && scroll_delta != 0.0 {
                        let factor = if scroll_delta > 0.0 { 1.08 } else { 0.92 };
                        self.canvas_zoom = (self.canvas_zoom * factor).clamp(0.7, 2.2);
                    }
                }

                // Custom Test Case Sandbox Input Bar
                self.render_custom_playground_bar(ui, p);
                ui.add_space(8.0);

                if let Some(step) = self.steps.get(self.current_step_idx) {
                    // Live State Inspector Banner with Zoom Controls
                    egui::Frame::none()
                        .fill(p.sidebar_bg)
                        .rounding(Rounding::same(8.0))
                        .stroke(Stroke::new(1.0_f32, p.cyan))
                        .inner_margin(egui::Margin::symmetric(14.0, 10.0))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        let zoom_pct = (self.canvas_zoom * 100.0).round() as u32;
                                        if ui.button("Reset").clicked() {
                                            self.canvas_zoom = 1.0;
                                        }
                                        if ui.button("+").clicked() {
                                            self.canvas_zoom = (self.canvas_zoom + 0.1).min(2.2);
                                        }
                                        if ui.button("−").clicked() {
                                            self.canvas_zoom = (self.canvas_zoom - 0.1).max(0.7);
                                        }
                                        ui.label(
                                            RichText::new(format!("🔍 {}%", zoom_pct))
                                                .font(egui::FontId::monospace(11.0))
                                                .color(p.cyan),
                                        );

                                        ui.with_layout(
                                            egui::Layout::left_to_right(egui::Align::Center),
                                            |ui| {
                                                ui.style_mut().wrap_mode =
                                                    Some(egui::TextWrapMode::Truncate);
                                                ui.label(
                                                    RichText::new("📊 Live State Inspector")
                                                        .font(egui::FontId::proportional(12.0))
                                                        .color(p.cyan)
                                                        .strong(),
                                                );
                                                ui.separator();
                                                let step_lbl = ui.label(
                                                    RichText::new(&step.description)
                                                        .font(egui::FontId::proportional(13.0))
                                                        .color(p.text_primary),
                                                );
                                                step_lbl.on_hover_text(&step.description);
                                            },
                                        );
                                    },
                                );
                            });
                        });
                    ui.add_space(14.0);

                    egui::ScrollArea::both().show(ui, |ui| match &step.visual {
                        VisualState::ContainsDuplicate {
                            nums,
                            active_idx,
                            seen_set,
                            duplicate_val,
                            has_duplicate,
                        } => {
                            self.render_contains_duplicate(
                                ui,
                                p,
                                nums,
                                *active_idx,
                                seen_set,
                                *duplicate_val,
                                *has_duplicate,
                            );
                        }
                        VisualState::GroupAnagrams {
                            input_strs,
                            active_idx,
                            key_fmt,
                            groups,
                        } => {
                            self.render_group_anagrams(
                                ui,
                                p,
                                input_strs,
                                *active_idx,
                                key_fmt,
                                groups,
                            );
                        }
                        VisualState::TwoSum {
                            nums,
                            target,
                            active_idx,
                            secondary_idx,
                            map,
                            found_indices,
                        } => {
                            self.render_two_sum(
                                ui,
                                p,
                                nums,
                                *target,
                                *active_idx,
                                *secondary_idx,
                                map,
                                *found_indices,
                            );
                        }
                        VisualState::ValidAnagram {
                            s,
                            t,
                            s_counts,
                            t_counts,
                            active_s_idx,
                            active_t_idx,
                            is_anagram,
                        } => {
                            self.render_valid_anagram(
                                ui,
                                p,
                                s,
                                t,
                                s_counts,
                                t_counts,
                                *active_s_idx,
                                *active_t_idx,
                                *is_anagram,
                            );
                        }
                        VisualState::TwoPointers {
                            chars,
                            left,
                            right,
                            is_valid,
                            skipped,
                        } => {
                            self.render_two_pointers(
                                ui, p, chars, *left, *right, *is_valid, *skipped,
                            );
                        }
                        VisualState::Stack {
                            chars,
                            active_idx,
                            stack,
                            is_valid,
                        } => {
                            self.render_stack(ui, p, chars, *active_idx, stack, *is_valid);
                        }
                        VisualState::BestTimeStock {
                            prices,
                            left_buy,
                            right_sell,
                            current_profit,
                            max_profit,
                        } => {
                            self.render_stock(
                                ui,
                                p,
                                prices,
                                *left_buy,
                                *right_sell,
                                *current_profit,
                                *max_profit,
                            );
                        }
                        VisualState::BinarySearch {
                            nums,
                            target,
                            left,
                            right,
                            mid,
                            found_idx,
                        } => {
                            self.render_binary_search(
                                ui, p, nums, *target, *left, *right, *mid, *found_idx,
                            );
                        }
                        VisualState::LinkedList {
                            nodes,
                            prev_idx,
                            curr_idx,
                            next_idx,
                            reversed_so_far,
                        } => {
                            self.render_linked_list(
                                ui,
                                p,
                                nodes,
                                *prev_idx,
                                *curr_idx,
                                *next_idx,
                                reversed_so_far,
                            );
                        }
                        VisualState::MergeLinkedLists {
                            list1,
                            list2,
                            p1_idx,
                            p2_idx,
                            merged_so_far,
                        } => {
                            self.render_merge_lists(
                                ui,
                                p,
                                list1,
                                list2,
                                *p1_idx,
                                *p2_idx,
                                merged_so_far,
                            );
                        }
                        VisualState::LinkedListCycle {
                            nodes,
                            cycle_target_idx,
                            slow_idx,
                            fast_idx,
                            has_cycle,
                        } => {
                            self.render_list_cycle(
                                ui,
                                p,
                                nodes,
                                *cycle_target_idx,
                                *slow_idx,
                                *fast_idx,
                                *has_cycle,
                            );
                        }
                        VisualState::TreeVisual {
                            tree_nodes,
                            active_node_idx,
                            secondary_node_idx,
                            depth_val,
                            max_diameter,
                        } => {
                            self.render_tree(
                                ui,
                                p,
                                tree_nodes,
                                *active_node_idx,
                                *secondary_node_idx,
                                *depth_val,
                                *max_diameter,
                            );
                        }
                        VisualState::ValidSudoku {
                            board,
                            active_r,
                            active_c,
                            duplicate_pos,
                            is_valid,
                        } => {
                            self.render_sudoku(
                                ui,
                                p,
                                board,
                                *active_r,
                                *active_c,
                                *duplicate_pos,
                                *is_valid,
                            );
                        }
                        VisualState::LongestConsecutive {
                            nums,
                            num_set,
                            current_num,
                            current_seq,
                            max_length,
                            is_seq_start,
                        } => {
                            self.render_longest_consecutive(
                                ui,
                                p,
                                nums,
                                num_set,
                                *current_num,
                                current_seq,
                                *max_length,
                                *is_seq_start,
                            );
                        }
                        VisualState::TopK {
                            nums,
                            active_nums_idx,
                            count_map,
                            buckets,
                            active_bucket_idx,
                            result,
                        } => {
                            self.render_topk(
                                ui,
                                p,
                                nums,
                                *active_nums_idx,
                                count_map,
                                buckets,
                                *active_bucket_idx,
                                result,
                            );
                        }
                        VisualState::EncodeDecode {
                            input_strs,
                            encoded_so_far,
                            decoded_so_far,
                            pointer,
                            active_str_idx,
                            phase,
                        } => {
                            self.render_encode_decode(
                                ui,
                                p,
                                input_strs,
                                encoded_so_far,
                                decoded_so_far,
                                *pointer,
                                *active_str_idx,
                                phase,
                            );
                        }
                        VisualState::Product {
                            nums,
                            output,
                            active_idx,
                            prefix_val,
                            suffix_val,
                            phase,
                        } => {
                            self.render_product(
                                ui,
                                p,
                                nums,
                                output,
                                *active_idx,
                                *prefix_val,
                                *suffix_val,
                                phase,
                            );
                        }
                        VisualState::Trie { .. } => {
                            self.render_trie(ui, p);
                        }
                        VisualState::HeapVisual {
                            heap_elements,
                            active_idx,
                            swapped_pair,
                            heap_type_label,
                        } => {
                            self.render_heap_visualizer(
                                ui,
                                p,
                                heap_elements,
                                *active_idx,
                                *swapped_pair,
                                heap_type_label,
                            );
                        }
                        VisualState::DecisionTreeVisual {
                            current_path,
                            active_choice,
                            completed_results,
                        } => {
                            self.render_decision_tree_visualizer(
                                ui,
                                p,
                                current_path,
                                active_choice.as_deref(),
                                completed_results,
                            );
                        }
                        VisualState::GridGraph {
                            rows,
                            cols,
                            grid,
                            active_cell,
                            visited_cells,
                            frontier_cells,
                            message,
                        } => {
                            self.render_grid_graph(
                                ui,
                                p,
                                *rows,
                                *cols,
                                grid,
                                *active_cell,
                                visited_cells,
                                frontier_cells,
                                message,
                            );
                        }
                        VisualState::NodeGraph {
                            nodes,
                            node_labels,
                            edges,
                            active_node,
                            active_edge,
                            visited_nodes,
                            cycle_edges,
                            topo_order,
                            message,
                        } => {
                            self.render_node_graph(
                                ui,
                                p,
                                nodes,
                                node_labels,
                                edges,
                                *active_node,
                                *active_edge,
                                visited_nodes,
                                cycle_edges,
                                topo_order,
                                message,
                            );
                        }
                        VisualState::Array1D {
                            title,
                            elements,
                            active_idx,
                            secondary_idx,
                            pointers,
                            status_message,
                            is_success,
                        } => {
                            self.render_array_1d(
                                ui,
                                p,
                                title,
                                elements,
                                *active_idx,
                                *secondary_idx,
                                pointers,
                                status_message,
                                *is_success,
                            );
                        }
                    });
                }
            });
    }
}

impl VisualizerApp {
    #[allow(clippy::too_many_arguments)]
    fn render_array_1d(
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
    fn render_heap_visualizer(
        &self,
        ui: &mut egui::Ui,
        p: &ThemePalette,
        heap: &[i32],
        active_idx: Option<usize>,
        swapped: Option<(usize, usize)>,
        label: &str,
    ) {
        let z = self.canvas_zoom;
        ui.heading(
            RichText::new(format!("Dual Tree & Array Heap View: {}", label))
                .color(p.amber)
                .size(16.0 * z),
        );
        ui.add_space(12.0 * z);

        // 1. Array Representation with 2*i+1 and 2*i+2 formulas
        ui.group(|ui| {
            ui.label(
                RichText::new("UNDERLYING HEAP ARRAY [Index: 2*i + 1, 2*i + 2]")
                    .font(egui::FontId::monospace(11.0 * z))
                    .color(p.cyan),
            );
            ui.add_space(6.0 * z);
            ui.horizontal(|ui| {
                if heap.is_empty() {
                    ui.label(RichText::new("(Heap is Empty)").italics().color(p.text_dim));
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

        ui.add_space(16.0 * z);

        // 2. Binary Tree Node Layout
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

    fn render_decision_tree_visualizer(
        &self,
        ui: &mut egui::Ui,
        p: &ThemePalette,
        current_path: &[i32],
        active_choice: Option<&str>,
        completed_results: &[Vec<i32>],
    ) {
        let z = self.canvas_zoom;
        ui.heading(
            RichText::new("Backtracking & Recursion Decision Tree Visualizer")
                .color(p.cyan)
                .size(16.0 * z),
        );
        ui.add_space(12.0 * z);

        ui.horizontal(|ui| {
            // Current Active Recursive Branch Path
            ui.group(|ui| {
                ui.label(
                    RichText::new("ACTIVE DECISION BRANCH PATH")
                        .font(egui::FontId::monospace(11.0 * z))
                        .color(p.amber),
                );
                ui.add_space(6.0 * z);
                ui.horizontal(|ui| {
                    if current_path.is_empty() {
                        ui.label(
                            RichText::new("[] (Root Level)")
                                .font(egui::FontId::monospace((14.0 * z).max(9.0)))
                                .color(p.text_dim),
                        );
                    } else {
                        for (i, &val) in current_path.iter().enumerate() {
                            egui::Frame::none()
                                .fill(p.cyan)
                                .rounding(Rounding::same(6.0 * z))
                                .inner_margin((8.0 * z).max(4.0))
                                .show(ui, |ui| {
                                    ui.label(
                                        RichText::new(val.to_string())
                                            .font(egui::FontId::monospace((16.0 * z).max(10.0)))
                                            .strong()
                                            .color(Color32::WHITE),
                                    );
                                });
                            if i + 1 < current_path.len() {
                                ui.label(RichText::new("➔").color(p.amber));
                            }
                        }
                    }
                });
                if let Some(choice) = active_choice {
                    ui.add_space(6.0 * z);
                    ui.label(
                        RichText::new(format!("Current Decision: {}", choice))
                            .font(egui::FontId::proportional(12.0 * z))
                            .color(p.emerald_text)
                            .strong(),
                    );
                }
            });

            ui.add_space(16.0 * z);

            // Completed Subsets / Permutations Grid
            ui.group(|ui| {
                ui.label(
                    RichText::new(format!("GENERATED SOLUTIONS ({})", completed_results.len()))
                        .font(egui::FontId::monospace(11.0 * z))
                        .color(p.emerald_text),
                );
                ui.add_space(6.0 * z);
                ui.horizontal_wrapped(|ui| {
                    for res in completed_results {
                        egui::Frame::none()
                            .fill(p.step_box_bg)
                            .rounding(Rounding::same(4.0 * z))
                            .inner_margin((6.0 * z).max(3.0))
                            .show(ui, |ui| {
                                ui.label(
                                    RichText::new(format!("{:?}", res))
                                        .font(egui::FontId::monospace((12.0 * z).max(8.0)))
                                        .color(p.text_primary),
                                );
                            });
                    }
                });
            });
        });
    }

    #[allow(clippy::too_many_arguments)]
    fn render_grid_graph(
        &self,
        ui: &mut egui::Ui,
        p: &ThemePalette,
        rows: usize,
        cols: usize,
        grid: &[Vec<String>],
        active_cell: Option<(usize, usize)>,
        visited_cells: &std::collections::BTreeSet<(usize, usize)>,
        _frontier_cells: &std::collections::BTreeSet<(usize, usize)>,
        message: &str,
    ) {
        let z = self.canvas_zoom;
        ui.heading(
            RichText::new(format!("2D Grid Graph Explorer: {}", message))
                .color(p.cyan)
                .size(16.0 * z),
        );
        ui.add_space(12.0 * z);

        ui.group(|ui| {
            ui.label(
                RichText::new(format!("GRID MATRIX ({} Rows x {} Cols)", rows, cols))
                    .font(egui::FontId::monospace(11.0 * z))
                    .color(p.purple),
            );
            ui.add_space(8.0 * z);

            egui::Grid::new("graph_grid_view")
                .spacing([6.0 * z, 6.0 * z])
                .show(ui, |ui| {
                    for r in 0..rows {
                        for c in 0..cols {
                            let val = grid
                                .get(r)
                                .and_then(|row| row.get(c))
                                .cloned()
                                .unwrap_or_default();
                            let is_active = active_cell == Some((r, c));
                            let is_visited = visited_cells.contains(&(r, c));

                            let bg = if is_active {
                                p.amber
                            } else if is_visited {
                                p.purple
                            } else if val == "1" {
                                p.emerald
                            } else if val == "0" {
                                p.cyan
                            } else if val == "-1" {
                                p.cell_border
                            } else {
                                p.cell_bg
                            };

                            let text_color = if is_active || is_visited || val == "1" || val == "0"
                            {
                                Color32::WHITE
                            } else {
                                p.text_primary
                            };

                            egui::Frame::none()
                                .fill(bg)
                                .rounding(Rounding::same(6.0 * z))
                                .stroke(Stroke::new(
                                    1.0 * z,
                                    if is_active { p.red } else { p.cell_border },
                                ))
                                .inner_margin(egui::Margin::symmetric(
                                    (14.0 * z).max(8.0),
                                    (10.0 * z).max(6.0),
                                ))
                                .show(ui, |ui| {
                                    ui.label(
                                        RichText::new(&val)
                                            .font(egui::FontId::monospace((15.0 * z).max(10.0)))
                                            .strong()
                                            .color(text_color),
                                    );
                                });
                        }
                        ui.end_row();
                    }
                });
        });
    }

    #[allow(clippy::too_many_arguments)]
    fn render_node_graph(
        &self,
        ui: &mut egui::Ui,
        p: &ThemePalette,
        nodes: &[usize],
        node_labels: &[String],
        edges: &[(usize, usize)],
        active_node: Option<usize>,
        active_edge: Option<(usize, usize)>,
        visited_nodes: &std::collections::BTreeSet<usize>,
        cycle_edges: &std::collections::BTreeSet<(usize, usize)>,
        topo_order: &[usize],
        message: &str,
    ) {
        let z = self.canvas_zoom;
        ui.heading(
            RichText::new(format!("Graph Topology & Connectivity: {}", message))
                .color(p.cyan)
                .size(16.0 * z),
        );
        ui.add_space(10.0 * z);

        ui.group(|ui| {
            ui.label(
                RichText::new("2D GRAPH TOPOLOGY & DIRECTED EDGES CANVAS")
                    .font(egui::FontId::monospace(11.0 * z))
                    .color(p.emerald_text),
            );
            ui.add_space(8.0 * z);

            let canvas_width = (520.0 * z).max(320.0);
            let canvas_height = (320.0 * z).max(220.0);

            let (response, painter) = ui.allocate_painter(
                egui::vec2(canvas_width, canvas_height),
                egui::Sense::hover(),
            );
            let rect = response.rect;

            // Background canvas fill
            painter.rect_filled(rect, 8.0 * z, p.step_box_bg);
            painter.rect_stroke(rect, 8.0 * z, egui::Stroke::new(1.0 * z, p.cell_border));

            let n = nodes.len();
            if n > 0 {
                let center = rect.center();
                let radius = (canvas_height / 2.0 - 45.0 * z).max(50.0);

                // Compute 2D circular positions for all nodes
                let node_positions: std::collections::BTreeMap<usize, egui::Pos2> = nodes
                    .iter()
                    .enumerate()
                    .map(|(idx, &u)| {
                        let angle = (idx as f32 / n as f32) * std::f32::consts::TAU
                            - std::f32::consts::FRAC_PI_2;
                        let x = center.x + radius * angle.cos();
                        let y = center.y + radius * angle.sin();
                        (u, egui::pos2(x, y))
                    })
                    .collect();

                // 1. Draw Directed Edges & Arrowheads
                for &(u, v) in edges {
                    if let (Some(&pos_u), Some(&pos_v)) =
                        (node_positions.get(&u), node_positions.get(&v))
                    {
                        let is_act = active_edge == Some((u, v));
                        let is_cycle = cycle_edges.contains(&(u, v));

                        let stroke_color = if is_cycle {
                            p.red
                        } else if is_act {
                            p.amber
                        } else {
                            p.cyan
                        };

                        let stroke_width = if is_cycle || is_act { 3.0 * z } else { 1.5 * z };

                        // Shorten line to stop at node circle boundary
                        let dir = (pos_v - pos_u).normalized();
                        let node_r = 22.0 * z;
                        let start = pos_u + dir * node_r;
                        let end = pos_v - dir * node_r;

                        // Draw edge line
                        painter.line_segment(
                            [start, end],
                            egui::Stroke::new(stroke_width, stroke_color),
                        );

                        // Draw arrowhead
                        let arrow_len = 10.0 * z;
                        let perp = egui::vec2(-dir.y, dir.x);
                        let p1 = end - dir * arrow_len + perp * (arrow_len * 0.5);
                        let p2 = end - dir * arrow_len - perp * (arrow_len * 0.5);
                        painter
                            .line_segment([end, p1], egui::Stroke::new(stroke_width, stroke_color));
                        painter
                            .line_segment([end, p2], egui::Stroke::new(stroke_width, stroke_color));
                    }
                }

                // 2. Draw Circular Nodes with Labels
                for (idx, &u) in nodes.iter().enumerate() {
                    if let Some(&pos) = node_positions.get(&u) {
                        let label = node_labels
                            .get(idx)
                            .cloned()
                            .unwrap_or_else(|| format!("{}", u));
                        let short_label = if label.starts_with("Course ") {
                            label.replace("Course ", "C")
                        } else if label.starts_with("Node ") {
                            label.replace("Node ", "N")
                        } else {
                            label.clone()
                        };

                        let is_act = active_node == Some(u);
                        let is_vis = visited_nodes.contains(&u);

                        let fill_color = if is_act {
                            p.amber
                        } else if is_vis {
                            p.purple
                        } else {
                            p.cell_bg
                        };

                        let border_color = if is_act {
                            p.red
                        } else if is_vis {
                            p.emerald_text
                        } else {
                            p.cyan
                        };
                        let node_r = 22.0 * z;

                        // Draw node circle
                        painter.circle_filled(pos, node_r, fill_color);
                        painter.circle_stroke(
                            pos,
                            node_r,
                            egui::Stroke::new(2.0 * z, border_color),
                        );

                        // Draw node text label centered
                        painter.text(
                            pos,
                            egui::Align2::CENTER_CENTER,
                            short_label,
                            egui::FontId::monospace((13.0 * z).max(9.0)),
                            Color32::WHITE,
                        );
                    }
                }
            }

            if !topo_order.is_empty() {
                ui.add_space(10.0 * z);
                ui.label(
                    RichText::new(format!("TOPOLOGICAL SORT ORDER: {:?}", topo_order))
                        .font(egui::FontId::monospace(12.0 * z))
                        .color(p.cyan)
                        .strong(),
                );
            }
        });
    }

    #[allow(clippy::too_many_arguments)]
    fn render_contains_duplicate(
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

    fn render_group_anagrams(
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
            ui.horizontal_wrapped(|ui| {
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
                    }
                }
            });
        });
    }

    #[allow(clippy::too_many_arguments)]
    fn render_longest_consecutive(
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

    #[allow(clippy::too_many_arguments, clippy::needless_range_loop)]
    fn render_sudoku(
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

    #[allow(clippy::too_many_arguments)]
    fn render_merge_lists(
        &self,
        ui: &mut egui::Ui,
        p: &ThemePalette,
        list1: &[i32],
        list2: &[i32],
        p1_idx: Option<usize>,
        p2_idx: Option<usize>,
        merged: &[i32],
    ) {
        let z = self.canvas_zoom;
        let font_title = (16.0 * z).max(10.0);
        let font_label = (11.0 * z).max(8.0);
        let font_node = (14.0 * z).max(9.0);
        let margin = (8.0 * z).max(4.0);

        ui.heading(
            RichText::new("Merge Two Sorted Linked Lists")
                .color(p.cyan)
                .size(font_title),
        );
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
                RichText::new("MERGED SORTED LIST (TAIL ATTACHMENTS)")
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
                                    RichText::new(format!("( {} ) ->", val))
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
    fn render_list_cycle(
        &self,
        ui: &mut egui::Ui,
        p: &ThemePalette,
        nodes: &[i32],
        cycle_target: Option<usize>,
        slow: Option<usize>,
        fast: Option<usize>,
        has_cycle: Option<bool>,
    ) {
        let z = self.canvas_zoom;
        let font_title = (16.0 * z).max(10.0);
        let font_label = (11.0 * z).max(8.0);
        let font_node = (16.0 * z).max(9.0);
        let margin = (10.0 * z).max(4.0);

        ui.heading(
            RichText::new("Floyd's Tortoise and Hare Cycle Detection")
                .color(p.cyan)
                .size(font_title),
        );
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

                    let fill = if is_slow && is_fast {
                        p.purple
                    } else if is_slow {
                        p.cyan
                    } else if is_fast {
                        p.pink
                    } else if is_cycle_target {
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
                            ui.vertical(|ui| {
                                let mut label = String::new();
                                if is_slow && is_fast {
                                    label.push_str("S & F");
                                } else if is_slow {
                                    label.push_str("slow");
                                } else if is_fast {
                                    label.push_str("fast");
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

        if let Some(cycle) = has_cycle {
            ui.add_space(20.0 * z);
            if cycle {
                ui.heading(
                    RichText::new("Cycle Detected! Slow & Fast Pointers Met.")
                        .color(p.emerald_text)
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
    fn render_tree(
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
                                    "Child"
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
                egui::Frame::none()
                    .fill(p.cell_bg)
                    .rounding(Rounding::same(8.0 * z))
                    .stroke(Stroke::new(1.0_f32 * z, p.cyan))
                    .inner_margin((12.0 * z).max(5.0))
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.label(
                                RichText::new("Current / Max Tree Depth")
                                    .font(egui::FontId::proportional((11.0 * z).max(8.0)))
                                    .color(p.text_muted),
                            );
                            ui.label(
                                RichText::new(format!("Depth: {}", d))
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
    fn render_stock(
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
    fn render_binary_search(
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
    fn render_linked_list(
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

        ui.heading(
            RichText::new("Singly-Linked List Pointer Reversal")
                .color(p.cyan)
                .size(font_title),
        );
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
                                if is_prev {
                                    label.push_str("prev ");
                                }
                                if is_curr {
                                    label.push_str("curr ");
                                }
                                if is_nxt {
                                    label.push_str("nxt ");
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

    #[allow(clippy::too_many_arguments)]
    fn render_two_sum(
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
    fn render_valid_anagram(
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
    fn render_two_pointers(
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

    fn render_stack(
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

        ui.heading(
            RichText::new("Vertical Stack Push / Pop Trace")
                .color(p.cyan)
                .size(font_title),
        );
        ui.add_space(8.0 * z);

        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.label(
                    RichText::new("EXPRESSION")
                        .font(egui::FontId::monospace((11.0 * z).max(8.0)))
                        .color(p.text_muted),
                );
                ui.horizontal(|ui| {
                    for (i, &c) in chars.iter().enumerate() {
                        let fill = if active_idx == Some(i) {
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

            ui.add_space(30.0 * z);

            ui.group(|ui| {
                ui.label(
                    RichText::new("STACK (Top on right/bottom)")
                        .font(egui::FontId::monospace((11.0 * z).max(8.0)))
                        .color(p.text_muted),
                );
                ui.vertical(|ui| {
                    if stack.is_empty() {
                        ui.label(
                            RichText::new("Stack is Empty []")
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
        });

        if let Some(valid) = is_valid {
            ui.add_space(20.0 * z);
            if valid {
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
    fn render_topk(
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
                self.topk_k_input
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
    fn render_encode_decode(
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

    #[allow(clippy::too_many_arguments)]
    fn render_product(
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

    fn render_trie(&self, ui: &mut egui::Ui, p: &ThemePalette) {
        let z = self.canvas_zoom;
        let font_title = (16.0 * z).max(10.0);
        let font_label = (11.0 * z).max(8.0);
        let font_root = (14.0 * z).max(9.0);
        let font_word_idx = (12.0 * z).max(8.0);
        let font_char = (13.0 * z).max(8.0);
        let margin = (10.0 * z).max(4.0);

        ui.heading(
            RichText::new("🌲 Trie (Prefix Tree) Character Node Hierarchy")
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
                // Render Root Node
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

                // Render Sample Word Nodes dynamically
                let words: Vec<&str> = match self.current_problem {
                    Problem::ImplementTrie => self
                        .trie_words_input
                        .split(',')
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                        .collect(),
                    Problem::WordDictionary => self
                        .word_dict_words_input
                        .split(',')
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                        .collect(),
                    Problem::WordSearchII => self
                        .word_search_ii_words_input
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
                                                        RichText::new(format!("'{}' ★", ch))
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
