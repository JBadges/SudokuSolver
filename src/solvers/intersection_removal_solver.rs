use crate::sudoku_visualizer_builder::Colors;

use super::super::sudoku_grid::*;
use super::sudoku_solver::*;

pub struct IntersectionRemovalSolver;

// An intersection removal incorporates both pointing pairs/triples and
// box line reductions.

// pointing pairs/triples is a reduction where if we only have 2/3 places for a digit within a box, and they are
// all on the same row/col, those candidate numbers can be removed from all other instances in that row/col.

// box line reductions is similar, if on any row/col you can only place a number in 2/3 places and they are all contained within
// one box, the candidate number of that line can be removed from the rest of the box. Its like the opposite of a pointing pair.
impl SudokuSolveMethod for IntersectionRemovalSolver {
    fn apply(&self, sgrid: &SudokuGrid) -> Option<SolverResult> {
        for unit_type in [UnitType::Row, UnitType::Col] {
            if let Some(ret) = self.pointing_pair(sgrid, unit_type) { return Some(ret) };
        }

        for unit_type in [UnitType::Row, UnitType::Col] {
            if let Some(ret) = self.box_line_reduction(sgrid, unit_type) { return Some(ret) };
        }

        None
    }
}

impl IntersectionRemovalSolver {
    fn pointing_pair(&self, sgrid: &SudokuGrid, pointing_check_unit: UnitType) -> Option<SolverResult> {

        for unit in &SudokuGrid::get_all_units_from_unit_type(UnitType::Box) {
            for num in 1..=9 {

                // Find all of the rows in which this number appears
                let appeared_in_unit: Vec<(usize, usize)> = unit
                    .iter()
                    .filter(|&&(row, col)| sgrid.candidates[row][col].contains(&num) && sgrid.grid[row][col] == 0)
                    .cloned()
                    .collect();


                // The cells must all be contained by the pointing direction unit given
                if !SudokuGrid::get_contained_units(&appeared_in_unit).contains(&pointing_check_unit) { continue; }

                let mut reductions = Vec::default();
                let mut visualizer_updates = Vec::default();

                visualizer_updates.push(VisualizerUpdate::SetTitle("Intersection Removal:Pointing Pair".to_string()));
                visualizer_updates.push(VisualizerUpdate::SetDescription(
                    format!(
                        "The only placement for the digit {0} within the marked box lies in a {1}. Therefore this {1} must contain this digit within this box. All other cells in the same {1} can be reduced.",
                        num,
                        match pointing_check_unit {
                            UnitType::Row => "row",
                            UnitType::Col => "column",
                            UnitType::Box => panic!("Box type makes no sense"),
                        }
                    )
                ));

                for &(row, col) in unit {
                    visualizer_updates.push(VisualizerUpdate::ColorCell(row, col, Colors::CELL_USED_TO_DETERMINE_SOLUTION));
                }

                for &(row, col) in &appeared_in_unit {
                    if sgrid.candidates[row][col].contains(&num) {
                        visualizer_updates.push(VisualizerUpdate::ColorCandidate(row, col, num, Colors::DIGIT_USED_TO_DETERMINE_SOLUTION));
                    }
                }


                // All digits in the rows/col except for those in the box used can be candadite reduced.
                for position_on_unit in 0..9 {
                    let (row, col) = match pointing_check_unit {
                        UnitType::Row => (appeared_in_unit[0].0, position_on_unit),
                        UnitType::Col => (position_on_unit, appeared_in_unit[0].1),
                        UnitType::Box => panic!("Box type makes no sense"),
                    };
                    if unit.contains(&(row, col)) { continue; }
                    visualizer_updates.push(VisualizerUpdate::ColorCell(row, col, Colors::CELL_MARKED_FOR_CANDIDATE_REMOVEAL));
                    if sgrid.candidates[row][col].contains(&num) {
                        visualizer_updates.push(VisualizerUpdate::ColorCandidate(row, col, num, Colors::CANDIDATE_MARKED_FOR_REMOVAL));
                        reductions.push(SolverAction::CandidateReduction(row, col, num));
                    }
                }                

                if !reductions.is_empty() { return Some((reductions, visualizer_updates)); }
            }
        }

        None
    }

    fn box_line_reduction(&self, sgrid: &SudokuGrid, pointing_check_unit: UnitType) -> Option<SolverResult> {

        for unit in &SudokuGrid::get_all_units_from_unit_type(pointing_check_unit) {
            for num in 1..=9 {

                // Find all of the rows in which this number appears
                let appeared_in_unit: Vec<(usize, usize)> = unit
                    .iter()
                    .filter(|&&(row, col)| sgrid.candidates[row][col].contains(&num) && sgrid.grid[row][col] == 0)
                    .cloned()
                    .collect();


                // It must appear in the same box for the reduction
                if !SudokuGrid::get_contained_units(&appeared_in_unit).contains(&UnitType::Box) { continue; }

                let mut reductions = Vec::default();
                let mut visualizer_updates = Vec::default();

                visualizer_updates.push(VisualizerUpdate::SetTitle("Intersection Removal:Box Line".to_string()));
                visualizer_updates.push(VisualizerUpdate::SetDescription(
                    format!(
                        "The only placement for the digit {0} within the marked {1} lies in a box. Therefore this box must contain this digit within this {1}. All other cells that are in the same box can be reduced.",
                        num,
                        match pointing_check_unit {
                            UnitType::Row => "row",
                            UnitType::Col => "column",
                            UnitType::Box => panic!("Box type makes no sense"),
                        }
                    )
                ));

                for &(row, col) in unit {
                    visualizer_updates.push(VisualizerUpdate::ColorCell(row, col, Colors::CELL_USED_TO_DETERMINE_SOLUTION));
                    if sgrid.candidates[row][col].contains(&num) {
                        visualizer_updates.push(VisualizerUpdate::ColorCandidate(row, col, num, Colors::DIGIT_USED_TO_DETERMINE_SOLUTION));
                    }
                }

                // All digits in the box except for those in the row/col unit used can be candadite reduced.
                for &(row, col) in SudokuGrid::get_cells_in_unit_from(UnitType::Box, appeared_in_unit[0]).iter().filter(|&cord| !unit.contains(cord)) {
                    visualizer_updates.push(VisualizerUpdate::ColorCell(row, col, Colors::CELL_MARKED_FOR_CANDIDATE_REMOVEAL));
                    if sgrid.candidates[row][col].contains(&num) {
                        visualizer_updates.push(VisualizerUpdate::ColorCandidate(row, col, num, Colors::CANDIDATE_MARKED_FOR_REMOVAL));
                        reductions.push(SolverAction::CandidateReduction(row, col, num));
                    }
                }                

                if !reductions.is_empty() { return Some((reductions, visualizer_updates)); }
            }
        }

        None
    }
}