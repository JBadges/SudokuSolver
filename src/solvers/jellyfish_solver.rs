use super::sudoku_solver::*;
use super::super::sudoku_grid::*;
use super::swordfish_solver::SwordfishSolver;


pub struct JellyfishSolver;

impl SudokuSolveMethod for JellyfishSolver {
    fn apply(&self, sgrid: &SudokuGrid) -> Option<SolverResult> {
        for unit_type in [UnitType::Row, UnitType::Col] {
            if let Some(ret) = SwordfishSolver::apply_fish_on_axis(sgrid, unit_type, 4) { return Some(ret) };
        }
        None
    }
}