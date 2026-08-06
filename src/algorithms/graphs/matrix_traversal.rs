use crate::model::{Step, VisualState};
use std::collections::BTreeSet;

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
