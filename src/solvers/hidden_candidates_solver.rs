use super::sudoku_solver::*;
use super::super::sudoku_grid::*;

use std::collections::HashSet;
use itertools::Itertools;

pub struct HiddenCandidatesSolver;

impl SudokuSolveMethod for HiddenCandidatesSolver {
    fn apply(&self, sgrid: &mut SudokuGrid) -> bool {
        let mut applied = false;

        fn determine_hidden_candidates(sgrid: &mut SudokuGrid, cells: &[(usize, usize)], removal: Vec<(usize, usize)>) -> bool {
            let mut applied = false;
            let mut candidates_set: HashSet<u8> = HashSet::new();
            for &(row, col) in cells {
                for &candidate in &sgrid.candidates[row][col] {
                    candidates_set.insert(candidate);
                }
            }
    'outer: for candidate_comb in candidates_set.iter().combinations(cells.len()) {
                for &(row, col) in &removal {
                    if cells.contains(&(row, col)) {
                        continue;
                    }
                    for val in &candidate_comb {
                        if sgrid.candidates[row][col].contains(val) {
                            continue 'outer;
                        }
                    }
                }
                let candidate_comb_u8: HashSet<u8> = candidate_comb.iter().map(|&&x| x).collect();
                let mut changes_made = false;
                let mut removals: Vec<((usize, usize), Vec<u8>)> = Vec::new();

                for (i, j) in cells.iter() {
                    let cell_candidates = &mut sgrid.candidates[*i][*j];
                    let original_candidates: HashSet<u8> = cell_candidates.clone();
                    cell_candidates.retain(|&x| candidate_comb_u8.contains(&x));

                    if original_candidates.len() != cell_candidates.len() {
                        applied = true;
                        changes_made = true;
                        let removed_values: Vec<u8> = original_candidates.difference(&cell_candidates).cloned().collect();
                        removals.push(((*i, *j), removed_values));
                    }
                }

                if changes_made {
                    println!("Solver [HiddenCandidatesSolver] found candidate set {:?} at ({:?})", candidate_comb, cells);
                    for ((i, j), removed_values) in removals {
                        println!("Removed values {:?} from candidate location ({}, {})", removed_values, i, j);
                    }
                }

            }
            applied
        }

        // Box
        for i in (0..9).step_by(3) {
            for j in (0..9).step_by(3)  {
                let cells: Vec<(usize, usize)> = (i..i+3)
                    .flat_map(|i| (j..j+3).map(move |j| (i, j)))
                    .filter(|&(row, col)| sgrid.grid[row][col] == 0)
                    .collect();

                for combs in 2..=4 {
                    for comb in cells.iter().combinations(combs) {
                        applied |= determine_hidden_candidates(sgrid, &comb.into_iter().map(|&x| x).collect::<Vec<_>>()[..], cells.clone());
                    }
                }
            }
        }

        // Row
        for row in 0..9 {
            let cells: Vec<(usize, usize)> = (0..9).map(|col| (row, col))
                .filter(|&(row, col)| sgrid.grid[row][col] == 0)
                .collect();

            for combs in 2..=4 {
                for comb in cells.iter().combinations(combs) {
                    applied |= determine_hidden_candidates(sgrid, &comb.into_iter().map(|&x| x).collect::<Vec<_>>()[..], cells.clone());
                }
            }
        }

        // Column
        for col in 0..9 {
            let cells: Vec<(usize, usize)> = (0..9).map(|row| (row, col))
                .filter(|&(row, col)| sgrid.grid[row][col] == 0)
                .collect();
            for combs in 2..=4 {
                for comb in cells.iter().combinations(combs) {
                    applied |= determine_hidden_candidates(sgrid, &comb.into_iter().map(|&x| x).collect::<Vec<_>>()[..], cells.clone());
                }
            }
        }

        applied
    }
}
