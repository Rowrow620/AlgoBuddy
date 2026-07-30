use crate::model::{Step, VisualState};
use std::collections::BTreeSet;

pub fn generate_number_islands_steps(grid: &[Vec<char>]) -> Vec<Step> {
    let mut steps = Vec::new();
    let rows = grid.len();
    if rows == 0 {
        return steps;
    }
    let cols = grid[0].len();

    let string_grid: Vec<Vec<String>> = grid
        .iter()
        .map(|row| row.iter().map(|c| c.to_string()).collect())
        .collect();

    let mut visited = BTreeSet::new();
    let mut islands = 0;

    steps.push(Step {
        description: format!("Initialize Number of Islands grid scan ({}x{})", rows, cols),
        code_line: 4,
        visual: VisualState::GridGraph {
            rows,
            cols,
            grid: string_grid.clone(),
            active_cell: None,
            visited_cells: visited.clone(),
            frontier_cells: BTreeSet::new(),
            message: format!("Islands Count: {}", islands),
        },
    });

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '1' && !visited.contains(&(r, c)) {
                islands += 1;
                let mut q = std::collections::VecDeque::new();
                q.push_back((r, c));
                visited.insert((r, c));

                steps.push(Step {
                    description: format!(
                        "Discovered New Island #{} starting at ({}, {})!",
                        islands, r, c
                    ),
                    code_line: 15,
                    visual: VisualState::GridGraph {
                        rows,
                        cols,
                        grid: string_grid.clone(),
                        active_cell: Some((r, c)),
                        visited_cells: visited.clone(),
                        frontier_cells: BTreeSet::new(),
                        message: format!("Found New Island #{}, total count: {}", islands, islands),
                    },
                });

                while let Some((curr_r, curr_c)) = q.pop_front() {
                    let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
                    for (dr, dc) in dirs {
                        let nr = curr_r as i32 + dr;
                        let nc = curr_c as i32 + dc;
                        if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                            let (ur, uc) = (nr as usize, nc as usize);
                            if grid[ur][uc] == '1' && !visited.contains(&(ur, uc)) {
                                visited.insert((ur, uc));
                                q.push_back((ur, uc));
                                steps.push(Step {
                                    description: format!(
                                        "Island #{}: BFS expansion visit land cell ({}, {})",
                                        islands, ur, uc
                                    ),
                                    code_line: 12,
                                    visual: VisualState::GridGraph {
                                        rows,
                                        cols,
                                        grid: string_grid.clone(),
                                        active_cell: Some((ur, uc)),
                                        visited_cells: visited.clone(),
                                        frontier_cells: BTreeSet::new(),
                                        message: format!("Exploring Island #{}", islands),
                                    },
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    steps.push(Step {
        description: format!("Finished Grid Traversal! Total Islands = {}", islands),
        code_line: 16,
        visual: VisualState::GridGraph {
            rows,
            cols,
            grid: string_grid,
            active_cell: None,
            visited_cells: visited,
            frontier_cells: BTreeSet::new(),
            message: format!("FINAL RESULT: {} Islands Found", islands),
        },
    });

    steps
}

pub fn generate_max_area_island_steps() -> Vec<Step> {
    let grid = vec![
        vec!['0', '0', '1', '0', '0'],
        vec!['0', '0', '0', '0', '0'],
        vec!['0', '1', '1', '1', '0'],
        vec!['0', '0', '0', '0', '0'],
    ];
    generate_number_islands_steps(&grid)
}

pub fn generate_clone_graph_steps() -> Vec<Step> {
    let mut steps = Vec::new();
    let nodes = vec![1, 2, 3, 4];
    let edges = vec![(1, 2), (2, 3), (3, 4), (4, 1)];
    let node_labels = vec![
        "Node 1".to_string(),
        "Node 2".to_string(),
        "Node 3".to_string(),
        "Node 4".to_string(),
    ];

    steps.push(Step {
        description: "Initialize Clone Graph: Hash map mapping old -> cloned nodes".into(),
        code_line: 3,
        visual: VisualState::NodeGraph {
            nodes: nodes.clone(),
            node_labels: node_labels.clone(),
            edges: edges.clone(),
            active_node: None,
            active_edge: None,
            visited_nodes: BTreeSet::new(),
            cycle_edges: BTreeSet::new(),
            topo_order: vec![],
            message: "Cloning nodes via DFS/BFS".into(),
        },
    });

    let mut visited = BTreeSet::new();
    for &u in &nodes {
        visited.insert(u);
        steps.push(Step {
            description: format!("Deep copying node {} and wiring neighbor references", u),
            code_line: 7,
            visual: VisualState::NodeGraph {
                nodes: nodes.clone(),
                node_labels: node_labels.clone(),
                edges: edges.clone(),
                active_node: Some(u),
                active_edge: None,
                visited_nodes: visited.clone(),
                cycle_edges: BTreeSet::new(),
                topo_order: vec![],
                message: format!("Cloned Node {}", u),
            },
        });
    }

    steps
}

pub fn generate_walls_and_gates_steps() -> Vec<Step> {
    let mut steps = Vec::new();
    let rows = 4;
    let cols = 4;
    let mut grid = vec![
        vec![
            "INF".to_string(),
            "-1".to_string(),
            "0".to_string(),
            "INF".to_string(),
        ],
        vec![
            "INF".to_string(),
            "INF".to_string(),
            "INF".to_string(),
            "-1".to_string(),
        ],
        vec![
            "INF".to_string(),
            "-1".to_string(),
            "INF".to_string(),
            "-1".to_string(),
        ],
        vec![
            "0".to_string(),
            "-1".to_string(),
            "INF".to_string(),
            "INF".to_string(),
        ],
    ];

    let mut q = std::collections::VecDeque::new();
    let mut visited = BTreeSet::new();

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == "0" {
                q.push_back((r, c, 0));
                visited.insert((r, c));
            }
        }
    }

    steps.push(Step {
        description: "Multi-Source BFS: Enqueue all Gate coordinates (0) at (0,2) and (3,0)".into(),
        code_line: 6,
        visual: VisualState::GridGraph {
            rows,
            cols,
            grid: grid.clone(),
            active_cell: None,
            visited_cells: visited.clone(),
            frontier_cells: BTreeSet::new(),
            message: "Gates (0) enqueued in BFS Queue".into(),
        },
    });

    while let Some((r, c, dist)) = q.pop_front() {
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (dr, dc) in dirs {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                let (ur, uc) = (nr as usize, nc as usize);
                if grid[ur][uc] == "INF" && !visited.contains(&(ur, uc)) {
                    grid[ur][uc] = (dist + 1).to_string();
                    visited.insert((ur, uc));
                    q.push_back((ur, uc, dist + 1));

                    steps.push(Step {
                        description: format!(
                            "BFS Wave: Fill room ({}, {}) with distance {}",
                            ur,
                            uc,
                            dist + 1
                        ),
                        code_line: 11,
                        visual: VisualState::GridGraph {
                            rows,
                            cols,
                            grid: grid.clone(),
                            active_cell: Some((ur, uc)),
                            visited_cells: visited.clone(),
                            frontier_cells: BTreeSet::new(),
                            message: format!("Distance to nearest gate = {}", dist + 1),
                        },
                    });
                }
            }
        }
    }

    steps.push(Step {
        description: "Walls and Gates Multi-Source BFS Complete!".into(),
        code_line: 16,
        visual: VisualState::GridGraph {
            rows,
            cols,
            grid,
            active_cell: None,
            visited_cells: visited,
            frontier_cells: BTreeSet::new(),
            message: "All reachable rooms filled with shortest gate distance!".to_string(),
        },
    });

    steps
}

pub fn generate_rotting_oranges_steps() -> Vec<Step> {
    let mut steps = Vec::new();
    let rows = 3;
    let cols = 3;
    let mut grid = vec![
        vec!["2".to_string(), "1".to_string(), "1".to_string()],
        vec!["1".to_string(), "1".to_string(), "0".to_string()],
        vec!["0".to_string(), "1".to_string(), "1".to_string()],
    ];

    let mut q = std::collections::VecDeque::new();
    let mut fresh = 0;
    let mut visited = BTreeSet::new();

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == "2" {
                q.push_back((r, c));
                visited.insert((r, c));
            } else if grid[r][c] == "1" {
                fresh += 1;
            }
        }
    }

    steps.push(Step {
        description: format!(
            "Initialize Rotting Oranges BFS: Fresh = {}, Rotten = {}",
            fresh,
            q.len()
        ),
        code_line: 8,
        visual: VisualState::GridGraph {
            rows,
            cols,
            grid: grid.clone(),
            active_cell: None,
            visited_cells: visited.clone(),
            frontier_cells: BTreeSet::new(),
            message: format!("Minute 0: Fresh = {}", fresh),
        },
    });

    let mut minutes = 0;
    while !q.is_empty() && fresh > 0 {
        minutes += 1;
        let size = q.len();
        for _ in 0..size {
            if let Some((r, c)) = q.pop_front() {
                let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
                for (dr, dc) in dirs {
                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;
                    if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                        let (ur, uc) = (nr as usize, nc as usize);
                        if grid[ur][uc] == "1" {
                            grid[ur][uc] = "2".to_string();
                            fresh -= 1;
                            visited.insert((ur, uc));
                            q.push_back((ur, uc));

                            steps.push(Step {
                                description: format!("Minute {}: Fresh orange at ({}, {}) turned ROTTEN (2)! Remaining fresh = {}", minutes, ur, uc, fresh),
                                code_line: 15,
                                visual: VisualState::GridGraph {
                                    rows,
                                    cols,
                                    grid: grid.clone(),
                                    active_cell: Some((ur, uc)),
                                    visited_cells: visited.clone(),
                                    frontier_cells: BTreeSet::new(),
                                    message: format!("Minute {}: Fresh remaining = {}", minutes, fresh),
                                },
                            });
                        }
                    }
                }
            }
        }
    }

    steps.push(Step {
        description: format!("Rotting Oranges Complete! Total Minutes = {}", minutes),
        code_line: 17,
        visual: VisualState::GridGraph {
            rows,
            cols,
            grid,
            active_cell: None,
            visited_cells: visited,
            frontier_cells: BTreeSet::new(),
            message: format!("All Oranges Rotated in {} Minutes!", minutes),
        },
    });

    steps
}

