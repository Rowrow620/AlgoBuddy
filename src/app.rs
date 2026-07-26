use eframe::egui::{self, Color32, Frame, RichText, Rounding, Stroke};
use web_time::Instant;

use crate::algorithms::{
    bucket_sort::generate_bucket_sort_steps,
    min_heap::generate_min_heap_steps,
    sorting::generate_sorting_steps,
    encode_decode::generate_encode_decode_steps,
    product_except_self::generate_product_steps,
    two_sum::generate_two_sum_steps,
    valid_anagram::generate_valid_anagram_steps,
    valid_palindrome::generate_valid_palindrome_steps,
    valid_parentheses::generate_valid_parentheses_steps,
    best_time_stock::generate_best_time_stock_steps,
    binary_search::generate_binary_search_steps,
    reverse_linked_list::generate_reverse_linked_list_steps,
    merge_two_lists::generate_merge_two_lists_steps,
    linked_list_cycle::generate_linked_list_cycle_steps,
    invert_tree::generate_invert_tree_steps,
    max_depth_tree::generate_max_depth_tree_steps,
    diameter_tree::generate_diameter_tree_steps,
    valid_sudoku::generate_valid_sudoku_steps,
    longest_consecutive::generate_longest_consecutive_steps,
    contains_duplicate::generate_contains_duplicate_steps,
    group_anagrams::generate_group_anagrams_steps,
    balanced_tree::generate_balanced_tree_steps,
    same_tree::generate_same_tree_steps,
    subtree::generate_subtree_steps,
    climbing_stairs::generate_climbing_stairs_steps,
    min_cost_stairs::generate_min_cost_stairs_steps,
    kth_largest_stream::generate_kth_largest_stream_steps,
    last_stone::generate_last_stone_steps,
    meeting_rooms::generate_meeting_rooms_steps,
    happy_number::generate_happy_number_steps,
    plus_one::generate_plus_one_steps,
    single_number::generate_single_number_steps,
    count_bits::generate_count_bits_steps,
    counting_bits::generate_counting_bits_array_steps,
    reverse_bits::generate_reverse_bits_steps,
    missing_number::generate_missing_number_steps,
    two_sum_ii::generate_two_sum_ii_steps,
    three_sum::generate_three_sum_steps,
    container_water::generate_container_water_steps,
    trapping_rain::generate_trapping_rain_steps,
    min_stack::generate_min_stack_steps,
    eval_rpn::generate_eval_rpn_steps,
    length_of_longest_substring::generate_longest_substring_steps,
    search_2d_matrix::generate_search_2d_matrix_steps,
    house_robber::generate_house_robber_steps,
    generate_parentheses::generate_parentheses_combinations_steps,
    daily_temperatures::generate_daily_temperatures_steps,
    car_fleet::generate_car_fleet_steps,
    largest_rectangle::generate_largest_rectangle_steps,
    character_replacement::generate_character_replacement_steps,
    permutation_in_string::generate_permutation_in_string_steps,
    min_window_substring::generate_min_window_substring_steps,
    sliding_window_max::generate_sliding_window_max_steps,
    search_rotated_array::generate_search_rotated_array_steps,
    find_min_rotated::generate_find_min_rotated_steps,
    time_key_value_store::generate_time_key_value_store_steps,
    find_median_sorted_arrays::generate_find_median_sorted_arrays_steps,
    trie::*,
    heap::*,
    backtracking::*,
};


use crate::model::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RightTab {
    CodeTrace,
    ProblemDetails,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    Visualizer,
    RoadmapDashboard,
}

pub struct VisualizerApp {
    // Theme & Settings
    theme: Theme,
    colorblind_mode: ColorblindMode,
    show_settings_modal: bool,
    view_mode: ViewMode,
    completed_problems: std::collections::HashSet<u32>,

    // Navigation state & Sidebar Visibility
    show_roadmap_sidebar: bool,
    show_right_sidebar: bool,
    current_problem: Problem,
    selected_approach_id: usize,
    selected_difficulty: Option<Difficulty>,
    search_query: String,
    right_tab: RightTab,

    // Inputs per problem
    contains_dup_nums_input: String,

    two_sum_nums_input: String,
    two_sum_target_input: i32,

    valid_anagram_s_input: String,
    valid_anagram_t_input: String,

    group_anagrams_input: String,

    topk_nums_input: String,
    topk_k_input: usize,
    topk_nums: Vec<i32>,
    topk_k: usize,

    ed_strs_input: String,
    ed_strs: Vec<String>,

    prod_nums_input: String,
    prod_nums: Vec<i32>,

    palindrome_s_input: String,
    parentheses_s_input: String,

    stock_prices_input: String,
    binary_search_nums_input: String,
    binary_search_target_input: i32,
    linked_list_nodes_input: String,

    merge_list1_input: String,
    merge_list2_input: String,
    cycle_nodes_input: String,
    cycle_index_input: i32,

    tree_nodes_input: String,
    sudoku_preset_valid: bool,
    longest_consecutive_nums_input: String,

    two_pointer_nums_input: String,
    two_pointer_target_input: i32,

    trie_words_input: String,
    trie_search_input: String,
    word_dict_words_input: String,
    word_dict_pattern_input: String,
    word_search_ii_words_input: String,

    // Playback state

    steps: Vec<Step>,
    current_step_idx: usize,
    is_playing: bool,
    playback_speed_ms: u64,
    playback_speed_mult: f32,
    last_step_time: Instant,


    canvas_zoom: f32,
}


impl Default for VisualizerApp {
    fn default() -> Self {
        let mut app = Self {
            theme: Theme::DarkVSCode, // Default to user's favorite VS Code Dark style!
            colorblind_mode: ColorblindMode::Off,
            show_settings_modal: false,
            view_mode: ViewMode::Visualizer,
            completed_problems: Problem::all().iter().map(|p| p.id()).collect(),

            show_roadmap_sidebar: true,
            show_right_sidebar: true,
            current_problem: Problem::ContainsDuplicate,
            selected_approach_id: 0,
            selected_difficulty: None,
            search_query: String::new(),
            right_tab: RightTab::CodeTrace,

            contains_dup_nums_input: "1, 2, 3, 1".to_string(),

            two_sum_nums_input: "2, 7, 11, 15".to_string(),
            two_sum_target_input: 9,

            valid_anagram_s_input: "anagram".to_string(),
            valid_anagram_t_input: "nagaram".to_string(),

            group_anagrams_input: "eat, tea, tan, ate, nat, bat".to_string(),

            topk_nums_input: "1, 1, 1, 2, 2, 3".to_string(),
            topk_k_input: 2,
            topk_nums: vec![1, 1, 1, 2, 2, 3],
            topk_k: 2,

            ed_strs_input: "Hello, World".to_string(),
            ed_strs: vec!["Hello".to_string(), "World".to_string()],

            prod_nums_input: "1, 2, 4, 6".to_string(),
            prod_nums: vec![1, 2, 4, 6],

            palindrome_s_input: "Was it a car or a cat I saw?".to_string(),
            parentheses_s_input: "([{}])".to_string(),

            stock_prices_input: "10, 1, 5, 6, 7, 1".to_string(),
            binary_search_nums_input: "-1, 0, 2, 4, 6, 8".to_string(),
            binary_search_target_input: 4,
            linked_list_nodes_input: "0, 1, 2, 3".to_string(),

            merge_list1_input: "1, 2, 4".to_string(),
            merge_list2_input: "1, 3, 5".to_string(),
            cycle_nodes_input: "1, 2, 3, 4".to_string(),
            cycle_index_input: 1,

            tree_nodes_input: "1, 2, 3, 4, 5, 6, 7".to_string(),
            sudoku_preset_valid: true,
            longest_consecutive_nums_input: "2, 20, 4, 10, 3, 4, 5".to_string(),

            two_pointer_nums_input: "2, 7, 11, 15".to_string(),
            two_pointer_target_input: 9,

            trie_words_input: "apple, app, ape".to_string(),
            trie_search_input: "app".to_string(),
            word_dict_words_input: "bad, dad, mad".to_string(),
            word_dict_pattern_input: ".ad".to_string(),
            word_search_ii_words_input: "oath, pea, eat, rain".to_string(),

            steps: Vec::new(),

            current_step_idx: 0,
            is_playing: false,
            playback_speed_ms: 600,
            playback_speed_mult: 1.0,
            last_step_time: Instant::now(),


            canvas_zoom: 1.0,
        };

        app.recompute_steps();
        app
    }
}

