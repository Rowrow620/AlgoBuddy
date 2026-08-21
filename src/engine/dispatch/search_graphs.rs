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
        Problem::CombinationSum => {
            let candidates = input::i32_list(
                app,
                Problem::CombinationSum,
                "candidates",
                "2, 3, 6, 7",
                &[2, 3, 6, 7],
            );
            let target = app.get_input_int(Problem::CombinationSum, "target", 7);
            generate_combination_sum_steps(&candidates, target)
        }
        Problem::SubsetsII => {
            let nums = input::i32_list(app, Problem::SubsetsII, "nums", "1, 2, 2", &[1, 2, 2]);
            generate_subsets_ii_steps(&nums)
        }
        Problem::CombinationSumII => {
            let candidates = input::i32_list(
                app,
                Problem::CombinationSumII,
                "candidates",
                "10, 1, 2, 7, 6, 1, 5",
                &[10, 1, 2, 7, 6, 1, 5],
            );
            let target = app.get_input_int(Problem::CombinationSumII, "target", 8);
            generate_combination_sum_ii_steps(&candidates, target)
        }
        Problem::WordSearch => {
            let word = app.get_input_str(Problem::WordSearch, "word", "ABCCED");
            generate_word_search_steps(
                &[
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E'],
                ],
                word,
            )
        }
        Problem::NQueens => {
            let n = app.get_input_int(Problem::NQueens, "n", 4).max(1) as usize;
            generate_n_queens_steps(n)
        }
        Problem::PalindromePartitioning => {
            let s = app.get_input_str(Problem::PalindromePartitioning, "s", "aab");
            generate_palindrome_partitioning_steps(s)
        }
        Problem::LetterCombinations => {
            let digits = app.get_input_str(Problem::LetterCombinations, "digits", "23");
            generate_letter_combinations_steps(digits)
        }
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
            let begin = app.get_input_str(Problem::WordLadder, "begin", "hit");
            let end = app.get_input_str(Problem::WordLadder, "end", "cog");
            let word_list = input::string_list(
                app,
                Problem::WordLadder,
                "word_list",
                "hot, dot, dog, lot, log, cog",
                &["hot", "dot", "dog", "lot", "log", "cog"],
            );
            let word_refs: Vec<&str> = word_list.iter().map(|s| s.as_str()).collect();
            generate_word_ladder_steps(begin, end, &word_refs)
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
