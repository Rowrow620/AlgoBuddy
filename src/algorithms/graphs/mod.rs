mod connectivity;
mod courses;
mod islands;
mod matrix_traversal;
mod word_ladder;

pub use connectivity::{
    generate_connected_components_steps, generate_graph_valid_tree_steps,
    generate_redundant_connection_steps,
};
pub use courses::{generate_course_schedule_ii_steps, generate_course_schedule_steps};
pub use islands::{generate_max_area_island_steps, generate_number_islands_steps};
pub use matrix_traversal::{
    generate_clone_graph_steps, generate_pacific_atlantic_steps, generate_rotting_oranges_steps,
    generate_surrounded_regions_steps, generate_walls_and_gates_steps,
};
pub use word_ladder::generate_word_ladder_steps;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Step;

    fn assert_characterization(
        steps: &[Step],
        expected_len: usize,
        expected_first_line: usize,
        expected_last_line: usize,
        expected_first_description: &str,
        expected_last_description: &str,
    ) {
        let first = steps.first().expect("graph generator must emit steps");
        let last = steps.last().expect("graph generator must emit steps");
        assert_eq!(steps.len(), expected_len);
        assert_eq!(first.code_line, expected_first_line);
        assert_eq!(last.code_line, expected_last_line);
        assert_eq!(first.description, expected_first_description);
        assert_eq!(last.description, expected_last_description);
    }

    #[test]
    fn graph_generators_keep_their_step_boundaries() {
        assert_characterization(
            &generate_number_islands_steps(&[
                vec!['1', '1', '0'],
                vec!['0', '1', '0'],
                vec!['1', '0', '1'],
            ]),
            7,
            4,
            16,
            "Initialize Number of Islands grid scan (3x3)",
            "Finished Grid Traversal! Total Islands = 3",
        );
        assert_characterization(
            &generate_max_area_island_steps(),
            6,
            4,
            16,
            "Initialize Number of Islands grid scan (4x5)",
            "Finished Grid Traversal! Total Islands = 2",
        );
        assert_characterization(
            &generate_clone_graph_steps(),
            5,
            3,
            7,
            "Initialize Clone Graph: Hash map mapping old -> cloned nodes",
            "Deep copying node 4 and wiring neighbor references",
        );
        assert_characterization(
            &generate_walls_and_gates_steps(),
            11,
            6,
            16,
            "Multi-Source BFS: Enqueue all Gate coordinates (0) at (0,2) and (3,0)",
            "Walls and Gates Multi-Source BFS Complete!",
        );
        assert_characterization(
            &generate_rotting_oranges_steps(),
            8,
            8,
            17,
            "Initialize Rotting Oranges BFS: Fresh = 6, Rotten = 1",
            "Rotting Oranges Complete! Total Minutes = 4",
        );
        assert_characterization(
            &generate_pacific_atlantic_steps(),
            10,
            3,
            10,
            "Pacific Atlantic Water Flow: Reverse DFS from Ocean borders uphill",
            "Pacific Atlantic Flow Complete! Dual ocean reachable cells: {}",
        );
        assert_characterization(
            &generate_surrounded_regions_steps(),
            6,
            4,
            13,
            "Surrounded Regions Step 1: Scan borders for 'O' cells",
            "Restore Border Safe Cells: Flip 'T' back to 'O' at (3,1)",
        );
        assert_characterization(
            &generate_course_schedule_steps(2, &[[1, 0]]),
            10,
            4,
            16,
            "Course Schedule: Build dependency graph with 4 courses",
            "Course Schedule Verified: No directed cycles detected! All courses can be completed.",
        );
        assert_characterization(
            &generate_course_schedule_ii_steps(4, &[[1, 0], [2, 0], [3, 1], [3, 2]]),
            5,
            4,
            12,
            "Course Schedule II: Initialize Topological Sort for 4 courses",
            "Post-Order DFS: Add Course 0 to Topological Order output array",
        );
        assert_characterization(
            &generate_graph_valid_tree_steps(5, &[[0, 1], [0, 2], [0, 3], [1, 4]]),
            7,
            3,
            14,
            "Graph Valid Tree Step 1: Verify Edge Count E == V - 1 (4 == 4)",
            "Graph Valid Tree Verified: Single connected component with 0 cycles!",
        );
        assert_characterization(
            &generate_connected_components_steps(5, &[[0, 1], [1, 2], [3, 4]]),
            4,
            3,
            15,
            "Initialize Union-Find Connected Components: Initial count = 5",
            "Union edge (3 ➔ 4) -> Merge roots, decrement components to 2",
        );
        assert_characterization(
            &generate_redundant_connection_steps(&[[1, 2], [1, 3], [2, 3]]),
            4,
            3,
            15,
            "Initialize Union-Find Cycle Edge Search",
            "Union-Find detected cycle edge [2, 3]! Redundant Connection found.",
        );
        assert_characterization(
            &generate_word_ladder_steps("hit", "cog", &["hot", "dot", "dog", "lot", "log", "cog"]),
            6,
            4,
            17,
            "Initialize Word Ladder BFS from start word 'hit'",
            "BFS Level 5: Word transformation 'cog'",
        );
    }
}
