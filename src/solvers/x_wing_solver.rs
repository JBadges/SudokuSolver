use super::sudoku_solver::*;
use super::super::sudoku_grid::*;

use std::collections::HashSet;
use itertools::Itertools;

pub struct XWingSolver;

impl SudokuSolveMethod for XWingSolver {
    fn apply(&self, sgrid: &mut SudokuGrid) -> bool {
        // X-wing, any given number if it only appears in two cells in the same 
        // [row|col] and in matching [col|row] in another [row|col] then all
        // candidates of the same value can be removed from the row & col, except for those cells

        self.apply_x_wing_on_axis(sgrid, GridAxis::Row) | self.apply_x_wing_on_axis(sgrid, GridAxis::Col)
    }
}

impl XWingSolver {
    // Applies the X-wing rule on either rows or columns based on the 'axis' argument
    fn apply_x_wing_on_axis(&self, sgrid: &mut SudokuGrid, axis: GridAxis) -> bool {
        let mut changes_made = false;
        
        for num in 1..=9 {
            let mut candidate_positions: [HashSet<usize>; 9] = Default::default();
            
            for primary in 0..9 {
                for secondary in 0..9 {
                    let (row, col) = match &axis {
                        GridAxis::Row => (primary, secondary),
                        GridAxis::Col => (secondary, primary),
                        _ => panic!("Box axis does not make sense for xwing")
                    };
                    
                    if sgrid.candidates[row][col].len() > 1 && sgrid.candidates[row][col].contains(&num) {
                        candidate_positions[primary].insert(secondary);
                    }
                }
            }

            for pair in candidate_positions.iter().enumerate().combinations(2) {
                // Positions are the two locations of the digit within an axis
                let (index1, positions1) = pair[0];
                let (index2, positions2) = pair[1];

                // Two two positions of any given candidate must only appear twice in any Axis
                if positions1.len() != 2 || positions1 != positions2 { continue; }

                for &pos in positions1.iter() {
                    for primary in 0..9 {
                        if primary != index1 && primary != index2 {
                            let (row, col) = match axis {
                                GridAxis::Row => (primary, pos),
                                GridAxis::Col => (pos, primary),
                                _ => panic!("Box axis does not make sense for xwing")
                            };

                            if sgrid.candidates[row][col].remove(&num) {
                                println!("Removed value {} from candidate location ({}, {})", num, row, col);
                                changes_made = true;
                            }
                        }
                    }
                }
            }
        }
        
        changes_made
    }

}
