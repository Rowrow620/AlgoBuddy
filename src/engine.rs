use crate::algorithms::{
    advanced_graphs::*, backtracking::*, best_time_stock::generate_best_time_stock_steps,
    binary_search::generate_binary_search_steps, bit_math::*,
    bucket_sort::generate_bucket_sort_steps, car_fleet::generate_car_fleet_steps,
    character_replacement::generate_character_replacement_steps,
    climbing_stairs::generate_climbing_stairs_steps,
    container_water::generate_container_water_steps,
    contains_duplicate::generate_contains_duplicate_steps,
    daily_temperatures::generate_daily_temperatures_steps, dp1d::*, dp2d::*,
    encode_decode::generate_encode_decode_steps, eval_rpn::generate_eval_rpn_steps,
    find_median_sorted_arrays::generate_find_median_sorted_arrays_steps,
    find_min_rotated::generate_find_min_rotated_steps,
    generate_parentheses::generate_parentheses_combinations_steps, graphs::*, greedy_intervals::*,
    group_anagrams::generate_group_anagrams_steps, happy_number::generate_happy_number_steps,
    heap::*, house_robber::generate_house_robber_steps,
    koko_bananas::generate_koko_eating_bananas_steps,
    kth_largest_stream::generate_kth_largest_stream_steps,
    largest_rectangle::generate_largest_rectangle_steps,
    length_of_longest_substring::generate_longest_substring_steps,
    linked_list_cycle::generate_linked_list_cycle_steps, linked_list_full::*,
    longest_consecutive::generate_longest_consecutive_steps,
    meeting_rooms::generate_meeting_rooms_steps, merge_two_lists::generate_merge_two_lists_steps,
    min_cost_stairs::generate_min_cost_stairs_steps, min_heap::generate_min_heap_steps,
    min_stack::generate_min_stack_steps, min_window_substring::generate_min_window_substring_steps,
    missing_number::generate_missing_number_steps,
    permutation_in_string::generate_permutation_in_string_steps, plus_one::generate_plus_one_steps,
    product_except_self::generate_product_steps, reverse_bits::generate_reverse_bits_steps,
    reverse_linked_list::generate_reverse_linked_list_steps,
    search_2d_matrix::generate_search_2d_matrix_steps,
    search_rotated_array::generate_search_rotated_array_steps,
    single_number::generate_single_number_steps,
    sliding_window_max::generate_sliding_window_max_steps, sorting::generate_sorting_steps,
    three_sum::generate_three_sum_steps, time_key_value_store::generate_time_key_value_store_steps,
    trapping_rain::generate_trapping_rain_steps, trees::*, trie::*,
    two_sum::generate_two_sum_steps, two_sum_ii::generate_two_sum_ii_steps,
    valid_anagram::generate_valid_anagram_steps, valid_palindrome::generate_valid_palindrome_steps,
    valid_parentheses::generate_valid_parentheses_steps, valid_sudoku::generate_valid_sudoku_steps,
};
use crate::app::VisualizerApp;
use crate::model::*;

