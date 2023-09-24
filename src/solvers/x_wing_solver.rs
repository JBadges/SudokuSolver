use crate::sudoku_visualizer_builder::Colors;

use super::super::sudoku_grid::*;
use super::sudoku_solver::*;

use itertools::Itertools;
use std::collections::HashSet;

pub struct XWingSolver;

impl SudokuSolveMethod for XWingSolver {
    fn apply(&self, sgrid: &SudokuGrid) -> Option<SolverResult> {
        // X-wing, any given number if it only appears in two cells in the same
        // [row|col] and in matching [col|row] in another [row|col] then all
        // candidates of the same value can be removed from the row & col, except for those cells
        for unit_type in [UnitType::Row, UnitType::Col] {
            if let Some(ret) = self.apply_x_wing_on_axis(sgrid, unit_type) {
                return Some(ret);
            };
        }

        None
    }
}

impl XWingSolver {
    // Applies the X-wing rule on either rows or columns based on the 'unit' argument
    fn apply_x_wing_on_axis(
        &self,
        sgrid: &SudokuGrid,
        unit_type: UnitType,
    ) -> Option<SolverResult> {
        for num in 1..=9 {
            // Determine all locations for this number in units
            // For an x-wing on the columns, these will be the row positions wiithin any column unit
            let mut candidate_positions: [Vec<(usize, usize)>; 9] = Default::default();

            for unit in SudokuGrid::get_all_units_from_unit_type(unit_type) {
                for (row, col) in unit {
                    if sgrid.candidates[row][col].contains(&num) {
                        match unit_type {
                            UnitType::Row => {
                                candidate_positions[row].push((row, col));
                            }
                            UnitType::Col => {
                                candidate_positions[col].push((row, col));
                            }
                            UnitType::Box => {
                                panic!("Box unit type does not make sense for an x-wing")
                            }
                        }
                    }
                }
            }

            let possible_cells_for_xwing: Vec<Vec<(usize, usize)>> = candidate_positions
                .iter()
                .filter(|&x| x.len() == 2)
                .cloned()
                .collect();

            for unit_pair in possible_cells_for_xwing.iter().clone().combinations(2) {
                let mut axis_counter = HashSet::new();
                for vec in &unit_pair {
                    for &(row, col) in *vec {
                        match unit_type {
                            UnitType::Row => {
                                axis_counter.insert(col);
                            }
                            UnitType::Col => {
                                axis_counter.insert(row);
                            }
                            UnitType::Box => {
                                panic!("Box unit type does not make sense for an x-wing")
                            }
                        }
                    }
                }

                // If we have more than 2 of the other axis then this is not an x-wing
                if axis_counter.len() != 2 {
                    continue;
                }

                // We have an x-wing check for candidate reductions.
                let mut visualizer_updates = Vec::new();
                let mut reductions = Vec::new();
                visualizer_updates.push(VisualizerUpdate::SetTitle("X-Wing".to_string()));

                for cell_pair in &unit_pair {
                    for (row, col) in SudokuGrid::get_cells_in_unit_from(unit_type, cell_pair[0]) {
                        visualizer_updates.push(VisualizerUpdate::ColorCell(
                            row,
                            col,
                            Colors::CELL_USED_TO_DETERMINE_SOLUTION,
                        ));
                        if sgrid.candidates[row][col].contains(&num) {
                            visualizer_updates.push(VisualizerUpdate::ColorCandidate(
                                row,
                                col,
                                num,
                                Colors::DIGIT_USED_TO_DETERMINE_SOLUTION,
                            ));
                        }
                    }
                }

                // Draw X-Wing chain
                let mut x_wing_cells: Vec<(usize, usize)> =
                    unit_pair.iter().flat_map(|v| v.iter()).cloned().collect();
                x_wing_cells.sort();
                let diagonal1 = (x_wing_cells[0], x_wing_cells[3]);
                let diagonal2 = (x_wing_cells[1], x_wing_cells[2]);
                visualizer_updates.push(VisualizerUpdate::CreateChain(
                    diagonal1.0 .0,
                    diagonal1.0 .1,
                    num,
                    diagonal1.1 .0,
                    diagonal1.1 .1,
                    num,
                    Colors::CHAIN_COLOR,
                ));
                visualizer_updates.push(VisualizerUpdate::CreateChain(
                    diagonal2.0 .0,
                    diagonal2.0 .1,
                    num,
                    diagonal2.1 .0,
                    diagonal2.1 .1,
                    num,
                    Colors::CHAIN_COLOR,
                ));

                for &cell_in_xwing in unit_pair[0] {
                    for (row, col) in SudokuGrid::get_cells_in_unit_from(
                        match unit_type {
                            UnitType::Row => UnitType::Col,
                            UnitType::Col => UnitType::Row,
                            UnitType::Box => {
                                panic!("Box unit type does not make sense for an x-wing")
                            }
                        },
                        cell_in_xwing,
                    ) {
                        if unit_pair[0].contains(&(row, col)) || unit_pair[1].contains(&(row, col))
                        {
                            continue;
                        }
                        visualizer_updates.push(VisualizerUpdate::ColorCell(
                            row,
                            col,
                            Colors::CELL_MARKED_FOR_CANDIDATE_REMOVEAL,
                        ));
                        if sgrid.candidates[row][col].contains(&num) {
                            visualizer_updates.push(VisualizerUpdate::ColorCandidate(
                                row,
                                col,
                                num,
                                Colors::CANDIDATE_MARKED_FOR_REMOVAL,
                            ));
                            reductions.push(SolverAction::CandidateReduction(row, col, num));
                        }
                    }
                }
                if !reductions.is_empty() {
                    return Some((reductions, visualizer_updates));
                }
            }
        }
        None
    }
}
