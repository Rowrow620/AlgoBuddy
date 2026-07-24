use std::collections::HashSet;
use crate::model::{Step, VisualState};

pub fn generate_valid_sudoku_steps(board: &[[char; 9]; 9]) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut rows: Vec<HashSet<char>> = vec![HashSet::new(); 9];
    let mut cols: Vec<HashSet<char>> = vec![HashSet::new(); 9];
    let mut squares: Vec<HashSet<char>> = vec![HashSet::new(); 9]; // index = (r/3)*3 + c/3

    steps.push(Step {
        code_line: 5,
        description: "Initialized HashSets for 9 rows, 9 columns, and 9 (3x3) sub-boxes.".to_string(),
        visual: VisualState::ValidSudoku {
            board: *board,
            active_r: None,
            active_c: None,
            duplicate_pos: None,
            is_valid: None,
        },
    });

    for r in 0..9 {
        for c in 0..9 {
            let val = board[r][c];
            if val == '.' {
                continue;
            }

            let box_idx = (r / 3) * 3 + (c / 3);

            steps.push(Step {
                code_line: 8,
                description: format!("Checking cell (row={}, col={}): digit '{}' (sub-box {}).", r, c, val, box_idx),
                visual: VisualState::ValidSudoku {
                    board: *board,
                    active_r: Some(r),
                    active_c: Some(c),
                    duplicate_pos: None,
                    is_valid: None,
                },
            });

            if rows[r].contains(&val) || cols[c].contains(&val) || squares[box_idx].contains(&val) {
                let duplicate_type = if rows[r].contains(&val) {
                    format!("row {}", r)
                } else if cols[c].contains(&val) {
                    format!("col {}", c)
                } else {
                    format!("3x3 sub-box {}", box_idx)
                };

                steps.push(Step {
                    code_line: 11,
                    description: format!("Duplicate digit '{}' detected in {} at cell (row={}, col={})! Board is INVALID. Return False.", val, duplicate_type, r, c),
                    visual: VisualState::ValidSudoku {
                        board: *board,
                        active_r: Some(r),
                        active_c: Some(c),
                        duplicate_pos: Some((r, c)),
                        is_valid: Some(false),
                    },
                });

                return steps;
            }

            rows[r].insert(val);
            cols[c].insert(val);
            squares[box_idx].insert(val);

            steps.push(Step {
                code_line: 12,
                description: format!("Inserted digit '{}' into row {}, col {}, and sub-box {}.", val, r, c, box_idx),
                visual: VisualState::ValidSudoku {
                    board: *board,
                    active_r: Some(r),
                    active_c: Some(c),
                    duplicate_pos: None,
                    is_valid: None,
                },
            });
        }
    }

    steps.push(Step {
        code_line: 13,
        description: "Scanned all 81 board cells. No duplicates found in any row, column, or 3x3 sub-box! Board is VALID. Return True.".to_string(),
        visual: VisualState::ValidSudoku {
            board: *board,
            active_r: None,
            active_c: None,
            duplicate_pos: None,
            is_valid: Some(true),
        },
    });

    steps
}