pub(crate) fn recompute_steps(app: &mut VisualizerApp) {
    let app_id = app.selected_approach_id;
    app.steps = match app.current_problem {
        Problem::ContainsDuplicate => {
            let parsed: Vec<i32> = app
                .get_input_str(Problem::ContainsDuplicate, "nums", "1, 2, 3, 1")
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            let nums = if parsed.is_empty() {
                vec![1, 2, 3, 1]
            } else {
                parsed
            };
            generate_contains_duplicate_steps(&nums, app_id)
        }
        Problem::TwoSum => {
            let parsed: Vec<i32> = app
                .get_input_str(Problem::TwoSum, "nums", "2, 7, 11, 15")
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            let nums = if parsed.is_empty() {
                vec![2, 7, 11, 15]
            } else {
                parsed
            };
            let target = app.get_input_int(Problem::TwoSum, "target", 9);
            generate_two_sum_steps(&nums, target, app_id)
        }
        Problem::ValidAnagram => {
            let s = app.get_input_str(Problem::ValidAnagram, "s", "anagram");
            let t = app.get_input_str(Problem::ValidAnagram, "t", "nagaram");
            generate_valid_anagram_steps(s, t, app_id)
        }
        Problem::GroupAnagrams => {
            let strs: Vec<String> = app
                .get_input_str(
                    Problem::GroupAnagrams,
                    "strs",
                    "eat, tea, tan, ate, nat, bat",
                )
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            let input_strs = if strs.is_empty() {
                vec![
                    "eat".into(),
                    "tea".into(),
                    "tan".into(),
                    "ate".into(),
                    "nat".into(),
                    "bat".into(),
                ]
            } else {
                strs
            };
            generate_group_anagrams_steps(&input_strs, app_id)
        }
        Problem::TopKFrequent => {
            let parsed: Vec<i32> = app
                .get_input_str(Problem::TopKFrequent, "nums", "1, 1, 1, 2, 2, 3")
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            let nums = if parsed.is_empty() {
                vec![1, 1, 1, 2, 2, 3]
            } else {
                parsed
            };
            let unique = nums.iter().collect::<std::collections::HashSet<_>>().len();
            let k_val = app.get_input_int(Problem::TopKFrequent, "k", 2);
            let k = k_val.clamp(1, unique.max(1) as i32) as usize;
            app.set_input_int(Problem::TopKFrequent, "k", k as i32);

            match app_id {
                0 => generate_bucket_sort_steps(&nums, k),
                1 => generate_min_heap_steps(&nums, k),
                _ => generate_sorting_steps(&nums, k),
            }
        }
        Problem::ProductExceptSelf => {
            let parsed: Vec<i32> = app
                .get_input_str(Problem::ProductExceptSelf, "nums", "1, 2, 4, 6")
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            let nums = if parsed.is_empty() {
                vec![1, 2, 4, 6]
            } else {
                parsed
            };
            generate_product_steps(&nums)
        }
        Problem::EncodeDecode => {
            let parsed: Vec<String> = app
                .get_input_str(Problem::EncodeDecode, "strs", "Hello, World")
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            let strs = if parsed.is_empty() {
                vec!["Hello".into(), "World".into()]
            } else {
                parsed
            };
            generate_encode_decode_steps(&strs)
        }
        Problem::ValidSudoku => {
            let board = app.get_sudoku_board();
            generate_valid_sudoku_steps(&board)
        }
        Problem::LongestConsecutive => {
            let parsed: Vec<i32> = app
                .get_input_str(Problem::LongestConsecutive, "nums", "2, 20, 4, 10, 3, 4, 5")
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            let nums = if parsed.is_empty() {
                vec![2, 20, 4, 10, 3, 4, 5]
            } else {
                parsed
            };
            generate_longest_consecutive_steps(&nums)
        }
        Problem::ValidPalindrome => {
            let s = app.get_input_str(
                Problem::ValidPalindrome,
                "s",
                "Was it a car or a cat I saw?",
            );
            generate_valid_palindrome_steps(s, app_id)
        }
        Problem::BestTimeStock => {
            let parsed: Vec<i32> = app
                .get_input_str(Problem::BestTimeStock, "prices", "10, 1, 5, 6, 7, 1")
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            let prices = if parsed.is_empty() {
                vec![10, 1, 5, 6, 7, 1]
            } else {
                parsed
            };
            generate_best_time_stock_steps(&prices)
        }
        Problem::ValidParentheses => {
            let s = app.get_input_str(Problem::ValidParentheses, "s", "([{}])");
            generate_valid_parentheses_steps(s)
        }
        Problem::BinarySearch => {
            let parsed: Vec<i32> = app
                .get_input_str(Problem::BinarySearch, "nums", "-1, 0, 2, 4, 6, 8")
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            let nums = if parsed.is_empty() {
                vec![-1, 0, 2, 4, 6, 8]
            } else {
                parsed
            };
            let target = app.get_input_int(Problem::BinarySearch, "target", 4);
            generate_binary_search_steps(&nums, target)
        }
        Problem::ReverseLinkedList => {
            let parsed: Vec<i32> = app
                .get_input_str(Problem::ReverseLinkedList, "nodes", "0, 1, 2, 3")
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            let nodes = if parsed.is_empty() {
                vec![0, 1, 2, 3]
            } else {
                parsed
            };
            generate_reverse_linked_list_steps(&nodes)
        }
        Problem::MergeTwoLists => {
            let l1: Vec<i32> = app
                .get_input_str(Problem::MergeTwoLists, "list1", "1, 2, 4")
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            let l2: Vec<i32> = app
                .get_input_str(Problem::MergeTwoLists, "list2", "1, 3, 5")
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            generate_merge_two_lists_steps(&l1, &l2)
        }
        Problem::LinkedListCycle => {
            let nodes: Vec<i32> = app
                .get_input_str(Problem::LinkedListCycle, "nodes", "1, 2, 3, 4")
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            let cycle_idx = app.get_input_int(Problem::LinkedListCycle, "cycle_idx", 1);
            generate_linked_list_cycle_steps(&nodes, cycle_idx)
        }
        Problem::InvertTree => {
            let tree = app.parse_tree_input();
            generate_invert_tree_steps(&tree)
        }
        Problem::MaxDepthTree => {
            let tree = app.parse_tree_input();
            generate_max_depth_tree_steps(&tree)
        }
        Problem::DiameterTree => {
            let tree = app.parse_tree_input();
            generate_diameter_tree_steps(&tree)
        }
        Problem::BalancedTree => {
            let tree = app.parse_tree_input();
            generate_balanced_tree_steps(&tree)
        }
        Problem::SameTree => {
            let tree = app.parse_tree_input();
            generate_same_tree_steps(&tree, &tree)
        }
        Problem::Subtree => {
            let tree = app.parse_tree_input();
            generate_subtree_steps(&tree, &[tree.get(1).cloned().flatten()])
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
            let parsed: Vec<i32> = app
                .get_input_str(Problem::TwoSumII, "nums", "2, 7, 11, 15")
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            let nums = if parsed.is_empty() {
                vec![2, 7, 11, 15]
            } else {
                parsed
            };
            let target = app.get_input_int(Problem::TwoSumII, "target", 9);
            generate_two_sum_ii_steps(&nums, target)
        }
        Problem::ThreeSum => {
            let parsed: Vec<i32> = app
                .get_input_str(Problem::ThreeSum, "nums", "-1, 0, 1, 2, -1, -4")
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            let nums = if parsed.is_empty() {
                vec![-1, 0, 1, 2, -1, -4]
            } else {
                parsed
            };
            generate_three_sum_steps(&nums)
        }
        Problem::ContainerWater => {
            let parsed: Vec<i32> = app
                .get_input_str(Problem::ContainerWater, "nums", "1, 8, 6, 2, 5, 4, 8, 3, 7")
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            let nums = if parsed.is_empty() {
                vec![1, 8, 6, 2, 5, 4, 8, 3, 7]
            } else {
                parsed
            };
            generate_container_water_steps(&nums)
        }
        Problem::TrappingRain => {
            let parsed: Vec<i32> = app
                .get_input_str(
                    Problem::TrappingRain,
                    "nums",
                    "0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1",
                )
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            let nums = if parsed.is_empty() {
                vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]
            } else {
                parsed
            };
            generate_trapping_rain_steps(&nums)
        }
        Problem::MinStack => generate_min_stack_steps(&[
            ("push", Some(-2)),
            ("push", Some(0)),
            ("push", Some(-3)),
            ("getMin", None),
            ("pop", None),
            ("top", None),
            ("getMin", None),
        ]),
        Problem::EvalRPN => {
            let tokens = vec![
                "2".to_string(),
                "1".to_string(),
                "+".to_string(),
                "3".to_string(),
                "*".to_string(),
            ];
            generate_eval_rpn_steps(&tokens)
        }
        Problem::LongestSubstring => {
            let s_input = app.get_input_str(Problem::LongestSubstring, "s", "abcabcbb");
            let s = if s_input.is_empty() {
                "abcabcbb"
            } else {
                s_input
            };
            generate_longest_substring_steps(s)
        }
        Problem::Search2DMatrix => {
            let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
            generate_search_2d_matrix_steps(&matrix, 3)
        }
        Problem::HouseRobber => {
            let parsed: Vec<i32> = app
                .get_input_str(Problem::HouseRobber, "nums", "1, 2, 3, 1")
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            let nums = if parsed.is_empty() {
                vec![1, 2, 3, 1]
            } else {
                parsed
            };
            generate_house_robber_steps(&nums)
        }
        Problem::GenerateParentheses => generate_parentheses_combinations_steps(3),
        Problem::DailyTemperatures => {
            generate_daily_temperatures_steps(&[73, 74, 75, 71, 69, 72, 76, 73])
        }
        Problem::CarFleet => generate_car_fleet_steps(12, &[10, 8, 0, 5, 3], &[2, 4, 1, 1, 3]),
        Problem::LargestRectangle => generate_largest_rectangle_steps(&[2, 1, 5, 6, 2, 3]),
        Problem::CharacterReplacement => generate_character_replacement_steps("ABAB", 2),
        Problem::PermutationInString => generate_permutation_in_string_steps("ab", "eidbaooo"),
        Problem::MinWindowSubstring => generate_min_window_substring_steps("ADOBECODEBANC", "ABC"),
        Problem::SlidingWindowMax => {
            generate_sliding_window_max_steps(&[1, 3, -1, -3, 5, 3, 6, 7], 3)
        }
        Problem::SearchRotatedArray => {
            let parsed: Vec<i32> = app
                .get_input_str(Problem::SearchRotatedArray, "nums", "4, 5, 6, 7, 0, 1, 2")
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            let nums = if parsed.is_empty() {
                vec![4, 5, 6, 7, 0, 1, 2]
            } else {
                parsed
            };
            let target = app.get_input_int(Problem::SearchRotatedArray, "target", 0);
            generate_search_rotated_array_steps(&nums, target)
        }
        Problem::FindMinRotated => {
            let parsed: Vec<i32> = app
                .get_input_str(Problem::FindMinRotated, "nums", "3, 4, 5, 1, 2")
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            let nums = if parsed.is_empty() {
                vec![3, 4, 5, 1, 2]
            } else {
                parsed
            };
            generate_find_min_rotated_steps(&nums)
        }
        Problem::TimeKeyValueStore => generate_time_key_value_store_steps(),
        Problem::FindMedianSortedArrays => {
            generate_find_median_sorted_arrays_steps(&[1, 3], &[2, 4])
        }
        Problem::KokoEatingBananas => {
            let parsed: Vec<i32> = app
                .get_input_str(Problem::KokoEatingBananas, "nums", "3, 6, 7, 11")
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            let piles = if parsed.is_empty() {
                vec![3, 6, 7, 11]
            } else {
                parsed
            };
            let raw_target = app.get_input_int(Problem::KokoEatingBananas, "target", 8);
            let target_h = if raw_target <= 0 { 8 } else { raw_target };
            generate_koko_eating_bananas_steps(&piles, target_h)
        }
        Problem::ImplementTrie => {
            let insert_words: Vec<String> = app
                .get_input_str(Problem::ImplementTrie, "words", "apple, app, ape")
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            let search_word = app.get_input_str(Problem::ImplementTrie, "search", "app");
            generate_implement_trie_steps(&insert_words, search_word)
        }
        Problem::WordDictionary => {
            let words: Vec<String> = app
                .get_input_str(Problem::WordDictionary, "words", "bad, dad, mad")
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            let pattern = app.get_input_str(Problem::WordDictionary, "pattern", ".ad");
            generate_word_dictionary_steps(&words, pattern)
        }
        Problem::WordSearchII => {
            let words: Vec<String> = app
                .get_input_str(Problem::WordSearchII, "words", "oath, pea, eat, rain")
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            generate_word_search_ii_steps(&words)
        }
        Problem::Subsets => {
            let parsed: Vec<i32> = app
                .get_input_str(Problem::Subsets, "nums", "1, 2, 3")
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            let nums = if parsed.is_empty() {
                vec![1, 2, 3]
            } else {
                parsed
            };
            generate_subsets_steps(&nums)
        }
        Problem::Permutations => {
            let parsed: Vec<i32> = app
                .get_input_str(Problem::Permutations, "nums", "1, 2, 3")
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            let nums = if parsed.is_empty() {
                vec![1, 2, 3]
            } else {
                parsed
            };
            generate_permutations_steps(&nums)
        }
        Problem::KClosestPoints => generate_k_closest_points_steps(&[(1, 3), (-2, 2), (5, 8)], 1),
        Problem::TaskScheduler => generate_task_scheduler_steps(&['A', 'A', 'A', 'B', 'B', 'B'], 2),
        Problem::FindMedianDataStream => {
            let parsed: Vec<i32> = app
                .get_input_str(Problem::FindMedianDataStream, "nums", "1, 2, 5, 10, 3")
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            let nums = if parsed.is_empty() {
                vec![1, 2, 5, 10, 3]
            } else {
                parsed
            };
            generate_find_median_steps(&nums)
        }
        Problem::CombinationSum => generate_combination_sum_steps(&[2, 3, 6, 7], 7),
        Problem::SubsetsII => {
            let parsed: Vec<i32> = app
                .get_input_str(Problem::SubsetsII, "nums", "1, 2, 2")
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            let nums = if parsed.is_empty() {
                vec![1, 2, 2]
            } else {
                parsed
            };
            generate_subsets_ii_steps(&nums)
        }
        Problem::CombinationSumII => generate_combination_sum_ii_steps(&[10, 1, 2, 7, 6, 1, 5], 8),
        Problem::WordSearch => generate_word_search_steps(
            &[
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E'],
            ],
            "ABCCED",
        ),
        Problem::NQueens => generate_n_queens_steps(4),
        Problem::KthLargestArray => generate_kth_largest_array_steps(&[3, 2, 1, 5, 6, 4], 2),
        Problem::DesignTwitter => generate_design_twitter_steps(),
        Problem::PalindromePartitioning => generate_palindrome_partitioning_steps("aab"),
        Problem::LetterCombinations => generate_letter_combinations_steps("23"),
        Problem::HouseRobberII => {
            let parsed: Vec<i32> = app
                .get_input_str(Problem::HouseRobberII, "nums", "2, 3, 2")
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            let nums = if parsed.is_empty() {
                vec![2, 3, 2]
            } else {
                parsed
            };
            generate_house_robber_ii_steps(&nums)
        }
        Problem::LongestPalindromicSubstring => {
            generate_longest_palindromic_substring_steps("babad")
        }
        Problem::PalindromicSubstrings => generate_palindromic_substrings_steps("aaa"),
        Problem::DecodeWays => generate_decode_ways_steps("226"),
        Problem::CoinChange => generate_coin_change_steps(&[1, 2, 5], 11),
        Problem::MaxProductSubarray => {
            let parsed: Vec<i32> = app
                .get_input_str(Problem::MaxProductSubarray, "nums", "2, 3, -2, 4")
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            let nums = if parsed.is_empty() {
                vec![2, 3, -2, 4]
            } else {
                parsed
            };
            generate_max_product_subarray_steps(&nums)
        }
        Problem::WordBreak => {
            let words: Vec<String> = vec!["leet".to_string(), "code".to_string()];
            generate_word_break_steps("leetcode", &words)
        }
        Problem::LongestIncreasingSubsequence => {
            let parsed: Vec<i32> = app
                .get_input_str(
                    Problem::LongestIncreasingSubsequence,
                    "nums",
                    "10, 9, 2, 5, 3, 7, 101, 18",
                )
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            let nums = if parsed.is_empty() {
                vec![10, 9, 2, 5, 3, 7, 101, 18]
            } else {
                parsed
            };
            generate_longest_increasing_subsequence_steps(&nums)
        }
        Problem::PartitionEqualSubsetSum => {
            let parsed: Vec<i32> = app
                .get_input_str(Problem::PartitionEqualSubsetSum, "nums", "1, 5, 11, 5")
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            let nums = if parsed.is_empty() {
                vec![1, 5, 11, 5]
            } else {
                parsed
            };
            generate_partition_equal_subset_sum_steps(&nums)
        }
        Problem::Number1Bits => generate_number_1_bits_steps(11),
        Problem::SumTwoIntegers => generate_sum_two_integers_steps(1, 2),
        Problem::ReverseInteger => generate_reverse_integer_steps(123),
        Problem::RotateImage => {
            generate_rotate_image_steps(&[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
        }
        Problem::SpiralMatrix => {
            generate_spiral_matrix_steps(&[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
        }
        Problem::SetMatrixZeroes => {
            generate_set_matrix_zeroes_steps(&[vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]])
        }
        Problem::PowXN => generate_pow_xn_steps(2.0, 10),
        Problem::MultiplyStrings => generate_multiply_strings_steps("2", "3"),
        Problem::DetectSquares => generate_detect_squares_steps(),
        Problem::MaximumSubarray => {
            let parsed: Vec<i32> = app
                .get_input_str(
                    Problem::MaximumSubarray,
                    "nums",
                    "-2, 1, -3, 4, -1, 2, 1, -5, 4",
                )
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            let nums = if parsed.is_empty() {
                vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]
            } else {
                parsed
            };
            generate_maximum_subarray_steps(&nums)
        }
        Problem::JumpGame => {
            let parsed: Vec<i32> = app
                .get_input_str(Problem::JumpGame, "nums", "2, 3, 1, 1, 4")
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            let nums = if parsed.is_empty() {
                vec![2, 3, 1, 1, 4]
            } else {
                parsed
            };
            generate_jump_game_steps(&nums)
        }
        Problem::JumpGameII => {
            let parsed: Vec<i32> = app
                .get_input_str(Problem::JumpGameII, "nums", "2, 3, 1, 1, 4")
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            let nums = if parsed.is_empty() {
                vec![2, 3, 1, 1, 4]
            } else {
                parsed
            };
            generate_jump_game_ii_steps(&nums)
        }
        Problem::GasStation => generate_gas_station_steps(&[1, 2, 3, 4, 5], &[3, 4, 5, 1, 2]),
        Problem::HandOfStraights => {
            generate_hand_of_straights_steps(&[1, 2, 3, 6, 2, 3, 4, 7, 8], 3)
        }
        Problem::MergeTriplets => generate_merge_triplets_steps(),
        Problem::PartitionLabels => generate_partition_labels_steps("ababcbacadefegdehijhklij"),
        Problem::ValidParenthesisString => generate_valid_parenthesis_string_steps("(*)"),
        Problem::InsertInterval => generate_insert_interval_steps(),
        Problem::MergeIntervals => generate_merge_intervals_steps(),
        Problem::NonOverlappingIntervals => generate_non_overlapping_intervals_steps(),
        Problem::MeetingRoomsII => generate_meeting_rooms_ii_steps(),
        Problem::MinIntervalQuery => generate_min_interval_query_steps(),
        Problem::NumberIslands => generate_number_islands_steps(&[
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ]),
        Problem::MaxAreaIsland => generate_max_area_island_steps(),
        Problem::CloneGraph => generate_clone_graph_steps(),
        Problem::WallsAndGates => generate_walls_and_gates_steps(),
        Problem::RottingOranges => generate_rotting_oranges_steps(),
        Problem::PacificAtlantic => generate_pacific_atlantic_steps(),
        Problem::SurroundedRegions => generate_surrounded_regions_steps(),
        Problem::CourseSchedule => generate_course_schedule_steps(2, &[[1, 0]]),
        Problem::CourseScheduleII => generate_course_schedule_ii_steps(2, &[[1, 0]]),
        Problem::GraphValidTree => {
            generate_graph_valid_tree_steps(5, &[[0, 1], [0, 2], [0, 3], [1, 4]])
        }
        Problem::ConnectedComponents => {
            generate_connected_components_steps(5, &[[0, 1], [1, 2], [3, 4]])
        }
        Problem::RedundantConnection => {
            generate_redundant_connection_steps(&[[1, 2], [1, 3], [2, 3]])
        }
        Problem::WordLadder => {
            generate_word_ladder_steps("hit", "cog", &["hot", "dot", "dog", "lot", "log", "cog"])
        }
        Problem::UniquePaths => generate_unique_paths_steps(3, 7),
        Problem::LongestCommonSubsequence => generate_lcs_steps("abcde", "ace"),
        Problem::BestTimeStockCooldown => generate_stock_cooldown_steps(&[1, 2, 3, 0, 2]),
        Problem::CoinChangeII => generate_coin_change_ii_steps(5, &[1, 2, 5]),
        Problem::TargetSum => generate_target_sum_steps(&[1, 1, 1, 1, 1], 3),
        Problem::InterleavingString => {
            generate_interleaving_string_steps("aabcc", "dbbca", "aadbbcbcac")
        }
        Problem::LongestIncreasingPath => generate_lip_steps(),
        Problem::DistinctSubsequences => generate_distinct_subsequences_steps("rabbbit", "rabbit"),
        Problem::EditDistance => generate_edit_distance_steps("horse", "ros"),
        Problem::BurstBalloons => generate_burst_balloons_steps(),
        Problem::RegularExpressionMatching => generate_regex_matching_steps(),
        Problem::ReconstructItinerary => generate_reconstruct_itinerary_steps(),
        Problem::MinCostConnectPoints => generate_min_cost_points_steps(),
        Problem::NetworkDelayTime => generate_network_delay_steps(),
        Problem::SwimInRisingWater => generate_swim_rising_water_steps(),
        Problem::AlienDictionary => generate_alien_dictionary_steps(),
        Problem::CheapestFlights => generate_cheapest_flights_steps(),
        Problem::ReorderList => generate_reorder_list_steps(&[1, 2, 3, 4, 5]),
        Problem::RemoveNthNodeFromEnd => generate_remove_nth_node_steps(&[1, 2, 3, 4, 5], 2),
        Problem::CopyListWithRandomPointer => generate_copy_list_random_steps(&[7, 13, 11, 10, 1]),
        Problem::AddTwoNumbers => generate_add_two_numbers_steps(&[2, 4, 3], &[5, 6, 4]),
        Problem::FindDuplicateNumber => generate_find_duplicate_number_steps(&[1, 3, 4, 2, 2]),
        Problem::LruCache => generate_lru_cache_steps(
            2,
            &[
                ("put", 1, 1),
                ("put", 2, 2),
                ("get", 1, 0),
                ("put", 3, 3),
                ("get", 2, 0),
            ],
        ),
        Problem::MergeKSortedLists => {
            generate_merge_k_lists_steps(&[vec![1, 4, 5], vec![1, 3, 4], vec![2, 6]])
        }
        Problem::ReverseNodesInKGroup => generate_reverse_k_group_steps(&[1, 2, 3, 4, 5], 2),
        Problem::BinaryTreeLevelOrderTraversal => generate_level_order_traversal_steps(&[
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]),
        Problem::BinaryTreeRightSideView => generate_right_side_view_steps(&[
            Some(1),
            Some(2),
            Some(3),
            None,
            Some(5),
            None,
            Some(4),
        ]),
        Problem::CountGoodNodes => generate_count_good_nodes_steps(&[
            Some(3),
            Some(1),
            Some(4),
            Some(3),
            None,
            Some(1),
            Some(5),
        ]),
        Problem::KthSmallestElementBst => {
            generate_kth_smallest_bst_steps(&[Some(3), Some(1), Some(4), None, Some(2)], 1)
        }
        Problem::ConstructBinaryTreePreorderInorder => {
            generate_construct_tree_pre_in_steps(&[3, 9, 20, 15, 7], &[9, 3, 15, 20, 7])
        }
        Problem::BinaryTreeMaxPathSum => generate_tree_max_path_sum_steps(&[
            Some(-10),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]),
        Problem::SerializeDeserializeBinaryTree => generate_serialize_deserialize_tree_steps(&[
            Some(1),
            Some(2),
            Some(3),
            None,
            None,
            Some(4),
            Some(5),
        ]),
    };

    app.current_step_idx = 0;
    app.last_focused_step_idx = None;
    app.is_playing = false;
}

pub(crate) fn select_problem(app: &mut VisualizerApp, problem: Problem) {
    if app.current_problem != problem {
        app.current_problem = problem;
        app.selected_approach_id = 0;
        app.recompute_steps();
    }
}
