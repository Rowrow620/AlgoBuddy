use super::*;

pub(super) fn generate_steps(app: &mut VisualizerApp) -> Vec<Step> {
    match app.current_problem {
        Problem::Subsets => {
            let nums = input::i32_list(app, Problem::Subsets, "nums", "1, 2, 3", &[1, 2, 3]);
            generate_subsets_steps(&nums)
        }
        Problem::Permutations => {
            let nums = input::i32_list(app, Problem::Permutations, "nums", "1, 2, 3", &[1, 2, 3]);
            generate_permutations_steps(&nums)
        }
        Problem::CombinationSum => generate_combination_sum_steps(&[2, 3, 6, 7], 7),
        Problem::SubsetsII => {
            let nums = input::i32_list(app, Problem::SubsetsII, "nums", "1, 2, 2", &[1, 2, 2]);
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
        Problem::PalindromePartitioning => generate_palindrome_partitioning_steps("aab"),
        Problem::LetterCombinations => generate_letter_combinations_steps("23"),
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
        Problem::ReconstructItinerary => generate_reconstruct_itinerary_steps(),
        Problem::MinCostConnectPoints => generate_min_cost_points_steps(),
        Problem::NetworkDelayTime => generate_network_delay_steps(),
        Problem::SwimInRisingWater => generate_swim_rising_water_steps(),
        Problem::AlienDictionary => generate_alien_dictionary_steps(),
        Problem::CheapestFlights => generate_cheapest_flights_steps(),
        _ => unreachable!("problem routed to the wrong search_graphs engine"),
    }
}
