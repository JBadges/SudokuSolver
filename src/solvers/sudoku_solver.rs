use crate::sudoku_grid::SudokuGrid;

pub trait SudokuSolveMethod {
    // Returns if the rule made any deductions
    fn apply(&self, sgrid: &mut SudokuGrid) -> bool;
}

