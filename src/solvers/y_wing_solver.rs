use core::num;
use std::collections::HashSet;
use std::hash::Hash;

use super::sudoku_solver::*;
use super::super::sudoku_grid::*;

use itertools::Itertools;

pub struct YWingSolver;

impl SudokuSolveMethod for YWingSolver {
    fn apply(&self, sgrid: &mut SudokuGrid) -> bool {
        let mut applied = false;

        // Find all possible y-wing hinges and wings
        let cells_with_two_candidates: Vec<(usize, usize)> = (0..9).flat_map(|row| (0..9).map(move |col| (row, col)))
            .filter(|&(row, col)| sgrid.candidates[row][col].len() == 2)
            .collect();

        for hinge in &cells_with_two_candidates {
            // Find all possible wings
            let mut possible_wings = Vec::new();
            for possible_wing in &cells_with_two_candidates {
                // Wings are valid if they are in the same row, col, or box
                if possible_wing.0 == hinge.0 { possible_wings.push(possible_wing); }
                else if possible_wing.1 == hinge.1 { possible_wings.push(possible_wing); }
                else if possible_wing.0/3 == hinge.0/3 && possible_wing.1/3 == hinge.1/3 { possible_wings.push(possible_wing); }
            }

            // For pairs of wings check if they work
            for wings in possible_wings.iter().combinations(2) {
                // The wings can't see each other to be valid
                if wings[0].0 == wings[1].0 { continue; }
                else if wings[0].1 == wings[1].1 { continue; }
                else if wings[0].0/3 == wings[1].0/3 && wings[0].1/3 == wings[1].1/3 { continue; }

                // The wings also must have 3 total distinct candidates where Wings share 1
                // TODO(JBadges): This if statement is so gross, there has to be a better way to do this
                if sgrid.candidates[wings[0].0][wings[0].1]
                    .union(&sgrid.candidates[wings[1].0][wings[1].1])
                    .cloned()
                    .collect::<HashSet<u8>>()
                    .union(&sgrid.candidates[hinge.0][hinge.1])
                    .collect::<Vec<_>>()
                    .len() != 3 {
                    continue;
                }

                let candidate_intersection: Vec<_> = sgrid.candidates[wings[0].0][wings[0].1]
                    .intersection(&sgrid.candidates[wings[1].0][wings[1].1])
                    .cloned()
                    .collect();

                if candidate_intersection.len() != 1 {
                    continue;
                }
                let num_to_remove = candidate_intersection[0];

                // We can remove the shared candidate between the wings
                // in all cells where the wings intersect
                fn generate_cells_seen_from_cord(cord: (usize, usize)) -> HashSet<(usize, usize)> {
                    let (row, col) = cord;
                    let mut cells = HashSet::new();
                    for i in 0..9 {
                        cells.insert((i, col));
                        cells.insert((row, i));
                    }
                    for box_row in 3*(row/3)..3*(row/3)+3 {
                        for box_col in 3*(col/3)..3*(col/3)+3 {
                            cells.insert((box_row, box_col));
                        }
                    }
                    cells
                }

                let cells_seen_from_wing1 = generate_cells_seen_from_cord(**wings[0]);
                let cells_seen_from_wing2 = generate_cells_seen_from_cord(**wings[1]);

                let mut first_print = true;

                let shared_cells = cells_seen_from_wing1.intersection(&cells_seen_from_wing2);
                for cell in shared_cells {
                    if *cell == *hinge || *cell == **wings[0] || *cell == **wings[1] { continue; }
                    let (row, col) = cell;
                    if sgrid.candidates[*row][*col].remove(&num_to_remove) {
                        if first_print {
                            println!("Solver [YWingSolver] hinge {:?} wings ({:?})", hinge, wings);
                            first_print = false;
                        }
                        println!("Solver [YWingSolver] removed value {} from candidate location ({}, {})", num_to_remove, row, col);
                        applied = true;
                    }
                }
            }
        }

        applied
    }
}
