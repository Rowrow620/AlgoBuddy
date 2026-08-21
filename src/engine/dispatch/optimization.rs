use super::*;

pub(super) fn generate_steps(app: &mut VisualizerApp) -> Vec<Step> {
    let approach_id = app.selected_approach_id;
    match app.current_problem {
        Problem::ClimbingStairs if approach_id == 1 => {
            crate::algorithms::climbing_stairs::generate_climbing_stairs_recursive_steps(5)
        }
        Problem::ClimbingStairs => generate_climbing_stairs_steps(5),
        Problem::MinCostStairs if approach_id == 1 => {
            crate::algorithms::min_cost_stairs::generate_min_cost_stairs_recursive_steps(&[
                10, 15, 20,
            ])
        }
        Problem::MinCostStairs => generate_min_cost_stairs_steps(&[10, 15, 20]),
        Problem::MeetingRooms if approach_id == 1 => {
            crate::algorithms::meeting_rooms::generate_meeting_rooms_all_pairs_steps(&[
                (0, 30),
                (5, 10),
                (15, 20),
            ])
        }
        Problem::MeetingRooms => generate_meeting_rooms_steps(&[(0, 30), (5, 10), (15, 20)]),
        Problem::HouseRobber => {
            let nums = input::i32_list(
                app,
                Problem::HouseRobber,
                "nums",
                "1, 2, 3, 1",
                &[1, 2, 3, 1],
            );
            generate_house_robber_steps(&nums)
        }
        Problem::HouseRobberII => {
            let nums = input::i32_list(app, Problem::HouseRobberII, "nums", "2, 3, 2", &[2, 3, 2]);
            generate_house_robber_ii_steps(&nums)
        }
        Problem::LongestPalindromicSubstring => {
            let s = app.get_input_str(Problem::LongestPalindromicSubstring, "s", "babad");
            generate_longest_palindromic_substring_steps(s)
        }
        Problem::PalindromicSubstrings => {
            let s = app.get_input_str(Problem::PalindromicSubstrings, "s", "aaa");
            generate_palindromic_substrings_steps(s)
        }
        Problem::DecodeWays => {
            let s = app.get_input_str(Problem::DecodeWays, "s", "226");
            generate_decode_ways_steps(s)
        }
        Problem::CoinChange => {
            let coins = input::i32_list(app, Problem::CoinChange, "coins", "1, 2, 5", &[1, 2, 5]);
            let amount = app.get_input_int(Problem::CoinChange, "amount", 11);
            generate_coin_change_steps(&coins, amount)
        }
        Problem::MaxProductSubarray => {
            let nums = input::i32_list(
                app,
                Problem::MaxProductSubarray,
                "nums",
                "2, 3, -2, 4",
                &[2, 3, -2, 4],
            );
            generate_max_product_subarray_steps(&nums)
        }
        Problem::WordBreak => {
            let s = app.get_input_str(Problem::WordBreak, "s", "leetcode");
            let words = input::string_list(
                app,
                Problem::WordBreak,
                "words",
                "leet, code",
                &["leet", "code"],
            );
            generate_word_break_steps(s, &words)
        }
        Problem::LongestIncreasingSubsequence => {
            let nums = input::i32_list(
                app,
                Problem::LongestIncreasingSubsequence,
                "nums",
                "10, 9, 2, 5, 3, 7, 101, 18",
                &[10, 9, 2, 5, 3, 7, 101, 18],
            );
            generate_longest_increasing_subsequence_steps(&nums)
        }
        Problem::PartitionEqualSubsetSum => {
            let nums = input::i32_list(
                app,
                Problem::PartitionEqualSubsetSum,
                "nums",
                "1, 5, 11, 5",
                &[1, 5, 11, 5],
            );
            generate_partition_equal_subset_sum_steps(&nums)
        }
        Problem::MaximumSubarray => {
            let nums = input::i32_list(
                app,
                Problem::MaximumSubarray,
                "nums",
                "-2, 1, -3, 4, -1, 2, 1, -5, 4",
                &[-2, 1, -3, 4, -1, 2, 1, -5, 4],
            );
            generate_maximum_subarray_steps(&nums)
        }
        Problem::JumpGame => {
            let nums = input::i32_list(
                app,
                Problem::JumpGame,
                "nums",
                "2, 3, 1, 1, 4",
                &[2, 3, 1, 1, 4],
            );
            generate_jump_game_steps(&nums)
        }
        Problem::JumpGameII => {
            let nums = input::i32_list(
                app,
                Problem::JumpGameII,
                "nums",
                "2, 3, 1, 1, 4",
                &[2, 3, 1, 1, 4],
            );
            generate_jump_game_ii_steps(&nums)
        }
        Problem::GasStation => generate_gas_station_steps(&[1, 2, 3, 4, 5], &[3, 4, 5, 1, 2]),
        Problem::HandOfStraights => {
            generate_hand_of_straights_steps(&[1, 2, 3, 6, 2, 3, 4, 7, 8], 3)
        }
        Problem::MergeTriplets => generate_merge_triplets_steps(),
        Problem::PartitionLabels => {
            let s = app.get_input_str(Problem::PartitionLabels, "s", "ababcbacadefegdehijhklij");
            generate_partition_labels_steps(s)
        }
        Problem::ValidParenthesisString => {
            let s = app.get_input_str(Problem::ValidParenthesisString, "s", "(*)");
            generate_valid_parenthesis_string_steps(s)
        }
        Problem::InsertInterval => generate_insert_interval_steps(),
        Problem::MergeIntervals => generate_merge_intervals_steps(),
        Problem::NonOverlappingIntervals => generate_non_overlapping_intervals_steps(),
        Problem::MeetingRoomsII => generate_meeting_rooms_ii_steps(),
        Problem::MinIntervalQuery => generate_min_interval_query_steps(),
        Problem::UniquePaths => {
            let m = app.get_input_int(Problem::UniquePaths, "m", 3).max(1) as usize;
            let n = app.get_input_int(Problem::UniquePaths, "n", 7).max(1) as usize;
            generate_unique_paths_steps(m, n)
        }
        Problem::LongestCommonSubsequence => {
            let text1 = app.get_input_str(Problem::LongestCommonSubsequence, "text1", "abcde");
            let text2 = app.get_input_str(Problem::LongestCommonSubsequence, "text2", "ace");
            generate_lcs_steps(text1, text2)
        }
        Problem::BestTimeStockCooldown => {
            let prices = input::i32_list(
                app,
                Problem::BestTimeStockCooldown,
                "prices",
                "1, 2, 3, 0, 2",
                &[1, 2, 3, 0, 2],
            );
            generate_stock_cooldown_steps(&prices)
        }
        Problem::CoinChangeII => {
            let coins: Vec<usize> =
                input::i32_list(app, Problem::CoinChangeII, "coins", "1, 2, 5", &[1, 2, 5])
                    .into_iter()
                    .map(|c| c.max(1) as usize)
                    .collect();
            let amount = app.get_input_int(Problem::CoinChangeII, "amount", 5).max(0) as usize;
            generate_coin_change_ii_steps(amount, &coins)
        }
        Problem::TargetSum => {
            let nums = input::i32_list(
                app,
                Problem::TargetSum,
                "nums",
                "1, 1, 1, 1, 1",
                &[1, 1, 1, 1, 1],
            );
            let target = app.get_input_int(Problem::TargetSum, "target", 3);
            generate_target_sum_steps(&nums, target)
        }
        Problem::InterleavingString => {
            let s1 = app.get_input_str(Problem::InterleavingString, "s1", "aabcc");
            let s2 = app.get_input_str(Problem::InterleavingString, "s2", "dbbca");
            let s3 = app.get_input_str(Problem::InterleavingString, "s3", "aadbbcbcac");
            generate_interleaving_string_steps(s1, s2, s3)
        }
        Problem::LongestIncreasingPath => generate_lip_steps(),
        Problem::DistinctSubsequences => {
            let s = app.get_input_str(Problem::DistinctSubsequences, "s", "rabbbit");
            let t = app.get_input_str(Problem::DistinctSubsequences, "t", "rabbit");
            generate_distinct_subsequences_steps(s, t)
        }
        Problem::EditDistance => {
            let word1 = app.get_input_str(Problem::EditDistance, "word1", "horse");
            let word2 = app.get_input_str(Problem::EditDistance, "word2", "ros");
            generate_edit_distance_steps(word1, word2)
        }
        Problem::BurstBalloons => generate_burst_balloons_steps(),
        Problem::RegularExpressionMatching => generate_regex_matching_steps(),
        _ => unreachable!("problem routed to the wrong optimization engine"),
    }
}
