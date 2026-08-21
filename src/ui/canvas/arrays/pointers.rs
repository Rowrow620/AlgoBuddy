use crate::app::VisualizerApp;
use crate::model::{Problem, ThemePalette};
use eframe::egui::{self, Color32, RichText};

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

        let heading = if self.current_problem == Problem::ValidPalindrome {
            self.current_problem
                .details()
                .approach_by_id(self.selected_approach_id)
                .map_or("Palindrome Check", |approach| approach.name)
        } else {
            "Two Pointers Convergence"
        };
        ui.heading(RichText::new(heading).color(p.cyan).size(font_title));
        ui.add_space(8.0 * z);

        let uses_pointer_scan =
            !(self.current_problem == Problem::ValidPalindrome && self.selected_approach_id == 1);
        ui.horizontal_wrapped(|ui| {
            for (i, &c) in chars.iter().enumerate() {
                let is_left = uses_pointer_scan && i == left;
                let is_right = uses_pointer_scan && i == right;

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

                let ptr_label = if is_left && is_right {
                    "L & R"
                } else if is_left {
                    "L ->"
                } else if is_right {
                    "<- R"
                } else {
                    ""
                };

                Self::render_canvas_cell(
                    ui,
                    &c.to_string(),
                    Some(ptr_label),
                    None,
                    fill,
                    p.cell_border,
                    Color32::WHITE,
                    z,
                );
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
