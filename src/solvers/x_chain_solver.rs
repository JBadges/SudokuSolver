use itertools::Itertools;

use crate::{sudoku_grid::SudokuGrid, sudoku_visualizer_builder::Colors};

use super::sudoku_solver::{SolverAction::*, SolverResult, SudokuSolveMethod, VisualizerUpdate::*};

pub struct XChainSolver;

// An X-Chain constructs a chain of Weak & Strong links where there can never be two connected
// Weak links and it must start and end on a strong link.

impl SudokuSolveMethod for XChainSolver {
    fn apply(&self, sgrid: &SudokuGrid) -> Option<SolverResult> {
        for chain_length in (3..=9).step_by(2) {
            for num in 1..=9 {
                let conjugate_pairs = sgrid.get_conjugate_pairs(num).to_unique_pairs();

                for pair in &conjugate_pairs {
                    for conjugate_pairs_arrangement in &conjugate_pairs
                        .iter()
                        .cloned()
                        .permutations(conjugate_pairs.len())
                        .collect::<Vec<Vec<((usize, usize, usize), (usize, usize, usize))>>>()
                    {
                        let mut chain = vec![pair.0, pair.1];
                        let mut link_is_strong = vec![true];

                        while chain.len() < chain_length {
                            let mut extended = false;
                            let start_of_chain = chain.first().unwrap();
                            let end_of_chain = chain.last().unwrap();

                            // Extract row and col from the tuple
                            let start_coords = (start_of_chain.0, start_of_chain.1);
                            let end_coords = (end_of_chain.0, end_of_chain.1);

                            // Check if the start of the chain can see another end of a conjugate pair
                            for other_pair in conjugate_pairs_arrangement {
                                if chain.contains(&other_pair.0) && chain.contains(&other_pair.1) {
                                    continue;
                                }
                                // Strong->Strong
                                if chain.last().unwrap() == &other_pair.0 {
                                    chain.push(other_pair.1);
                                    link_is_strong.push(true);
                                    extended = true;
                                    break;
                                } else if chain.last().unwrap() == &other_pair.1 {
                                    chain.push(other_pair.0);
                                    link_is_strong.push(true);
                                    extended = true;
                                    break;
                                } else if chain.first().unwrap() == &other_pair.0 {
                                    chain.insert(0, other_pair.1);
                                    link_is_strong.push(true);
                                    extended = true;
                                    break;
                                } else if chain.first().unwrap() == &other_pair.1 {
                                    chain.insert(0, other_pair.0);
                                    link_is_strong.push(true);
                                    extended = true;
                                    break;
                                }

                                // Strong->Weak->Strong
                                let other_pair_start_coords = (other_pair.0 .0, other_pair.0 .1);
                                let other_pair_end_coords = (other_pair.1 .0, other_pair.1 .1);

                                if SudokuGrid::cells_see_each_other(
                                    start_coords,
                                    other_pair_start_coords,
                                ) {
                                    chain.insert(0, other_pair.0);
                                    chain.insert(0, other_pair.1);
                                    link_is_strong.push(false);
                                    link_is_strong.push(true);
                                    extended = true;
                                    break;
                                } else if SudokuGrid::cells_see_each_other(
                                    start_coords,
                                    other_pair_end_coords,
                                ) {
                                    chain.insert(0, other_pair.1);
                                    chain.insert(0, other_pair.0);
                                    link_is_strong.push(false);
                                    link_is_strong.push(true);
                                    extended = true;
                                    break;
                                }

                                if SudokuGrid::cells_see_each_other(
                                    end_coords,
                                    other_pair_start_coords,
                                ) {
                                    chain.push(other_pair.0);
                                    chain.push(other_pair.1);
                                    link_is_strong.push(false);
                                    link_is_strong.push(true);
                                    extended = true;
                                    break;
                                } else if SudokuGrid::cells_see_each_other(
                                    end_coords,
                                    other_pair_end_coords,
                                ) {
                                    chain.push(other_pair.1);
                                    chain.push(other_pair.0);
                                    link_is_strong.push(false);
                                    link_is_strong.push(true);
                                    extended = true;
                                    break;
                                }
                            }

                            if !extended {
                                break;
                            }
                        }

                        if chain.len() != chain_length + 1 {
                            continue;
                        }

                        // An x_chain works if the two ends of the chain can see cells of each other
                        let first = chain.first().unwrap();
                        let last = chain.last().unwrap();
                        let cells_to_check: Vec<(usize, usize)> =
                            SudokuGrid::generate_cells_seen_from_cord((first.0, first.1))
                                .intersection(&SudokuGrid::generate_cells_seen_from_cord((
                                    last.0, last.1,
                                )))
                                .cloned()
                                .collect();

                        let mut viualizer_updates = Vec::new();
                        viualizer_updates
                            .push(SetTitle(format!("X-Chain length {}", chain_length)));
                        let mut reductions = Vec::new();
                        for (row, col) in cells_to_check {
                            if chain.contains(&(row, col, num)) {
                                continue;
                            }
                            viualizer_updates.push(ColorCell(
                                row,
                                col,
                                Colors::CELL_MARKED_FOR_CANDIDATE_REMOVEAL,
                            ));
                            if sgrid.candidates[row][col].contains(&num) {
                                reductions.push(CandidateReduction(row, col, num));
                                viualizer_updates.push(ColorCandidate(
                                    row,
                                    col,
                                    num,
                                    Colors::CANDIDATE_MARKED_FOR_REMOVAL,
                                ));
                            }
                        }

                        for i in 0..chain.len() {
                            let candidate_color = match i % 2 == 0 {
                                true => Colors::CHAIN_BLUE,
                                false => Colors::CHAIN_RED,
                            };
                            viualizer_updates.push(BackgroundCandidate(
                                chain[i].0,
                                chain[i].1,
                                num,
                                candidate_color,
                            ));
                            if i != chain.len() - 1 {
                                let chain_color = match link_is_strong[i] {
                                    true => Colors::CHAIN_STRONG,
                                    false => Colors::CHAIN_WEAK,
                                };
                                viualizer_updates.push(CreateChain(
                                    chain[i].0,
                                    chain[i].1,
                                    num,
                                    chain[i + 1].0,
                                    chain[i + 1].1,
                                    num,
                                    chain_color,
                                ));
                            }
                        }

                        if !reductions.is_empty() {
                            println!("{:?}", chain);
                            return Some((reductions, viualizer_updates));
                        }
                    }
                }
            }
        }
        None
    }
}