impl VisualizerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        #[cfg(target_arch = "wasm32")]
        if let Some(loading) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("loading_text"))
        {
            loading.remove();
        }

        let mut app = Self::default();
        if let Some(storage) = cc.storage {
            if let Some(saved_completed) = eframe::get_value::<std::collections::HashSet<u32>>(storage, "algobuddy_completed_problems") {
                app.completed_problems = saved_completed;
            }
        }
        app
    }


    fn current_palette(&self) -> ThemePalette {
        self.theme.palette(self.colorblind_mode)
    }

    fn parse_tree_input(&self) -> Vec<Option<i32>> {
        self.tree_nodes_input.split(',')
            .map(|s| {
                let trimmed = s.trim();
                if trimmed.eq_ignore_ascii_case("null") || trimmed.is_empty() {
                    None
                } else {
                    trimmed.parse::<i32>().ok()
                }
            })
            .collect()
    }

    fn get_sudoku_board(&self) -> [[char; 9]; 9] {
        if self.sudoku_preset_valid {
            [
                ['1','2','.','.','3','.','.','.','.'],
                ['4','.','.','5','.','.','.','.','.'],
                ['.','9','8','.','.','.','.','.','3'],
                ['5','.','.','.','6','.','.','.','4'],
                ['.','.','.','8','.','3','.','.','5'],
                ['7','.','.','.','2','.','.','.','6'],
                ['.','.','.','.','.','.','2','.','.'],
                ['.','.','.','4','1','9','.','.','8'],
                ['.','.','.','.','8','.','.','7','9'],
            ]
        } else {
            [
                ['1','2','.','.','3','.','.','.','.'],
                ['4','.','.','5','.','.','.','.','.'],
                ['.','9','1','.','.','.','.','.','3'],
                ['5','.','.','.','6','.','.','.','4'],
                ['.','.','.','8','.','3','.','.','5'],
                ['7','.','.','.','2','.','.','.','6'],
                ['.','.','.','.','.','.','2','.','.'],
                ['.','.','.','4','1','9','.','.','8'],
                ['.','.','.','.','8','.','.','7','9'],
            ]
        }
    }

    fn recompute_steps(&mut self) {
        let app_id = self.selected_approach_id;
        self.steps = match self.current_problem {
            Problem::ContainsDuplicate => {
                let parsed: Vec<i32> = self.contains_dup_nums_input.split(',')
                    .filter_map(|s| s.trim().parse().ok()).collect();
                let nums = if parsed.is_empty() { vec![1, 2, 3, 1] } else { parsed };
                generate_contains_duplicate_steps(&nums, app_id)
            }
            Problem::TwoSum => {
                let parsed: Vec<i32> = self.two_sum_nums_input.split(',')
                    .filter_map(|s| s.trim().parse().ok()).collect();
                let nums = if parsed.is_empty() { vec![2, 7, 11, 15] } else { parsed };
                generate_two_sum_steps(&nums, self.two_sum_target_input, app_id)
            }
            Problem::ValidAnagram => {
                generate_valid_anagram_steps(&self.valid_anagram_s_input, &self.valid_anagram_t_input, app_id)
            }
            Problem::GroupAnagrams => {
                let strs: Vec<String> = self.group_anagrams_input.split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                let input_strs = if strs.is_empty() { vec!["eat".into(), "tea".into(), "tan".into(), "ate".into(), "nat".into(), "bat".into()] } else { strs };
                generate_group_anagrams_steps(&input_strs, app_id)
            }
            Problem::TopKFrequent => {
                let parsed: Vec<i32> = self.topk_nums_input.split(',')
                    .filter_map(|s| s.trim().parse().ok()).collect();
                if !parsed.is_empty() { self.topk_nums = parsed; }
                let unique = self.topk_nums.iter().collect::<std::collections::HashSet<_>>().len();
                self.topk_k = self.topk_k_input.clamp(1, unique.max(1));
                self.topk_k_input = self.topk_k;

                match app_id {
                    0 => generate_bucket_sort_steps(&self.topk_nums, self.topk_k),
                    1 => generate_min_heap_steps(&self.topk_nums, self.topk_k),
                    _ => generate_sorting_steps(&self.topk_nums, self.topk_k),
                }
            }
            Problem::ProductExceptSelf => {
                let parsed: Vec<i32> = self.prod_nums_input.split(',')
                    .filter_map(|s| s.trim().parse().ok()).collect();
                if !parsed.is_empty() { self.prod_nums = parsed; }
                generate_product_steps(&self.prod_nums)
            }
            Problem::EncodeDecode => {
                self.ed_strs = self.ed_strs_input.split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                if self.ed_strs.is_empty() { self.ed_strs = vec!["".to_string()]; }
                generate_encode_decode_steps(&self.ed_strs)
            }
            Problem::ValidSudoku => {
                let board = self.get_sudoku_board();
                generate_valid_sudoku_steps(&board)
            }
            Problem::LongestConsecutive => {
                let parsed: Vec<i32> = self.longest_consecutive_nums_input.split(',')
                    .filter_map(|s| s.trim().parse().ok()).collect();
                let nums = if parsed.is_empty() { vec![2, 20, 4, 10, 3, 4, 5] } else { parsed };
                generate_longest_consecutive_steps(&nums)
            }
            Problem::ValidPalindrome => {
                generate_valid_palindrome_steps(&self.palindrome_s_input, app_id)
            }
            Problem::BestTimeStock => {
                let parsed: Vec<i32> = self.stock_prices_input.split(',')
                    .filter_map(|s| s.trim().parse().ok()).collect();
                let prices = if parsed.is_empty() { vec![10, 1, 5, 6, 7, 1] } else { parsed };
                generate_best_time_stock_steps(&prices)
            }
            Problem::ValidParentheses => {
                generate_valid_parentheses_steps(&self.parentheses_s_input)
            }
            Problem::BinarySearch => {
                let parsed: Vec<i32> = self.binary_search_nums_input.split(',')
                    .filter_map(|s| s.trim().parse().ok()).collect();
                let nums = if parsed.is_empty() { vec![-1, 0, 2, 4, 6, 8] } else { parsed };
                generate_binary_search_steps(&nums, self.binary_search_target_input)
            }
            Problem::ReverseLinkedList => {
                let parsed: Vec<i32> = self.linked_list_nodes_input.split(',')
                    .filter_map(|s| s.trim().parse().ok()).collect();
                let nodes = if parsed.is_empty() { vec![0, 1, 2, 3] } else { parsed };
                generate_reverse_linked_list_steps(&nodes)
            }
            Problem::MergeTwoLists => {
                let l1: Vec<i32> = self.merge_list1_input.split(',').filter_map(|s| s.trim().parse().ok()).collect();
                let l2: Vec<i32> = self.merge_list2_input.split(',').filter_map(|s| s.trim().parse().ok()).collect();
                generate_merge_two_lists_steps(&l1, &l2)
            }
            Problem::LinkedListCycle => {
                let nodes: Vec<i32> = self.cycle_nodes_input.split(',').filter_map(|s| s.trim().parse().ok()).collect();
                generate_linked_list_cycle_steps(&nodes, self.cycle_index_input)
            }
            Problem::InvertTree => {
                let tree = self.parse_tree_input();
                generate_invert_tree_steps(&tree)
            }
            Problem::MaxDepthTree => {
                let tree = self.parse_tree_input();
                generate_max_depth_tree_steps(&tree)
            }
            Problem::DiameterTree => {
                let tree = self.parse_tree_input();
                generate_diameter_tree_steps(&tree)
            }
            Problem::BalancedTree => {
                let tree = self.parse_tree_input();
                generate_balanced_tree_steps(&tree)
            }
            Problem::SameTree => {
                let tree = self.parse_tree_input();
                generate_same_tree_steps(&tree, &tree)
            }
            Problem::Subtree => {
                let tree = self.parse_tree_input();
                generate_subtree_steps(&tree, &vec![tree.get(1).cloned().flatten()])
            }
            Problem::ClimbingStairs => generate_climbing_stairs_steps(5),
            Problem::MinCostStairs => generate_min_cost_stairs_steps(&[10, 15, 20]),
            Problem::KthLargestStream => generate_kth_largest_stream_steps(3, &[4, 5, 8, 2], 3),
            Problem::LastStone => generate_last_stone_weight_steps(&[2, 7, 4, 1, 8, 1]),
            Problem::MeetingRooms => generate_meeting_rooms_steps(&[(0, 30), (5, 10), (15, 20)]),
            Problem::HappyNumber => generate_happy_number_steps(19),
            Problem::PlusOne => generate_plus_one_steps(&[1, 2, 3]),
            Problem::SingleNumber => generate_single_number_steps(&[4, 1, 2, 1, 2]),
            Problem::CountBits => generate_count_bits_steps(11),
            Problem::CountingBits => generate_counting_bits_array_steps(5),
            Problem::ReverseBits => generate_reverse_bits_steps(43261596),
            Problem::MissingNumber => generate_missing_number_steps(&[3, 0, 1]),
            Problem::TwoSumII => {
                let parsed: Vec<i32> = self.two_pointer_nums_input.split(',')
                    .filter_map(|s| s.trim().parse().ok()).collect();
                let nums = if parsed.is_empty() { vec![2, 7, 11, 15] } else { parsed };
                generate_two_sum_ii_steps(&nums, self.two_pointer_target_input)
            }
            Problem::ThreeSum => {
                let parsed: Vec<i32> = self.two_pointer_nums_input.split(',')
                    .filter_map(|s| s.trim().parse().ok()).collect();
                let nums = if parsed.is_empty() { vec![-1, 0, 1, 2, -1, -4] } else { parsed };
                generate_three_sum_steps(&nums)
            }
            Problem::ContainerWater => {
                let parsed: Vec<i32> = self.two_pointer_nums_input.split(',')
                    .filter_map(|s| s.trim().parse().ok()).collect();
                let nums = if parsed.is_empty() { vec![1, 8, 6, 2, 5, 4, 8, 3, 7] } else { parsed };
                generate_container_water_steps(&nums)
            }
            Problem::TrappingRain => {
                let parsed: Vec<i32> = self.two_pointer_nums_input.split(',')
                    .filter_map(|s| s.trim().parse().ok()).collect();
                let nums = if parsed.is_empty() { vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1] } else { parsed };
                generate_trapping_rain_steps(&nums)
            }
            Problem::MinStack => {
                generate_min_stack_steps(&[
                    ("push", Some(-2)),
                    ("push", Some(0)),
                    ("push", Some(-3)),
                    ("getMin", None),
                    ("pop", None),
                    ("top", None),
                    ("getMin", None),
                ])
            }
            Problem::EvalRPN => {
                let tokens = vec!["2".to_string(), "1".to_string(), "+".to_string(), "3".to_string(), "*".to_string()];
                generate_eval_rpn_steps(&tokens)
            }
            Problem::LongestSubstring => {
                let s = if self.palindrome_s_input.is_empty() { "abcabcbb" } else { &self.palindrome_s_input };
                generate_longest_substring_steps(s)
            }
            Problem::Search2DMatrix => {
                let matrix = vec![
                    vec![1, 3, 5, 7],
                    vec![10, 11, 16, 20],
                    vec![23, 30, 34, 60],
                ];
                generate_search_2d_matrix_steps(&matrix, 3)
            }
            Problem::HouseRobber => {
                let parsed: Vec<i32> = self.two_pointer_nums_input.split(',')
                    .filter_map(|s| s.trim().parse().ok()).collect();
                let nums = if parsed.is_empty() { vec![1, 2, 3, 1] } else { parsed };
                generate_house_robber_steps(&nums)
            }
            Problem::GenerateParentheses => generate_parentheses_combinations_steps(3),
            Problem::DailyTemperatures => generate_daily_temperatures_steps(&[73, 74, 75, 71, 69, 72, 76, 73]),
            Problem::CarFleet => generate_car_fleet_steps(12, &[10, 8, 0, 5, 3], &[2, 4, 1, 1, 3]),
            Problem::LargestRectangle => generate_largest_rectangle_steps(&[2, 1, 5, 6, 2, 3]),
            Problem::CharacterReplacement => generate_character_replacement_steps("ABAB", 2),
            Problem::PermutationInString => generate_permutation_in_string_steps("ab", "eidbaooo"),
            Problem::MinWindowSubstring => generate_min_window_substring_steps("ADOBECODEBANC", "ABC"),
            Problem::SlidingWindowMax => generate_sliding_window_max_steps(&[1, 3, -1, -3, 5, 3, 6, 7], 3),
            Problem::SearchRotatedArray => {
                let parsed: Vec<i32> = self.binary_search_nums_input.split(',')
                    .filter_map(|s| s.trim().parse().ok()).collect();
                let nums = if parsed.is_empty() { vec![4, 5, 6, 7, 0, 1, 2] } else { parsed };
                generate_search_rotated_array_steps(&nums, self.binary_search_target_input)
            }
            Problem::FindMinRotated => {
                let parsed: Vec<i32> = self.binary_search_nums_input.split(',')
                    .filter_map(|s| s.trim().parse().ok()).collect();
                let nums = if parsed.is_empty() { vec![3, 4, 5, 1, 2] } else { parsed };
                generate_find_min_rotated_steps(&nums)
            }
            Problem::TimeKeyValueStore => generate_time_key_value_store_steps(),
            Problem::FindMedianSortedArrays => generate_find_median_sorted_arrays_steps(&[1, 3], &[2, 4]),
            Problem::ImplementTrie => {
                let insert_words: Vec<String> = self.trie_words_input.split(',')
                    .map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
                generate_implement_trie_steps(&insert_words, &self.trie_search_input)
            }
            Problem::WordDictionary => {
                let words: Vec<String> = self.word_dict_words_input.split(',')
                    .map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
                generate_word_dictionary_steps(&words, &self.word_dict_pattern_input)
            }
            Problem::WordSearchII => {
                let words: Vec<String> = self.word_search_ii_words_input.split(',')
                    .map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
                generate_word_search_ii_steps(&words)
            }
            Problem::Subsets => {
                let parsed: Vec<i32> = self.contains_dup_nums_input.split(',')
                    .filter_map(|s| s.trim().parse().ok()).collect();
                let nums = if parsed.is_empty() { vec![1, 2, 3] } else { parsed };
                generate_subsets_steps(&nums)
            }
            Problem::Permutations => {
                let parsed: Vec<i32> = self.contains_dup_nums_input.split(',')
                    .filter_map(|s| s.trim().parse().ok()).collect();
                let nums = if parsed.is_empty() { vec![1, 2, 3] } else { parsed };
                generate_permutations_steps(&nums)
            }
            Problem::KClosestPoints => generate_k_closest_points_steps(&[(1, 3), (-2, 2), (5, 8)], 1),
            Problem::TaskScheduler => generate_task_scheduler_steps(&['A', 'A', 'A', 'B', 'B', 'B'], 2),
        };


        self.current_step_idx = 0;
        self.is_playing = false;
    }

    fn select_problem(&mut self, problem: Problem) {
        if self.current_problem != problem {
            self.current_problem = problem;
            self.selected_approach_id = 0;
            self.recompute_steps();
        }
    }

    fn render_settings_modal(&mut self, ctx: &egui::Context) {
        if !self.show_settings_modal { return; }

        let p = self.current_palette();
        let mut is_open = true;

        egui::Window::new("⚙ AlgoBuddy UI Settings & Accessibility")
            .open(&mut is_open)
            .resizable(false)
            .collapsible(false)
            .default_width(380.0)
            .frame(Frame::window(&ctx.style()).fill(p.sidebar_bg).stroke(Stroke::new(1.0_f32, p.cell_border)))
            .show(ctx, |ui| {
                ui.heading(RichText::new("UI Theme Selection").color(p.cyan).strong().size(15.0));
                ui.add_space(6.0);

                ui.horizontal(|ui| {
                    if ui.selectable_label(self.theme == Theme::DarkVSCode, "VS Code Dark").clicked() {
                        self.theme = Theme::DarkVSCode;
                    }
                    if ui.selectable_label(self.theme == Theme::DarkCyber, "Cyber Navy").clicked() {
                        self.theme = Theme::DarkCyber;
                    }
                    if ui.selectable_label(self.theme == Theme::LightClean, "Clean Light").clicked() {
                        self.theme = Theme::LightClean;
                    }
                });

                ui.add_space(14.0);
                ui.separator();
                ui.add_space(8.0);

                ui.heading(RichText::new("Colorblindness & Accessibility Filter").color(p.cyan).strong().size(15.0));
                ui.add_space(6.0);

                if ui.selectable_label(self.colorblind_mode == ColorblindMode::Off, "Off (Standard Red / Emerald Green)").clicked() {
                    self.colorblind_mode = ColorblindMode::Off;
                }
                if ui.selectable_label(self.colorblind_mode == ColorblindMode::RedGreenSafe, "Protan / Deuteran (Cobalt Blue / Safety Orange)").clicked() {
                    self.colorblind_mode = ColorblindMode::RedGreenSafe;
                }
                if ui.selectable_label(self.colorblind_mode == ColorblindMode::HighContrast, "High Contrast B&W").clicked() {
                    self.colorblind_mode = ColorblindMode::HighContrast;
                }                ui.add_space(14.0);
                ui.separator();
                ui.add_space(8.0);

                ui.heading(RichText::new("Keyboard Shortcuts").color(p.cyan).strong().size(15.0));
                ui.add_space(6.0);
                egui::Grid::new("keyboard_shortcuts_grid").num_columns(2).spacing([16.0, 6.0]).show(ui, |ui| {
                    ui.label(RichText::new("Spacebar").strong().color(p.text_primary));
                    ui.label("Play / Pause Timeline Animation");
                    ui.end_row();

                    ui.label(RichText::new("← / → Arrows").strong().color(p.text_primary));
                    ui.label("Previous / Next Step");
                    ui.end_row();

                    ui.label(RichText::new("R Key").strong().color(p.text_primary));
                    ui.label("Reset Timeline to Step 1");
                    ui.end_row();

                    ui.label(RichText::new("↑ / ↓ Arrows").strong().color(p.text_primary));
                    ui.label("Speed Up / Slow Down Playback");
                    ui.end_row();
                });

                ui.add_space(14.0);
                ui.separator();
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.label(RichText::new("Active Theme:").color(p.text_muted));
                    ui.label(RichText::new(self.theme.label()).color(p.text_primary).strong());
                });
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Accessibility:").color(p.text_muted));
                    ui.label(RichText::new(self.colorblind_mode.label()).color(p.emerald_text).strong());
                });

                ui.add_space(12.0);
                if ui.button(RichText::new("Close Settings").strong()).clicked() {
                    self.show_settings_modal = false;
                }
            });

        if !is_open {
            self.show_settings_modal = false;
        }
    }

    fn render_fullscreen_roadmap_dashboard(&mut self, ctx: &egui::Context, p: &ThemePalette) {
        let mut prob_to_select = None;
        let total_solved = self.completed_problems.len();
        let overall_pct = (total_solved as f32 / 150.0) * 100.0;

        egui::CentralPanel::default()
            .frame(Frame::none().fill(p.sidebar_bg).inner_margin(24.0))
            .show(ctx, |ui| {
                // Top Navigation & Action Bar
                ui.horizontal(|ui| {
                    if ui.button(RichText::new("◀ Back to Visualizer").strong().color(p.cyan).size(14.0)).clicked() {
                        self.view_mode = ViewMode::Visualizer;
                    }
                    ui.add_space(16.0);
                    ui.heading(RichText::new("🏆 NeetCode 150 Mastery Dashboard").color(p.amber).strong().size(20.0));

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(RichText::new("Reset All Progress").strong().color(p.red)).clicked() {
                            self.completed_problems.clear();
                        }

                        ui.add_space(12.0);
                        egui::Frame::none().fill(p.cell_bg).rounding(Rounding::same(6.0)).inner_margin(8.0).show(ui, |ui| {
                            ui.label(RichText::new(format!("Overall Progress: {} / 150 Solved ({:.1}%)", total_solved, overall_pct)).font(egui::FontId::monospace(13.0)).color(p.emerald_text).strong());
                        });
                    });
                });

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(16.0);

                // 2-Column Dashboard Layout
                ui.columns(2, |cols| {
                    // Left Column: Category Progress List
                    cols[0].heading(RichText::new("Category Completion Breakdown").color(p.cyan).size(15.0));
                    cols[0].add_space(10.0);

                    egui::ScrollArea::vertical().id_source("cat_scroll").show(&mut cols[0], |ui| {
                        for &category in Category::all() {
                            let total_in_cat = Problem::all().iter().filter(|p| p.category() == category).count().max(1);
                            let solved_in_cat = Problem::all().iter().filter(|p| p.category() == category && self.completed_problems.contains(&p.id())).count();
                            let pct = (solved_in_cat as f32 / total_in_cat as f32) * 100.0;
                            let col = if solved_in_cat == total_in_cat && solved_in_cat > 0 { p.emerald_text } else if solved_in_cat > 0 { p.amber } else { p.text_dim };

                            egui::Frame::group(ui.style())
                                .fill(p.step_box_bg)
                                .rounding(Rounding::same(6.0))
                                .inner_margin(8.0)
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(RichText::new(category.name()).strong().color(p.text_primary));
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            ui.label(RichText::new(format!("{}/{} ({:.0}%)", solved_in_cat, total_in_cat, pct)).font(egui::FontId::monospace(12.0)).color(col).strong());
                                        });
                                    });
                                    ui.add_space(4.0);
                                    ui.add(egui::ProgressBar::new(solved_in_cat as f32 / total_in_cat as f32).text(""));
                                });
                            ui.add_space(6.0);
                        }
                    });

                    // Right Column: Problem Completion & Launcher Grid
                    cols[1].heading(RichText::new("Implemented Problems & Checkmarks").color(p.amber).size(15.0));
                    cols[1].add_space(10.0);

                    egui::ScrollArea::vertical().id_source("prob_scroll").show(&mut cols[1], |ui| {
                        egui::Grid::new("fullscreen_roadmap_grid")
                            .striped(true)
                            .spacing([12.0, 8.0])
                            .show(ui, |ui| {
                                for prob in Problem::all() {
                                    let details = prob.details();
                                    let d_color = difficulty_color(details.difficulty, p);
                                    let mut is_completed = self.completed_problems.contains(&details.id);

                                    if ui.checkbox(&mut is_completed, "").changed() {
                                        if is_completed {
                                            self.completed_problems.insert(details.id);
                                        } else {
                                            self.completed_problems.remove(&details.id);
                                        }
                                    }

                                    if ui.button(RichText::new(format!("#{} {}", details.id, details.title)).color(if is_completed { p.emerald_text } else { p.cyan }).strong()).clicked() {
                                        prob_to_select = Some(*prob);
                                    }
                                    ui.label(RichText::new(details.difficulty.label()).color(d_color).strong());
                                    ui.label(RichText::new(details.category.name()).color(p.text_muted));
                                    ui.end_row();
                                }
                            });
                    });
                });
            });

        if let Some(prob) = prob_to_select {
            self.select_problem(prob);
            self.view_mode = ViewMode::Visualizer;
        }
    }
}

fn difficulty_color(d: Difficulty, p: &ThemePalette) -> Color32 {
    match d {
        Difficulty::Easy => p.emerald_text,
        Difficulty::Medium => p.amber,
        Difficulty::Hard => p.red,
    }
}

