use crate::app::VisualizerApp;
use crate::model::{Problem, ThemePalette};
use eframe::egui::{self, RichText, Rounding, Stroke};

impl VisualizerApp {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn playground_text_input(
        &mut self,
        ui: &mut egui::Ui,
        problem: Problem,
        key: &'static str,
        label: &str,
        default: &str,
        width: f32,
        p: &ThemePalette,
    ) -> bool {
        ui.label(
            RichText::new(label)
                .font(egui::FontId::monospace(12.0))
                .color(p.text_muted),
        );
        let input = self.get_input_str_mut(problem, key, default);
        ui.add(egui::TextEdit::singleline(input).desired_width(width))
            .changed()
    }

    pub(crate) fn playground_int_input(
        &mut self,
        ui: &mut egui::Ui,
        problem: Problem,
        key: &'static str,
        label: &str,
        default: i32,
        p: &ThemePalette,
    ) -> bool {
        ui.label(
            RichText::new(label)
                .font(egui::FontId::monospace(12.0))
                .color(p.text_muted),
        );
        let input = self.get_input_int_mut(problem, key, default);
        ui.add(egui::DragValue::new(input)).changed()
    }

    pub(crate) fn playground_warning(&self, ui: &mut egui::Ui, message: &str, p: &ThemePalette) {
        ui.label(RichText::new(message).size(11.0).strong().color(p.red));
    }