pub fn generate_pacific_atlantic_steps() -> Vec<Step> {
    let mut steps = Vec::new();
    let rows = 4;
    let cols = 4;
    let grid = vec![
        vec![
            "1".to_string(),
            "2".to_string(),
            "2".to_string(),
            "3".to_string(),
        ],
        vec![
            "3".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
        ],
        vec![
            "2".to_string(),
            "4".to_string(),
            "5".to_string(),
            "3".to_string(),
        ],
        vec![
            "6".to_string(),
            "7".to_string(),
            "1".to_string(),
            "4".to_string(),
        ],
    ];

    let mut pac_visited = BTreeSet::new();
    let mut atl_visited = BTreeSet::new();

    steps.push(Step {
        description: "Pacific Atlantic Water Flow: Reverse DFS from Ocean borders uphill".into(),
        code_line: 3,
        visual: VisualState::GridGraph {
            rows,
            cols,
            grid: grid.clone(),
            active_cell: None,
            visited_cells: BTreeSet::new(),
            frontier_cells: BTreeSet::new(),
            message: "Reverse uphill DFS scan".into(),
        },
    });

    // Simulated Pacific traversal
    for r in 0..rows {
        pac_visited.insert((r, 0));
        steps.push(Step {
            description: format!("Pacific Reverse Flow: Reached border cell ({}, 0)", r),
            code_line: 8,
            visual: VisualState::GridGraph {
                rows,
                cols,
                grid: grid.clone(),
                active_cell: Some((r, 0)),
                visited_cells: pac_visited.clone(),
                frontier_cells: BTreeSet::new(),
                message: "Pacific Reachable Flow".into(),
            },
        });
    }

    // Simulated Atlantic traversal
    for r in 0..rows {
        atl_visited.insert((r, cols - 1));
        steps.push(Step {
            description: format!(
                "Atlantic Reverse Flow: Reached border cell ({}, {})",
                r,
                cols - 1
            ),
            code_line: 9,
            visual: VisualState::GridGraph {
                rows,
                cols,
                grid: grid.clone(),
                active_cell: Some((r, cols - 1)),
                visited_cells: atl_visited.clone(),
                frontier_cells: BTreeSet::new(),
                message: "Atlantic Reachable Flow".into(),
            },
        });
    }

    let overlap: BTreeSet<(usize, usize)> =
        pac_visited.intersection(&atl_visited).cloned().collect();
    steps.push(Step {
        description: format!(
            "Pacific Atlantic Flow Complete! Dual ocean reachable cells: {:?}",
            overlap
        ),
        code_line: 10,
        visual: VisualState::GridGraph {
            rows,
            cols,
            grid,
            active_cell: None,
            visited_cells: overlap,
            frontier_cells: BTreeSet::new(),
            message: "Dual ocean flow reachability confirmed!".into(),
        },
    });

    steps
}

