use crate::sudoku_visualizer_builder::Colors;

use super::sudoku_solver::*;
use super::super::sudoku_grid::*;

use std::collections::{HashMap, HashSet};
use itertools::Itertools;

pub struct HiddenCandidatesSolver<const NUM_CANDIDATES: usize>;

// Hidden candidates are like naked candidates except there can be extra
// candidates inside its cells. If two numbers can only go in two cells 
// it does not matter if other numbers exist as those two numbers MUST 
// go in either of those cells
impl<const NUM_CANDIDATES: usize> SudokuSolveMethod for HiddenCandidatesSolver<NUM_CANDIDATES> {
    fn apply(&self, sgrid: &SudokuGrid) -> Option<SolverResult> {

        for unit_type in [UnitType::Box, UnitType::Row, UnitType::Col] {
            if let Some(ret) = self.check_units(sgrid, NUM_CANDIDATES, unit_type) { return Some(ret) };
        }

        None
    }
}
impl<const NUM_CANDIDATES: usize> HiddenCandidatesSolver<NUM_CANDIDATES> {
    fn check_units(&self, sgrid: &SudokuGrid, combs: usize, unit_type: UnitType) -> Option<SolverResult> {
        for unit in SudokuGrid::get_all_units_from_unit_type(unit_type) {
            let unsolved_cells_in_unit: Vec<(usize, usize)> = unit.iter().filter(|&&(row, col)| sgrid.grid[row][col] == 0).cloned().collect();
            let mut candidate_appearance_count: HashMap<usize, i32> = HashMap::new();
            for &(row, col) in &unsolved_cells_in_unit {
                for &candidate in &sgrid.candidates[row][col] {
                    *(candidate_appearance_count.entry(candidate).or_insert(0)) += 1;
                }
            }

            // If a candidate appears more times than the combinations of cells we are checking for, it can't be a hidden candidate.
            let valid_candidates: Vec<usize> = candidate_appearance_count.keys().filter(|&x| candidate_appearance_count[x] <= combs as i32).cloned().collect();
            // If we have less candidates than the number of cells to check we don't have hidden candidates
            if valid_candidates.len() < combs { continue; }

            for candidates_combination in valid_candidates.iter().cloned().combinations(combs) {
                let has_candidate_from_candidates_combination = |&&(row, col): &&(usize, usize)| {
                    sgrid.candidates[row][col]
                        .iter()
                        .any(|candidate| candidates_combination.contains(candidate))
                };
                
                let hidden_candidate_candidate_cells: Vec<_> = unsolved_cells_in_unit
                    .iter()
                    .filter(has_candidate_from_candidates_combination)
                    .cloned()
                    .collect();

                // We need to have exactly combs number of cells which meet these requirements or
                // this isnt a hidden candidate.
                if hidden_candidate_candidate_cells.len() != combs { continue; }

                let mut reductions = Vec::new();
                let mut visualizer_updates = Vec::new();

                visualizer_updates.push(VisualizerUpdate::SetTitle(
                    format!("Hidden Candidates {}", match combs {
                    2 => "Pairs",
                    3 => "Triples",
                    4 => "Quads",
                    _ => "Unkown",
                    })
                ));

                visualizer_updates.push(
                    VisualizerUpdate::SetDescription(
                        format!(
                            "{1} cells within the same {2} contain the only occurances of the digits [{0}], forming a Hidden Candidate, the rest of the candidates of these cells can be removed.", 
                            candidates_combination.iter().map(|&v| v.to_string()).sorted().collect::<Vec<_>>().join(", "),
                            combs,
                            match unit_type {
                                UnitType::Box => "box",
                                UnitType::Row => "row",
                                UnitType::Col => "column",
                            },
                        )
                    )
                );

                for &(row, col) in &unit {
                    visualizer_updates.push(VisualizerUpdate::ColorCell(row, col, Colors::CELL_USED_TO_DETERMINE_SOLUTION));
                }

                for (row, col) in hidden_candidate_candidate_cells {
                    for num in 1..=9 {
                        if candidates_combination.contains(&num) { continue; }

                        if sgrid.candidates[row][col].contains(&num) {
                            for &val in &candidates_combination {
                                if sgrid.candidates[row][col].contains(&val) {
                                    visualizer_updates.push(VisualizerUpdate::ColorCandidate(row, col, val, Colors::DIGIT_USED_TO_DETERMINE_SOLUTION));
                                }
                            }
                            visualizer_updates.push(VisualizerUpdate::ColorCell(row, col, Colors::CELL_MARKED_FOR_CANDIDATE_REMOVEAL));
                            visualizer_updates.push(VisualizerUpdate::ColorCandidate(row, col, num, Colors::CANDIDATE_MARKED_FOR_REMOVAL));
                            reductions.push(SolverAction::CandidateReduction(row, col, num));
                        }
                    }
                }
                if !reductions.is_empty() { return Some((reductions, visualizer_updates)); }
            }
        }

        Default::default()
    }
}