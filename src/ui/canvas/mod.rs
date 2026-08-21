use crate::app::{
    canvas_zoom_in, canvas_zoom_out, VisualizerApp, CANVAS_ZOOM_DEFAULT, CANVAS_ZOOM_MAX,
    CANVAS_ZOOM_MIN,
};
use crate::model::{ThemePalette, VisualState};
use crate::shortcuts::ShortcutAction;
use eframe::egui::{self, Frame, RichText, Rounding, Stroke};

mod arrays;
mod collections;
mod graphs;
mod linked_lists;
mod problem_views;
mod trees;

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
                // Ctrl+wheel changes canvas zoom without scrolling the surrounding UI.
                if ui.rect_contains_pointer(ui.max_rect()) {
                    let scroll_delta = ctx.input(|i| i.raw_scroll_delta.y);
                    let ctrl_down = ctx.input(|i| i.modifiers.ctrl);
                    if ctrl_down && scroll_delta != 0.0 {
                        let factor = if scroll_delta > 0.0 { 1.08 } else { 0.92 };
                        self.canvas_zoom =
                            (self.canvas_zoom * factor).clamp(CANVAS_ZOOM_MIN, CANVAS_ZOOM_MAX);
                    }
                }

                // Problem-specific input controls update the current trace.
                self.render_custom_playground_bar(ui, p);
                ui.add_space(8.0);

                if let Some(step) = self.steps.get(self.current_step_idx) {
                    // Current step summary and canvas zoom controls.
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
                                        let reset_help = self.shortcut_bindings.hint(
                                            ShortcutAction::ResetZoom,
                                            "Reset visualization zoom to 100%",
                                        );
                                        let reset_response =
                                            ui.button("Reset").on_hover_text(&reset_help);
                                        reset_response.widget_info(|| {
                                            egui::WidgetInfo::labeled(
                                                egui::WidgetType::Button,
                                                ui.is_enabled(),
                                                &reset_help,
                                            )
                                        });
                                        if reset_response.clicked() {
                                            self.canvas_zoom = CANVAS_ZOOM_DEFAULT;
                                        }
                                        let zoom_in_help = self.shortcut_bindings.hint(
                                            ShortcutAction::ZoomIn,
                                            "Zoom in on the visualization",
                                        );
                                        let zoom_in_response =
                                            ui.button("+").on_hover_text(&zoom_in_help);
                                        zoom_in_response.widget_info(|| {
                                            egui::WidgetInfo::labeled(
                                                egui::WidgetType::Button,
                                                ui.is_enabled(),
                                                &zoom_in_help,
                                            )
                                        });
                                        if zoom_in_response.clicked() {
                                            self.canvas_zoom = canvas_zoom_in(self.canvas_zoom);
                                        }
                                        let zoom_out_help = self.shortcut_bindings.hint(
                                            ShortcutAction::ZoomOut,
                                            "Zoom out of the visualization",
                                        );
                                        let zoom_out_response =
                                            ui.button("−").on_hover_text(&zoom_out_help);
                                        zoom_out_response.widget_info(|| {
                                            egui::WidgetInfo::labeled(
                                                egui::WidgetType::Button,
                                                ui.is_enabled(),
                                                &zoom_out_help,
                                            )
                                        });
                                        if zoom_out_response.clicked() {
                                            self.canvas_zoom = canvas_zoom_out(self.canvas_zoom);
                                        }
                                        ui.label(
                                            RichText::new(format!("Zoom: {}%", zoom_pct))
                                                .font(egui::FontId::monospace(11.0))
                                                .color(p.cyan),
                                        );

                                        ui.with_layout(
                                            egui::Layout::left_to_right(egui::Align::Center),
                                            |ui| {
                                                ui.style_mut().wrap_mode =
                                                    Some(egui::TextWrapMode::Truncate);
                                                ui.label(
                                                    RichText::new("Live State Inspector")
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
                        VisualState::TraceUnavailable { message } => {
                            egui::Frame::group(ui.style())
                                .fill(p.step_box_bg)
                                .rounding(Rounding::same(8.0))
                                .stroke(Stroke::new(1.0_f32, p.red))
                                .inner_margin(12.0)
                                .show(ui, |ui| {
                                    ui.heading(
                                        RichText::new("Trace unavailable for this input")
                                            .color(p.red),
                                    );
                                    ui.add_space(6.0);
                                    ui.label(RichText::new(message).color(p.text_primary));
                                });
                        }
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
                            strings_are_sorted,
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
                                *strings_are_sorted,
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
                            phase,
                        } => {
                            self.render_merge_lists(
                                ui,
                                p,
                                list1,
                                list2,
                                *p1_idx,
                                *p2_idx,
                                merged_so_far,
                                *phase,
                            );
                        }
                        VisualState::LinkedListCycle {
                            nodes,
                            cycle_target_idx,
                            slow_idx,
                            fast_idx,
                            visited_indices,
                            has_cycle,
                        } => {
                            self.render_list_cycle(
                                ui,
                                p,
                                nodes,
                                *cycle_target_idx,
                                *slow_idx,
                                *fast_idx,
                                visited_indices,
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
                        VisualState::TreeMaxPathVisual {
                            tree_nodes,
                            active_node_idx,
                            secondary_node_idx,
                            left_gain,
                            right_gain,
                            through_node_sum,
                            returned_gain,
                            max_path_sum,
                        } => {
                            self.render_tree_max_path(
                                ui,
                                p,
                                tree_nodes,
                                *active_node_idx,
                                *secondary_node_idx,
                                *left_gain,
                                *right_gain,
                                *through_node_sum,
                                *returned_gain,
                                *max_path_sum,
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

                ui.add_space(10.0);
                crate::ui::ai_chat_dock::render_ai_chat_dock(self, ui, p);
            });
    }

    /// Renders a standardized canvas array cell box with optional top/bottom labels.
    #[allow(clippy::too_many_arguments)]
    pub(in crate::ui::canvas) fn render_canvas_cell(
        ui: &mut egui::Ui,
        value: &str,
        top_label: Option<&str>,
        sub_label: Option<&str>,
        bg_color: egui::Color32,
        border_color: egui::Color32,
        text_color: egui::Color32,
        zoom: f32,
    ) {
        let z = zoom;
        let font_sub = (10.0 * z).max(8.0);
        let font_val = (16.0 * z).max(9.0);
        let margin = egui::Margin::symmetric((10.0 * z).max(4.0), (8.0 * z).max(4.0));

        ui.vertical(|ui| {
            if let Some(top) = top_label {
                let top_str = if top.is_empty() { " " } else { top };
                ui.label(
                    RichText::new(top_str)
                        .font(egui::FontId::monospace(font_sub))
                        .strong()
                        .color(egui::Color32::WHITE),
                );
            }

            egui::Frame::none()
                .fill(bg_color)
                .rounding(Rounding::same(6.0 * z))
                .stroke(Stroke::new(1.0_f32 * z, border_color))
                .inner_margin(margin)
                .show(ui, |ui| {
                    ui.label(
                        RichText::new(value)
                            .font(egui::FontId::monospace(font_val))
                            .strong()
                            .color(text_color),
                    );
                });

            if let Some(sub) = sub_label {
                ui.label(
                    RichText::new(sub)
                        .font(egui::FontId::monospace(font_sub))
                        .color(egui::Color32::from_gray(160)),
                );
            }
        });
    }
}
