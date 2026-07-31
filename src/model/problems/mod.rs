pub mod advanced_graphs;
pub mod arrays_hashing;
pub mod backtracking;
pub mod binary_search;
pub mod bit_manipulation;
pub mod graphs;
pub mod greedy;
pub mod heap;
pub mod intervals;
pub mod linked_list;
pub mod math_geometry;
pub mod one_d_dp;
pub mod sliding_window;
pub mod stack;
pub mod trees;
pub mod tries;
pub mod two_d_dp;
pub mod two_pointers;

use crate::model::problem::{Problem, ProblemDetails};

pub fn get_problem_details(problem: Problem) -> ProblemDetails {
    if let Some(details) = advanced_graphs::get_details(problem) {
        return details;
    }
    if let Some(details) = arrays_hashing::get_details(problem) {
        return details;
    }
    if let Some(details) = backtracking::get_details(problem) {
        return details;
    }
    if let Some(details) = binary_search::get_details(problem) {
        return details;
    }
    if let Some(details) = bit_manipulation::get_details(problem) {
        return details;
    }
    if let Some(details) = graphs::get_details(problem) {
        return details;
    }
    if let Some(details) = greedy::get_details(problem) {
        return details;
    }
    if let Some(details) = heap::get_details(problem) {
        return details;
    }
    if let Some(details) = intervals::get_details(problem) {
        return details;
    }
    if let Some(details) = linked_list::get_details(problem) {
        return details;
    }
    if let Some(details) = math_geometry::get_details(problem) {
        return details;
    }
    if let Some(details) = one_d_dp::get_details(problem) {
        return details;
    }
    if let Some(details) = sliding_window::get_details(problem) {
        return details;
    }
    if let Some(details) = stack::get_details(problem) {
        return details;
    }
    if let Some(details) = trees::get_details(problem) {
        return details;
    }
    if let Some(details) = tries::get_details(problem) {
        return details;
    }
    if let Some(details) = two_d_dp::get_details(problem) {
        return details;
    }
    if let Some(details) = two_pointers::get_details(problem) {
        return details;
    }
    panic!("Missing ProblemDetails for {:?}", problem);
}

pub fn get_problem_code_lines(problem: Problem, approach_id: usize) -> Vec<(usize, &'static str)> {
    if let Some(lines) = advanced_graphs::get_code_lines(problem, approach_id) {
        return lines;
    }
    if let Some(lines) = arrays_hashing::get_code_lines(problem, approach_id) {
        return lines;
    }
    if let Some(lines) = backtracking::get_code_lines(problem, approach_id) {
        return lines;
    }
    if let Some(lines) = binary_search::get_code_lines(problem, approach_id) {
        return lines;
    }
    if let Some(lines) = bit_manipulation::get_code_lines(problem, approach_id) {
        return lines;
    }
    if let Some(lines) = graphs::get_code_lines(problem, approach_id) {
        return lines;
    }
    if let Some(lines) = greedy::get_code_lines(problem, approach_id) {
        return lines;
    }
    if let Some(lines) = heap::get_code_lines(problem, approach_id) {
        return lines;
    }
    if let Some(lines) = intervals::get_code_lines(problem, approach_id) {
        return lines;
    }
    if let Some(lines) = linked_list::get_code_lines(problem, approach_id) {
        return lines;
    }
    if let Some(lines) = math_geometry::get_code_lines(problem, approach_id) {
        return lines;
    }
    if let Some(lines) = one_d_dp::get_code_lines(problem, approach_id) {
        return lines;
    }
    if let Some(lines) = sliding_window::get_code_lines(problem, approach_id) {
        return lines;
    }
    if let Some(lines) = stack::get_code_lines(problem, approach_id) {
        return lines;
    }
    if let Some(lines) = trees::get_code_lines(problem, approach_id) {
        return lines;
    }
    if let Some(lines) = tries::get_code_lines(problem, approach_id) {
        return lines;
    }
    if let Some(lines) = two_d_dp::get_code_lines(problem, approach_id) {
        return lines;
    }
    if let Some(lines) = two_pointers::get_code_lines(problem, approach_id) {
        return lines;
    }
    vec![(1, "# Approach implementation trace")]
}
