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
    let grid = [
        ['0', '0', '1', '0', '0'],
        ['0', '0', '0', '0', '0'],
        ['0', '1', '1', '1', '0'],
        ['0', '0', '0', '0', '0'],
    ];
    let rows = grid.len();
    let cols = grid[0].len();
    let string_grid: Vec<Vec<String>> = grid
        .iter()
        .map(|row| row.iter().map(char::to_string).collect())
        .collect();
    let mut visited = BTreeSet::new();
    let mut max_area = 0;
    let mut steps = vec![Step {
        description: format!("Initialize Max Area of Island scan for a {rows}x{cols} grid."),
        code_line: 3,
        visual: VisualState::GridGraph {
            rows,
            cols,
            grid: string_grid.clone(),
            active_cell: None,
            visited_cells: visited.clone(),
            frontier_cells: BTreeSet::new(),
            message: "Maximum island area: 0".to_string(),
        },
    }];

    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] != '1' || visited.contains(&(row, col)) {
                continue;
            }

            let mut area = 0;
            let mut stack = vec![(row, col)];
            visited.insert((row, col));

            while let Some((current_row, current_col)) = stack.pop() {
                area += 1;
                steps.push(Step {
                    description: format!(
                        "Visit land cell ({current_row}, {current_col}); current island area = {area}."
                    ),
                    code_line: 6,
                    visual: VisualState::GridGraph {
                        rows,
                        cols,
                        grid: string_grid.clone(),
                        active_cell: Some((current_row, current_col)),
                        visited_cells: visited.clone(),
                        frontier_cells: stack.iter().copied().collect(),
                        message: format!("Current island area: {area}; maximum: {max_area}"),
                    },
                });

                for (row_delta, col_delta) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    let next_row = current_row as i32 + row_delta;
                    let next_col = current_col as i32 + col_delta;
                    if next_row >= 0
                        && next_row < rows as i32
                        && next_col >= 0
                        && next_col < cols as i32
                    {
                        let next = (next_row as usize, next_col as usize);
                        if grid[next.0][next.1] == '1' && visited.insert(next) {
                            stack.push(next);
                        }
                    }
                }
            }

            max_area = max_area.max(area);
            steps.push(Step {
                description: format!(
                    "Island complete with area {area}; maximum area is now {max_area}."
                ),
                code_line: 10,
                visual: VisualState::GridGraph {
                    rows,
                    cols,
                    grid: string_grid.clone(),
                    active_cell: None,
                    visited_cells: visited.clone(),
                    frontier_cells: BTreeSet::new(),
                    message: format!("Maximum island area: {max_area}"),
                },
            });
        }
    }

    steps.push(Step {
        description: format!("Grid scan complete; maximum island area = {max_area}."),
        code_line: 11,
        visual: VisualState::GridGraph {
            rows,
            cols,
            grid: string_grid,
            active_cell: None,
            visited_cells: visited,
            frontier_cells: BTreeSet::new(),
            message: format!("Final maximum island area: {max_area}"),
        },
    });

    steps
}