impl eframe::App for VisualizerApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, "algobuddy_completed_problems", &self.completed_problems);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ── Keyboard Shortcuts (Only active when not typing in text fields) ──
        if !ctx.wants_keyboard_input() {
            ctx.input(|i| {
                if i.key_pressed(egui::Key::Space) {
                    if self.current_step_idx >= self.steps.len().saturating_sub(1) {
                        self.current_step_idx = 0;
                    }
                    self.is_playing = !self.is_playing;
                    self.last_step_time = Instant::now();
                }
                if i.key_pressed(egui::Key::ArrowLeft) {
                    self.is_playing = false;
                    self.current_step_idx = self.current_step_idx.saturating_sub(1);
                }
                if i.key_pressed(egui::Key::ArrowRight) {
                    self.is_playing = false;
                    if self.current_step_idx < self.steps.len().saturating_sub(1) {
                        self.current_step_idx += 1;
                    }
                }
                if i.key_pressed(egui::Key::R) {
                    self.is_playing = false;
                    self.current_step_idx = 0;
                }
                if i.key_pressed(egui::Key::ArrowUp) || i.key_pressed(egui::Key::Plus) || i.key_pressed(egui::Key::Equals) {

                    self.playback_speed_ms = self.playback_speed_ms.saturating_sub(100).max(100);
                }
                if i.key_pressed(egui::Key::ArrowDown) || i.key_pressed(egui::Key::Minus) {
                    self.playback_speed_ms = (self.playback_speed_ms + 100).min(1500);
                }
            });
        }

        if self.is_playing {
            if self.last_step_time.elapsed().as_millis() as u64 >= self.playback_speed_ms {
                if self.current_step_idx < self.steps.len().saturating_sub(1) {
                    self.current_step_idx += 1;
                } else {
                    self.is_playing = false;
                }
                self.last_step_time = Instant::now();
            }
            ctx.request_repaint();
        }

        let p = self.current_palette();


        self.render_settings_modal(ctx);

        if self.view_mode == ViewMode::RoadmapDashboard {
            self.render_fullscreen_roadmap_dashboard(ctx, &p);
            return;
        }

        if self.show_roadmap_sidebar {
            egui::SidePanel::left("roadmap_sidebar")
                .min_width(280.0)
                .max_width(450.0)
                .default_width(320.0)
                .frame(Frame::none().inner_margin(12.0).fill(p.sidebar_bg))
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.heading(RichText::new("NeetCode Roadmap").color(p.cyan).strong().size(18.0));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button(RichText::new("◀ Hide").font(egui::FontId::proportional(11.0)).color(p.text_muted)).clicked() {
                                self.show_roadmap_sidebar = false;
                            }
                        });
                    });

                    ui.add_space(8.0);

                    // Filter controls: Direct keyword search & Difficulty toggles
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("🔍").font(egui::FontId::proportional(12.0)));
                        ui.add(
                            egui::TextEdit::singleline(&mut self.search_query)
                                .hint_text("Search problem...")
                                .desired_width(180.0),
                        );
                    });

                    ui.add_space(4.0);

                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Diff:").font(egui::FontId::proportional(11.0)).color(p.text_muted));

                        if ui.selectable_label(self.selected_difficulty.is_none(), "All").clicked() {
                            self.selected_difficulty = None;
                        }
                        if ui.selectable_label(self.selected_difficulty == Some(Difficulty::Easy), RichText::new("Easy").color(difficulty_color(Difficulty::Easy, &p))).clicked() {
                            self.selected_difficulty = Some(Difficulty::Easy);
                        }
                        if ui.selectable_label(self.selected_difficulty == Some(Difficulty::Medium), RichText::new("Med").color(difficulty_color(Difficulty::Medium, &p))).clicked() {
                            self.selected_difficulty = Some(Difficulty::Medium);
                        }
                        if ui.selectable_label(self.selected_difficulty == Some(Difficulty::Hard), RichText::new("Hard").color(difficulty_color(Difficulty::Hard, &p))).clicked() {
                            self.selected_difficulty = Some(Difficulty::Hard);
                        }
                    });


                    ui.add_space(6.0);
                    ui.separator();
                    ui.add_space(6.0);

                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for &category in Category::all() {
                            let problems_in_cat: Vec<Problem> = Problem::all()
                                .iter()
                                .copied()
                                .filter(|p| p.category() == category)
                                .filter(|p| {
                                    if let Some(diff) = self.selected_difficulty { p.difficulty() == diff } else { true }
                                })
                                .filter(|p| {
                                    if self.search_query.trim().is_empty() {
                                        true
                                    } else {
                                        let q = self.search_query.to_lowercase();
                                        p.title().to_lowercase().contains(&q) || p.id().to_string().contains(&q)
                                    }
                                })
                                .collect();

                            let total_in_cat = Problem::all().iter().filter(|p| p.category() == category).count();
                            let header_text = format!("{} ({})", category.name(), problems_in_cat.len());

                            let is_active_cat = problems_in_cat.contains(&self.current_problem);
                            let header_color = if is_active_cat { p.cyan } else { p.text_primary };

                            egui::CollapsingHeader::new(RichText::new(header_text).color(header_color).strong())
                                .default_open(is_active_cat || !problems_in_cat.is_empty())
                                .show(ui, |ui| {
                                    if problems_in_cat.is_empty() {
                                        if total_in_cat == 0 {
                                            ui.label(RichText::new("  (Coming Soon)").italics().font(egui::FontId::proportional(11.0)).color(p.text_dim));
                                        } else {
                                            ui.label(RichText::new("  (Filtered Out)").italics().font(egui::FontId::proportional(11.0)).color(p.text_dim));
                                        }
                                    } else {
                                        for prob in problems_in_cat {
                                            let is_selected = self.current_problem == prob;
                                            let diff_color = difficulty_color(prob.difficulty(), &p);

                                            ui.horizontal(|ui| {
                                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                    ui.label(RichText::new(prob.difficulty().label()).font(egui::FontId::monospace(10.0)).color(diff_color));

                                                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                                        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);

                                                        let btn_rt = RichText::new(format!("#{} {}", prob.id(), prob.title()))
                                                            .font(egui::FontId::proportional(12.0));
                                                        let btn_text = if is_selected {
                                                            btn_rt.color(p.cyan).strong()
                                                        } else {
                                                            btn_rt.color(p.text_primary)

                                                        };

                                                        let resp = ui.selectable_label(is_selected, btn_text);
                                                        if resp.clicked() {
                                                            self.select_problem(prob);
                                                        }
                                                        resp.on_hover_text(format!("#{} {} ({})", prob.id(), prob.title(), prob.difficulty().label()));
                                                    });
                                                });
                                            });

                                        }
                                    }
                                });
                        }
                    });
                });
        }


        // ── Top Header Panel ──
        egui::TopBottomPanel::top("header_panel")
            .frame(Frame::none().inner_margin(12.0).fill(p.bg_dark))
            .show(ctx, |ui| {
                let prob = self.current_problem;
                let details = prob.details();

                ui.horizontal(|ui| {
                    if !self.show_roadmap_sidebar {
                        if ui.button(RichText::new("▶ Show Roadmap").strong().color(p.cyan)).clicked() {
                            self.show_roadmap_sidebar = true;
                        }
                        ui.add_space(8.0);
                    }

                    ui.heading(
                        RichText::new(format!("#{} {}", prob.id(), prob.title()))
                            .font(egui::FontId::proportional(18.0))
                            .strong()
                            .color(p.cyan),
                    );

                    let d_color = difficulty_color(prob.difficulty(), &p);
                    egui::Frame::none().fill(p.cell_bg).rounding(Rounding::same(4.0)).inner_margin(4.0).show(ui, |ui| {
                        ui.label(RichText::new(prob.difficulty().label()).font(egui::FontId::monospace(11.0)).strong().color(d_color));
                    });

                    ui.label(RichText::new(format!("Category: {}", prob.category().name())).font(egui::FontId::proportional(12.0)).color(p.text_muted));

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if !self.show_right_sidebar {
                            if ui.button(RichText::new("Code & Problem ◀").strong().color(p.cyan)).clicked() {
                                self.show_right_sidebar = true;
                            }
                            ui.add_space(12.0);
                        }

                        if ui.button(RichText::new("⚙ Settings").strong().color(p.cyan)).clicked() {
                            self.show_settings_modal = true;
                        }

                        ui.add_space(8.0);
                        let solved_count = self.completed_problems.len();
                        let pct = (solved_count as f32 / 150.0) * 100.0;
                        if ui.button(RichText::new(format!("🏆 {} / 150 Solved ({:.1}%)", solved_count, pct)).strong().color(p.amber)).clicked() {
                            self.view_mode = ViewMode::RoadmapDashboard;
                        }

                        ui.add_space(8.0);
                        if ui.button(RichText::new(format!("🌐 LeetCode #{} ↗", details.id)).strong().color(p.cyan)).clicked() {
                            #[cfg(not(target_arch = "wasm32"))]
                            let _ = open::that(details.leetcode_url);
                            #[cfg(target_arch = "wasm32")]
                            if let Some(win) = web_sys::window() {
                                let _ = win.open_with_url_and_target(details.leetcode_url, "_blank");
                            }
                        }
                    });
                });

                ui.add_space(6.0);

                // Multi-Approach Selector Row
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Approach:").strong().color(p.text_primary));
                    for approach in details.approaches {
                        let is_sel = self.selected_approach_id == approach.id;
                        let btn_label = format!("{} ({})", approach.name, approach.time_complexity);
                        if ui.selectable_label(is_sel, RichText::new(btn_label).color(if is_sel { p.cyan } else { p.text_primary }).strong()).clicked() {
                            self.selected_approach_id = approach.id;
                            self.recompute_steps();
                        }
                    }
                });

                ui.add_space(6.0);

                // Playback Control Bar
                ui.horizontal(|ui| {
                    let play_text = if self.is_playing { "Pause" } else { "Play" };
                    if ui.button(RichText::new(play_text).strong()).clicked() {
                        if self.current_step_idx >= self.steps.len().saturating_sub(1) {
                            self.current_step_idx = 0;
                        }
                        self.is_playing = !self.is_playing;
                        self.last_step_time = Instant::now();

                    }
                    if ui.button("Prev").clicked() { self.is_playing = false; self.current_step_idx = self.current_step_idx.saturating_sub(1); }
                    if ui.button("Next").clicked() { self.is_playing = false; if self.current_step_idx < self.steps.len().saturating_sub(1) { self.current_step_idx += 1; } }
                    if ui.button("Reset").clicked() { self.is_playing = false; self.current_step_idx = 0; }

                    ui.separator();
                    ui.label(RichText::new(format!("Step {} / {}", self.current_step_idx + 1, self.steps.len())).strong());
                    let max_idx = self.steps.len().saturating_sub(1);
                    ui.add(egui::Slider::new(&mut self.current_step_idx, 0..=max_idx).show_value(false));

                    ui.separator();
                    ui.label(RichText::new("Speed:").strong().color(p.text_primary));
                    let slider_resp = ui.add(
                        egui::Slider::new(&mut self.playback_speed_mult, 0.25..=4.0)
                            .step_by(0.05)
                            .custom_formatter(|val, _| format!("{:.2}x", val))
                    );
                    if slider_resp.changed() {
                        self.playback_speed_ms = (600.0 / self.playback_speed_mult.max(0.1)).round() as u64;
                    }
                });
            });

        // ── Right Sidebar: Tabbed Code Trace & Problem Details ──
        if self.show_right_sidebar {
            egui::SidePanel::right("right_sidebar")
                .min_width(300.0)
                .max_width(600.0)
                .default_width(400.0)
                .frame(Frame::none().inner_margin(12.0).fill(p.sidebar_bg))
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.selectable_label(self.right_tab == RightTab::CodeTrace, RichText::new("💻 Code Trace").strong()).clicked() {
                            self.right_tab = RightTab::CodeTrace;
                        }
                        if ui.selectable_label(self.right_tab == RightTab::ProblemDetails, RichText::new("📄 Problem Statement & Examples").strong()).clicked() {
                            self.right_tab = RightTab::ProblemDetails;
                        }
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button(RichText::new("Hide ▶").font(egui::FontId::proportional(11.0)).color(p.text_muted)).clicked() {
                                self.show_right_sidebar = false;
                            }
                        });
                    });

                    ui.separator();
                    ui.add_space(6.0);

                    match self.right_tab {
                        RightTab::CodeTrace => {
                            if let Some(step) = self.steps.get(self.current_step_idx) {
                                egui::Frame::group(ui.style())
                                    .fill(p.step_box_bg)
                                    .rounding(Rounding::same(8.0))
                                    .inner_margin(10.0)
                                    .show(ui, |ui| {
                                        ui.label(
                                            RichText::new(format!("STEP {} / {}", self.current_step_idx + 1, self.steps.len()))
                                                .font(egui::FontId::monospace(11.0))
                                                .color(p.cyan).strong(),
                                        );
                                        ui.add_space(4.0);
                                        ui.label(RichText::new(&step.description).font(egui::FontId::proportional(13.0)).color(p.text_primary));
                                    });

                                ui.add_space(8.0);
                                
                                // Algorithm Complexity Card
                                if let Some(app_meta) = self.current_problem.details().approaches.get(self.selected_approach_id) {
                                    render_complexity_card(ui, app_meta, &p);
                                }

                                ui.add_space(8.0);
                                // Variable Scope Table
                                render_variable_scope_table(ui, step, &p);

                                ui.add_space(10.0);
                                ui.label(RichText::new("Python Implementation").strong().color(p.text_muted));
                                ui.add_space(6.0);

                                let code_lines = approach_code_lines(self.current_problem, self.selected_approach_id);

                                egui::ScrollArea::vertical().show(ui, |ui| {
                                    for (line_num, line_text) in &code_lines {
                                        let is_active = step.code_line == *line_num;
                                        let text_color = if is_active { p.text_primary } else { p.text_muted };
                                        let bg = if is_active { p.code_active_bg } else { Color32::TRANSPARENT };

                                        egui::Frame::none()
                                            .fill(bg)
                                            .rounding(Rounding::same(4.0))
                                            .inner_margin(3.0)
                                            .show(ui, |ui| {
                                                ui.horizontal(|ui| {
                                                    ui.label(RichText::new(format!("{:2} | ", line_num)).font(egui::FontId::monospace(11.0)).color(p.text_dim));
                                                    let mut rt = RichText::new(*line_text).font(egui::FontId::monospace(12.0)).color(text_color);
                                                    if is_active { rt = rt.strong(); }
                                                    ui.label(rt);
                                                });
                                            });
                                    }
                                });
                            }
                        }
                        RightTab::ProblemDetails => {
                            let details = self.current_problem.details();
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                ui.heading(RichText::new(format!("#{} {}", details.id, details.title)).color(p.cyan).strong().size(18.0));
                                ui.add_space(4.0);

                                ui.horizontal(|ui| {
                                    let d_color = difficulty_color(details.difficulty, &p);
                                    ui.label(RichText::new(details.difficulty.label()).color(d_color).strong());
                                    ui.label(RichText::new(format!("• Category: {}", details.category.name())).color(p.text_muted));
                                });

                                ui.add_space(10.0);
                                ui.label(RichText::new("Description").strong().color(p.text_primary));
                                ui.add_space(4.0);
                                ui.label(RichText::new(details.statement).font(egui::FontId::proportional(13.0)).color(p.text_primary));

                                ui.add_space(14.0);
                                ui.label(RichText::new("Examples").strong().color(p.text_primary));
                                ui.add_space(4.0);

                                for (ex_idx, ex) in details.examples.iter().enumerate() {
                                    egui::Frame::group(ui.style())
                                        .fill(p.step_box_bg)
                                        .rounding(Rounding::same(8.0))
                                        .inner_margin(10.0)
                                        .show(ui, |ui| {
                                            ui.label(RichText::new(format!("Example {}", ex_idx + 1)).strong().color(p.amber));
                                            ui.add_space(4.0);
                                            ui.label(RichText::new(format!("Input: {}", ex.input)).font(egui::FontId::monospace(12.0)).color(p.cyan));
                                            ui.label(RichText::new(format!("Output: {}", ex.output)).font(egui::FontId::monospace(12.0)).color(p.emerald_text));
                                            if !ex.explanation.is_empty() {
                                                ui.label(RichText::new(format!("Explanation: {}", ex.explanation)).font(egui::FontId::proportional(12.0)).color(p.text_muted));
                                            }
                                        });
                                    ui.add_space(6.0);
                                }

                                ui.add_space(10.0);
                                ui.label(RichText::new("Constraints").strong().color(p.text_primary));
                                ui.add_space(4.0);
                                for constraint in details.constraints {
                                    ui.label(RichText::new(format!("• {}", constraint)).font(egui::FontId::monospace(12.0)).color(p.text_muted));
                                }

                                ui.add_space(14.0);
                                ui.label(RichText::new("Solution Approaches").strong().color(p.text_primary));
                                ui.add_space(4.0);
                                for app_meta in details.approaches {
                                    let is_selected = app_meta.id == self.selected_approach_id;
                                    let bg = if is_selected { p.code_active_bg } else { p.step_box_bg };
                                    egui::Frame::group(ui.style())
                                        .fill(bg)
                                        .rounding(Rounding::same(8.0))
                                        .inner_margin(10.0)
                                        .show(ui, |ui| {
                                            ui.label(RichText::new(format!("Approach {}: {}", app_meta.id + 1, app_meta.name)).strong().color(if is_selected { p.cyan } else { p.text_primary }));
                                            ui.label(RichText::new(format!("Time: {} | Space: {}", app_meta.time_complexity, app_meta.space_complexity)).font(egui::FontId::monospace(11.0)).color(p.text_muted));
                                            if !app_meta.description.is_empty() {
                                                ui.add_space(4.0);
                                                ui.label(RichText::new(app_meta.description).font(egui::FontId::proportional(12.0)).color(p.text_primary));
                                            }
                                        });
                                    ui.add_space(6.0);
                                }

                                ui.add_space(14.0);
                                if ui.button(RichText::new("🌐 Open on LeetCode.com ↗").strong().color(p.cyan)).clicked() {
                                    #[cfg(not(target_arch = "wasm32"))]
                                    let _ = open::that(details.leetcode_url);
                                    #[cfg(target_arch = "wasm32")]
                                    if let Some(win) = web_sys::window() {
                                        let _ = win.open_with_url_and_target(details.leetcode_url, "_blank");
                                    }
                                }

                            });
                        }
                    }
                });
        }

        // ── Central Canvas ──
        egui::CentralPanel::default()
            .frame(Frame::none().inner_margin(16.0).fill(p.bg_dark))
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
                self.render_custom_playground_bar(ui, &p);
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
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
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
                                    ui.label(RichText::new(format!("🔍 {}%", zoom_pct)).font(egui::FontId::monospace(11.0)).color(p.cyan));

                                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);
                                        ui.label(RichText::new("📊 Live State Inspector").font(egui::FontId::proportional(12.0)).color(p.cyan).strong());
                                        ui.separator();
                                        let step_lbl = ui.label(RichText::new(&step.description).font(egui::FontId::proportional(13.0)).color(p.text_primary));
                                        step_lbl.on_hover_text(&step.description);
                                    });
                                });
                            });
                        });
                    ui.add_space(14.0);


                    egui::ScrollArea::both().show(ui, |ui| {

                        match &step.visual {
                            VisualState::ContainsDuplicate { nums, active_idx, seen_set, duplicate_val, has_duplicate } => {
                                self.render_contains_duplicate(ui, &p, nums, *active_idx, seen_set, *duplicate_val, *has_duplicate);
                            }
                            VisualState::GroupAnagrams { input_strs, active_idx, key_fmt, groups } => {
                                self.render_group_anagrams(ui, &p, input_strs, *active_idx, key_fmt, groups);
                            }
                            VisualState::TwoSum { nums, target, active_idx, secondary_idx, map, found_indices } => {
                                self.render_two_sum(ui, &p, nums, *target, *active_idx, *secondary_idx, map, *found_indices);
                            }
                            VisualState::ValidAnagram { s, t, s_counts, t_counts, active_s_idx, active_t_idx, is_anagram } => {
                                self.render_valid_anagram(ui, &p, s, t, s_counts, t_counts, *active_s_idx, *active_t_idx, *is_anagram);
                            }
                            VisualState::TwoPointers { chars, left, right, is_valid, skipped } => {
                                self.render_two_pointers(ui, &p, chars, *left, *right, *is_valid, *skipped);
                            }
                            VisualState::Stack { chars, active_idx, stack, is_valid } => {
                                self.render_stack(ui, &p, chars, *active_idx, stack, *is_valid);
                            }
                            VisualState::BestTimeStock { prices, left_buy, right_sell, current_profit, max_profit } => {
                                self.render_stock(ui, &p, prices, *left_buy, *right_sell, *current_profit, *max_profit);
                            }
                            VisualState::BinarySearch { nums, target, left, right, mid, found_idx } => {
                                self.render_binary_search(ui, &p, nums, *target, *left, *right, *mid, *found_idx);
                            }
                            VisualState::LinkedList { nodes, prev_idx, curr_idx, next_idx, reversed_so_far } => {
                                self.render_linked_list(ui, &p, nodes, *prev_idx, *curr_idx, *next_idx, reversed_so_far);
                            }
                            VisualState::MergeLinkedLists { list1, list2, p1_idx, p2_idx, merged_so_far } => {
                                self.render_merge_lists(ui, &p, list1, list2, *p1_idx, *p2_idx, merged_so_far);
                            }
                            VisualState::LinkedListCycle { nodes, cycle_target_idx, slow_idx, fast_idx, has_cycle } => {
                                self.render_list_cycle(ui, &p, nodes, *cycle_target_idx, *slow_idx, *fast_idx, *has_cycle);
                            }
                            VisualState::TreeVisual { tree_nodes, active_node_idx, secondary_node_idx, depth_val, max_diameter } => {
                                self.render_tree(ui, &p, tree_nodes, *active_node_idx, *secondary_node_idx, *depth_val, *max_diameter);
                            }
                            VisualState::ValidSudoku { board, active_r, active_c, duplicate_pos, is_valid } => {
                                self.render_sudoku(ui, &p, board, *active_r, *active_c, *duplicate_pos, *is_valid);
                            }
                            VisualState::LongestConsecutive { nums, num_set, current_num, current_seq, max_length, is_seq_start } => {
                                self.render_longest_consecutive(ui, &p, nums, num_set, *current_num, current_seq, *max_length, *is_seq_start);
                            }
                            VisualState::TopK { nums, active_nums_idx, count_map, buckets, active_bucket_idx, result } => {
                                self.render_topk(ui, &p, nums, *active_nums_idx, count_map, buckets, *active_bucket_idx, result);
                            }
                            VisualState::EncodeDecode { input_strs, encoded_so_far, decoded_so_far, pointer, active_str_idx, phase } => {
                                self.render_encode_decode(ui, &p, input_strs, encoded_so_far, decoded_so_far, *pointer, *active_str_idx, phase);
                            }
                            VisualState::Product { nums, output, active_idx, prefix_val, suffix_val, phase } => {
                                self.render_product(ui, &p, nums, output, *active_idx, *prefix_val, *suffix_val, phase);
                            }
                            VisualState::Trie { .. } => {
                                self.render_trie(ui, &p);
                            }
                            VisualState::HeapVisual { heap_elements, active_idx, swapped_pair, heap_type_label } => {
                                self.render_heap_visualizer(ui, &p, heap_elements, *active_idx, *swapped_pair, heap_type_label);
                            }
                            VisualState::DecisionTreeVisual { current_path, active_choice, completed_results } => {
                                self.render_decision_tree_visualizer(ui, &p, current_path, active_choice.as_deref(), completed_results);
                            }
                        }

                    });

                }
            });

    }
}


