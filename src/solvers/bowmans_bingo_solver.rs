use std::collections::HashMap;

use itertools::iproduct;
use raylib::prelude::Color;

use crate::{sudoku_grid::SudokuGrid, sudoku_visualizer_builder::Colors};

use super::sudoku_solver::{SolverAction, SolverResult, SudokuSolveMethod, VisualizerUpdate};

#[derive(PartialEq, Debug)]
enum CandidateState {
    Available,
    Forced,
    Disabled,
    Enforced,
}

pub struct BowmansBingoSolver;

impl SudokuSolveMethod for BowmansBingoSolver {
    fn apply(&self, sgrid: &SudokuGrid) -> Option<SolverResult> {
        // Step 1: Filter out any (row, col) pairs where sgrid.grid[row][col] != 0.
        let valid_positions: Vec<_> = iproduct!(0..9, 0..9)
            .filter(|&(row, col)| sgrid.grid[row][col] == 0)
            .collect();

        // Step 2: Sort the remaining pairs based on the length of sgrid.candidates[row][col].
        let mut sorted_positions = valid_positions.clone();
        sorted_positions.sort_by_key(|&(row, col)| sgrid.candidates[row][col].len());

        // Step 3: Create a list of all options for each (row, col) pair.
        let options: Vec<_> = sorted_positions
            .iter()
            .flat_map(|&(row, col)| {
                sgrid.candidates[row][col]
                    .iter()
                    .cloned()
                    .map(move |num| (row, col, num))
            })
            .collect();

        'nobingo: for (start_row, start_col, start_num) in options {
            let mut candidate_states: HashMap<(usize, usize, usize), CandidateState> =
                HashMap::new();
            let mut visualizer_updates = Vec::new();

            // Initialize all candidates to the Untested state
            for (row, col) in iproduct!(0..9, 0..9).filter(|&(row, col)| sgrid.grid[row][col] == 0)
            {
                for &num in &sgrid.candidates[row][col] {
                    candidate_states.insert((row, col, num), CandidateState::Available);
                }
            }
            // Place the starting candidate face down
            candidate_states.insert((start_row, start_col, start_num), CandidateState::Forced);

            visualizer_updates.push(VisualizerUpdate::ColorCell(
                start_row,
                start_col,
                Colors::CELL_MARKED_FOR_CANDIDATE_REMOVEAL,
            ));
            visualizer_updates.push(VisualizerUpdate::BackgroundCandidate(
                start_row,
                start_col,
                start_num,
                Colors::CHAIN_RED,
            ));

            // Main loop for Bowman's Bingo
            loop {
                let mut forced_candidates = candidate_states
                    .iter()
                    .filter(|(_, state)| **state == CandidateState::Forced);
                if let Some((&coords, _)) = forced_candidates.next() {
                    candidate_states.insert(coords, CandidateState::Enforced);

                    let (row, col, num) = coords;

                    for candidate in 1..=9 {
                        if candidate == num {
                            continue;
                        }
                        if candidate_states.contains_key(&(row, col, candidate)) {
                            candidate_states
                                .insert((row, col, candidate), CandidateState::Disabled);
                        }
                    }

                    for (seen_row, seen_col) in
                        SudokuGrid::generate_cells_seen_from_cord((row, col))
                    {
                        if (seen_row, seen_col) == (row, col) {
                            continue;
                        }
                        if candidate_states.contains_key(&(seen_row, seen_col, num)) {
                            candidate_states
                                .insert((seen_row, seen_col, num), CandidateState::Disabled);
                            visualizer_updates.push(VisualizerUpdate::ColorCandidate(
                                seen_row,
                                seen_col,
                                num,
                                Color::new(25, 25, 25, 100),
                            ));
                        }
                        let mut num_available = 0;
                        let mut last_cand = 0;
                        for candidate in 1..=9 {
                            if let Some(state) =
                                candidate_states.get(&(seen_row, seen_col, candidate))
                            {
                                if *state == CandidateState::Available {
                                    num_available += 1;
                                    last_cand = candidate;
                                }
                            }
                        }
                        if num_available == 1 {
                            candidate_states
                                .insert((seen_row, seen_col, last_cand), CandidateState::Forced);
                            visualizer_updates.push(VisualizerUpdate::CreateChain(
                                row,
                                col,
                                num,
                                seen_row,
                                seen_col,
                                last_cand,
                                Colors::CHAIN_COLOR,
                            ));
                        }
                    }
                } else {
                    continue 'nobingo;
                }

                // Check for contradictions
                for (&candidate_cell, _) in candidate_states.iter().filter(|(_, state)| {
                    **state == CandidateState::Enforced || **state == CandidateState::Forced
                }) {
                    let (row, col, num) = candidate_cell;
                    let cell = (row, col);

                    let peer_cells = SudokuGrid::generate_cells_seen_from_cord(cell);
                    let peer_cells_with_same_num = peer_cells
                        .iter()
                        .filter(|&&(row, col)| candidate_states.contains_key(&(row, col, num)))
                        .filter(|&&(row, col)| {
                            if let Some(state) = candidate_states.get(&(row, col, num)) {
                                *state == CandidateState::Enforced
                                    || *state == CandidateState::Forced
                            } else {
                                false
                            }
                        })
                        .collect::<Vec<_>>();

                    if peer_cells_with_same_num.len() > 1 {
                        for &(row, col) in peer_cells_with_same_num {
                            visualizer_updates.push(VisualizerUpdate::ColorCell(
                                row,
                                col,
                                Colors::CELL_USED_TO_DETERMINE_SOLUTION,
                            ));
                        }
                        // Contradiction found
                        visualizer_updates
                            .push(VisualizerUpdate::SetTitle("Bowmans Bingo".to_string()));
                        visualizer_updates.push(VisualizerUpdate::ColorCandidate(
                            start_row,
                            start_col,
                            start_num,
                            Colors::CANDIDATE_MARKED_FOR_REMOVAL,
                        ));

                        return Some((
                            vec![SolverAction::CandidateReduction(
                                start_row, start_col, start_num,
                            )],
                            visualizer_updates,
                        ));
                    }
                }
            }
        }
        None
    }
}
