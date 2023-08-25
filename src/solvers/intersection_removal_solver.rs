use super::sudoku_solver::*;
use super::super::sudoku_grid::*;

use std::collections::HashSet;

pub struct IntersectionRemovalSolver;

impl SudokuSolveMethod for IntersectionRemovalSolver {
    fn apply(&self, sgrid: &mut SudokuGrid) -> bool {
        let mut applied = false;

        // Box-Line
        for num in 1..=9 {
            for box_row in (0..9).step_by(3) {
                for box_col in (0..9).step_by(3) {
                    let mut rows_with_num = HashSet::new();
                    let mut cols_with_num = HashSet::new();

                    for i in box_row..box_row+3 {
                        for j in box_col..box_col+3 {
                            if sgrid.candidates[i][j].contains(&num) {
                                rows_with_num.insert(i);
                                cols_with_num.insert(j);
                            }
                        }
                    }

                    if rows_with_num.len() == 1 {
                        let row = *rows_with_num.iter().next().unwrap();
                        for col in 0..9 {
                            if col < box_col || col >= box_col + 3 {
                                if sgrid.candidates[row][col].remove(&num) {
                                    applied = true;
                                    println!("Solver [IntersectionRemovalSolver:BoxLine] removed values {} from candidate location ({}, {})", num, row, col);
                                }
                            }
                        }
                    }

                    if cols_with_num.len() == 1 {
                        let col = *cols_with_num.iter().next().unwrap();
                        for row in 0..9 {
                            if row < box_row || row >= box_row + 3 {
                                if sgrid.candidates[row][col].remove(&num) {
                                    applied = true;
                                    println!("Solver [IntersectionRemovalSolver:BoxLine] removed values {} from candidate location ({}, {})", num, row, col);
                                }
                            }
                        }
                    }
                }
            }
        }

        // Line-Box
        for num in 1..=9 {
            for col in 0..9 {
                let mut box_with_num = HashSet::new();
                    
                for row in 0..9 {
                    if sgrid.candidates[row][col].contains(&num) {
                        box_with_num.insert(row/3);
                    }
                }

                if box_with_num.len() == 1 {
                    let box_row = 3 * *box_with_num.iter().next().unwrap();
                    let box_col = 3 * (col / 3);
                    for box_row_offset in box_row..box_row+3 {
                        for box_col_offset in box_col..box_col+3 {
                            if box_col_offset == col { continue; }
                            if sgrid.candidates[box_row_offset][box_col_offset].remove(&num) {
                                applied = true;
                                println!("Solver [IntersectionRemovalSolver:LineBox] removed values {} from candidate location ({}, {})", num, box_row_offset, box_col_offset);
                            }
                        }
                    }
                }
            }
        }

        // Line-Box
        for num in 1..=9 {
            for row in 0..9 {
                let mut box_with_num = HashSet::new();
                
                for col in 0..9 {
                    if sgrid.candidates[row][col].contains(&num) {
                        box_with_num.insert(col/3);
                    }
                }

                if box_with_num.len() == 1 {
                    let box_col = 3 * *box_with_num.iter().next().unwrap();
                    let box_row = 3 * (row / 3);
                    for box_row_offset in box_row..box_row+3 {
                        for box_col_offset in box_col..box_col+3 {
                            if box_row_offset == row { continue; }
                            if sgrid.candidates[box_row_offset][box_col_offset].remove(&num) {
                                applied = true;
                                println!("Solver [IntersectionRemovalSolver:LineBox] removed values {} from candidate location ({}, {})", num, box_row_offset, box_col_offset);
                            }
                        }
                    }
                }
            }
        }

        applied
    }
}
