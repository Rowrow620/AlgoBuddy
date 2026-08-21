use super::*;

pub(super) fn generate_steps(app: &mut VisualizerApp) -> Vec<Step> {
    let app_id = app.selected_approach_id;
    match app.current_problem {
        Problem::ContainsDuplicate => {
            let nums = input::i32_list(
                app,
                Problem::ContainsDuplicate,
                "nums",
                "1, 2, 3, 1",
                &[1, 2, 3, 1],
            );
            generate_contains_duplicate_steps(&nums, app_id)
        }
        Problem::TwoSum => {
            let nums = input::i32_list(
                app,
                Problem::TwoSum,
                "nums",
                "2, 7, 11, 15",
                &[2, 7, 11, 15],
            );
            let target = app.get_input_int(Problem::TwoSum, "target", 9);
            generate_two_sum_steps(&nums, target, app_id)
        }
        Problem::ValidAnagram => {
            let s = app.get_input_str(Problem::ValidAnagram, "s", "anagram");
            let t = app.get_input_str(Problem::ValidAnagram, "t", "nagaram");
            generate_valid_anagram_steps(s, t, app_id)
        }
        Problem::GroupAnagrams => {
            let input_strs = input::string_list(
                app,
                Problem::GroupAnagrams,
                "strs",
                "eat, tea, tan, ate, nat, bat",
                &["eat", "tea", "tan", "ate", "nat", "bat"],
            );
            generate_group_anagrams_steps(&input_strs, app_id)
        }
        Problem::TopKFrequent => {
            let nums = input::i32_list(
                app,
                Problem::TopKFrequent,
                "nums",
                "1, 1, 1, 2, 2, 3",
                &[1, 1, 1, 2, 2, 3],
            );
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
        Problem::LongestSubstring => {
            let s = app.get_input_str(Problem::LongestSubstring, "s", "abcabcbb");
            generate_longest_substring_steps(s)
        }
        Problem::CharacterReplacement => {
            let s = app.get_input_str(Problem::CharacterReplacement, "s", "ABAB");
            let k = app.get_input_int(Problem::CharacterReplacement, "k", 2) as usize;
            generate_character_replacement_steps(s, k)
        }
        Problem::PermutationInString => {
            let s1 = app.get_input_str(Problem::PermutationInString, "s1", "ab");
            let s2 = app.get_input_str(Problem::PermutationInString, "s2", "eidbaooo");
            generate_permutation_in_string_steps(s1, s2)
        }
        Problem::MinWindowSubstring => {
            let s = app.get_input_str(Problem::MinWindowSubstring, "s", "ADOBECODEBANC");
            let t = app.get_input_str(Problem::MinWindowSubstring, "t", "ABC");
            generate_min_window_substring_steps(s, t)
        }
        Problem::SlidingWindowMax => {
            let nums = input::i32_list(
                app,
                Problem::SlidingWindowMax,
                "nums",
                "1, 3, -1, -3, 5, 3, 6, 7",
                &[1, 3, -1, -3, 5, 3, 6, 7],
            );
            let k = app.get_input_int(Problem::SlidingWindowMax, "k", 3) as usize;
            generate_sliding_window_max_steps(&nums, k)
        }
        Problem::ProductExceptSelf => {
            let nums = input::i32_list(
                app,
                Problem::ProductExceptSelf,
                "nums",
                "1, 2, 4, 6",
                &[1, 2, 4, 6],
            );
            generate_product_steps(&nums)
        }
        Problem::EncodeDecode => {
            let strs = input::string_list(
                app,
                Problem::EncodeDecode,
                "strs",
                "Hello, World",
                &["Hello", "World"],
            );
            generate_encode_decode_steps(&strs)
        }
        Problem::ValidSudoku => {
            let board = app.get_sudoku_board();
            generate_valid_sudoku_steps(&board)
        }
        Problem::LongestConsecutive => {
            let nums = input::i32_list(
                app,
                Problem::LongestConsecutive,
                "nums",
                "2, 20, 4, 10, 3, 4, 5",
                &[2, 20, 4, 10, 3, 4, 5],
            );
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
            let prices = input::i32_list(
                app,
                Problem::BestTimeStock,
                "prices",
                "7, 1, 5, 3, 6, 4",
                &[7, 1, 5, 3, 6, 4],
            );
            generate_best_time_stock_steps(&prices, app_id)
        }
        Problem::TwoSumII => {
            let nums = input::i32_list(
                app,
                Problem::TwoSumII,
                "nums",
                "2, 7, 11, 15",
                &[2, 7, 11, 15],
            );
            let target = app.get_input_int(Problem::TwoSumII, "target", 9);
            generate_two_sum_ii_steps(&nums, target)
        }
        Problem::ThreeSum => {
            let nums = input::i32_list(
                app,
                Problem::ThreeSum,
                "nums",
                "-1, 0, 1, 2, -1, -4",
                &[-1, 0, 1, 2, -1, -4],
            );
            generate_three_sum_steps(&nums)
        }
        Problem::ContainerWater => {
            let height = input::i32_list(
                app,
                Problem::ContainerWater,
                "height",
                "1, 8, 6, 2, 5, 4, 8, 3, 7",
                &[1, 8, 6, 2, 5, 4, 8, 3, 7],
            );
            generate_container_water_steps(&height)
        }
        Problem::TrappingRain => {
            let height = input::i32_list(
                app,
                Problem::TrappingRain,
                "height",
                "0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1",
                &[0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1],
            );
            generate_trapping_rain_steps(&height)
        }
        Problem::ValidParentheses => {
            let s = app.get_input_str(Problem::ValidParentheses, "s", "()[]{}");
            generate_valid_parentheses_steps(s, app_id)
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
            let tokens = input::string_list(
                app,
                Problem::EvalRPN,
                "tokens",
                "2, 1, +, 3, *",
                &["2", "1", "+", "3", "*"],
            );
            generate_eval_rpn_steps(&tokens)
        }
        Problem::Search2DMatrix => generate_search_2d_matrix_steps(
            &[vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            3,
        ),
        Problem::GenerateParentheses => {
            let n = app.get_input_int(Problem::GenerateParentheses, "n", 3);
            generate_parentheses_combinations_steps(n.max(1) as usize)
        }
        Problem::DailyTemperatures => {
            let temperatures = input::i32_list(
                app,
                Problem::DailyTemperatures,
                "temperatures",
                "73, 74, 75, 71, 69, 72, 76, 73",
                &[73, 74, 75, 71, 69, 72, 76, 73],
            );
            generate_daily_temperatures_steps(&temperatures)
        }
        Problem::CarFleet => generate_car_fleet_steps(12, &[10, 8, 0, 5, 3], &[2, 4, 1, 1, 3]),
        Problem::LargestRectangle => {
            let heights = input::i32_list(
                app,
                Problem::LargestRectangle,
                "heights",
                "2, 1, 5, 6, 2, 3",
                &[2, 1, 5, 6, 2, 3],
            );
            generate_largest_rectangle_steps(&heights)
        }
        Problem::BinarySearch => {
            let nums = input::i32_list(
                app,
                Problem::BinarySearch,
                "nums",
                "-1, 0, 3, 5, 9, 12",
                &[-1, 0, 3, 5, 9, 12],
            );
            let target = app.get_input_int(Problem::BinarySearch, "target", 9);
            generate_binary_search_steps(&nums, target, app_id)
        }
        Problem::KokoEatingBananas => {
            let piles = input::i32_list(
                app,
                Problem::KokoEatingBananas,
                "piles",
                "3, 6, 7, 11",
                &[3, 6, 7, 11],
            );
            let h = app.get_input_int(Problem::KokoEatingBananas, "h", 8);
            generate_koko_eating_bananas_steps(&piles, h)
        }
        Problem::FindMinRotated => {
            let nums = input::i32_list(
                app,
                Problem::FindMinRotated,
                "nums",
                "3, 4, 5, 1, 2",
                &[3, 4, 5, 1, 2],
            );
            generate_find_min_rotated_steps(&nums)
        }
        Problem::SearchRotatedArray => {
            let nums = input::i32_list(
                app,
                Problem::SearchRotatedArray,
                "nums",
                "4, 5, 6, 7, 0, 1, 2",
                &[4, 5, 6, 7, 0, 1, 2],
            );
            let target = app.get_input_int(Problem::SearchRotatedArray, "target", 0);
            generate_search_rotated_array_steps(&nums, target)
        }
        Problem::FindMedianSortedArrays => {
            let nums1 = input::i32_list(
                app,
                Problem::FindMedianSortedArrays,
                "nums1",
                "1, 3",
                &[1, 3],
            );
            let nums2 = input::i32_list(app, Problem::FindMedianSortedArrays, "nums2", "2", &[2]);
            generate_find_median_sorted_arrays_steps(&nums1, &nums2)
        }
        Problem::TimeKeyValueStore => generate_time_key_value_store_steps(),
        _ => unreachable!("problem routed to the wrong foundations engine"),
    }
}
