use super::*;
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

mod foundations;
mod numeric;
mod optimization;
mod search_graphs;
mod structures;

pub(super) fn generate_steps(app: &mut VisualizerApp) -> Vec<Step> {
    match app.current_problem.category() {
        Category::ArraysAndHashing
        | Category::TwoPointers
        | Category::Stack
        | Category::BinarySearch
        | Category::SlidingWindow => foundations::generate_steps(app),
        Category::LinkedList | Category::Trees | Category::Tries | Category::HeapPriorityQueue => {
            structures::generate_steps(app)
        }
        Category::Backtracking | Category::Graphs | Category::AdvancedGraphs => {
            search_graphs::generate_steps(app)
        }
        Category::OneDDp | Category::TwoDDp | Category::Greedy | Category::Intervals => {
            optimization::generate_steps(app)
        }
        Category::BitManipulation | Category::MathAndGeometry => numeric::generate_steps(app),
    }
}