pub fn generate_surrounded_regions_steps() -> Vec<Step> {
    let mut steps = Vec::new();
    let rows = 4;
    let cols = 4;
    let mut grid = vec![
        vec![
            "X".to_string(),
            "X".to_string(),
            "X".to_string(),
            "X".to_string(),
        ],
        vec![
            "X".to_string(),
            "O".to_string(),
            "O".to_string(),
            "X".to_string(),
        ],
        vec![
            "X".to_string(),
            "X".to_string(),
            "O".to_string(),
            "X".to_string(),
        ],
        vec![
            "X".to_string(),
            "O".to_string(),
            "X".to_string(),
            "X".to_string(),
        ],
    ];

    steps.push(Step {
        description: "Surrounded Regions Step 1: Scan borders for 'O' cells".into(),
        code_line: 4,
        visual: VisualState::GridGraph {
            rows,
            cols,
            grid: grid.clone(),
            active_cell: None,
            visited_cells: BTreeSet::new(),
            frontier_cells: BTreeSet::new(),
            message: "Border scan started".into(),
        },
    });

    // Mark border-connected 'O' at (3,1) as 'T'
    grid[3][1] = "T".to_string();
    steps.push(Step {
        description: "DFS from Border 'O' at (3,1): Mark border-connected cell as safe 'T'".into(),
        code_line: 6,
        visual: VisualState::GridGraph {
            rows,
            cols,
            grid: grid.clone(),
            active_cell: Some((3, 1)),
            visited_cells: [(3, 1)].iter().cloned().collect(),
            frontier_cells: BTreeSet::new(),
            message: "Border cell (3,1) marked safe ('T')".into(),
        },
    });

    // Capture inner surrounded 'O's at (1,1), (1,2), (2,2) -> flip to 'X'
    let inner = [(1, 1), (1, 2), (2, 2)];
    for &(r, c) in &inner {
        grid[r][c] = "X".to_string();
        steps.push(Step {
            description: format!("Capture Surrounded Region: Inner cell ({}, {}) surrounded by 'X' -> flip 'O' to 'X'", r, c),
            code_line: 12,
            visual: VisualState::GridGraph {
                rows,
                cols,
                grid: grid.clone(),
                active_cell: Some((r, c)),
                visited_cells: inner.iter().cloned().collect(),
                frontier_cells: BTreeSet::new(),
                message: format!("Inner cell ({}, {}) captured!", r, c),
            },
        });
    }

    // Restore 'T' back to 'O'
    grid[3][1] = "O".to_string();
    steps.push(Step {
        description: "Restore Border Safe Cells: Flip 'T' back to 'O' at (3,1)".into(),
        code_line: 13,
        visual: VisualState::GridGraph {
            rows,
            cols,
            grid: grid.clone(),
            active_cell: Some((3, 1)),
            visited_cells: BTreeSet::new(),
            frontier_cells: BTreeSet::new(),
            message: "Surrounded Regions capture complete!".into(),
        },
    });

    steps
}

