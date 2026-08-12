use crate::app::VisualizerApp;
use crate::model::ThemePalette;
use eframe::egui::{self, Color32, RichText, Rounding, Stroke};

impl VisualizerApp {
    pub(super) fn render_decision_tree_visualizer(
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
    pub(super) fn render_grid_graph(
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
    pub(super) fn render_node_graph(
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
}
