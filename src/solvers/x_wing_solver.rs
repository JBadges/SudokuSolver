use super::sudoku_solver::*;
use super::super::sudoku_grid::*;

use std::collections::HashSet;
use itertools::Itertools;

pub struct XWingSolver;

impl SudokuSolveMethod for XWingSolver {
    fn apply(&self, sgrid: &mut SudokuGrid) -> bool {
        let mut applied = false;

        // X-wing, any given number if it only appears in two cells in the same 
        // [row|col] and in matching [col|row] in another [row|col] then all
        // candidates of the same value can be removed from the row & col, except for those cells

        // X-wing rows
        for num in 1..=9 {
            let mut placement_in_rows: [HashSet<usize>; 9] = Default::default();
            for row in placement_in_rows.iter_mut() {
                *row = HashSet::new();
            }
            for row in 0..9 {
                // For each row store where the digits can go

                for col in 0..9 {
                    if sgrid.candidates[row][col].len() > 1 && sgrid.candidates[row][col].contains(&num) {
                        placement_in_rows[row].insert(col);
                    }
                }
            }

            // For all combinations of placements with 2 entries see if they both line up
            for pair in placement_in_rows.iter().enumerate().combinations(2) {
                let (row1, cols1) = pair[0];
                let (row2, cols2) = pair[1];

                if cols1.len() != 2 || cols2.len() != 2 { continue; }
                if cols1 != cols2 { continue; }

                // X-Wing
                // Remove candidate columns
                for col in cols1.iter() {
                    for row in 0..9 {
                        if row != row1 && row != row2 {
                            if sgrid.candidates[row][*col].remove(&num) {
                                println!("Solver [XWingSolver:Row] removed value {} from candidate location ({}, {})", num, row, col);
                                applied = true;
                            }
                        }
                    }
                }
            }
        }

        // X-wing columns
    for num in 1..=9 {
        let mut placement_in_cols: [HashSet<usize>; 9] = Default::default();
        for col in placement_in_cols.iter_mut() {
            *col = HashSet::new();
        }
        for col in 0..9 {
            // For each column store where the digits can go
            for row in 0..9 {
                if sgrid.candidates[row][col].len() > 1 && sgrid.candidates[row][col].contains(&num) {
                    placement_in_cols[col].insert(row);
                }
            }
        }

        // For all combinations of placements with 2 entries see if they both line up
        for pair in placement_in_cols.iter().enumerate().combinations(2) {
            let (col1, rows1) = pair[0];
            let (col2, rows2) = pair[1];

            if rows1.len() != 2 || rows2.len() != 2 { continue; }
            if rows1 != rows2 { continue; }

            // X-Wing
            // Remove candidate rows
            for row in rows1.iter() {
                for col in 0..9 {
                    if col != col1 && col != col2 {
                        if sgrid.candidates[*row][col].remove(&num) {
                            println!("Solver [XWingSolver:Col] removed value {} from candidate location ({}, {})", num, *row, col);
                            applied = true;
                        }
                    }
                }
            }
        }
    }


        applied
    }
}
