use crate::sudoku_visualizer_builder::Colors;

use super::sudoku_solver::*;
use super::super::sudoku_grid::*;

use std::collections::HashSet;
use itertools::Itertools;

pub struct SwordfishSolver;

impl SudokuSolveMethod for SwordfishSolver {
    fn apply(&self, sgrid: &SudokuGrid) -> Option<SolverResult> {
        for unit_type in [UnitType::Row, UnitType::Col] {
            if let Some(ret) = SwordfishSolver::apply_fish_on_axis(sgrid, unit_type, 3) { return Some(ret) };
        }
        None
    }
}

impl SwordfishSolver {
    pub fn apply_fish_on_axis(sgrid: &SudokuGrid, unit_type: UnitType, fish_size: usize) -> Option<SolverResult> {
        assert!(fish_size == 3 || fish_size == 4, "Swordfish is fish_size=3 and Jellyfish if fish_size=4. No other types supported.");
        for num in 1..=9 {
            let mut candidate_positions: [Vec<(usize, usize)>; 9] = Default::default();
    
            for unit in SudokuGrid::get_all_units_from_unit_type(unit_type) {
                for (row, col) in unit {
                    if sgrid.candidates[row][col].contains(&num) {
                        match unit_type {
                            UnitType::Row => { candidate_positions[row].push((row, col)); },
                            UnitType::Col => { candidate_positions[col].push((row, col)); },
                            UnitType::Box => panic!("Box unit type does not make sense for a fish"),
                        }
                    }
                }
            }
    
            let possible_cells_for_fish: Vec<Vec<(usize, usize)>> = candidate_positions
                .iter()
                .filter(|&x| x.len() >= 2 && x.len() <= fish_size)
                .cloned()
                .collect();
    
            for unit_set in possible_cells_for_fish.iter().clone().combinations(fish_size) {
                let mut axis_counter = HashSet::new();
                for &vec in &unit_set {
                    for &(row, col) in vec {
                        match unit_type {
                            UnitType::Row => { axis_counter.insert(col); },
                            UnitType::Col => { axis_counter.insert(row); },
                            UnitType::Box => panic!("Box unit type does not make sense for a fish"),
                        }
                    }
                }
    
                if axis_counter.len() != fish_size { continue; }
    
                // We have a fish, check for candidate reductions.
                let mut visualizer_updates = Vec::new();
                let mut reductions = Vec::new();
                visualizer_updates.push(VisualizerUpdate::SetTitle(match fish_size {
                    3 => "Swordfish".to_string(),
                    4 => "Jellyfish".to_string(),
                    _ => panic!("Unsupported fish solver amount"),
                }));
    
                for cell_set in &unit_set {
                    for (row, col) in SudokuGrid::get_cells_in_unit_from(unit_type, cell_set[0]) {
                        visualizer_updates.push(VisualizerUpdate::ColorCell(row, col, Colors::CELL_USED_TO_DETERMINE_SOLUTION));
                        if sgrid.candidates[row][col].contains(&num) {
                            visualizer_updates.push(VisualizerUpdate::ColorCandidate(row, col, num, Colors::DIGIT_USED_TO_DETERMINE_SOLUTION));
                        }
                    }
                }
    
                for &cell_set in &unit_set {
                    for &cell_in_fish in cell_set {
                        for (row, col) in SudokuGrid::get_cells_in_unit_from(match unit_type {
                            UnitType::Row => UnitType::Col,
                            UnitType::Col => UnitType::Row,
                            UnitType::Box => panic!("Box unit type does not make sense for a fish"),
                        }, cell_in_fish) {
                            // Check if the cell is part of the rows or columns used by the fish
                            match unit_type {
                                UnitType::Row => {
                                    if unit_set.iter().any(|set| set[0].0 == row) { continue; }
                                },
                                UnitType::Col => {
                                    if unit_set.iter().any(|set| set[0].1 == col) { continue; }
                                },
                                _ => {}
                            }
                    
                            visualizer_updates.push(VisualizerUpdate::ColorCell(row, col, Colors::CELL_MARKED_FOR_CANDIDATE_REMOVEAL));
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
