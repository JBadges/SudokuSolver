use super::sudoku_solver::*;
use super::super::sudoku_grid::*;
use crate::sudoku_latex_builder::SudokuLatexBuilder;

pub struct Medusa3DSolver;

impl SudokuSolveMethod for Medusa3DSolver {
    fn apply(&self, sgrid: &mut SudokuGrid, visualizer: &mut SudokuLatexBuilder) -> bool {

        let mut applied = false;
        
        applied
    }
}