// ── Visual Canvas Renderers ──

impl VisualizerApp {
    fn render_heap_visualizer(&self, ui: &mut egui::Ui, p: &ThemePalette, heap: &[i32], active_idx: Option<usize>, swapped: Option<(usize, usize)>, label: &str) {
        let z = self.canvas_zoom;
        ui.heading(RichText::new(format!("Dual Tree & Array Heap View: {}", label)).color(p.amber).size(16.0 * z));
        ui.add_space(12.0 * z);

        // 1. Array Representation with 2*i+1 and 2*i+2 formulas
        ui.group(|ui| {
            ui.label(RichText::new("UNDERLYING HEAP ARRAY [Index: 2*i + 1, 2*i + 2]").font(egui::FontId::monospace(11.0 * z)).color(p.cyan));
            ui.add_space(6.0 * z);
            ui.horizontal(|ui| {
                if heap.is_empty() {
                    ui.label(RichText::new("(Heap is Empty)").italics().color(p.text_dim));
                }
                for (i, &val) in heap.iter().enumerate() {
                    let is_act = active_idx == Some(i);
                    let is_swp = swapped.map_or(false, |(a, b)| a == i || b == i);
                    let bg = if is_swp { p.red } else if is_act { p.amber } else { p.cell_bg };

                    egui::Frame::none().fill(bg).rounding(Rounding::same(6.0 * z)).stroke(Stroke::new(1.0_f32 * z, p.purple)).inner_margin((10.0 * z).max(6.0)).show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.label(RichText::new(format!("i={}", i)).font(egui::FontId::monospace((10.0 * z).max(7.0))).color(p.text_muted));
                            ui.label(RichText::new(val.to_string()).font(egui::FontId::monospace((16.0 * z).max(10.0))).strong().color(Color32::WHITE));
                        });
                    });
                }
            });
        });

        ui.add_space(16.0 * z);

        // 2. Binary Tree Node Layout
        ui.group(|ui| {
            ui.label(RichText::new("BINARY TREE STRUCTURAL VIEW").font(egui::FontId::monospace(11.0 * z)).color(p.emerald_text));
            ui.add_space(6.0 * z);

            ui.horizontal(|ui| {
                for (i, &val) in heap.iter().enumerate() {
                    let is_act = active_idx == Some(i);
                    let bg = if is_act { p.amber } else { p.emerald };

                    egui::Frame::none().fill(bg).rounding(Rounding::same(20.0 * z)).inner_margin((10.0 * z).max(6.0)).show(ui, |ui| {
                        ui.label(RichText::new(val.to_string()).font(egui::FontId::monospace((14.0 * z).max(9.0))).strong().color(Color32::WHITE));
                    });
                    if i + 1 < heap.len() {
                        ui.label(RichText::new("•").color(p.text_dim));
                    }
                }
            });
        });
    }

    fn render_decision_tree_visualizer(&self, ui: &mut egui::Ui, p: &ThemePalette, current_path: &[i32], active_choice: Option<&str>, completed_results: &[Vec<i32>]) {
        let z = self.canvas_zoom;
        ui.heading(RichText::new("Backtracking & Recursion Decision Tree Visualizer").color(p.cyan).size(16.0 * z));
        ui.add_space(12.0 * z);

        ui.horizontal(|ui| {
            // Current Active Recursive Branch Path
            ui.group(|ui| {
                ui.label(RichText::new("ACTIVE DECISION BRANCH PATH").font(egui::FontId::monospace(11.0 * z)).color(p.amber));
                ui.add_space(6.0 * z);
                ui.horizontal(|ui| {
                    if current_path.is_empty() {
                        ui.label(RichText::new("[] (Root Level)").font(egui::FontId::monospace((14.0 * z).max(9.0))).color(p.text_dim));
                    } else {
                        for (i, &val) in current_path.iter().enumerate() {
                            egui::Frame::none().fill(p.cyan).rounding(Rounding::same(6.0 * z)).inner_margin((8.0 * z).max(4.0)).show(ui, |ui| {
                                ui.label(RichText::new(val.to_string()).font(egui::FontId::monospace((16.0 * z).max(10.0))).strong().color(Color32::WHITE));
                            });
                            if i + 1 < current_path.len() {
                                ui.label(RichText::new("➔").color(p.amber));
                            }
                        }
                    }
                });
                if let Some(choice) = active_choice {
                    ui.add_space(6.0 * z);
                    ui.label(RichText::new(format!("Current Decision: {}", choice)).font(egui::FontId::proportional(12.0 * z)).color(p.emerald_text).strong());
                }
            });

            ui.add_space(16.0 * z);

            // Completed Subsets / Permutations Grid
            ui.group(|ui| {
                ui.label(RichText::new(format!("GENERATED SOLUTIONS ({})", completed_results.len())).font(egui::FontId::monospace(11.0 * z)).color(p.emerald_text));
                ui.add_space(6.0 * z);
                ui.horizontal_wrapped(|ui| {
                    for res in completed_results {
                        egui::Frame::none().fill(p.step_box_bg).rounding(Rounding::same(4.0 * z)).inner_margin((6.0 * z).max(3.0)).show(ui, |ui| {
                            ui.label(RichText::new(format!("{:?}", res)).font(egui::FontId::monospace((12.0 * z).max(8.0))).color(p.text_primary));
                        });
                    }
                });
            });
        });
    }

    fn render_contains_duplicate(&self, ui: &mut egui::Ui, p: &ThemePalette, nums: &[i32], active_idx: Option<usize>, seen_set: &std::collections::BTreeSet<i32>, dup_val: Option<i32>, has_dup: Option<bool>) {
        let z = self.canvas_zoom;
        let font_sz = 16.0 * z;
        let label_sz = (10.0 * z).max(8.0);
        let margin = (10.0 * z).max(6.0);

        ui.heading(RichText::new("Contains Duplicate Detection (HashSet O(N))").color(p.cyan).size(16.0 * z));
        ui.add_space(8.0 * z);

        ui.group(|ui| {
            ui.label(RichText::new("INPUT NUMS ARRAY").font(egui::FontId::monospace(11.0 * z)).color(p.text_muted));
            ui.horizontal(|ui| {
                for (i, &val) in nums.iter().enumerate() {
                    let is_active = active_idx == Some(i);
                    let is_dup = dup_val == Some(val) && is_active;
                    let fill = if is_dup { p.red } else if is_active { p.amber } else { p.cell_bg };
                    let (label_color, val_color) = if is_dup || is_active {
                        (Color32::from_rgb(30, 35, 45), Color32::from_rgb(30, 35, 45))
                    } else {
                        (p.text_muted, Color32::WHITE)
                    };




                    egui::Frame::none().fill(fill).rounding(Rounding::same(8.0 * z)).stroke(Stroke::new(1.0_f32, p.cell_border)).inner_margin(margin).show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.label(RichText::new(format!("i={}", i)).font(egui::FontId::proportional(label_sz)).color(label_color));
                            ui.label(RichText::new(val.to_string()).font(egui::FontId::monospace(font_sz)).strong().color(val_color));
                        });
                    });
                }

            });
        });

        ui.add_space(20.0 * z);

        ui.group(|ui| {
            ui.label(RichText::new("HASHSET `SEEN`").font(egui::FontId::monospace(11.0 * z)).color(p.text_muted));
            ui.horizontal_wrapped(|ui| {
                if seen_set.is_empty() {
                    ui.label(RichText::new("Set is empty {}").italics().color(p.text_dim));
                } else {
                    for &val in seen_set {
                        let is_dup = dup_val == Some(val);
                        let fill = if is_dup { p.red } else { p.cell_bg };
                        egui::Frame::none().fill(fill).rounding(Rounding::same(6.0 * z)).stroke(Stroke::new(1.0_f32, p.purple)).inner_margin(margin).show(ui, |ui| {
                            ui.label(RichText::new(val.to_string()).font(egui::FontId::monospace(font_sz)).strong().color(Color32::WHITE));
                        });
                    }
                }
            });
        });





        if let Some(dup) = has_dup {
            ui.add_space(20.0);
            if dup {
                ui.heading(RichText::new(format!("Duplicate Found! Value {} appears at least twice.", dup_val.unwrap_or(0))).color(p.red).size(18.0));
            } else {
                ui.heading(RichText::new("All Elements Are Distinct! (Return False)").color(p.emerald_text).size(18.0));
            }
        }
    }

    fn render_group_anagrams(&self, ui: &mut egui::Ui, p: &ThemePalette, input_strs: &[String], active_idx: Option<usize>, key_fmt: &str, groups: &std::collections::BTreeMap<String, Vec<String>>) {
        let z = self.canvas_zoom;
        let font_title = (16.0 * z).max(10.0);
        let font_label = (11.0 * z).max(8.0);
        let font_sz = (14.0 * z).max(9.0);
        let font_small = (10.0 * z).max(8.0);
        let margin = (10.0 * z).max(4.0);

        ui.heading(RichText::new("Group Anagrams (HashMap Buckets)").color(p.cyan).size(font_title));
        ui.add_space(8.0 * z);

        ui.group(|ui| {
            ui.label(RichText::new("INPUT STRINGS ARRAY").font(egui::FontId::monospace(font_label)).color(p.text_muted));
            ui.horizontal(|ui| {
                for (i, s) in input_strs.iter().enumerate() {
                    let is_active = active_idx == Some(i);
                    let fill = if is_active { p.amber } else { p.cell_bg };

                    egui::Frame::none().fill(fill).rounding(Rounding::same(8.0 * z)).stroke(Stroke::new(1.0_f32 * z, p.cell_border)).inner_margin(margin).show(ui, |ui| {
                        ui.label(RichText::new(format!("\"{}\"", s)).font(egui::FontId::monospace(font_sz)).strong().color(Color32::WHITE));
                    });
                }
            });
        });

        if !key_fmt.is_empty() {
            ui.add_space(16.0 * z);
            egui::Frame::none().fill(p.cell_bg).rounding(Rounding::same(8.0 * z)).stroke(Stroke::new(1.0_f32 * z, p.cyan)).inner_margin(margin).show(ui, |ui| {
                ui.label(RichText::new(format!("Computed Anagram Key Signature: {}", key_fmt)).font(egui::FontId::monospace((13.0 * z).max(9.0))).strong().color(p.cyan));
            });
        }

        ui.add_space(20.0 * z);

        ui.group(|ui| {
            ui.label(RichText::new("HASHMAP GROUPS {signature -> list of words}").font(egui::FontId::monospace(font_label)).color(p.emerald_text));
            ui.horizontal_wrapped(|ui| {
                if groups.is_empty() {
                    ui.label(RichText::new("No groups formed yet...").italics().color(p.text_dim));
                } else {
                    for (key, items) in groups {
                        egui::Frame::none().fill(p.sidebar_bg).rounding(Rounding::same(10.0 * z)).stroke(Stroke::new(1.0_f32 * z, p.emerald)).inner_margin((12.0 * z).max(6.0)).show(ui, |ui| {
                            ui.vertical(|ui| {
                                ui.label(RichText::new(format!("Key: {}", key)).font(egui::FontId::monospace(font_small)).color(p.text_muted));
                                ui.separator();
                                ui.horizontal(|ui| {
                                    for word in items {
                                        egui::Frame::none().fill(p.emerald).rounding(Rounding::same(6.0 * z)).inner_margin((6.0 * z).max(3.0)).show(ui, |ui| {
                                            ui.label(RichText::new(format!("\"{}\"", word)).font(egui::FontId::monospace(font_sz)).strong().color(Color32::WHITE));
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

    fn render_longest_consecutive(&self, ui: &mut egui::Ui, p: &ThemePalette, nums: &[i32], num_set: &std::collections::BTreeSet<i32>, curr_num: Option<i32>, curr_seq: &[i32], max_len: usize, is_seq_start: Option<bool>) {
        let z = self.canvas_zoom;
        let font_title = (16.0 * z).max(10.0);
        let font_label = (11.0 * z).max(8.0);
        let font_sz = (16.0 * z).max(9.0);
        let margin = (8.0 * z).max(4.0);

        ui.heading(RichText::new("Longest Consecutive Sequence (HashSet O(N))").color(p.cyan).size(font_title));
        ui.add_space(8.0 * z);

        ui.group(|ui| {
            ui.label(RichText::new("INPUT ARRAY (nums)").font(egui::FontId::monospace(font_label)).color(p.text_muted));
            ui.horizontal(|ui| {
                for &val in nums {
                    let is_curr = curr_num == Some(val);
                    let is_in_seq = curr_seq.contains(&val);
                    let fill = if is_in_seq { p.emerald } else if is_curr { p.amber } else { p.cell_bg };

                    egui::Frame::none().fill(fill).rounding(Rounding::same(8.0 * z)).stroke(Stroke::new(1.0_f32 * z, p.cell_border)).inner_margin(margin).show(ui, |ui| {
                        ui.label(RichText::new(val.to_string()).font(egui::FontId::monospace(font_sz)).strong().color(Color32::WHITE));
                    });
                }
            });
        });

        ui.add_space(16.0 * z);

        ui.group(|ui| {
            ui.label(RichText::new("NUMSET (HashSet of unique values)").font(egui::FontId::monospace(font_label)).color(p.text_muted));
            ui.horizontal_wrapped(|ui| {
                for &val in num_set {
                    let is_curr = curr_num == Some(val);
                    let is_in_seq = curr_seq.contains(&val);

                    let fill = if is_in_seq {
                        p.emerald
                    } else if is_curr {
                        if is_seq_start == Some(true) { p.amber } else { p.text_dim }
                    } else {
                        p.cell_bg
                    };

                    egui::Frame::none().fill(fill).rounding(Rounding::same(6.0 * z)).stroke(Stroke::new(1.0_f32 * z, p.purple)).inner_margin(margin).show(ui, |ui| {
                        ui.label(RichText::new(val.to_string()).font(egui::FontId::monospace((14.0 * z).max(9.0))).strong().color(Color32::WHITE));
                    });
                }
            });
        });

        ui.add_space(20.0 * z);

        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.label(RichText::new("CURRENT STREAK SEQUENCE").font(egui::FontId::monospace(font_label)).color(p.emerald_text));
                ui.horizontal(|ui| {
                    if curr_seq.is_empty() {
                        ui.label(RichText::new("None (searching for sequence start...)").font(egui::FontId::monospace((14.0 * z).max(9.0))).color(p.text_dim));
                    } else {
                        for (i, &val) in curr_seq.iter().enumerate() {
                            egui::Frame::none().fill(p.emerald).rounding(Rounding::same(8.0 * z)).inner_margin((10.0 * z).max(4.0)).show(ui, |ui| {
                                ui.label(RichText::new(val.to_string()).font(egui::FontId::monospace((18.0 * z).max(10.0))).strong().color(Color32::WHITE));
                            });
                            if i + 1 < curr_seq.len() {
                                ui.label(RichText::new("->").font(egui::FontId::monospace((14.0 * z).max(9.0))).color(p.cyan));
                            }
                        }
                    }
                });
            });

            ui.add_space(20.0 * z);

            egui::Frame::none().fill(p.cell_bg).rounding(Rounding::same(8.0 * z)).stroke(Stroke::new(1.0_f32 * z, p.emerald_text)).inner_margin((12.0 * z).max(5.0)).show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.label(RichText::new("Max Streak (longest)").font(egui::FontId::proportional((11.0 * z).max(8.0))).color(p.text_muted));
                    ui.label(RichText::new(format!("{}", max_len)).font(egui::FontId::monospace((22.0 * z).max(12.0))).strong().color(p.emerald_text));
                });
            });
        });
    }


    fn render_sudoku(&self, ui: &mut egui::Ui, p: &ThemePalette, board: &[[char; 9]; 9], active_r: Option<usize>, active_c: Option<usize>, dup_pos: Option<(usize, usize)>, is_valid: Option<bool>) {
        let z = self.canvas_zoom;
        let font_title = (16.0 * z).max(10.0);
        let font_cell = (16.0 * z).max(9.0);
        let margin = (8.0 * z).max(3.0);

        ui.heading(RichText::new("9x9 Sudoku Board Validation Grid").color(p.cyan).size(font_title));
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

                            let border_color = if (r / 3 * 3 + c / 3) % 2 == 0 { p.purple } else { p.cell_border };

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
                ui.heading(RichText::new("Valid Sudoku Board! All rows, cols & 3x3 boxes satisfy constraint.").color(p.emerald_text).size((18.0 * z).max(11.0)));
            } else {
                ui.heading(RichText::new("Invalid Sudoku Board! Duplicate digit detected.").color(p.red).size((18.0 * z).max(11.0)));
            }
        }
    }

    fn render_merge_lists(&self, ui: &mut egui::Ui, p: &ThemePalette, list1: &[i32], list2: &[i32], p1_idx: Option<usize>, p2_idx: Option<usize>, merged: &[i32]) {
        let z = self.canvas_zoom;
        let font_title = (16.0 * z).max(10.0);
        let font_label = (11.0 * z).max(8.0);
        let font_node = (14.0 * z).max(9.0);
        let margin = (8.0 * z).max(4.0);

        ui.heading(RichText::new("Merge Two Sorted Linked Lists").color(p.cyan).size(font_title));
        ui.add_space(8.0 * z);

        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.label(RichText::new("LIST 1").font(egui::FontId::monospace(font_label)).color(p.text_muted));
                ui.horizontal(|ui| {
                    for (i, &val) in list1.iter().enumerate() {
                        let fill = if p1_idx == Some(i) { p.cyan } else { p.cell_bg };
                        egui::Frame::none().fill(fill).rounding(Rounding::same(8.0 * z)).stroke(Stroke::new(1.0_f32 * z, p.cell_border)).inner_margin(margin).show(ui, |ui| {
                            ui.label(RichText::new(format!("( {} ) ->", val)).font(egui::FontId::monospace(font_node)).strong().color(Color32::WHITE));
                        });
                    }
                    ui.label(RichText::new("None").font(egui::FontId::monospace((12.0 * z).max(8.0))).color(p.text_dim));
                });
            });

            ui.add_space(20.0 * z);

            ui.group(|ui| {
                ui.label(RichText::new("LIST 2").font(egui::FontId::monospace(font_label)).color(p.text_muted));
                ui.horizontal(|ui| {
                    for (i, &val) in list2.iter().enumerate() {
                        let fill = if p2_idx == Some(i) { p.pink } else { p.cell_bg };
                        egui::Frame::none().fill(fill).rounding(Rounding::same(8.0 * z)).stroke(Stroke::new(1.0_f32 * z, p.cell_border)).inner_margin(margin).show(ui, |ui| {
                            ui.label(RichText::new(format!("( {} ) ->", val)).font(egui::FontId::monospace(font_node)).strong().color(Color32::WHITE));
                        });
                    }
                    ui.label(RichText::new("None").font(egui::FontId::monospace((12.0 * z).max(8.0))).color(p.text_dim));
                });
            });
        });

        ui.add_space(20.0 * z);

        ui.group(|ui| {
            ui.label(RichText::new("MERGED SORTED LIST (TAIL ATTACHMENTS)").font(egui::FontId::monospace(font_label)).color(p.emerald_text));
            ui.horizontal(|ui| {
                if merged.is_empty() {
                    ui.label(RichText::new("Dummy Head -> None").font(egui::FontId::monospace((14.0 * z).max(9.0))).color(p.text_dim));
                } else {
                    for &val in merged {
                        egui::Frame::none().fill(p.emerald).rounding(Rounding::same(8.0 * z)).inner_margin((10.0 * z).max(4.0)).show(ui, |ui| {
                            ui.label(RichText::new(format!("( {} ) ->", val)).font(egui::FontId::monospace((16.0 * z).max(10.0))).strong().color(Color32::WHITE));
                        });
                    }
                    ui.label(RichText::new("None").font(egui::FontId::monospace((14.0 * z).max(9.0))).color(p.text_dim));
                }
            });
        });
    }


    fn render_list_cycle(&self, ui: &mut egui::Ui, p: &ThemePalette, nodes: &[i32], cycle_target: Option<usize>, slow: Option<usize>, fast: Option<usize>, has_cycle: Option<bool>) {
        let z = self.canvas_zoom;
        let font_title = (16.0 * z).max(10.0);
        let font_label = (11.0 * z).max(8.0);
        let font_node = (16.0 * z).max(9.0);
        let margin = (10.0 * z).max(4.0);

        ui.heading(RichText::new("Floyd's Tortoise and Hare Cycle Detection").color(p.cyan).size(font_title));
        ui.add_space(8.0 * z);

        ui.group(|ui| {
            ui.label(RichText::new("LINKED LIST NODES").font(egui::FontId::monospace(font_label)).color(p.text_muted));
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

                    egui::Frame::none().fill(fill).rounding(Rounding::same(8.0 * z)).stroke(Stroke::new(1.0_f32 * z, p.cell_border)).inner_margin(margin).show(ui, |ui| {
                        ui.vertical(|ui| {
                            let mut label = String::new();
                            if is_slow && is_fast { label.push_str("S & F"); }
                            else if is_slow { label.push_str("slow"); }
                            else if is_fast { label.push_str("fast"); }

                            ui.label(RichText::new(format!("idx {} {}", i, label)).font(egui::FontId::proportional((10.0 * z).max(8.0))).color(Color32::WHITE));
                            ui.label(RichText::new(format!("( {} ) ->", val)).font(egui::FontId::monospace(font_node)).strong().color(Color32::WHITE));
                        });
                    });
                }

                if let Some(target) = cycle_target {
                    ui.label(RichText::new(format!("↺ [Cycle -> node idx {}]", target)).font(egui::FontId::monospace((14.0 * z).max(9.0))).strong().color(p.amber));
                } else {
                    ui.label(RichText::new("None").font(egui::FontId::monospace((14.0 * z).max(9.0))).color(p.text_dim));
                }
            });
        });

        if let Some(cycle) = has_cycle {
            ui.add_space(20.0 * z);
            if cycle {
                ui.heading(RichText::new("Cycle Detected! Slow & Fast Pointers Met.").color(p.emerald_text).size((18.0 * z).max(11.0)));
            } else {
                ui.heading(RichText::new("No Cycle Exists (Fast Pointer Reached End)").color(p.red).size((18.0 * z).max(11.0)));
            }
        }
    }

    fn render_tree(&self, ui: &mut egui::Ui, p: &ThemePalette, tree_nodes: &[Option<i32>], active_idx: Option<usize>, sec_idx: Option<usize>, depth_val: Option<i32>, max_diameter: Option<i32>) {
        let z = self.canvas_zoom;
        let font_title = (16.0 * z).max(10.0);
        let font_label = (11.0 * z).max(8.0);
        let font_node = (16.0 * z).max(9.0);
        let margin = (10.0 * z).max(4.0);

        ui.heading(RichText::new("Binary Tree Node Graph Hierarchy").color(p.cyan).size(font_title));
        ui.add_space(8.0 * z);

        ui.group(|ui| {
            ui.label(RichText::new("BINARY TREE LEVEL-ORDER NODES").font(egui::FontId::monospace(font_label)).color(p.text_muted));
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

                    egui::Frame::none().fill(fill).rounding(Rounding::same(8.0 * z)).stroke(Stroke::new(1.0_f32 * z, p.cell_border)).inner_margin(margin).show(ui, |ui| {
                        ui.vertical(|ui| {
                            let label = if is_active { "Active" } else if is_sec { "Child" } else { "" };
                            ui.label(RichText::new(format!("i={} {}", i, label)).font(egui::FontId::proportional((10.0 * z).max(8.0))).color(Color32::WHITE));
                            let val_str = match node_opt {
                                Some(v) => format!("[ {} ]", v),
                                None => "null".to_string(),
                            };
                            ui.label(RichText::new(val_str).font(egui::FontId::monospace(font_node)).strong().color(Color32::WHITE));
                        });
                    });
                }
            });
        });

        ui.add_space(20.0 * z);

        ui.horizontal(|ui| {
            if let Some(d) = depth_val {
                egui::Frame::none().fill(p.cell_bg).rounding(Rounding::same(8.0 * z)).stroke(Stroke::new(1.0_f32 * z, p.cyan)).inner_margin((12.0 * z).max(5.0)).show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new("Current / Max Tree Depth").font(egui::FontId::proportional((11.0 * z).max(8.0))).color(p.text_muted));
                        ui.label(RichText::new(format!("Depth: {}", d)).font(egui::FontId::monospace((18.0 * z).max(10.0))).strong().color(p.cyan));
                    });
                });
            }

            if let Some(diam) = max_diameter {
                ui.add_space(16.0 * z);
                egui::Frame::none().fill(p.cell_bg).rounding(Rounding::same(8.0 * z)).stroke(Stroke::new(1.0_f32 * z, p.emerald_text)).inner_margin((12.0 * z).max(5.0)).show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new("Maximum Tree Diameter (Edges Path)").font(egui::FontId::proportional((11.0 * z).max(8.0))).color(p.text_muted));
                        ui.label(RichText::new(format!("Diameter: {}", diam)).font(egui::FontId::monospace((18.0 * z).max(10.0))).strong().color(p.emerald_text));
                    });
                });
            }
        });
    }

    fn render_stock(&self, ui: &mut egui::Ui, p: &ThemePalette, prices: &[i32], left_buy: usize, right_sell: usize, current_profit: i32, max_profit: i32) {
        let z = self.canvas_zoom;
        let font_title = (16.0 * z).max(10.0);
        let font_label = (11.0 * z).max(8.0);
        let font_price = (16.0 * z).max(9.0);
        let margin = (10.0 * z).max(4.0);

        ui.heading(RichText::new("Sliding Window / Buy & Sell Stock Trace").color(p.cyan).size(font_title));
        ui.add_space(8.0 * z);

        ui.group(|ui| {
            ui.label(RichText::new("STOCK PRICES ARRAY (Days 0..N-1)").font(egui::FontId::monospace(font_label)).color(p.text_muted));
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

                    egui::Frame::none().fill(fill).rounding(Rounding::same(8.0 * z)).stroke(Stroke::new(1.0_f32 * z, p.cell_border)).inner_margin(margin).show(ui, |ui| {
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
                            ui.label(RichText::new(format!("day {} {}", i, label)).font(egui::FontId::proportional((10.0 * z).max(8.0))).color(p.text_muted));
                            ui.label(RichText::new(format!("${}", price)).font(egui::FontId::monospace(font_price)).strong().color(Color32::WHITE));
                        });
                    });
                }
            });
        });

        ui.add_space(20.0 * z);

        ui.horizontal(|ui| {
            egui::Frame::none().fill(p.cell_bg).rounding(Rounding::same(8.0 * z)).stroke(Stroke::new(1.0_f32 * z, p.pink)).inner_margin((12.0 * z).max(5.0)).show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.label(RichText::new("Current Profit (prices[r] - prices[l])").font(egui::FontId::proportional((11.0 * z).max(8.0))).color(p.text_muted));
                    ui.label(RichText::new(format!("${}", current_profit)).font(egui::FontId::monospace((18.0 * z).max(10.0))).strong().color(p.pink));
                });
            });

            ui.add_space(16.0 * z);

            egui::Frame::none().fill(p.cell_bg).rounding(Rounding::same(8.0 * z)).stroke(Stroke::new(1.0_f32 * z, p.emerald_text)).inner_margin((12.0 * z).max(5.0)).show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.label(RichText::new("Maximum Achieved Profit (maxP)").font(egui::FontId::proportional((11.0 * z).max(8.0))).color(p.text_muted));
                    ui.label(RichText::new(format!("${}", max_profit)).font(egui::FontId::monospace((18.0 * z).max(10.0))).strong().color(p.emerald_text));
                });
            });
        });
    }

    fn render_binary_search(&self, ui: &mut egui::Ui, p: &ThemePalette, nums: &[i32], target: i32, left: usize, right: usize, mid: Option<usize>, found_idx: Option<usize>) {
        let z = self.canvas_zoom;
        let margin = (12.0 * z).max(4.0);
        let font_sz = (18.0 * z).max(9.0);
        let font_title = (18.0 * z).max(10.0);

        ui.heading(RichText::new(format!("Binary Search bounds (l={}, r={}) | Target = {}", left, right, target)).color(p.cyan).size(font_title));
        ui.add_space(10.0 * z);

        ui.group(|ui| {
            ui.label(RichText::new("SORTED ARRAY").font(egui::FontId::monospace((11.0 * z).max(8.0))).color(p.text_muted));
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

                    egui::Frame::none().fill(fill).rounding(Rounding::same(8.0 * z)).stroke(Stroke::new(1.0_f32 * z, p.cell_border)).inner_margin(margin).show(ui, |ui| {
                        ui.vertical(|ui| {
                            let mut ptr_label = String::new();
                            if i == left { ptr_label.push_str("L "); }
                            if is_mid { ptr_label.push_str("MID "); }
                            if i == right { ptr_label.push_str("R"); }

                            ui.label(RichText::new(format!("i={} {}", i, ptr_label)).font(egui::FontId::proportional((11.0 * z).max(8.0))).color(Color32::WHITE));
                            ui.label(RichText::new(num.to_string()).font(egui::FontId::monospace(font_sz)).strong().color(Color32::WHITE));
                        });
                    });
                }
            });
        });

        if let Some(f) = found_idx {
            ui.add_space(20.0 * z);
            ui.heading(RichText::new(format!("Target {} Found at Index {}!", target, f)).color(p.emerald_text).size((18.0 * z).max(11.0)));
        }
    }

    fn render_linked_list(&self, ui: &mut egui::Ui, p: &ThemePalette, nodes: &[i32], prev_idx: Option<usize>, curr_idx: Option<usize>, next_idx: Option<usize>, reversed_so_far: &[i32]) {
        let z = self.canvas_zoom;
        let font_title = (16.0 * z).max(10.0);
        let font_label = (11.0 * z).max(8.0);
        let font_node = (16.0 * z).max(9.0);
        let margin = (10.0 * z).max(4.0);

        ui.heading(RichText::new("Singly-Linked List Pointer Reversal").color(p.cyan).size(font_title));
        ui.add_space(8.0 * z);

        ui.group(|ui| {
            ui.label(RichText::new("ORIGINAL LINKED LIST NODES").font(egui::FontId::monospace(font_label)).color(p.text_muted));
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

                    egui::Frame::none().fill(fill).rounding(Rounding::same(8.0 * z)).stroke(Stroke::new(1.0_f32 * z, p.cell_border)).inner_margin(margin).show(ui, |ui| {
                        ui.vertical(|ui| {
                            let mut label = String::new();
                            if is_prev { label.push_str("prev "); }
                            if is_curr { label.push_str("curr "); }
                            if is_nxt { label.push_str("nxt "); }

                            ui.label(RichText::new(format!("idx {} {}", i, label)).font(egui::FontId::proportional((10.0 * z).max(8.0))).color(Color32::WHITE));
                            ui.label(RichText::new(format!("( {} ) ->", val)).font(egui::FontId::monospace(font_node)).strong().color(Color32::WHITE));
                        });
                    });
                }
                ui.label(RichText::new("None").font(egui::FontId::monospace((14.0 * z).max(9.0))).color(p.text_dim));
            });
        });

        ui.add_space(20.0 * z);

        ui.group(|ui| {
            ui.label(RichText::new("REVERSED LINKED LIST (Constructed from head)").font(egui::FontId::monospace(font_label)).color(p.emerald_text));
            ui.horizontal(|ui| {
                if reversed_so_far.is_empty() {
                    ui.label(RichText::new("None").font(egui::FontId::monospace((14.0 * z).max(9.0))).color(p.text_dim));
                } else {
                    for (i, &val) in reversed_so_far.iter().enumerate() {
                        let fill = if i == 0 { p.emerald } else { p.cell_bg };
                        egui::Frame::none().fill(fill).rounding(Rounding::same(8.0 * z)).stroke(Stroke::new(1.0_f32 * z, p.emerald_text)).inner_margin(margin).show(ui, |ui| {
                            ui.label(RichText::new(format!("( {} ) ->", val)).font(egui::FontId::monospace(font_node)).strong().color(Color32::WHITE));
                        });
                    }
                    ui.label(RichText::new("None").font(egui::FontId::monospace((14.0 * z).max(9.0))).color(p.text_dim));
                }
            });
        });
    }


    fn render_two_sum(&self, ui: &mut egui::Ui, p: &ThemePalette, nums: &[i32], target: i32, active_idx: Option<usize>, secondary_idx: Option<usize>, map: &std::collections::BTreeMap<i32, usize>, found: Option<(usize, usize)>) {
        let z = self.canvas_zoom;
        let margin = (12.0 * z).max(4.0);
        let font_sz = (18.0 * z).max(9.0);
        let font_title = (18.0 * z).max(10.0);

        ui.heading(RichText::new(format!("Target Sum: {}", target)).color(p.cyan).size(font_title));
        ui.add_space(10.0 * z);

        ui.group(|ui| {
            ui.label(RichText::new("NUMS ARRAY").font(egui::FontId::monospace((11.0 * z).max(8.0))).color(p.text_muted));
            ui.add_space(4.0 * z);
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center).with_main_align(egui::Align::Center), |ui| {
                for (i, &num) in nums.iter().enumerate() {
                    let is_found = found.map_or(false, |(a, b)| a == i || b == i);
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

                    egui::Frame::none().fill(fill).rounding(Rounding::same(8.0 * z)).stroke(Stroke::new(1.0_f32 * z, p.cell_border)).inner_margin(margin).show(ui, |ui| {
                        ui.vertical(|ui| {
                            let label = if is_primary { "i" } else if is_sec { "j" } else { "" };
                            ui.label(RichText::new(format!("i={} {}", i, label)).font(egui::FontId::proportional((11.0 * z).max(8.0))).color(label_color));
                            ui.label(RichText::new(num.to_string()).font(egui::FontId::monospace(font_sz)).strong().color(val_color));
                        });
                    });
                }
            });
        });

        ui.add_space(20.0 * z);

        if self.selected_approach_id == 0 {
            ui.group(|ui| {
                ui.label(RichText::new("PREVMAP {value -> index}").font(egui::FontId::monospace((11.0 * z).max(8.0))).color(p.text_muted));
                ui.add_space(4.0 * z);
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center).with_main_align(egui::Align::Center), |ui| {
                    if map.is_empty() {
                        ui.label(RichText::new("Empty {}").italics().color(p.text_dim));
                    } else {
                        for (&val, &idx) in map {
                            egui::Frame::none().fill(p.cell_bg).rounding(Rounding::same(8.0 * z)).stroke(Stroke::new(1.0_f32 * z, p.purple)).inner_margin(margin).show(ui, |ui| {
                                ui.vertical(|ui| {
                                    ui.label(RichText::new(format!("val={}", val)).font(egui::FontId::monospace((14.0 * z).max(9.0))).strong().color(p.cyan));
                                    ui.label(RichText::new(format!("idx={}", idx)).font(egui::FontId::monospace((12.0 * z).max(8.0))).color(p.text_muted));
                                });
                            });
                        }
                    }
                });
            });
        }

        if let Some((a, b)) = found {
            ui.add_space(20.0 * z);
            ui.heading(RichText::new(format!("Result Pair Found! Indices: [{}, {}]", a, b)).color(p.emerald_text).size((18.0 * z).max(11.0)));
        }
    }

    fn render_valid_anagram(&self, ui: &mut egui::Ui, p: &ThemePalette, s: &str, t: &str, s_counts: &[usize; 26], t_counts: &[usize; 26], active_s: Option<usize>, active_t: Option<usize>, is_anagram: Option<bool>) {
        let z = self.canvas_zoom;
        let font_title = (16.0 * z).max(10.0);
        let font_label = (12.0 * z).max(8.0);
        let font_char = (16.0 * z).max(9.0);
        let margin = (8.0 * z).max(4.0);

        ui.heading(RichText::new("Character Comparison").color(p.cyan).size(font_title));
        ui.add_space(8.0 * z);

        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.label(RichText::new(format!("STRING s: \"{}\"", s)).font(egui::FontId::monospace(font_label)).color(p.text_muted));
                ui.horizontal(|ui| {
                    for (i, c) in s.chars().enumerate() {
                        let fill = if active_s == Some(i) { p.amber } else { p.cell_bg };
                        egui::Frame::none().fill(fill).rounding(Rounding::same(6.0 * z)).inner_margin(margin).show(ui, |ui| {
                            ui.label(RichText::new(c.to_string()).font(egui::FontId::monospace(font_char)).strong().color(Color32::WHITE));
                        });
                    }
                });
            });

            ui.add_space(20.0 * z);

            ui.group(|ui| {
                ui.label(RichText::new(format!("STRING t: \"{}\"", t)).font(egui::FontId::monospace(font_label)).color(p.text_muted));
                ui.horizontal(|ui| {
                    for (i, c) in t.chars().enumerate() {
                        let fill = if active_t == Some(i) { p.pink } else { p.cell_bg };
                        egui::Frame::none().fill(fill).rounding(Rounding::same(6.0 * z)).inner_margin(margin).show(ui, |ui| {
                            ui.label(RichText::new(c.to_string()).font(egui::FontId::monospace(font_char)).strong().color(Color32::WHITE));
                        });
                    }
                });
            });
        });

        ui.add_space(20.0 * z);

        if self.selected_approach_id == 0 {
            ui.group(|ui| {
                ui.label(RichText::new("ALPHABET FREQUENCY LOG").font(egui::FontId::monospace((11.0 * z).max(8.0))).color(p.text_muted));
                ui.horizontal_wrapped(|ui| {
                    for i in 0..26 {
                        let ch = (b'a' + i as u8) as char;
                        if s_counts[i] > 0 || t_counts[i] > 0 {
                            let match_color = if s_counts[i] == t_counts[i] { p.emerald_text } else { p.red };
                            egui::Frame::none().fill(p.cell_bg).rounding(Rounding::same(6.0 * z)).stroke(Stroke::new(1.0_f32 * z, match_color)).inner_margin((6.0 * z).max(3.0)).show(ui, |ui| {
                                ui.vertical(|ui| {
                                    ui.label(RichText::new(ch.to_string()).font(egui::FontId::monospace((14.0 * z).max(9.0))).strong().color(p.cyan));
                                    ui.label(RichText::new(format!("s:{}", s_counts[i])).font(egui::FontId::monospace((11.0 * z).max(8.0))).color(p.text_muted));
                                    ui.label(RichText::new(format!("t:{}", t_counts[i])).font(egui::FontId::monospace((11.0 * z).max(8.0))).color(p.text_muted));
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
                ui.heading(RichText::new("Valid Anagram!").color(p.emerald_text).size((18.0 * z).max(11.0)));
            } else {
                ui.heading(RichText::new("Not an Anagram").color(p.red).size((18.0 * z).max(11.0)));
            }
        }
    }

    fn render_two_pointers(&self, ui: &mut egui::Ui, p: &ThemePalette, chars: &[char], left: usize, right: usize, is_valid: Option<bool>, skipped: bool) {
        let z = self.canvas_zoom;
        let font_title = (16.0 * z).max(10.0);
        let font_char = (16.0 * z).max(9.0);
        let margin = (8.0 * z).max(4.0);

        ui.heading(RichText::new("Two Pointers Convergence").color(p.cyan).size(font_title));
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

                egui::Frame::none().fill(fill).rounding(Rounding::same(6.0 * z)).stroke(Stroke::new(1.0_f32 * z, p.cell_border)).inner_margin(margin).show(ui, |ui| {
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
                        ui.label(RichText::new(ptr_label).font(egui::FontId::monospace((10.0 * z).max(8.0))).strong().color(Color32::WHITE));
                        ui.label(RichText::new(c.to_string()).font(egui::FontId::monospace(font_char)).strong().color(Color32::WHITE));
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
                        ui.heading(RichText::new("Valid Palindrome!").color(p.emerald_text).size(heading_sz));
                    } else {
                        ui.heading(RichText::new("Invalid Palindrome Mismatch").color(p.red).size(heading_sz));
                    }
                }
                Problem::TwoSumII => {
                    if valid {
                        ui.heading(RichText::new("Target Sum Pair Found!").color(p.emerald_text).size(heading_sz));
                    } else {
                        ui.heading(RichText::new("No Pair Sum Equals Target").color(p.red).size(heading_sz));
                    }
                }
                Problem::ThreeSum => {
                    if valid {
                        ui.heading(RichText::new("3Sum Triplets Search Complete!").color(p.emerald_text).size(heading_sz));
                    } else {
                        ui.heading(RichText::new("No Triplets Sum to 0").color(p.red).size(heading_sz));
                    }
                }
                Problem::ContainerWater => {
                    ui.heading(RichText::new("Maximum Water Container Area Computed!").color(p.emerald_text).size(heading_sz));
                }
                Problem::TrappingRain => {
                    ui.heading(RichText::new("Trapped Rain Water Traversal Complete!").color(p.emerald_text).size(heading_sz));
                }
                Problem::LongestSubstring => {
                    ui.heading(RichText::new("Longest Substring Without Repeating Characters Found!").color(p.emerald_text).size(heading_sz));
                }
                Problem::CharacterReplacement => {
                    ui.heading(RichText::new("Longest Repeating Character Replacement Window Found!").color(p.emerald_text).size(heading_sz));
                }
                Problem::PermutationInString => {
                    if valid {
                        ui.heading(RichText::new("Permutation of s1 Found in s2!").color(p.emerald_text).size(heading_sz));
                    } else {
                        ui.heading(RichText::new("No Permutation of s1 Found in s2").color(p.red).size(heading_sz));
                    }
                }
                Problem::MinWindowSubstring => {
                    if valid {
                        ui.heading(RichText::new("Minimum Window Substring Found!").color(p.emerald_text).size(heading_sz));
                    } else {
                        ui.heading(RichText::new("No Valid Window Substring Found").color(p.red).size(heading_sz));
                    }
                }
                Problem::SlidingWindowMax => {
                    ui.heading(RichText::new("Sliding Window Maximum Evaluation Complete!").color(p.emerald_text).size(heading_sz));
                }
                _ => {}
            }
        }
    }




    fn render_stack(&self, ui: &mut egui::Ui, p: &ThemePalette, chars: &[char], active_idx: Option<usize>, stack: &[char], is_valid: Option<bool>) {
        let z = self.canvas_zoom;
        let font_title = (16.0 * z).max(10.0);
        let font_char = (16.0 * z).max(9.0);
        let margin = (8.0 * z).max(4.0);

        ui.heading(RichText::new("Vertical Stack Push / Pop Trace").color(p.cyan).size(font_title));
        ui.add_space(8.0 * z);

        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.label(RichText::new("EXPRESSION").font(egui::FontId::monospace((11.0 * z).max(8.0))).color(p.text_muted));
                ui.horizontal(|ui| {
                    for (i, &c) in chars.iter().enumerate() {
                        let fill = if active_idx == Some(i) { p.amber } else { p.cell_bg };
                        egui::Frame::none().fill(fill).rounding(Rounding::same(6.0 * z)).inner_margin(margin).show(ui, |ui| {
                            ui.label(RichText::new(c.to_string()).font(egui::FontId::monospace(font_char)).strong().color(Color32::WHITE));
                        });
                    }
                });
            });

            ui.add_space(30.0 * z);

            ui.group(|ui| {
                ui.label(RichText::new("STACK (Top on right/bottom)").font(egui::FontId::monospace((11.0 * z).max(8.0))).color(p.text_muted));
                ui.vertical(|ui| {
                    if stack.is_empty() {
                        ui.label(RichText::new("Stack is Empty []").italics().color(p.text_dim));
                    } else {
                        for (idx, &c) in stack.iter().rev().enumerate() {
                            let is_top = idx == 0;
                            let fill = if is_top { p.purple } else { p.cell_bg };
                            egui::Frame::none().fill(fill).rounding(Rounding::same(6.0 * z)).stroke(Stroke::new(1.0_f32 * z, p.cyan)).inner_margin(margin).show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    if is_top {
                                        ui.label(RichText::new("TOP ->").font(egui::FontId::monospace((10.0 * z).max(8.0))).color(p.amber));
                                    }
                                    ui.label(RichText::new(c.to_string()).font(egui::FontId::monospace(font_char)).strong().color(Color32::WHITE));
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
                ui.heading(RichText::new("Valid Parentheses Expression!").color(p.emerald_text).size((18.0 * z).max(11.0)));
            } else {
                ui.heading(RichText::new("Invalid Parentheses Expression").color(p.red).size((18.0 * z).max(11.0)));
            }
        }
    }

    fn render_topk(&self, ui: &mut egui::Ui, p: &ThemePalette, nums: &[i32], active_nums_idx: Option<usize>, count_map: &std::collections::BTreeMap<i32, usize>, buckets: &[Vec<i32>], active_bucket_idx: Option<usize>, result: &[i32]) {
        let z = self.canvas_zoom;
        let font_title = (16.0 * z).max(10.0);
        let font_num = (16.0 * z).max(9.0);
        let margin = (10.0 * z).max(4.0);

        ui.heading(RichText::new("1. Input Array & Frequency Map").color(p.cyan).size(font_title));
        ui.add_space(8.0 * z);
        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.label(RichText::new("NUMS ARRAY").font(egui::FontId::monospace((11.0 * z).max(8.0))).color(p.text_muted));
                ui.horizontal(|ui| {
                    for (idx, &val) in nums.iter().enumerate() {
                        let fill = if active_nums_idx == Some(idx) { p.amber } else { p.cell_bg };
                        egui::Frame::none().fill(fill).rounding(Rounding::same(8.0 * z)).stroke(Stroke::new(1.0_f32 * z, p.cell_border)).inner_margin(margin).show(ui, |ui| {
                            ui.label(RichText::new(val.to_string()).font(egui::FontId::monospace(font_num)).strong().color(Color32::WHITE));
                        });
                    }
                });
            });
            ui.add_space(20.0 * z);
            ui.group(|ui| {
                ui.label(RichText::new("COUNT MAP {num: frequency}").font(egui::FontId::monospace((11.0 * z).max(8.0))).color(p.text_muted));
                ui.horizontal(|ui| {
                    if count_map.is_empty() {
                        ui.label(RichText::new("Empty {}").italics().color(p.text_dim));
                    } else {
                        for (&num, &cnt) in count_map.iter() {
                            egui::Frame::none().fill(p.cell_bg).rounding(Rounding::same(8.0 * z)).stroke(Stroke::new(1.0_f32 * z, p.purple)).inner_margin((8.0 * z).max(4.0)).show(ui, |ui| {
                                ui.vertical(|ui| {
                                    ui.label(RichText::new(format!("num: {}", num)).font(egui::FontId::proportional((12.0 * z).max(8.0))).color(p.text_primary));
                                    ui.label(RichText::new(format!("{}", cnt)).font(egui::FontId::monospace(font_num)).strong().color(p.purple));
                                });
                            });
                        }
                    }
                });
            });
        });

        ui.add_space(24.0 * z);

        ui.heading(RichText::new("2. Frequency Buckets (Index = Count)").color(p.purple).size(font_title));
        ui.add_space(8.0 * z);
        ui.horizontal(|ui| {
            for (idx, items) in buckets.iter().enumerate() {
                let is_active = active_bucket_idx == Some(idx);
                let fill = if is_active { p.pink } else { p.sidebar_bg };
                egui::Frame::none().fill(fill).rounding(Rounding::same(10.0 * z)).stroke(Stroke::new(1.0_f32 * z, if is_active { Color32::WHITE } else { p.cell_border })).inner_margin((12.0 * z).max(5.0)).show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new(format!("freq[{}]", idx)).font(egui::FontId::monospace((12.0 * z).max(8.0))).strong().color(p.text_muted));
                        ui.separator();
                        if items.is_empty() {
                            ui.label(RichText::new("—").color(p.text_dim));
                        } else {
                            for &item in items {
                                egui::Frame::none().fill(p.cyan).rounding(Rounding::same(6.0 * z)).inner_margin((6.0 * z).max(3.0)).show(ui, |ui| {
                                    ui.label(RichText::new(item.to_string()).font(egui::FontId::monospace((14.0 * z).max(9.0))).strong().color(Color32::BLACK));
                                });
                            }
                        }
                    });
                });
            }
        });

        ui.add_space(24.0 * z);

        ui.heading(RichText::new(format!("3. Result Collector (Target k = {})", self.topk_k)).color(p.emerald_text).size(font_title));
        ui.add_space(8.0 * z);
        ui.horizontal(|ui| {
            if result.is_empty() {
                ui.label(RichText::new("Result array is empty...").italics().color(p.text_dim));
            } else {
                for &val in result {
                    egui::Frame::none().fill(p.emerald).rounding(Rounding::same(10.0 * z)).inner_margin((12.0 * z).max(5.0)).show(ui, |ui| {
                        ui.label(RichText::new(val.to_string()).font(egui::FontId::monospace((18.0 * z).max(10.0))).strong().color(Color32::WHITE));
                    });
                }
            }
        });
    }

    fn render_encode_decode(&self, ui: &mut egui::Ui, p: &ThemePalette, input_strs: &[String], encoded_so_far: &str, decoded_so_far: &[String], pointer: usize, active_str_idx: Option<usize>, phase: &EncodeDecodePhase) {
        let z = self.canvas_zoom;
        let font_title = (16.0 * z).max(10.0);
        let font_sz = (14.0 * z).max(9.0);
        let margin = (10.0 * z).max(4.0);

        ui.heading(RichText::new("1. Input Strings").color(p.cyan).size(font_title));
        ui.add_space(8.0 * z);
        ui.horizontal(|ui| {
            for (idx, s) in input_strs.iter().enumerate() {
                let is_active = active_str_idx == Some(idx);
                let fill = if is_active { p.amber } else { p.cell_bg };
                egui::Frame::none().fill(fill).rounding(Rounding::same(8.0 * z)).stroke(Stroke::new(1.0_f32 * z, p.cell_border)).inner_margin(margin).show(ui, |ui| {
                    ui.label(RichText::new(format!("\"{}\"", s)).font(egui::FontId::monospace(font_sz)).strong().color(Color32::WHITE));
                });
            }
        });

        ui.add_space(24.0 * z);

        ui.heading(RichText::new("2. Encoded String").color(p.purple).size(font_title));
        ui.add_space(8.0 * z);
        if encoded_so_far.is_empty() {
            ui.label(RichText::new("\"\" (empty)").italics().color(p.text_dim));
        } else {
            ui.horizontal_wrapped(|ui| {
                for (i, ch) in encoded_so_far.chars().enumerate() {
                    let is_ptr = *phase == EncodeDecodePhase::Decoding && i == pointer;
                    let fill = if is_ptr { p.pink } else if ch == '#' { p.purple } else { p.cell_bg };
                    egui::Frame::none().fill(fill).rounding(Rounding::same(4.0 * z)).inner_margin((6.0 * z).max(3.0)).show(ui, |ui| {
                        ui.label(RichText::new(ch.to_string()).font(egui::FontId::monospace(font_sz)).strong().color(Color32::WHITE));
                    });
                }
            });
        }

        ui.add_space(24.0 * z);

        ui.heading(RichText::new("3. Decoded Strings").color(p.emerald_text).size(font_title));
        ui.add_space(8.0 * z);
        if decoded_so_far.is_empty() {
            ui.label(RichText::new("Decoded list is empty...").italics().color(p.text_dim));
        } else {
            ui.horizontal(|ui| {
                for s in decoded_so_far {
                    egui::Frame::none().fill(p.emerald).rounding(Rounding::same(10.0 * z)).inner_margin((12.0 * z).max(5.0)).show(ui, |ui| {
                        ui.label(RichText::new(format!("\"{}\"", s)).font(egui::FontId::monospace(font_sz)).strong().color(Color32::WHITE));
                    });
                }
            });
        }
    }

    fn render_product(&self, ui: &mut egui::Ui, p: &ThemePalette, nums: &[i32], output: &[i64], active_idx: Option<usize>, prefix_val: i64, suffix_val: i64, phase: &ProductPhase) {
        let z = self.canvas_zoom;
        let font_title = (16.0 * z).max(10.0);
        let font_num = (16.0 * z).max(9.0);
        let margin = (10.0 * z).max(4.0);

        ui.heading(RichText::new("1. Input Array (nums)").color(p.cyan).size(font_title));
        ui.add_space(8.0 * z);
        ui.horizontal(|ui| {
            for (idx, &val) in nums.iter().enumerate() {
                let is_active = active_idx == Some(idx);
                let fill = if is_active { p.amber } else { p.cell_bg };
                egui::Frame::none().fill(fill).rounding(Rounding::same(8.0 * z)).stroke(Stroke::new(1.0_f32 * z, p.cell_border)).inner_margin(margin).show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new(format!("i={}", idx)).font(egui::FontId::proportional((10.0 * z).max(8.0))).color(p.text_muted));
                        ui.label(RichText::new(val.to_string()).font(egui::FontId::monospace(font_num)).strong().color(Color32::WHITE));
                    });
                });
            }
        });

        ui.add_space(24.0 * z);

        ui.heading(RichText::new("2. Running Prefix / Suffix Values").color(p.purple).size(font_title));
        ui.add_space(8.0 * z);
        ui.horizontal(|ui| {
            egui::Frame::none().fill(p.cell_bg).rounding(Rounding::same(8.0 * z)).stroke(Stroke::new(1.0_f32 * z, p.cyan)).inner_margin((12.0 * z).max(5.0)).show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.label(RichText::new("prefix").font(egui::FontId::monospace((12.0 * z).max(8.0))).color(p.text_muted));
                    ui.label(RichText::new(prefix_val.to_string()).font(egui::FontId::monospace((18.0 * z).max(10.0))).strong().color(p.cyan));
                });
            });
            ui.add_space(16.0 * z);
            egui::Frame::none().fill(p.cell_bg).rounding(Rounding::same(8.0 * z)).stroke(Stroke::new(1.0_f32 * z, p.pink)).inner_margin((12.0 * z).max(5.0)).show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.label(RichText::new("suffix").font(egui::FontId::monospace((12.0 * z).max(8.0))).color(p.text_muted));
                    ui.label(RichText::new(suffix_val.to_string()).font(egui::FontId::monospace((18.0 * z).max(10.0))).strong().color(p.pink));
                });
            });
            ui.add_space(16.0 * z);
            let phase_label = match phase {
                ProductPhase::Init => "Initializing",
                ProductPhase::PrefixPass => "Prefix Pass (left to right)",
                ProductPhase::SuffixPass => "Suffix Pass (right to left)",
                ProductPhase::Complete => "Complete",
            };
            ui.label(RichText::new(format!("Phase: {}", phase_label)).font(egui::FontId::proportional((14.0 * z).max(9.0))).strong().color(p.text_primary));
        });

        ui.add_space(24.0 * z);

        ui.heading(RichText::new("3. Output Array").color(p.emerald_text).size(font_title));
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
                egui::Frame::none().fill(fill).rounding(Rounding::same(8.0 * z)).stroke(Stroke::new(1.0_f32 * z, p.emerald_text)).inner_margin((10.0 * z).max(4.0)).show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new(format!("o[{}]", idx)).font(egui::FontId::proportional((10.0 * z).max(8.0))).color(p.text_muted));
                        ui.label(RichText::new(val.to_string()).font(egui::FontId::monospace(font_num)).strong().color(Color32::WHITE));
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

        ui.heading(RichText::new("🌲 Trie (Prefix Tree) Character Node Hierarchy").color(p.cyan).size(font_title));
        ui.add_space(8.0 * z);

        ui.group(|ui| {
            ui.label(RichText::new("TRIE CHARACTER NODE PATHS").font(egui::FontId::monospace(font_label)).color(p.text_muted));
            ui.add_space(8.0 * z);

            ui.horizontal_wrapped(|ui| {
                // Render Root Node
                egui::Frame::none()
                    .fill(p.cyan)
                    .rounding(Rounding::same(20.0 * z))
                    .inner_margin(egui::Margin::symmetric(14.0 * z, 10.0 * z))
                    .show(ui, |ui| {
                        ui.label(RichText::new("ROOT (*)").font(egui::FontId::monospace(font_root)).strong().color(Color32::WHITE));
                    });

                ui.label(RichText::new(" ──► ").font(egui::FontId::monospace(16.0 * z)).color(p.cyan));

                // Render Sample Word Nodes dynamically
                let words: Vec<&str> = match self.current_problem {
                    Problem::ImplementTrie => self.trie_words_input.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect(),
                    Problem::WordDictionary => self.word_dict_words_input.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect(),
                    Problem::WordSearchII => self.word_search_ii_words_input.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect(),
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
                                ui.label(RichText::new(format!("Word #{}: \"{}\"", w_idx + 1, w)).font(egui::FontId::monospace(font_word_idx)).color(p.amber).strong());
                                ui.add_space(4.0 * z);
                                ui.horizontal(|ui| {
                                    for (c_idx, ch) in w.chars().enumerate() {
                                        let is_last = c_idx == w.len() - 1;
                                        let bg_color = if is_last { p.emerald } else { p.cell_bg };
                                        let text_color = if is_last { Color32::WHITE } else { p.text_primary };

                                        egui::Frame::none()
                                            .fill(bg_color)
                                            .rounding(Rounding::same(14.0 * z))
                                            .stroke(Stroke::new(1.0_f32 * z, p.cell_border))
                                            .inner_margin(egui::Margin::symmetric(8.0 * z, 4.0 * z))
                                            .show(ui, |ui| {
                                                if is_last {
                                                    ui.label(RichText::new(format!("'{}' ★", ch)).font(egui::FontId::monospace(font_char)).strong().color(text_color));
                                                } else {
                                                    ui.label(RichText::new(format!("'{}'", ch)).font(egui::FontId::monospace(font_char)).strong().color(text_color));
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

    fn render_custom_playground_bar(&mut self, ui: &mut egui::Ui, p: &ThemePalette) {
        egui::Frame::none()
            .fill(p.sidebar_bg)
            .rounding(Rounding::same(8.0))
            .stroke(Stroke::new(1.0_f32, p.amber))
            .inner_margin(egui::Margin::symmetric(14.0, 8.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("🎮 Custom Input Playground:").font(egui::FontId::proportional(12.0)).color(p.amber).strong());
                    ui.add_space(4.0);

                    let mut should_run = false;

                    match self.current_problem {
                        Problem::ContainsDuplicate => {
                            ui.label(RichText::new("nums =").font(egui::FontId::monospace(12.0)).color(p.text_muted));
                            if ui.add(egui::TextEdit::singleline(&mut self.contains_dup_nums_input).desired_width(160.0)).changed() { should_run = true; }
                            if ui.button("[1,2,3,1]").clicked() { self.contains_dup_nums_input = "1,2,3,1".into(); should_run = true; }
                            if ui.button("[1,2,3,4]").clicked() { self.contains_dup_nums_input = "1,2,3,4".into(); should_run = true; }
                        }
                        Problem::TwoSum => {
                            ui.label(RichText::new("nums =").font(egui::FontId::monospace(12.0)).color(p.text_muted));
                            if ui.add(egui::TextEdit::singleline(&mut self.two_sum_nums_input).desired_width(130.0)).changed() { should_run = true; }
                            ui.label(RichText::new("target =").font(egui::FontId::monospace(12.0)).color(p.text_muted));
                            if ui.add(egui::DragValue::new(&mut self.two_sum_target_input)).changed() { should_run = true; }
                            if ui.button("[2,7,11,15] t=9").clicked() { self.two_sum_nums_input = "2,7,11,15".into(); self.two_sum_target_input = 9; should_run = true; }
                        }
                        Problem::ValidAnagram => {
                            ui.label(RichText::new("s =").font(egui::FontId::monospace(12.0)).color(p.text_muted));
                            if ui.add(egui::TextEdit::singleline(&mut self.valid_anagram_s_input).desired_width(90.0)).changed() { should_run = true; }
                            ui.label(RichText::new("t =").font(egui::FontId::monospace(12.0)).color(p.text_muted));
                            if ui.add(egui::TextEdit::singleline(&mut self.valid_anagram_t_input).desired_width(90.0)).changed() { should_run = true; }
                            if ui.button("anagram / nagaram").clicked() { self.valid_anagram_s_input = "anagram".into(); self.valid_anagram_t_input = "nagaram".into(); should_run = true; }
                            if ui.button("rat / car").clicked() { self.valid_anagram_s_input = "rat".into(); self.valid_anagram_t_input = "car".into(); should_run = true; }
                        }
                        Problem::GroupAnagrams => {
                            ui.label(RichText::new("strs =").font(egui::FontId::monospace(12.0)).color(p.text_muted));
                            if ui.add(egui::TextEdit::singleline(&mut self.group_anagrams_input).desired_width(200.0)).changed() { should_run = true; }
                            if ui.button("eat, tea, tan, ate, nat, bat").clicked() { self.group_anagrams_input = "eat, tea, tan, ate, nat, bat".into(); should_run = true; }
                        }
                        Problem::TopKFrequent => {
                            ui.label(RichText::new("nums =").font(egui::FontId::monospace(12.0)).color(p.text_muted));
                            if ui.add(egui::TextEdit::singleline(&mut self.topk_nums_input).desired_width(120.0)).changed() { should_run = true; }
                            ui.label(RichText::new("k =").font(egui::FontId::monospace(12.0)).color(p.text_muted));
                            if ui.add(egui::DragValue::new(&mut self.topk_k_input).range(1..=10)).changed() { should_run = true; }
                            if ui.button("[1,1,1,2,2,3] k=2").clicked() { self.topk_nums_input = "1,1,1,2,2,3".into(); self.topk_k_input = 2; should_run = true; }
                        }
                        Problem::ProductExceptSelf => {
                            ui.label(RichText::new("nums =").font(egui::FontId::monospace(12.0)).color(p.text_muted));
                            if ui.add(egui::TextEdit::singleline(&mut self.prod_nums_input).desired_width(160.0)).changed() { should_run = true; }
                            if ui.button("[1,2,4,6]").clicked() { self.prod_nums_input = "1,2,4,6".into(); should_run = true; }
                        }
                        Problem::EncodeDecode => {
                            ui.label(RichText::new("strs =").font(egui::FontId::monospace(12.0)).color(p.text_muted));
                            if ui.add(egui::TextEdit::singleline(&mut self.ed_strs_input).desired_width(160.0)).changed() { should_run = true; }
                            if ui.button("Hello, World").clicked() { self.ed_strs_input = "Hello, World".into(); should_run = true; }
                        }
                        Problem::ValidSudoku => {
                            ui.label(RichText::new("Board Preset:").font(egui::FontId::monospace(12.0)).color(p.text_muted));
                            if ui.selectable_label(self.sudoku_preset_valid, "Valid Board Ex 1").clicked() { self.sudoku_preset_valid = true; should_run = true; }
                            if ui.selectable_label(!self.sudoku_preset_valid, "Invalid Board Ex 2").clicked() { self.sudoku_preset_valid = false; should_run = true; }
                        }
                        Problem::LongestConsecutive => {
                            ui.label(RichText::new("nums =").font(egui::FontId::monospace(12.0)).color(p.text_muted));
                            if ui.add(egui::TextEdit::singleline(&mut self.longest_consecutive_nums_input).desired_width(180.0)).changed() { should_run = true; }
                            if ui.button("[2,20,4,10,3,4,5]").clicked() { self.longest_consecutive_nums_input = "2,20,4,10,3,4,5".into(); should_run = true; }
                        }
                        Problem::ValidPalindrome => {
                            ui.label(RichText::new("s =").font(egui::FontId::monospace(12.0)).color(p.text_muted));
                            if ui.add(egui::TextEdit::singleline(&mut self.palindrome_s_input).desired_width(200.0)).changed() { should_run = true; }
                            if ui.button("Was it a car...").clicked() { self.palindrome_s_input = "Was it a car or a cat I saw?".into(); should_run = true; }
                        }
                        Problem::BestTimeStock => {
                            ui.label(RichText::new("prices =").font(egui::FontId::monospace(12.0)).color(p.text_muted));
                            if ui.add(egui::TextEdit::singleline(&mut self.stock_prices_input).desired_width(160.0)).changed() { should_run = true; }
                            if ui.button("[10,1,5,6,7,1]").clicked() { self.stock_prices_input = "10,1,5,6,7,1".into(); should_run = true; }
                        }
                        Problem::ValidParentheses => {
                            ui.label(RichText::new("s =").font(egui::FontId::monospace(12.0)).color(p.text_muted));
                            if ui.add(egui::TextEdit::singleline(&mut self.parentheses_s_input).desired_width(140.0)).changed() { should_run = true; }
                            if ui.button("([{}])").clicked() { self.parentheses_s_input = "([{}])".into(); should_run = true; }
                        }
                        Problem::BinarySearch => {
                            ui.label(RichText::new("nums =").font(egui::FontId::monospace(12.0)).color(p.text_muted));
                            if ui.add(egui::TextEdit::singleline(&mut self.binary_search_nums_input).desired_width(140.0)).changed() { should_run = true; }
                            ui.label(RichText::new("target =").font(egui::FontId::monospace(12.0)).color(p.text_muted));
                            if ui.add(egui::DragValue::new(&mut self.binary_search_target_input)).changed() { should_run = true; }
                            if ui.button("[-1,0,2,4,6,8] t=4").clicked() { self.binary_search_nums_input = "-1,0,2,4,6,8".into(); self.binary_search_target_input = 4; should_run = true; }
                        }
                        Problem::ReverseLinkedList => {
                            ui.label(RichText::new("nodes =").font(egui::FontId::monospace(12.0)).color(p.text_muted));
                            if ui.add(egui::TextEdit::singleline(&mut self.linked_list_nodes_input).desired_width(140.0)).changed() { should_run = true; }
                            if ui.button("[0,1,2,3]").clicked() { self.linked_list_nodes_input = "0,1,2,3".into(); should_run = true; }
                        }
                        Problem::MergeTwoLists => {
                            ui.label(RichText::new("list1 =").font(egui::FontId::monospace(12.0)).color(p.text_muted));
                            if ui.add(egui::TextEdit::singleline(&mut self.merge_list1_input).desired_width(100.0)).changed() { should_run = true; }
                            ui.label(RichText::new("list2 =").font(egui::FontId::monospace(12.0)).color(p.text_muted));
                            if ui.add(egui::TextEdit::singleline(&mut self.merge_list2_input).desired_width(100.0)).changed() { should_run = true; }
                            if ui.button("[1,2,4] & [1,3,5]").clicked() { self.merge_list1_input = "1,2,4".into(); self.merge_list2_input = "1,3,5".into(); should_run = true; }
                        }
                        Problem::LinkedListCycle => {
                            ui.label(RichText::new("nodes =").font(egui::FontId::monospace(12.0)).color(p.text_muted));
                            if ui.add(egui::TextEdit::singleline(&mut self.cycle_nodes_input).desired_width(120.0)).changed() { should_run = true; }
                            ui.label(RichText::new("cycle_idx =").font(egui::FontId::monospace(12.0)).color(p.text_muted));
                            if ui.add(egui::DragValue::new(&mut self.cycle_index_input).range(-1..=10)).changed() { should_run = true; }
                            if ui.button("[1,2,3,4] idx=1").clicked() { self.cycle_nodes_input = "1,2,3,4".into(); self.cycle_index_input = 1; should_run = true; }
                        }
                        Problem::InvertTree | Problem::MaxDepthTree | Problem::DiameterTree | Problem::BalancedTree | Problem::SameTree | Problem::Subtree => {
                            ui.label(RichText::new("tree nodes =").font(egui::FontId::monospace(12.0)).color(p.text_muted));
                            if ui.add(egui::TextEdit::singleline(&mut self.tree_nodes_input).desired_width(180.0)).changed() { should_run = true; }
                            if ui.button("[1,2,3,4,5,6,7]").clicked() { self.tree_nodes_input = "1,2,3,4,5,6,7".into(); should_run = true; }
                        }
                        Problem::TwoSumII => {
                            ui.label(RichText::new("nums =").font(egui::FontId::monospace(12.0)).color(p.text_muted));
                            if ui.add(egui::TextEdit::singleline(&mut self.two_pointer_nums_input).desired_width(140.0)).changed() { should_run = true; }
                            ui.label(RichText::new("target =").font(egui::FontId::monospace(12.0)).color(p.text_muted));
                            if ui.add(egui::DragValue::new(&mut self.two_pointer_target_input)).changed() { should_run = true; }
                            if ui.button("[2,7,11,15] t=9").clicked() { self.two_pointer_nums_input = "2,7,11,15".into(); self.two_pointer_target_input = 9; should_run = true; }
                        }
                        Problem::ThreeSum | Problem::ContainerWater | Problem::TrappingRain | Problem::HouseRobber => {
                            ui.label(RichText::new("nums =").font(egui::FontId::monospace(12.0)).color(p.text_muted));
                            if ui.add(egui::TextEdit::singleline(&mut self.two_pointer_nums_input).desired_width(180.0)).changed() { should_run = true; }
                            if ui.button("Default Preset").clicked() { self.two_pointer_nums_input = "-1,0,1,2,-1,-4".into(); should_run = true; }
                        }
                        Problem::SearchRotatedArray | Problem::FindMinRotated => {
                            ui.label(RichText::new("nums =").font(egui::FontId::monospace(12.0)).color(p.text_muted));
                            if ui.add(egui::TextEdit::singleline(&mut self.binary_search_nums_input).desired_width(140.0)).changed() { should_run = true; }
                            ui.label(RichText::new("target =").font(egui::FontId::monospace(12.0)).color(p.text_muted));
                            if ui.add(egui::DragValue::new(&mut self.binary_search_target_input)).changed() { should_run = true; }
                            if ui.button("[4,5,6,7,0,1,2] t=0").clicked() { self.binary_search_nums_input = "4,5,6,7,0,1,2".into(); self.binary_search_target_input = 0; should_run = true; }
                        }
                        Problem::ImplementTrie => {
                            ui.label(RichText::new("insert =").font(egui::FontId::monospace(12.0)).color(p.text_muted));
                            if ui.add(egui::TextEdit::singleline(&mut self.trie_words_input).desired_width(140.0)).changed() { should_run = true; }
                            ui.label(RichText::new("search =").font(egui::FontId::monospace(12.0)).color(p.text_muted));
                            if ui.add(egui::TextEdit::singleline(&mut self.trie_search_input).desired_width(80.0)).changed() { should_run = true; }
                            if ui.button("apple, app").clicked() { self.trie_words_input = "apple, app".into(); self.trie_search_input = "app".into(); should_run = true; }
                        }
                        Problem::WordDictionary => {
                            ui.label(RichText::new("words =").font(egui::FontId::monospace(12.0)).color(p.text_muted));
                            if ui.add(egui::TextEdit::singleline(&mut self.word_dict_words_input).desired_width(140.0)).changed() { should_run = true; }
                            ui.label(RichText::new("pattern =").font(egui::FontId::monospace(12.0)).color(p.text_muted));
                            if ui.add(egui::TextEdit::singleline(&mut self.word_dict_pattern_input).desired_width(80.0)).changed() { should_run = true; }
                        }
                        Problem::WordSearchII => {
                            ui.label(RichText::new("words =").font(egui::FontId::monospace(12.0)).color(p.text_muted));
                            if ui.add(egui::TextEdit::singleline(&mut self.word_search_ii_words_input).desired_width(200.0)).changed() { should_run = true; }
                        }
                        _ => {
                            ui.label(RichText::new("Default test dataset active.").font(egui::FontId::proportional(12.0)).color(p.text_muted));
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

fn render_complexity_card(ui: &mut egui::Ui, app_meta: &ApproachMeta, p: &ThemePalette) {
    egui::Frame::group(ui.style())
        .fill(p.step_box_bg)
        .rounding(Rounding::same(8.0))
        .inner_margin(10.0)
        .show(ui, |ui| {
            ui.label(RichText::new("📊 Algorithm Complexity Card").strong().color(p.cyan).size(12.0));
            ui.add_space(6.0);

            ui.horizontal(|ui| {
                let tc_color = if app_meta.time_complexity.contains("O(1)") || app_meta.time_complexity.contains("O(log") || app_meta.time_complexity == "O(N)" {
                    p.emerald_text
                } else if app_meta.time_complexity.contains("O(N log N)") || app_meta.time_complexity.contains("O(N * K)") {
                    p.amber
                } else {
                    p.red
                };

                egui::Frame::none()
                    .fill(tc_color.gamma_multiply(0.15))
                    .rounding(Rounding::same(4.0))
                    .inner_margin(egui::Margin::symmetric(8.0, 4.0))
                    .show(ui, |ui| {
                        ui.label(RichText::new(format!("⚡ Time: {}", app_meta.time_complexity)).font(egui::FontId::monospace(12.0)).color(tc_color).strong());
                    });

                ui.add_space(4.0);

                let sc_color = if app_meta.space_complexity.contains("O(1)") {
                    p.emerald_text
                } else if app_meta.space_complexity.contains("O(N)") || app_meta.space_complexity.contains("O(H)") {
                    p.cyan
                } else {
                    p.amber
                };

                egui::Frame::none()
                    .fill(sc_color.gamma_multiply(0.15))
                    .rounding(Rounding::same(4.0))
                    .inner_margin(egui::Margin::symmetric(8.0, 4.0))
                    .show(ui, |ui| {
                        ui.label(RichText::new(format!("💾 Space: {}", app_meta.space_complexity)).font(egui::FontId::monospace(12.0)).color(sc_color).strong());
                    });
            });

            if !app_meta.rationale.is_empty() {
                ui.add_space(8.0);
                ui.label(RichText::new(app_meta.rationale).font(egui::FontId::proportional(12.0)).color(p.text_primary));
            }
        });
}

fn render_variable_scope_table(ui: &mut egui::Ui, step: &Step, p: &ThemePalette) {
    let vars = step.visual.variables();
    if vars.is_empty() {
        return;
    }

    egui::Frame::group(ui.style())
        .fill(p.step_box_bg)
        .rounding(Rounding::same(8.0))
        .inner_margin(10.0)
        .show(ui, |ui| {
            ui.label(RichText::new("🔍 Variable Scope Table").strong().color(p.amber).size(12.0));
            ui.add_space(6.0);

            egui::Grid::new("var_scope_grid")
                .striped(true)
                .spacing([12.0, 4.0])
                .show(ui, |ui| {
                    ui.label(RichText::new("Variable").font(egui::FontId::monospace(11.0)).color(p.text_muted).strong());
                    ui.label(RichText::new("Value").font(egui::FontId::monospace(11.0)).color(p.text_muted).strong());
                    ui.end_row();

                    for (name, val) in vars {
                        ui.label(RichText::new(name).font(egui::FontId::monospace(12.0)).color(p.cyan).strong());
                        ui.label(RichText::new(val).font(egui::FontId::monospace(12.0)).color(p.emerald_text));
                        ui.end_row();
                    }
                });
        });
}

