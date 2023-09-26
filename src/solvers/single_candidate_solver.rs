use crate::sudoku_visualizer_builder::Colors;

use super::sudoku_solver::*;
use super::super::sudoku_grid::*;

pub struct SingleCandidateSolver;

// A Single Candidate Solver finds any cells which contain only 1 candidate number.
// Since a cell has 1 candidate only one number can be placed in the cell.
impl SudokuSolveMethod for SingleCandidateSolver {
    fn apply(&self, sgrid: &SudokuGrid) -> Option<SolverResult> {
        let mut visualizer_updates = Vec::new();
        let mut reductions = Vec::new();

        visualizer_updates.push(VisualizerUpdate::SetTitle("Single Candidate".to_string()));
        
        // Check for hidden singles in rows and columns
        for i in 0..9 {
            for j in 0..9 {
                if sgrid.grid[i][j] == 0 && sgrid.candidates[i][j].len() == 1 {
                    if let Some(digit) = sgrid.candidates[i][j].iter().next() {
                        visualizer_updates.push(VisualizerUpdate::ColorDigit(i, j, Colors::SOLVED_DIGIT));
                        visualizer_updates.push(VisualizerUpdate::ColorCell(i, j, Colors::CELL_USED_TO_DETERMINE_SOLUTION));
                        reductions.push(SolverAction::DigitSolve(i, j, *digit));
                    }
                }
            }
        }
        
        if !reductions.is_empty() { 
            visualizer_updates.push(VisualizerUpdate::SetDescription("These cells have only one valid candidate, making it the definitive number for that cell.".to_string())); 
            return Some((reductions, visualizer_updates))
        }
        
        None
    }
}