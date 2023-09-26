use crate::sudoku_visualizer_builder::Colors;

use super::sudoku_solver::*;
use super::super::sudoku_grid::*;

pub struct HiddenSinglesSolver;

// A Naked Singles Solver finds any unit (row, col, box) in which
// there exists a digit that can only be placed in one of the cells
impl SudokuSolveMethod for HiddenSinglesSolver {
    fn apply(&self, sgrid: &SudokuGrid) -> Option<SolverResult> {

        for unit_type in [UnitType::Box, UnitType::Row, UnitType::Col] {
            for unit in SudokuGrid::get_all_units_from_unit_type(unit_type) {
                if let Some(result) = self.find_hidden_single(sgrid, &unit, unit_type) { return Some(result); }
            }
        }

        None
    }
}

impl HiddenSinglesSolver {
    fn find_hidden_single(&self, sgrid: &SudokuGrid, vals: &Vec<(usize, usize)>, unit_type: UnitType) -> Option<SolverResult> {
        let mut visualizer_updates = Vec::new();
        let mut reductions = Vec::new();

        visualizer_updates.push(VisualizerUpdate::SetTitle("Naked Singles".to_string()));

        let mut candidate_count: Vec<i32> = vec![0; 10];
        let mut cordinate_of_candidate = vec![(0, 0); 10];
        
        for &(row, col) in vals.iter() {
            for &candidate in &sgrid.candidates[row][col] {
                candidate_count[candidate] += 1;
                cordinate_of_candidate[candidate] = (row, col);
                visualizer_updates.push(VisualizerUpdate::ColorCell(row, col, Colors::CELL_USED_TO_DETERMINE_SOLUTION));
            }
        }

        for num in 1..=9 {
            if candidate_count[num] == 1 {
                let (row, col) = cordinate_of_candidate[num];
                // Cell is already solved, we dont need to solve it again.
                if sgrid.grid[row][col] != 0 { continue; }
                visualizer_updates.push(VisualizerUpdate::ColorDigit(row, col, Colors::SOLVED_DIGIT));
                reductions.push(SolverAction::DigitSolve(row, col, num));
                visualizer_updates.push(VisualizerUpdate::SetDescription(
                    format!(
                        "In the marked {}, there is only 1 valid placement for the digit {}.",
                        match unit_type {
                            UnitType::Box => "box",
                            UnitType::Row => "row",
                            UnitType::Col => "column",
                        },
                        num
                    )
                ));
                return Some((reductions, visualizer_updates));
            }
        }

        None
    }
}