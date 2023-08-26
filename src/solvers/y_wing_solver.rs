use std::collections::HashSet;

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

                // Extract the candidates for the hinge and the two wings
                let hinge_candidates = &sgrid.candidates[hinge.0][hinge.1];
                let wing1_candidates = &sgrid.candidates[wings[0].0][wings[0].1];
                let wing2_candidates = &sgrid.candidates[wings[1].0][wings[1].1];
                
                // Its possible that even though these were all generated as len==2
                // that in a previous pass the candidates were reduced to 1
                if hinge_candidates.len() != 2 || 
                    wing1_candidates.len() != 2 || 
                    wing2_candidates.len() != 2  { continue; }

                // Extract the two candidates from the hinge
                let hinge_values: Vec<_> = hinge_candidates.iter().cloned().collect();
                let (a, b) = (hinge_values[0], hinge_values[1]);

                // Check if wing1 has A and another candidate (not B)
                let c_from_wing1 = if wing1_candidates.contains(&a) {
                    wing1_candidates.iter().find(|&&x| x != a).cloned()
                } else {
                    None
                };

                // Check if wing2 has B and another candidate (not A)
                let c_from_wing2 = if wing2_candidates.contains(&b) {
                    wing2_candidates.iter().find(|&&x| x != b).cloned()
                } else {
                    None
                };

                // Check if both wings have identified the same C and it's not None
                let num_to_remove = match (c_from_wing1, c_from_wing2) {
                    (Some(c1), Some(c2)) if c1 == c2 => c1,
                    _ => continue,
                };

                // We can remove the shared candidate between the wings
                // in all cells where the wings intersect
                let cells_seen_from_wing1 = SudokuGrid::generate_cells_seen_from_cord(**wings[0]);
                let cells_seen_from_wing2 = SudokuGrid::generate_cells_seen_from_cord(**wings[1]);

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
