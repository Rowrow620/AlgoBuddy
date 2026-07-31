use crate::app::VisualizerApp;
use crate::model::{Problem, ThemePalette};
use eframe::egui::{self, RichText, Rounding, Stroke};

impl VisualizerApp {
    pub(crate) fn render_custom_playground_bar(&mut self, ui: &mut egui::Ui, p: &ThemePalette) {
        egui::Frame::none()
            .fill(p.sidebar_bg)
            .rounding(Rounding::same(8.0))
            .stroke(Stroke::new(1.0_f32, p.amber))
            .inner_margin(egui::Margin::symmetric(14.0, 8.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("🎮 Custom Input Playground:")
                            .font(egui::FontId::proportional(12.0))
                            .color(p.amber)
                            .strong(),
                    );
                    ui.add_space(4.0);

                    let mut should_run = false;

                    match self.current_problem {
                        Problem::ContainsDuplicate => {
                            ui.label(
                                RichText::new("nums =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            if ui
                                .add(
                                    egui::TextEdit::singleline(&mut self.contains_dup_nums_input)
                                        .desired_width(160.0),
                                )
                                .changed()
                            {
                                should_run = true;
                            }
                            if ui.button("[1,2,3,1]").clicked() {
                                self.contains_dup_nums_input = "1,2,3,1".into();
                                should_run = true;
                            }
                            if ui.button("[1,2,3,4]").clicked() {
                                self.contains_dup_nums_input = "1,2,3,4".into();
                                should_run = true;
                            }
                        }
                        Problem::TwoSum => {
                            ui.label(
                                RichText::new("nums =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            if ui
                                .add(
                                    egui::TextEdit::singleline(&mut self.two_sum_nums_input)
                                        .desired_width(130.0),
                                )
                                .changed()
                            {
                                should_run = true;
                            }
                            ui.label(
                                RichText::new("target =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            if ui
                                .add(egui::DragValue::new(&mut self.two_sum_target_input))
                                .changed()
                            {
                                should_run = true;
                            }
                            if ui.button("[2,7,11,15] t=9").clicked() {
                                self.two_sum_nums_input = "2,7,11,15".into();
                                self.two_sum_target_input = 9;
                                should_run = true;
                            }
                        }
                        Problem::ValidAnagram => {
                            ui.label(
                                RichText::new("s =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            if ui
                                .add(
                                    egui::TextEdit::singleline(&mut self.valid_anagram_s_input)
                                        .desired_width(90.0),
                                )
                                .changed()
                            {
                                should_run = true;
                            }
                            ui.label(
                                RichText::new("t =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            if ui
                                .add(
                                    egui::TextEdit::singleline(&mut self.valid_anagram_t_input)
                                        .desired_width(90.0),
                                )
                                .changed()
                            {
                                should_run = true;
                            }
                            if ui.button("anagram / nagaram").clicked() {
                                self.valid_anagram_s_input = "anagram".into();
                                self.valid_anagram_t_input = "nagaram".into();
                                should_run = true;
                            }
                            if ui.button("rat / car").clicked() {
                                self.valid_anagram_s_input = "rat".into();
                                self.valid_anagram_t_input = "car".into();
                                should_run = true;
                            }
                        }
                        Problem::GroupAnagrams => {
                            ui.label(
                                RichText::new("strs =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            if ui
                                .add(
                                    egui::TextEdit::singleline(&mut self.group_anagrams_input)
                                        .desired_width(200.0),
                                )
                                .changed()
                            {
                                should_run = true;
                            }
                            if ui.button("eat, tea, tan, ate, nat, bat").clicked() {
                                self.group_anagrams_input = "eat, tea, tan, ate, nat, bat".into();
                                should_run = true;
                            }
                        }
                        Problem::TopKFrequent => {
                            ui.label(
                                RichText::new("nums =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            if ui
                                .add(
                                    egui::TextEdit::singleline(&mut self.topk_nums_input)
                                        .desired_width(120.0),
                                )
                                .changed()
                            {
                                should_run = true;
                            }
                            ui.label(
                                RichText::new("k =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            if ui
                                .add(egui::DragValue::new(&mut self.topk_k_input).range(1..=10))
                                .changed()
                            {
                                should_run = true;
                            }
                            if ui.button("[1,1,1,2,2,3] k=2").clicked() {
                                self.topk_nums_input = "1,1,1,2,2,3".into();
                                self.topk_k_input = 2;
                                should_run = true;
                            }
                        }
                        Problem::ProductExceptSelf => {
                            ui.label(
                                RichText::new("nums =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            if ui
                                .add(
                                    egui::TextEdit::singleline(&mut self.prod_nums_input)
                                        .desired_width(160.0),
                                )
                                .changed()
                            {
                                should_run = true;
                            }
                            if ui.button("[1,2,4,6]").clicked() {
                                self.prod_nums_input = "1,2,4,6".into();
                                should_run = true;
                            }
                        }
                        Problem::EncodeDecode => {
                            ui.label(
                                RichText::new("strs =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            if ui
                                .add(
                                    egui::TextEdit::singleline(&mut self.ed_strs_input)
                                        .desired_width(160.0),
                                )
                                .changed()
                            {
                                should_run = true;
                            }
                            if ui.button("Hello, World").clicked() {
                                self.ed_strs_input = "Hello, World".into();
                                should_run = true;
                            }
                        }
                        Problem::ValidSudoku => {
                            ui.label(
                                RichText::new("Board Preset:")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            if ui
                                .selectable_label(self.sudoku_preset_valid, "Valid Board Ex 1")
                                .clicked()
                            {
                                self.sudoku_preset_valid = true;
                                should_run = true;
                            }
                            if ui
                                .selectable_label(!self.sudoku_preset_valid, "Invalid Board Ex 2")
                                .clicked()
                            {
                                self.sudoku_preset_valid = false;
                                should_run = true;
                            }
                        }
                        Problem::LongestConsecutive => {
                            ui.label(
                                RichText::new("nums =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            if ui
                                .add(
                                    egui::TextEdit::singleline(
                                        &mut self.longest_consecutive_nums_input,
                                    )
                                    .desired_width(180.0),
                                )
                                .changed()
                            {
                                should_run = true;
                            }
                            if ui.button("[2,20,4,10,3,4,5]").clicked() {
                                self.longest_consecutive_nums_input = "2,20,4,10,3,4,5".into();
                                should_run = true;
                            }
                        }
                        Problem::ValidPalindrome => {
                            ui.label(
                                RichText::new("s =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            if ui
                                .add(
                                    egui::TextEdit::singleline(&mut self.palindrome_s_input)
                                        .desired_width(200.0),
                                )
                                .changed()
                            {
                                should_run = true;
                            }
                            if ui.button("Was it a car...").clicked() {
                                self.palindrome_s_input = "Was it a car or a cat I saw?".into();
                                should_run = true;
                            }
                        }
                        Problem::BestTimeStock => {
                            ui.label(
                                RichText::new("prices =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            if ui
                                .add(
                                    egui::TextEdit::singleline(&mut self.stock_prices_input)
                                        .desired_width(160.0),
                                )
                                .changed()
                            {
                                should_run = true;
                            }
                            if ui.button("[10,1,5,6,7,1]").clicked() {
                                self.stock_prices_input = "10,1,5,6,7,1".into();
                                should_run = true;
                            }
                        }
                        Problem::ValidParentheses => {
                            ui.label(
                                RichText::new("s =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            if ui
                                .add(
                                    egui::TextEdit::singleline(&mut self.parentheses_s_input)
                                        .desired_width(140.0),
                                )
                                .changed()
                            {
                                should_run = true;
                            }
                            if ui.button("([{}])").clicked() {
                                self.parentheses_s_input = "([{}])".into();
                                should_run = true;
                            }
                        }
                        Problem::BinarySearch => {
                            ui.label(
                                RichText::new("nums =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            if ui
                                .add(
                                    egui::TextEdit::singleline(&mut self.binary_search_nums_input)
                                        .desired_width(140.0),
                                )
                                .changed()
                            {
                                should_run = true;
                            }
                            ui.label(
                                RichText::new("target =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            if ui
                                .add(egui::DragValue::new(&mut self.binary_search_target_input))
                                .changed()
                            {
                                should_run = true;
                            }
                            if ui.button("[-1,0,2,4,6,8] t=4").clicked() {
                                self.binary_search_nums_input = "-1,0,2,4,6,8".into();
                                self.binary_search_target_input = 4;
                                should_run = true;
                            }
                        }
                        Problem::ReverseLinkedList => {
                            ui.label(
                                RichText::new("nodes =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            if ui
                                .add(
                                    egui::TextEdit::singleline(&mut self.linked_list_nodes_input)
                                        .desired_width(140.0),
                                )
                                .changed()
                            {
                                should_run = true;
                            }
                            if ui.button("[0,1,2,3]").clicked() {
                                self.linked_list_nodes_input = "0,1,2,3".into();
                                should_run = true;
                            }
                        }
                        Problem::MergeTwoLists => {
                            ui.label(
                                RichText::new("list1 =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            if ui
                                .add(
                                    egui::TextEdit::singleline(&mut self.merge_list1_input)
                                        .desired_width(100.0),
                                )
                                .changed()
                            {
                                should_run = true;
                            }
                            ui.label(
                                RichText::new("list2 =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            if ui
                                .add(
                                    egui::TextEdit::singleline(&mut self.merge_list2_input)
                                        .desired_width(100.0),
                                )
                                .changed()
                            {
                                should_run = true;
                            }
                            if ui.button("[1,2,4] & [1,3,5]").clicked() {
                                self.merge_list1_input = "1,2,4".into();
                                self.merge_list2_input = "1,3,5".into();
                                should_run = true;
                            }
                        }
                        Problem::LinkedListCycle => {
                            ui.label(
                                RichText::new("nodes =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            if ui
                                .add(
                                    egui::TextEdit::singleline(&mut self.cycle_nodes_input)
                                        .desired_width(120.0),
                                )
                                .changed()
                            {
                                should_run = true;
                            }
                            ui.label(
                                RichText::new("cycle_idx =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            if ui
                                .add(
                                    egui::DragValue::new(&mut self.cycle_index_input)
                                        .range(-1..=10),
                                )
                                .changed()
                            {
                                should_run = true;
                            }
                            if ui.button("[1,2,3,4] idx=1").clicked() {
                                self.cycle_nodes_input = "1,2,3,4".into();
                                self.cycle_index_input = 1;
                                should_run = true;
                            }
                        }
                        Problem::InvertTree
                        | Problem::MaxDepthTree
                        | Problem::DiameterTree
                        | Problem::BalancedTree
                        | Problem::SameTree
                        | Problem::Subtree => {
                            ui.label(
                                RichText::new("tree nodes =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            if ui
                                .add(
                                    egui::TextEdit::singleline(&mut self.tree_nodes_input)
                                        .desired_width(180.0),
                                )
                                .changed()
                            {
                                should_run = true;
                            }
                            if ui.button("[1,2,3,4,5,6,7]").clicked() {
                                self.tree_nodes_input = "1,2,3,4,5,6,7".into();
                                should_run = true;
                            }
                        }
                        Problem::TwoSumII => {
                            ui.label(
                                RichText::new("nums =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            if ui
                                .add(
                                    egui::TextEdit::singleline(&mut self.two_pointer_nums_input)
                                        .desired_width(140.0),
                                )
                                .changed()
                            {
                                should_run = true;
                            }
                            ui.label(
                                RichText::new("target =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            if ui
                                .add(egui::DragValue::new(&mut self.two_pointer_target_input))
                                .changed()
                            {
                                should_run = true;
                            }
                            if ui.button("[2,7,11,15] t=9").clicked() {
                                self.two_pointer_nums_input = "2,7,11,15".into();
                                self.two_pointer_target_input = 9;
                                should_run = true;
                            }
                        }
                        Problem::ThreeSum
                        | Problem::ContainerWater
                        | Problem::TrappingRain
                        | Problem::HouseRobber => {
                            ui.label(
                                RichText::new("nums =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            if ui
                                .add(
                                    egui::TextEdit::singleline(&mut self.two_pointer_nums_input)
                                        .desired_width(180.0),
                                )
                                .changed()
                            {
                                should_run = true;
                            }
                            if ui.button("Default Preset").clicked() {
                                self.two_pointer_nums_input = "-1,0,1,2,-1,-4".into();
                                should_run = true;
                            }
                        }
                        Problem::SearchRotatedArray | Problem::FindMinRotated => {
                            ui.label(
                                RichText::new("nums =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            if ui
                                .add(
                                    egui::TextEdit::singleline(&mut self.binary_search_nums_input)
                                        .desired_width(140.0),
                                )
                                .changed()
                            {
                                should_run = true;
                            }
                            ui.label(
                                RichText::new("target =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            if ui
                                .add(egui::DragValue::new(&mut self.binary_search_target_input))
                                .changed()
                            {
                                should_run = true;
                            }
                            if ui.button("[4,5,6,7,0,1,2] t=0").clicked() {
                                self.binary_search_nums_input = "4,5,6,7,0,1,2".into();
                                self.binary_search_target_input = 0;
                                should_run = true;
                            }
                        }
                        Problem::ImplementTrie => {
                            ui.label(
                                RichText::new("insert =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            if ui
                                .add(
                                    egui::TextEdit::singleline(&mut self.trie_words_input)
                                        .desired_width(140.0),
                                )
                                .changed()
                            {
                                should_run = true;
                            }
                            ui.label(
                                RichText::new("search =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            if ui
                                .add(
                                    egui::TextEdit::singleline(&mut self.trie_search_input)
                                        .desired_width(80.0),
                                )
                                .changed()
                            {
                                should_run = true;
                            }
                            if ui.button("apple, app").clicked() {
                                self.trie_words_input = "apple, app".into();
                                self.trie_search_input = "app".into();
                                should_run = true;
                            }
                        }
                        Problem::WordDictionary => {
                            ui.label(
                                RichText::new("words =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            if ui
                                .add(
                                    egui::TextEdit::singleline(&mut self.word_dict_words_input)
                                        .desired_width(140.0),
                                )
                                .changed()
                            {
                                should_run = true;
                            }
                            ui.label(
                                RichText::new("pattern =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            if ui
                                .add(
                                    egui::TextEdit::singleline(&mut self.word_dict_pattern_input)
                                        .desired_width(80.0),
                                )
                                .changed()
                            {
                                should_run = true;
                            }
                        }
                        Problem::WordSearchII => {
                            ui.label(
                                RichText::new("words =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            if ui
                                .add(
                                    egui::TextEdit::singleline(
                                        &mut self.word_search_ii_words_input,
                                    )
                                    .desired_width(200.0),
                                )
                                .changed()
                            {
                                should_run = true;
                            }
                        }
                        _ => {
                            ui.label(
                                RichText::new("Default test dataset active.")
                                    .font(egui::FontId::proportional(12.0))
                                    .color(p.text_muted),
                            );
                        }
                    }

                    if should_run {
                        self.recompute_steps();
                        self.current_step_idx = 0;
                        self.is_playing = false;
                    }
                });
            });
    }
}