    pub(crate) fn render_custom_playground_bar(&mut self, ui: &mut egui::Ui, p: &ThemePalette) {
        egui::Frame::none()
            .fill(p.sidebar_bg)
            .rounding(Rounding::same(8.0))
            .stroke(Stroke::new(1.0_f32, p.amber))
            .inner_margin(egui::Margin::symmetric(14.0, 8.0))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
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
                            should_run |= self.playground_text_input(
                                ui,
                                Problem::ContainsDuplicate,
                                "nums",
                                "nums =",
                                "1, 2, 3, 1",
                                160.0,
                                p,
                            );
                            if ui.button("[1,2,3,1]").clicked() {
                                self.set_input_str(Problem::ContainsDuplicate, "nums", "1,2,3,1");
                                should_run = true;
                            }
                            if ui.button("[1,2,3,4]").clicked() {
                                self.set_input_str(Problem::ContainsDuplicate, "nums", "1,2,3,4");
                                should_run = true;
                            }
                            let value_count = crate::utils::parse_i32_vec(
                                self.get_input_str(
                                    Problem::ContainsDuplicate,
                                    "nums",
                                    "1, 2, 3, 1",
                                ),
                                &[],
                            )
                            .len();
                            if value_count
                                > crate::algorithms::contains_duplicate::CONTAINS_DUPLICATE_VISUALIZATION_LIMIT
                            {
                                self.playground_warning(
                                    ui,
                                    &format!(
                                        "Detailed trace limit: {} values",
                                        crate::algorithms::contains_duplicate::CONTAINS_DUPLICATE_VISUALIZATION_LIMIT
                                    ),
                                    p,
                                );
                            }
                        }
                        Problem::TwoSum => {
                            should_run |= self.playground_text_input(
                                ui,
                                Problem::TwoSum,
                                "nums",
                                "nums =",
                                "2, 7, 11, 15",
                                130.0,
                                p,
                            );
                            should_run |= self.playground_int_input(
                                ui,
                                Problem::TwoSum,
                                "target",
                                "target =",
                                9,
                                p,
                            );
                            if ui.button("[2,7,11,15] t=9").clicked() {
                                self.set_input_str(Problem::TwoSum, "nums", "2,7,11,15");
                                self.set_input_int(Problem::TwoSum, "target", 9);
                                should_run = true;
                            }
                            let value_count = crate::utils::parse_i32_vec(
                                self.get_input_str(Problem::TwoSum, "nums", "2, 7, 11, 15"),
                                &[],
                            )
                            .len();
                            let (approach_name, visualization_limit) =
                                if self.selected_approach_id == 1 {
                                    (
                                        "Brute Force",
                                        crate::algorithms::two_sum::BRUTE_FORCE_VISUALIZATION_LIMIT,
                                    )
                                } else {
                                    (
                                        "Hash Map",
                                        crate::algorithms::two_sum::HASH_MAP_VISUALIZATION_LIMIT,
                                    )
                                };
                            if value_count > visualization_limit {
                                self.playground_warning(
                                    ui,
                                    &format!(
                                        "{} trace limit: {} values",
                                        approach_name, visualization_limit
                                    ),
                                    p,
                                );
                            }
                        }
                        Problem::ValidAnagram => {
                            should_run |= self.playground_text_input(
                                ui,
                                Problem::ValidAnagram,
                                "s",
                                "s =",
                                "anagram",
                                90.0,
                                p,
                            );
                            should_run |= self.playground_text_input(
                                ui,
                                Problem::ValidAnagram,
                                "t",
                                "t =",
                                "nagaram",
                                90.0,
                                p,
                            );
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
                            let s_value =
                                self.get_input_str(Problem::ValidAnagram, "s", "anagram");
                            let t_value =
                                self.get_input_str(Problem::ValidAnagram, "t", "nagaram");
                            let accepts_input = s_value
                                .bytes()
                                .all(|byte| byte.is_ascii_lowercase())
                                && t_value.bytes().all(|byte| byte.is_ascii_lowercase());
                            if !accepts_input {
                                self.playground_warning(
                                    ui,
                                    "Trace inputs must use lowercase a-z only",
                                    p,
                                );
                            } else if s_value.len().max(t_value.len())
                                > crate::algorithms::valid_anagram::VALID_ANAGRAM_VISUALIZATION_LIMIT
                            {
                                self.playground_warning(
                                    ui,
                                    &format!(
                                        "Detailed trace limit: {} characters each",
                                        crate::algorithms::valid_anagram::VALID_ANAGRAM_VISUALIZATION_LIMIT
                                    ),
                                    p,
                                );
                            }
                        }
                        Problem::GroupAnagrams => {
                            should_run |= self.playground_text_input(
                                ui,
                                Problem::GroupAnagrams,
                                "strs",
                                "strs =",
                                "eat, tea, tan, ate, nat, bat",
                                200.0,
                                p,
                            );
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
                            should_run |= self.playground_text_input(
                                ui,
                                Problem::TopKFrequent,
                                "nums",
                                "nums =",
                                "1, 1, 1, 2, 2, 3",
                                120.0,
                                p,
                            );
                            should_run |= self.playground_int_input(
                                ui,
                                Problem::TopKFrequent,
                                "k",
                                "k =",
                                2,
                                p,
                            );
                            if ui.button("[1,1,1,2,2,3] k=2").clicked() {
                                self.set_input_str(Problem::TopKFrequent, "nums", "1,1,1,2,2,3");
                                self.set_input_int(Problem::TopKFrequent, "k", 2);
                                should_run = true;
                            }
                        }
                        Problem::ProductExceptSelf => {
                            should_run |= self.playground_text_input(
                                ui,
                                Problem::ProductExceptSelf,
                                "nums",
                                "nums =",
                                "1, 2, 4, 6",
                                160.0,
                                p,
                            );
                            if ui.button("[1,2,4,6]").clicked() {
                                self.set_input_str(Problem::ProductExceptSelf, "nums", "1,2,4,6");
                                should_run = true;
                            }
                        }
                        Problem::EncodeDecode => {
                            should_run |= self.playground_text_input(
                                ui,
                                Problem::EncodeDecode,
                                "strs",
                                "strs =",
                                "Hello, World",
                                160.0,
                                p,
                            );
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
                            should_run |= self.playground_text_input(
                                ui,
                                Problem::LongestConsecutive,
                                "nums",
                                "nums =",
                                "2, 20, 4, 10, 3, 4, 5",
                                180.0,
                                p,
                            );
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
                            should_run |= self.playground_text_input(
                                ui,
                                Problem::ValidPalindrome,
                                "s",
                                "s =",
                                "Was it a car or a cat I saw?",
                                200.0,
                                p,
                            );
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
                            should_run |= self.playground_text_input(
                                ui,
                                Problem::BestTimeStock,
                                "prices",
                                "prices =",
                                "10, 1, 5, 6, 7, 1",
                                160.0,
                                p,
                            );
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
                            should_run |= self.playground_text_input(
                                ui,
                                Problem::ValidParentheses,
                                "s",
                                "s =",
                                "([{}])",
                                140.0,
                                p,
                            );
                            if ui.button("([{}])").clicked() {
                                self.set_input_str(Problem::ValidParentheses, "s", "([{}])");
                                should_run = true;
                            }
                        }
                        Problem::BinarySearch => {
                            should_run |= self.playground_text_input(
                                ui,
                                Problem::BinarySearch,
                                "nums",
                                "nums =",
                                "-1, 0, 2, 4, 6, 8",
                                140.0,
                                p,
                            );
                            should_run |= self.playground_int_input(
                                ui,
                                Problem::BinarySearch,
                                "target",
                                "target =",
                                4,
                                p,
                            );
                            if ui.button("[-1,0,2,4,6,8] t=4").clicked() {
                                self.set_input_str(Problem::BinarySearch, "nums", "-1,0,2,4,6,8");
                                self.set_input_int(Problem::BinarySearch, "target", 4);
                                should_run = true;
                            }
                        }
                        Problem::ReverseLinkedList => {
                            should_run |= self.playground_text_input(
                                ui,
                                Problem::ReverseLinkedList,
                                "nodes",
                                "nodes =",
                                "0, 1, 2, 3",
                                140.0,
                                p,
                            );
                            if ui.button("[0,1,2,3]").clicked() {
                                self.set_input_str(Problem::ReverseLinkedList, "nodes", "0,1,2,3");
                                should_run = true;
                            }
                        }
                        Problem::MergeTwoLists => {
                            should_run |= self.playground_text_input(
                                ui,
                                Problem::MergeTwoLists,
                                "list1",
                                "list1 =",
                                "1, 2, 4",
                                100.0,
                                p,
                            );
                            should_run |= self.playground_text_input(
                                ui,
                                Problem::MergeTwoLists,
                                "list2",
                                "list2 =",
                                "1, 3, 5",
                                100.0,
                                p,
                            );
                            if ui.button("[1,2,4] & [1,3,5]").clicked() {
                                self.set_input_str(Problem::MergeTwoLists, "list1", "1,2,4");
                                self.set_input_str(Problem::MergeTwoLists, "list2", "1,3,5");
                                should_run = true;
                            }
                        }
                        Problem::LinkedListCycle => {
                            should_run |= self.playground_text_input(
                                ui,
                                Problem::LinkedListCycle,
                                "nodes",
                                "nodes =",
                                "1, 2, 3, 4",
                                120.0,
                                p,
                            );
                            should_run |= self.playground_int_input(
                                ui,
                                Problem::LinkedListCycle,
                                "cycle_idx",
                                "cycle_idx =",
                                1,
                                p,
                            );
                            if ui.button("[1,2,3,4] idx=1").clicked() {
                                self.set_input_str(Problem::LinkedListCycle, "nodes", "1,2,3,4");
                                self.set_input_int(Problem::LinkedListCycle, "cycle_idx", 1);
                                should_run = true;
                            }
                        }
                        Problem::InvertTree
                        | Problem::MaxDepthTree
                        | Problem::DiameterTree
                        | Problem::BalancedTree => {
                            should_run |= self.playground_text_input(
                                ui,
                                self.current_problem,
                                "tree_nodes",
                                "tree nodes =",
                                "1, 2, 3, 4, 5, 6, 7",
                                180.0,
                                p,
                            );
                            if ui.button("[1,2,3,4,5,6,7]").clicked() {
                                self.set_input_str(
                                    self.current_problem,
                                    "tree_nodes",
                                    "1,2,3,4,5,6,7",
                                );
                                should_run = true;
                            }
                        }
                        Problem::SameTree => {
                            should_run |= self.playground_text_input(
                                ui,
                                Problem::SameTree,
                                "tree_p",
                                "p =",
                                "1, 2, 3",
                                120.0,
                                p,
                            );
                            should_run |= self.playground_text_input(
                                ui,
                                Problem::SameTree,
                                "tree_q",
                                "q =",
                                "1, 2, 3",
                                120.0,
                                p,
                            );
                        }
                        Problem::Subtree => {
                            should_run |= self.playground_text_input(
                                ui,
                                Problem::Subtree,
                                "tree_root",
                                "root =",
                                "3, 4, 5, 1, 2",
                                150.0,
                                p,
                            );
                            should_run |= self.playground_text_input(
                                ui,
                                Problem::Subtree,
                                "tree_sub_root",
                                "subRoot =",
                                "4, 1, 2",
                                120.0,
                                p,
                            );
                        }
                        Problem::TwoSumII => {
                            should_run |= self.playground_text_input(
                                ui,
                                Problem::TwoSumII,
                                "nums",
                                "nums =",
                                "2, 7, 11, 15",
                                140.0,
                                p,
                            );
                            should_run |= self.playground_int_input(
                                ui,
                                Problem::TwoSumII,
                                "target",
                                "target =",
                                9,
                                p,
                            );
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
                            should_run |= self.playground_text_input(
                                ui,
                                self.current_problem,
                                "nums",
                                "nums =",
                                "-1, 0, 1, 2, -1, -4",
                                180.0,
                                p,
                            );
                            if ui.button("Default Preset").clicked() {
                                self.set_input_str(
                                    self.current_problem,
                                    "nums",
                                    "-1,0,1,2,-1,-4",
                                );
                                should_run = true;
                            }
                        }
                        Problem::SearchRotatedArray | Problem::FindMinRotated => {
                            should_run |= self.playground_text_input(
                                ui,
                                self.current_problem,
                                "nums",
                                "nums =",
                                "4, 5, 6, 7, 0, 1, 2",
                                140.0,
                                p,
                            );
                            should_run |= self.playground_int_input(
                                ui,
                                self.current_problem,
                                "target",
                                "target =",
                                0,
                                p,
                            );
                            if ui.button("[4,5,6,7,0,1,2] t=0").clicked() {
                                self.set_input_str(
                                    self.current_problem,
                                    "nums",
                                    "4,5,6,7,0,1,2",
                                );
                                self.set_input_int(self.current_problem, "target", 0);
                                should_run = true;
                            }
                        }
                        Problem::ImplementTrie => {
                            should_run |= self.playground_text_input(
                                ui,
                                Problem::ImplementTrie,
                                "words",
                                "insert =",
                                "apple, app, ape",
                                140.0,
                                p,
                            );
                            should_run |= self.playground_text_input(
                                ui,
                                Problem::ImplementTrie,
                                "search",
                                "search =",
                                "app",
                                80.0,
                                p,
                            );
                            if ui.button("apple, app").clicked() {
                                self.set_input_str(Problem::ImplementTrie, "words", "apple, app");
                                self.set_input_str(Problem::ImplementTrie, "search", "app");
                                should_run = true;
                            }
                        }
                        Problem::WordDictionary => {
                            should_run |= self.playground_text_input(
                                ui,
                                Problem::WordDictionary,
                                "words",
                                "words =",
                                "bad, dad, mad",
                                140.0,
                                p,
                            );
                            should_run |= self.playground_text_input(
                                ui,
                                Problem::WordDictionary,
                                "pattern",
                                "pattern =",
                                ".ad",
                                80.0,
                                p,
                            );
                        }
                        Problem::WordSearchII => {
                            should_run |= self.playground_text_input(
                                ui,
                                Problem::WordSearchII,
                                "words",
                                "words =",
                                "oath, pea, eat, rain",
                                200.0,
                                p,
                            );
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