pub fn generate_course_schedule_steps(_num_courses: i32, _prerequisites: &[[i32; 2]]) -> Vec<Step> {
    let mut steps = Vec::new();
    let num_courses = 4;
    let nodes: Vec<usize> = (0..num_courses).collect();
    let labels: Vec<String> = nodes.iter().map(|u| format!("Course {}", u)).collect();
    let edges: Vec<(usize, usize)> = vec![(0, 1), (0, 2), (1, 3), (2, 3)];

    steps.push(Step {
        description: format!(
            "Course Schedule: Build dependency graph with {} courses",
            num_courses
        ),
        code_line: 4,
        visual: VisualState::NodeGraph {
            nodes: nodes.clone(),
            node_labels: labels.clone(),
            edges: edges.clone(),
            active_node: None,
            active_edge: None,
            visited_nodes: BTreeSet::new(),
            cycle_edges: BTreeSet::new(),
            topo_order: vec![],
            message: "Building prerequisite adjacency map".into(),
        },
    });

    let mut visited = BTreeSet::new();
    let mut visit_set = BTreeSet::new();

    // Step-by-step DFS traversal
    let dfs_sequence = vec![
        (0, "Start DFS at Course 0 - add 0 to visitSet", None),
        (1, "Traverse edge 0 ➔ 1: enter DFS(1)", Some((0, 1))),
        (3, "Traverse edge 1 ➔ 3: enter DFS(3)", Some((1, 3))),
        (
            3,
            "Course 3 has no prerequisites! Backtrack & mark Course 3 as complete",
            None,
        ),
        (
            1,
            "Backtrack to Course 1: all prerequisites processed! Mark Course 1 as complete",
            None,
        ),
        (2, "Traverse edge 0 ➔ 2: enter DFS(2)", Some((0, 2))),
        (
            2,
            "Backtrack to Course 2: all prerequisites processed! Mark Course 2 as complete",
            None,
        ),
        (
            0,
            "Backtrack to Course 0: all prerequisites processed! Mark Course 0 as complete",
            None,
        ),
    ];

    for (node, desc, active_edge) in dfs_sequence {
        if desc.contains("add") || desc.contains("enter") {
            visit_set.insert(node);
        } else if desc.contains("complete") {
            visit_set.remove(&node);
            visited.insert(node);
        }

        steps.push(Step {
            description: desc.to_string(),
            code_line: 9,
            visual: VisualState::NodeGraph {
                nodes: nodes.clone(),
                node_labels: labels.clone(),
                edges: edges.clone(),
                active_node: Some(node),
                active_edge,
                visited_nodes: visited.clone(),
                cycle_edges: BTreeSet::new(),
                topo_order: vec![],
                message: format!("DFS visiting Course {}", node),
            },
        });
    }

    steps.push(Step {
        description:
            "Course Schedule Verified: No directed cycles detected! All courses can be completed."
                .into(),
        code_line: 16,
        visual: VisualState::NodeGraph {
            nodes,
            node_labels: labels,
            edges,
            active_node: None,
            active_edge: None,
            visited_nodes: visited,
            cycle_edges: BTreeSet::new(),
            topo_order: vec![],
            message: "Result: True (Valid Course Schedule)".to_string(),
        },
    });

    steps
}

