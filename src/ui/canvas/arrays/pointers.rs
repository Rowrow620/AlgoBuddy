use crate::app::VisualizerApp;
use crate::model::{Problem, ThemePalette};
use eframe::egui::{self, Color32, RichText, Rounding, Stroke};

impl VisualizerApp {
    #[allow(clippy::too_many_arguments)]
    pub(in crate::ui::canvas) fn render_two_pointers(
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
}
