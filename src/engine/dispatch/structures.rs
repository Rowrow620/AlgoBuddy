use super::*;

pub(super) fn generate_steps(app: &mut VisualizerApp) -> Vec<Step> {
    let app_id = app.selected_approach_id;
    match app.current_problem {
        Problem::ReverseLinkedList => {
            let nodes = input::i32_list(
                app,
                Problem::ReverseLinkedList,
                "nodes",
                "0, 1, 2, 3",
                &[0, 1, 2, 3],
            );
            generate_reverse_linked_list_steps(&nodes, app_id)
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
            generate_merge_two_lists_steps(&l1, &l2, app_id)
        }
        Problem::LinkedListCycle => {
            let nodes: Vec<i32> = app
                .get_input_str(Problem::LinkedListCycle, "nodes", "1, 2, 3, 4")
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            let cycle_idx = app.get_input_int(Problem::LinkedListCycle, "cycle_idx", 1);
            generate_linked_list_cycle_steps(&nodes, cycle_idx, app_id)
        }
        Problem::InvertTree => {
            let tree = app.parse_tree_input();
            generate_invert_tree_steps(&tree, app_id)
        }
        Problem::MaxDepthTree => {
            let tree = app.parse_tree_input();
            generate_max_depth_tree_steps(&tree, app_id)
        }
        Problem::DiameterTree => {
            let tree = app.parse_tree_input();
            generate_diameter_tree_steps(&tree, app_id)
        }
        Problem::BalancedTree => {
            let tree = app.parse_tree_input();
            generate_balanced_tree_steps(&tree, app_id)
        }
        Problem::SameTree => {
            let p = crate::utils::parse_tree_nodes(
                app.get_input_str(Problem::SameTree, "tree_p", "1, 2, 3"),
                &[Some(1), Some(2), Some(3)],
            );
            let q = crate::utils::parse_tree_nodes(
                app.get_input_str(Problem::SameTree, "tree_q", "1, 2, 3"),
                &[Some(1), Some(2), Some(3)],
            );
            generate_same_tree_steps(&p, &q, app_id)
        }
        Problem::Subtree => {
            let root = crate::utils::parse_tree_nodes(
                app.get_input_str(Problem::Subtree, "tree_root", "3, 4, 5, 1, 2"),
                &[Some(3), Some(4), Some(5), Some(1), Some(2)],
            );
            let sub_root = crate::utils::parse_tree_nodes(
                app.get_input_str(Problem::Subtree, "tree_sub_root", "4, 1, 2"),
                &[Some(4), Some(1), Some(2)],
            );
            generate_subtree_steps(&root, &sub_root, app_id)
        }
        Problem::LowestCommonAncestorBst => generate_lowest_common_ancestor_bst_steps(
            &[
                Some(6),
                Some(2),
                Some(8),
                Some(0),
                Some(4),
                Some(7),
                Some(9),
                None,
                None,
                Some(3),
                Some(5),
            ],
            2,
            8,
        ),
        Problem::KthLargestStream => generate_kth_largest_stream_steps(3, &[4, 5, 8, 2], 3, app_id),
        Problem::LastStone => generate_last_stone_weight_steps(&[2, 7, 4, 1, 8, 1], app_id),
        Problem::ImplementTrie => {
            let insert_words = input::string_list_allow_empty(
                app,
                Problem::ImplementTrie,
                "words",
                "apple, app, ape",
            );
            let search_word = app.get_input_str(Problem::ImplementTrie, "search", "app");
            generate_implement_trie_steps(&insert_words, search_word)
        }
        Problem::WordDictionary => {
            let words = input::string_list_allow_empty(
                app,
                Problem::WordDictionary,
                "words",
                "bad, dad, mad",
            );
            let pattern = app.get_input_str(Problem::WordDictionary, "pattern", ".ad");
            generate_word_dictionary_steps(&words, pattern)
        }
        Problem::WordSearchII => {
            let words = input::string_list_allow_empty(
                app,
                Problem::WordSearchII,
                "words",
                "oath, pea, eat, rain",
            );
            generate_word_search_ii_steps(&words)
        }
        Problem::KClosestPoints => generate_k_closest_points_steps(&[(1, 3), (-2, 2), (5, 8)], 1),
        Problem::TaskScheduler => generate_task_scheduler_steps(&['A', 'A', 'A', 'B', 'B', 'B'], 2),
        Problem::FindMedianDataStream => {
            let nums = input::i32_list(
                app,
                Problem::FindMedianDataStream,
                "nums",
                "1, 2, 5, 10, 3",
                &[1, 2, 5, 10, 3],
            );
            generate_find_median_steps(&nums)
        }
        Problem::KthLargestArray => generate_kth_largest_array_steps(&[3, 2, 1, 5, 6, 4], 2),
        Problem::DesignTwitter => generate_design_twitter_steps(),
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
        Problem::ValidateBinarySearchTree => {
            generate_validate_bst_steps(&[Some(2), Some(1), Some(3)])
        }
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
        _ => unreachable!("problem routed to the wrong structures engine"),
    }
}