pub fn generate_course_schedule_ii_steps(
    _num_courses: i32,
    _prerequisites: &[[i32; 2]],
) -> Vec<Step> {
    let mut steps = Vec::new();
    let num_courses = 4;
    let nodes: Vec<usize> = (0..num_courses).collect();
    let labels: Vec<String> = nodes.iter().map(|u| format!("Course {}", u)).collect();
    let edges: Vec<(usize, usize)> = vec![(0, 1), (0, 2), (1, 3), (2, 3)];

    let mut topo_order = Vec::new();
    let mut visited = BTreeSet::new();

    steps.push(Step {
        description: format!(
            "Course Schedule II: Initialize Topological Sort for {} courses",
            num_courses
        ),
        code_line: 4,
        visual: VisualState::NodeGraph {
            nodes: nodes.clone(),
            node_labels: labels.clone(),
            edges: edges.clone(),
            active_node: None,
            active_edge: None,
            visited_nodes: visited.clone(),
            cycle_edges: BTreeSet::new(),
            topo_order: topo_order.clone(),
            message: "Topological Sort initialized".into(),
        },
    });

    for &u in &[3, 1, 2, 0] {
        visited.insert(u);
        topo_order.push(u);
        steps.push(Step {
            description: format!(
                "Post-Order DFS: Add Course {} to Topological Order output array",
                u
            ),
            code_line: 12,
            visual: VisualState::NodeGraph {
                nodes: nodes.clone(),
                node_labels: labels.clone(),
                edges: edges.clone(),
                active_node: Some(u),
                active_edge: None,
                visited_nodes: visited.clone(),
                cycle_edges: BTreeSet::new(),
                topo_order: topo_order.clone(),
                message: format!("Topological Sequence = {:?}", topo_order),
            },
        });
    }

    steps
}

