use super::sudoku_solver::*;
use super::super::sudoku_grid::*;

use std::collections::HashSet;
use itertools::Itertools;

pub struct NakedCandidatesSolver;

impl SudokuSolveMethod for NakedCandidatesSolver {
    fn apply(&self, sgrid: &mut SudokuGrid) -> bool {
        let mut applied = false;

        // for Pairs, Triples, and Quads
        // Given some set of cells within a unit determine if there are any NakedCandidates
        fn determine_candidates(sgrid: &mut SudokuGrid, cells: &[(usize, usize)], removal: Vec<(usize, usize)>) -> bool {
            let mut applied = false;

            let mut candidates_set: HashSet<u8> = HashSet::new();
            for &(row, col) in cells {
                for &candidate in &sgrid.candidates[row][col] {
                    candidates_set.insert(candidate);
                }
            }
            if candidates_set.len() == cells.len() {
                let mut print_once = true;

                for (i,j) in removal {
                    if cells.contains(&(i, j)) {
                        continue;
                    }
                    for &candidate in &candidates_set {
                        if sgrid.candidates[i][j].remove(&candidate) {
                            applied = true;
                            if print_once {
                                println!("Solver [NakedCandidatesSolver] found candidate set {:?} at ({:?})", candidates_set, cells);
                                print_once = false;
                            }
                            println!("Solver [NakedCandidatesSolver] found candidate reduction {} at ({},{})", candidate, i, j);
                        }
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
                        applied |= determine_candidates(sgrid, &comb.into_iter().map(|&x| x).collect::<Vec<_>>()[..], cells.clone());
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
                    applied |= determine_candidates(sgrid, &comb.into_iter().map(|&x| x).collect::<Vec<_>>()[..], cells.clone());
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
                    applied |= determine_candidates(sgrid, &comb.into_iter().map(|&x| x).collect::<Vec<_>>()[..], cells.clone());
                }
            }
        }

        applied
    }
}
