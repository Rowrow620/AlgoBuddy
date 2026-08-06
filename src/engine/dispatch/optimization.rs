use super::*;

pub(super) fn generate_steps(app: &mut VisualizerApp) -> Vec<Step> {
    match app.current_problem {
        Problem::ClimbingStairs => generate_climbing_stairs_steps(5),
        Problem::MinCostStairs => generate_min_cost_stairs_steps(&[10, 15, 20]),
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
            generate_longest_palindromic_substring_steps("babad")
        }
        Problem::PalindromicSubstrings => generate_palindromic_substrings_steps("aaa"),
        Problem::DecodeWays => generate_decode_ways_steps("226"),
        Problem::CoinChange => generate_coin_change_steps(&[1, 2, 5], 11),
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
            let words: Vec<String> = vec!["leet".to_string(), "code".to_string()];
            generate_word_break_steps("leetcode", &words)
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
        Problem::PartitionLabels => generate_partition_labels_steps("ababcbacadefegdehijhklij"),
        Problem::ValidParenthesisString => generate_valid_parenthesis_string_steps("(*)"),
        Problem::InsertInterval => generate_insert_interval_steps(),
        Problem::MergeIntervals => generate_merge_intervals_steps(),
        Problem::NonOverlappingIntervals => generate_non_overlapping_intervals_steps(),
        Problem::MeetingRoomsII => generate_meeting_rooms_ii_steps(),
        Problem::MinIntervalQuery => generate_min_interval_query_steps(),
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
        _ => unreachable!("problem routed to the wrong optimization engine"),
    }
}