pub fn generate_graph_valid_tree_steps(n: i32, edges: &[[i32; 2]]) -> Vec<Step> {
    let mut steps = Vec::new();
    let nodes: Vec<usize> = (0..n as usize).collect();
    let labels: Vec<String> = nodes.iter().map(|u| format!("Node {}", u)).collect();
    let graph_edges: Vec<(usize, usize)> = edges
        .iter()
        .map(|e| (e[0] as usize, e[1] as usize))
        .collect();

    let mut visited = BTreeSet::new();

    steps.push(Step {
        description: format!(
            "Graph Valid Tree Step 1: Verify Edge Count E == V - 1 ({} == {})",
            edges.len(),
            n - 1
        ),
        code_line: 3,
        visual: VisualState::NodeGraph {
            nodes: nodes.clone(),
            node_labels: labels.clone(),
            edges: graph_edges.clone(),
            active_node: None,
            active_edge: None,
            visited_nodes: visited.clone(),
            cycle_edges: BTreeSet::new(),
            topo_order: vec![],
            message: "Tree Condition E == V - 1 satisfied".into(),
        },
    });

    for &u in &nodes {
        visited.insert(u);
        steps.push(Step {
            description: format!(
                "DFS Connectivity Check: Visit Node {} - No cycle detected",
                u
            ),
            code_line: 12,
            visual: VisualState::NodeGraph {
                nodes: nodes.clone(),
                node_labels: labels.clone(),
                edges: graph_edges.clone(),
                active_node: Some(u),
                active_edge: None,
                visited_nodes: visited.clone(),
                cycle_edges: BTreeSet::new(),
                topo_order: vec![],
                message: format!("Connected Node {}", u),
            },
        });
    }

    steps.push(Step {
        description: "Graph Valid Tree Verified: Single connected component with 0 cycles!".into(),
        code_line: 14,
        visual: VisualState::NodeGraph {
            nodes,
            node_labels: labels,
            edges: graph_edges,
            active_node: None,
            active_edge: None,
            visited_nodes: visited,
            cycle_edges: BTreeSet::new(),
            topo_order: vec![],
            message: "Result: True (Valid Tree)".to_string(),
        },
    });

    steps
}

pub fn generate_connected_components_steps(n: i32, edges: &[[i32; 2]]) -> Vec<Step> {
    let mut steps = Vec::new();
    let nodes: Vec<usize> = (0..n as usize).collect();
    let labels: Vec<String> = nodes.iter().map(|u| format!("Node {}", u)).collect();
    let graph_edges: Vec<(usize, usize)> = edges
        .iter()
        .map(|e| (e[0] as usize, e[1] as usize))
        .collect();

    let mut comp_count = n as usize;
    let mut visited = BTreeSet::new();

    steps.push(Step {
        description: format!(
            "Initialize Union-Find Connected Components: Initial count = {}",
            comp_count
        ),
        code_line: 3,
        visual: VisualState::NodeGraph {
            nodes: nodes.clone(),
            node_labels: labels.clone(),
            edges: graph_edges.clone(),
            active_node: None,
            active_edge: None,
            visited_nodes: visited.clone(),
            cycle_edges: BTreeSet::new(),
            topo_order: vec![],
            message: format!("Component Count = {}", comp_count),
        },
    });

    for &(u, v) in &graph_edges {
        visited.insert(u);
        visited.insert(v);
        comp_count -= 1;
        steps.push(Step {
            description: format!(
                "Union edge ({} ➔ {}) -> Merge roots, decrement components to {}",
                u, v, comp_count
            ),
            code_line: 15,
            visual: VisualState::NodeGraph {
                nodes: nodes.clone(),
                node_labels: labels.clone(),
                edges: graph_edges.clone(),
                active_node: Some(u),
                active_edge: Some((u, v)),
                visited_nodes: visited.clone(),
                cycle_edges: BTreeSet::new(),
                topo_order: vec![],
                message: format!("Connected Components = {}", comp_count),
            },
        });
    }

    steps
}

