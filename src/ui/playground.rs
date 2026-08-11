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
                        RichText::new("Custom Input Playground:")
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
                            let input = self.get_input_str_mut(
                                Problem::ContainsDuplicate,
                                "nums",
                                "1, 2, 3, 1",
                            );
                            if ui
                                .add(egui::TextEdit::singleline(input).desired_width(160.0))
                                .changed()
                            {
                                should_run = true;
                            }
                            if ui.button("[1,2,3,1]").clicked() {
                                self.set_input_str(Problem::ContainsDuplicate, "nums", "1,2,3,1");
                                should_run = true;
                            }
                            if ui.button("[1,2,3,4]").clicked() {
                                self.set_input_str(Problem::ContainsDuplicate, "nums", "1,2,3,4");
                                should_run = true;
                            }
                        }
                        Problem::TwoSum => {
                            ui.label(
                                RichText::new("nums =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            let nums_input =
                                self.get_input_str_mut(Problem::TwoSum, "nums", "2, 7, 11, 15");
                            if ui
                                .add(egui::TextEdit::singleline(nums_input).desired_width(130.0))
                                .changed()
                            {
                                should_run = true;
                            }
                            ui.label(
                                RichText::new("target =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            let target_input = self.get_input_int_mut(Problem::TwoSum, "target", 9);
                            if ui.add(egui::DragValue::new(target_input)).changed() {
                                should_run = true;
                            }
                            if ui.button("[2,7,11,15] t=9").clicked() {
                                self.set_input_str(Problem::TwoSum, "nums", "2,7,11,15");
                                self.set_input_int(Problem::TwoSum, "target", 9);
                                should_run = true;
                            }
                        }
                        Problem::ValidAnagram => {
                            ui.label(
                                RichText::new("s =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            let s_input =
                                self.get_input_str_mut(Problem::ValidAnagram, "s", "anagram");
                            if ui
                                .add(egui::TextEdit::singleline(s_input).desired_width(90.0))
                                .changed()
                            {
                                should_run = true;
                            }
                            ui.label(
                                RichText::new("t =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            let t_input =
                                self.get_input_str_mut(Problem::ValidAnagram, "t", "nagaram");
                            if ui
                                .add(egui::TextEdit::singleline(t_input).desired_width(90.0))
                                .changed()
                            {
                                should_run = true;
                            }
                            if ui.button("anagram / nagaram").clicked() {
                                self.set_input_str(Problem::ValidAnagram, "s", "anagram");
                                self.set_input_str(Problem::ValidAnagram, "t", "nagaram");
                                should_run = true;
                            }
                            if ui.button("rat / car").clicked() {
                                self.set_input_str(Problem::ValidAnagram, "s", "rat");
                                self.set_input_str(Problem::ValidAnagram, "t", "car");
                                should_run = true;
                            }
                        }
                        Problem::GroupAnagrams => {
                            ui.label(
                                RichText::new("strs =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            let strs_input = self.get_input_str_mut(
                                Problem::GroupAnagrams,
                                "strs",
                                "eat, tea, tan, ate, nat, bat",
                            );
                            if ui
                                .add(egui::TextEdit::singleline(strs_input).desired_width(200.0))
                                .changed()
                            {
                                should_run = true;
                            }
                            if ui.button("eat, tea, tan, ate, nat, bat").clicked() {
                                self.set_input_str(
                                    Problem::GroupAnagrams,
                                    "strs",
                                    "eat, tea, tan, ate, nat, bat",
                                );
                                should_run = true;
                            }
                        }
                        Problem::TopKFrequent => {
                            ui.label(
                                RichText::new("nums =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            let nums_input = self.get_input_str_mut(
                                Problem::TopKFrequent,
                                "nums",
                                "1, 1, 1, 2, 2, 3",
                            );
                            if ui
                                .add(egui::TextEdit::singleline(nums_input).desired_width(120.0))
                                .changed()
                            {
                                should_run = true;
                            }
                            ui.label(
                                RichText::new("k =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            let k_input = self.get_input_int_mut(Problem::TopKFrequent, "k", 2);
                            if ui
                                .add(egui::DragValue::new(k_input).range(1..=10))
                                .changed()
                            {
                                should_run = true;
                            }
                            if ui.button("[1,1,1,2,2,3] k=2").clicked() {
                                self.set_input_str(Problem::TopKFrequent, "nums", "1,1,1,2,2,3");
                                self.set_input_int(Problem::TopKFrequent, "k", 2);
                                should_run = true;
                            }
                        }
                        Problem::ProductExceptSelf => {
                            ui.label(
                                RichText::new("nums =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            let nums_input = self.get_input_str_mut(
                                Problem::ProductExceptSelf,
                                "nums",
                                "1, 2, 4, 6",
                            );
                            if ui
                                .add(egui::TextEdit::singleline(nums_input).desired_width(160.0))
                                .changed()
                            {
                                should_run = true;
                            }
                            if ui.button("[1,2,4,6]").clicked() {
                                self.set_input_str(Problem::ProductExceptSelf, "nums", "1,2,4,6");
                                should_run = true;
                            }
                        }
                        Problem::EncodeDecode => {
                            ui.label(
                                RichText::new("strs =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            let strs_input = self.get_input_str_mut(
                                Problem::EncodeDecode,
                                "strs",
                                "Hello, World",
                            );
                            if ui
                                .add(egui::TextEdit::singleline(strs_input).desired_width(160.0))
                                .changed()
                            {
                                should_run = true;
                            }
                            if ui.button("Hello, World").clicked() {
                                self.set_input_str(Problem::EncodeDecode, "strs", "Hello, World");
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
                            let nums_input = self.get_input_str_mut(
                                Problem::LongestConsecutive,
                                "nums",
                                "2, 20, 4, 10, 3, 4, 5",
                            );
                            if ui
                                .add(egui::TextEdit::singleline(nums_input).desired_width(180.0))
                                .changed()
                            {
                                should_run = true;
                            }
                            if ui.button("[2,20,4,10,3,4,5]").clicked() {
                                self.set_input_str(
                                    Problem::LongestConsecutive,
                                    "nums",
                                    "2,20,4,10,3,4,5",
                                );
                                should_run = true;
                            }
                        }
                        Problem::ValidPalindrome => {
                            ui.label(
                                RichText::new("s =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            let s_input = self.get_input_str_mut(
                                Problem::ValidPalindrome,
                                "s",
                                "Was it a car or a cat I saw?",
                            );
                            if ui
                                .add(egui::TextEdit::singleline(s_input).desired_width(200.0))
                                .changed()
                            {
                                should_run = true;
                            }
                            if ui.button("Was it a car...").clicked() {
                                self.set_input_str(
                                    Problem::ValidPalindrome,
                                    "s",
                                    "Was it a car or a cat I saw?",
                                );
                                should_run = true;
                            }
                        }
                        Problem::BestTimeStock => {
                            ui.label(
                                RichText::new("prices =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            let prices_input = self.get_input_str_mut(
                                Problem::BestTimeStock,
                                "prices",
                                "10, 1, 5, 6, 7, 1",
                            );
                            if ui
                                .add(egui::TextEdit::singleline(prices_input).desired_width(160.0))
                                .changed()
                            {
                                should_run = true;
                            }
                            if ui.button("[10,1,5,6,7,1]").clicked() {
                                self.set_input_str(
                                    Problem::BestTimeStock,
                                    "prices",
                                    "10,1,5,6,7,1",
                                );
                                should_run = true;
                            }
                        }
                        Problem::ValidParentheses => {
                            ui.label(
                                RichText::new("s =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            let s_input =
                                self.get_input_str_mut(Problem::ValidParentheses, "s", "([{}])");
                            if ui
                                .add(egui::TextEdit::singleline(s_input).desired_width(140.0))
                                .changed()
                            {
                                should_run = true;
                            }
                            if ui.button("([{}])").clicked() {
                                self.set_input_str(Problem::ValidParentheses, "s", "([{}])");
                                should_run = true;
                            }
                        }
                        Problem::BinarySearch => {
                            ui.label(
                                RichText::new("nums =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            let nums_input = self.get_input_str_mut(
                                Problem::BinarySearch,
                                "nums",
                                "-1, 0, 2, 4, 6, 8",
                            );
                            if ui
                                .add(egui::TextEdit::singleline(nums_input).desired_width(140.0))
                                .changed()
                            {
                                should_run = true;
                            }
                            ui.label(
                                RichText::new("target =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            let target_input =
                                self.get_input_int_mut(Problem::BinarySearch, "target", 4);
                            if ui.add(egui::DragValue::new(target_input)).changed() {
                                should_run = true;
                            }
                            if ui.button("[-1,0,2,4,6,8] t=4").clicked() {
                                self.set_input_str(Problem::BinarySearch, "nums", "-1,0,2,4,6,8");
                                self.set_input_int(Problem::BinarySearch, "target", 4);
                                should_run = true;
                            }
                        }
                        Problem::ReverseLinkedList => {
                            ui.label(
                                RichText::new("nodes =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            let nodes_input = self.get_input_str_mut(
                                Problem::ReverseLinkedList,
                                "nodes",
                                "0, 1, 2, 3",
                            );
                            if ui
                                .add(egui::TextEdit::singleline(nodes_input).desired_width(140.0))
                                .changed()
                            {
                                should_run = true;
                            }
                            if ui.button("[0,1,2,3]").clicked() {
                                self.set_input_str(Problem::ReverseLinkedList, "nodes", "0,1,2,3");
                                should_run = true;
                            }
                        }
                        Problem::MergeTwoLists => {
                            ui.label(
                                RichText::new("list1 =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            let l1_input =
                                self.get_input_str_mut(Problem::MergeTwoLists, "list1", "1, 2, 4");
                            if ui
                                .add(egui::TextEdit::singleline(l1_input).desired_width(100.0))
                                .changed()
                            {
                                should_run = true;
                            }
                            ui.label(
                                RichText::new("list2 =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            let l2_input =
                                self.get_input_str_mut(Problem::MergeTwoLists, "list2", "1, 3, 5");
                            if ui
                                .add(egui::TextEdit::singleline(l2_input).desired_width(100.0))
                                .changed()
                            {
                                should_run = true;
                            }
                            if ui.button("[1,2,4] & [1,3,5]").clicked() {
                                self.set_input_str(Problem::MergeTwoLists, "list1", "1,2,4");
                                self.set_input_str(Problem::MergeTwoLists, "list2", "1,3,5");
                                should_run = true;
                            }
                        }
                        Problem::LinkedListCycle => {
                            ui.label(
                                RichText::new("nodes =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            let nodes_input = self.get_input_str_mut(
                                Problem::LinkedListCycle,
                                "nodes",
                                "1, 2, 3, 4",
                            );
                            if ui
                                .add(egui::TextEdit::singleline(nodes_input).desired_width(120.0))
                                .changed()
                            {
                                should_run = true;
                            }
                            ui.label(
                                RichText::new("cycle_idx =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            let cycle_idx_input =
                                self.get_input_int_mut(Problem::LinkedListCycle, "cycle_idx", 1);
                            if ui
                                .add(egui::DragValue::new(cycle_idx_input).range(-1..=10))
                                .changed()
                            {
                                should_run = true;
                            }
                            if ui.button("[1,2,3,4] idx=1").clicked() {
                                self.set_input_str(Problem::LinkedListCycle, "nodes", "1,2,3,4");
                                self.set_input_int(Problem::LinkedListCycle, "cycle_idx", 1);
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
                            let tree_input = self.get_input_str_mut(
                                self.current_problem,
                                "tree_nodes",
                                "1, 2, 3, 4, 5, 6, 7",
                            );
                            if ui
                                .add(egui::TextEdit::singleline(tree_input).desired_width(180.0))
                                .changed()
                            {
                                should_run = true;
                            }
                            if ui.button("[1,2,3,4,5,6,7]").clicked() {
                                self.set_input_str(
                                    self.current_problem,
                                    "tree_nodes",
                                    "1,2,3,4,5,6,7",
                                );
                                should_run = true;
                            }
                        }
                        Problem::TwoSumII => {
                            ui.label(
                                RichText::new("nums =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            let nums_input =
                                self.get_input_str_mut(Problem::TwoSumII, "nums", "2, 7, 11, 15");
                            if ui
                                .add(egui::TextEdit::singleline(nums_input).desired_width(140.0))
                                .changed()
                            {
                                should_run = true;
                            }
                            ui.label(
                                RichText::new("target =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            let target_input =
                                self.get_input_int_mut(Problem::TwoSumII, "target", 9);
                            if ui.add(egui::DragValue::new(target_input)).changed() {
                                should_run = true;
                            }
                            if ui.button("[2,7,11,15] t=9").clicked() {
                                self.set_input_str(Problem::TwoSumII, "nums", "2,7,11,15");
                                self.set_input_int(Problem::TwoSumII, "target", 9);
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
                            let nums_input = self.get_input_str_mut(
                                self.current_problem,
                                "nums",
                                "-1, 0, 1, 2, -1, -4",
                            );
                            if ui
                                .add(egui::TextEdit::singleline(nums_input).desired_width(180.0))
                                .changed()
                            {
                                should_run = true;
                            }
                            if ui.button("Default Preset").clicked() {
                                self.set_input_str(self.current_problem, "nums", "-1,0,1,2,-1,-4");
                                should_run = true;
                            }
                        }
                        Problem::SearchRotatedArray | Problem::FindMinRotated => {
                            ui.label(
                                RichText::new("nums =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            let nums_input = self.get_input_str_mut(
                                self.current_problem,
                                "nums",
                                "4, 5, 6, 7, 0, 1, 2",
                            );
                            if ui
                                .add(egui::TextEdit::singleline(nums_input).desired_width(140.0))
                                .changed()
                            {
                                should_run = true;
                            }
                            ui.label(
                                RichText::new("target =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            let target_input =
                                self.get_input_int_mut(self.current_problem, "target", 0);
                            if ui.add(egui::DragValue::new(target_input)).changed() {
                                should_run = true;
                            }
                            if ui.button("[4,5,6,7,0,1,2] t=0").clicked() {
                                self.set_input_str(self.current_problem, "nums", "4,5,6,7,0,1,2");
                                self.set_input_int(self.current_problem, "target", 0);
                                should_run = true;
                            }
                        }
                        Problem::ImplementTrie => {
                            ui.label(
                                RichText::new("insert =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            let words_input = self.get_input_str_mut(
                                Problem::ImplementTrie,
                                "words",
                                "apple, app, ape",
                            );
                            if ui
                                .add(egui::TextEdit::singleline(words_input).desired_width(140.0))
                                .changed()
                            {
                                should_run = true;
                            }
                            ui.label(
                                RichText::new("search =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            let search_input =
                                self.get_input_str_mut(Problem::ImplementTrie, "search", "app");
                            if ui
                                .add(egui::TextEdit::singleline(search_input).desired_width(80.0))
                                .changed()
                            {
                                should_run = true;
                            }
                            if ui.button("apple, app").clicked() {
                                self.set_input_str(Problem::ImplementTrie, "words", "apple, app");
                                self.set_input_str(Problem::ImplementTrie, "search", "app");
                                should_run = true;
                            }
                        }
                        Problem::WordDictionary => {
                            ui.label(
                                RichText::new("words =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            let words_input = self.get_input_str_mut(
                                Problem::WordDictionary,
                                "words",
                                "bad, dad, mad",
                            );
                            if ui
                                .add(egui::TextEdit::singleline(words_input).desired_width(140.0))
                                .changed()
                            {
                                should_run = true;
                            }
                            ui.label(
                                RichText::new("pattern =")
                                    .font(egui::FontId::monospace(12.0))
                                    .color(p.text_muted),
                            );
                            let pattern_input =
                                self.get_input_str_mut(Problem::WordDictionary, "pattern", ".ad");
                            if ui
                                .add(egui::TextEdit::singleline(pattern_input).desired_width(80.0))
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
                            let words_input = self.get_input_str_mut(
                                Problem::WordSearchII,
                                "words",
                                "oath, pea, eat, rain",
                            );
                            if ui
                                .add(egui::TextEdit::singleline(words_input).desired_width(200.0))
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
