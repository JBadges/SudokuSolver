use crate::sudoku_visualizer_builder::Colors;

use super::sudoku_solver::*;
use super::super::sudoku_grid::*;

use std::collections::HashSet;
use itertools::Itertools;

pub struct NakedCandidatesSolver<const NUM_CANDIDATES: usize>;

// A Naked Candidates Solver finds a group of n digits across any unit.
// Once found, it can remove all instances of those candidates in every shared unit.
impl<const NUM_CANDIDATES: usize> SudokuSolveMethod for NakedCandidatesSolver<NUM_CANDIDATES> {
    fn apply(&self, sgrid: &SudokuGrid) -> Option<SolverResult> {

        for unit_type in [UnitType::Box, UnitType::Row, UnitType::Col] {
            if let Some(ret) = self.check_units(sgrid, NUM_CANDIDATES, unit_type) { return Some(ret); };
        }

        None
    }
}

impl<const NUM_CANDIDATES: usize> NakedCandidatesSolver<NUM_CANDIDATES> {
    fn check_units(&self, sgrid: &SudokuGrid, combs: usize, unit_type: UnitType) -> Option<SolverResult> {       
        for all_cells in SudokuGrid::get_all_units_from_unit_type(unit_type) {
            let unsolved_cells: Vec<(usize, usize)> = all_cells.iter().filter(|&&(row, col)| sgrid.grid[row][col] == 0).cloned().collect();
            for n_cell_combination in unsolved_cells.iter().cloned().combinations(combs) {
                let all_candidates: HashSet<usize> = n_cell_combination.iter().flat_map(|(row, col)| &sgrid.candidates[*row][*col]).cloned().collect();
                // If we didnt find n candidates for n cells then this is not a naked candidate.
                if all_candidates.len() != combs { continue; }

                let mut visualizer_updates = Vec::new();
                let mut reductions = Vec::new();

                visualizer_updates.push(VisualizerUpdate::SetTitle(
                    format!("Naked Candidates {}", match combs {
                        2 => "Pairs",
                        3 => "Triples",
                        4 => "Quads",
                        _ => "Unkown",
                    })
                ));

                for &(row, col) in &n_cell_combination {
                    for &candidate in &sgrid.candidates[row][col] {
                        visualizer_updates.push(VisualizerUpdate::ColorCandidate(row, col, candidate, Colors::DIGIT_USED_TO_DETERMINE_SOLUTION));
                    }
                    visualizer_updates.push(VisualizerUpdate::ColorCell(row, col, Colors::CELL_USED_TO_DETERMINE_SOLUTION));
                }

                for unit in SudokuGrid::get_contained_units(&n_cell_combination) {
                    for (row, col) in SudokuGrid::get_cells_in_unit_from(unit, n_cell_combination[0]) {
                        if n_cell_combination.contains(&&(row, col)) { continue; }
                        visualizer_updates.push(VisualizerUpdate::ColorCell(row, col, Colors::CELL_MARKED_FOR_CANDIDATE_REMOVEAL));
                        for &num in &all_candidates {
                            if sgrid.candidates[row][col].contains(&num) {
                                visualizer_updates.push(VisualizerUpdate::ColorCandidate(row, col, num, Colors::CANDIDATE_MARKED_FOR_REMOVAL));
                                reductions.push(SolverAction::CandidateReduction(row, col, num));
                            }
                        }
                    }
                }
                if !reductions.is_empty() { return Some((reductions, visualizer_updates)); }
            }
        }

        None
    }
}