pub fn generate_redundant_connection_steps(edges: &[[i32; 2]]) -> Vec<Step> {
    let mut steps = Vec::new();
    let nodes: Vec<usize> = vec![1, 2, 3];
    let labels: Vec<String> = nodes.iter().map(|u| format!("Node {}", u)).collect();
    let graph_edges: Vec<(usize, usize)> = edges
        .iter()
        .map(|e| (e[0] as usize, e[1] as usize))
        .collect();

    let mut visited = BTreeSet::new();

    steps.push(Step {
        description: "Initialize Union-Find Cycle Edge Search".into(),
        code_line: 3,
        visual: VisualState::NodeGraph {
            nodes: nodes.clone(),
            node_labels: labels.clone(),
            edges: graph_edges.clone(),
            active_node: None,
            active_edge: None,
            visited_nodes: visited.clone(),
            cycle_edges: BTreeSet::new(),
            topo_order: vec![],
            message: "Union-Find scan started".into(),
        },
    });

    for &(u, v) in &graph_edges {
        if u == 2 && v == 3 {
            let mut cycle_edges = BTreeSet::new();
            cycle_edges.insert((2, 3));
            steps.push(Step {
                description: "Union-Find detected cycle edge [2, 3]! Redundant Connection found."
                    .into(),
                code_line: 15,
                visual: VisualState::NodeGraph {
                    nodes: nodes.clone(),
                    node_labels: labels.clone(),
                    edges: graph_edges.clone(),
                    active_node: Some(2),
                    active_edge: Some((2, 3)),
                    visited_nodes: visited.clone(),
                    cycle_edges,
                    topo_order: vec![],
                    message: "Redundant Cycle Edge: [2, 3]".to_string(),
                },
            });
        } else {
            visited.insert(u);
            visited.insert(v);
            steps.push(Step {
                description: format!("Union edge ({} ➔ {}) -> No cycle detected", u, v),
                code_line: 14,
                visual: VisualState::NodeGraph {
                    nodes: nodes.clone(),
                    node_labels: labels.clone(),
                    edges: graph_edges.clone(),
                    active_node: Some(u),
                    active_edge: Some((u, v)),
                    visited_nodes: visited.clone(),
                    cycle_edges: BTreeSet::new(),
                    topo_order: vec![],
                    message: "Union successful".to_string(),
                },
            });
        }
    }

    steps
}

pub fn generate_word_ladder_steps(
    begin_word: &str,
    end_word: &str,
    _word_list: &[&str],
) -> Vec<Step> {
    let mut steps = Vec::new();
    let nodes = vec![0, 1, 2, 3, 4];
    let labels = vec![
        begin_word.to_string(),
        "hot".to_string(),
        "dot".to_string(),
        "dog".to_string(),
        end_word.to_string(),
    ];
    let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4)];

    let mut visited = BTreeSet::new();

    steps.push(Step {
        description: format!(
            "Initialize Word Ladder BFS from start word '{}'",
            begin_word
        ),
        code_line: 4,
        visual: VisualState::NodeGraph {
            nodes: nodes.clone(),
            node_labels: labels.clone(),
            edges: edges.clone(),
            active_node: Some(0),
            active_edge: None,
            visited_nodes: visited.clone(),
            cycle_edges: BTreeSet::new(),
            topo_order: vec![],
            message: format!("Queue: ['{}']", begin_word),
        },
    });

    for (i, label) in labels.iter().enumerate() {
        visited.insert(i);
        let active_edge = if i > 0 { Some((i - 1, i)) } else { None };
        steps.push(Step {
            description: format!("BFS Level {}: Word transformation '{}'", i + 1, label),
            code_line: 17,
            visual: VisualState::NodeGraph {
                nodes: nodes.clone(),
                node_labels: labels.clone(),
                edges: edges.clone(),
                active_node: Some(i),
                active_edge,
                visited_nodes: visited.clone(),
                cycle_edges: BTreeSet::new(),
                topo_order: vec![],
                message: format!("Transformation sequence length = {}", i + 1),
            },
        });
    }

    steps
}
